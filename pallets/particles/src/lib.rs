#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, ensure, StorageValue,
	traits::{Randomness, Currency, ExistenceRequirement}, RuntimeDebug, dispatch::DispatchResult,
};
use sp_io::hashing::blake2_128;
use frame_system::{ensure_signed, offchain::{SendTransactionTypes}};
use sp_std::{vec::Vec};
use orml_utilities::with_transaction_result;
use orml_nft::Module as NftModule;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;

pub use weights::WeightInfo;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Particle {
	pub state: [u8; 16],
}

pub trait Config: orml_nft::Config<TokenData = Particle, ClassData = ()> + SendTransactionTypes<Call<Self>> {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Randomness: Randomness<Self::Hash>;
	type Currency: Currency<Self::AccountId>;
	type WeightInfo: WeightInfo;
}

type ParticleIndexOf<T> = <T as orml_nft::Config>::TokenId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


decl_storage! {
	trait Store for Module<T: Config> as Particles {
		/// Get particle price. None means not for sale.
		pub ParticlePrices get(fn particle_prices): map hasher(blake2_128_concat) ParticleIndexOf<T> => Option<BalanceOf<T>>;
		/// The class id for orml_nft
		pub ClassId get(fn class_id): T::ClassId;
	}
	add_extra_genesis {
		build(|_config| {
			// create a NTF class
			let class_id = NftModule::<T>::create_class(&Default::default(), Vec::new(), ()).expect("Cannot fail or invalid chain spec");
			ClassId::<T>::put(class_id);
		})
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Config>::AccountId,
		ParticleIndex = ParticleIndexOf<T>,
		Balance = BalanceOf<T>,
	{
		/// A particle is created. \[owner, particle_id, particle\]
		ParticleCreated(AccountId, ParticleIndex, Particle),
		/// A particle is transferred. \[from, to, particle_id\]
		ParticleTransferred(AccountId, AccountId, ParticleIndex),
		/// The price for a particle is updated. \[owner, particle_id, price\]
		ParticlePriceUpdated(AccountId, ParticleIndex, Option<Balance>),
		/// A particle is sold. \[old_owner, new_owner, particle_id, price\]
		ParticleSold(AccountId, AccountId, ParticleIndex, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		InvalidParticleId,
		NotOwner,
		NotForSale,
		PriceTooLow,
		BuyFromSelf,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new particle
		#[weight = T::WeightInfo::create()]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			let hash = Self::random_value(&sender);

			// Create and store particle
			let particle = Particle {
				state: hash,
			};
			let particle_id = NftModule::<T>::mint(&sender, Self::class_id(), Vec::new(), particle.clone())?;

			// Emit event
			Self::deposit_event(RawEvent::ParticleCreated(sender, particle_id, particle))
		}

		/// Transfer a particle to new owner
		#[weight = T::WeightInfo::transfer()]
		pub fn transfer(origin, to: T::AccountId, particle_id: ParticleIndexOf<T>) {
			let sender = ensure_signed(origin)?;

			NftModule::<T>::transfer(&sender, &to, (Self::class_id(), particle_id))?;

			if sender != to {
				ParticlePrices::<T>::remove(particle_id);
				Self::deposit_event(RawEvent::ParticleTransferred(sender, to, particle_id));
			}
		}

		/// Set a price for a particle for sale
		/// None to delist the particle
		#[weight = T::WeightInfo::set_price()]
		pub fn set_price(origin, particle_id: ParticleIndexOf<T>, new_price: Option<BalanceOf<T>>) {
			let sender = ensure_signed(origin)?;

			ensure!(orml_nft::TokensByOwner::<T>::contains_key(&sender, (Self::class_id(), particle_id)), Error::<T>::NotOwner);

			ParticlePrices::<T>::mutate_exists(particle_id, |price| *price = new_price);

			Self::deposit_event(RawEvent::ParticlePriceUpdated(sender, particle_id, new_price));
		}

		/// Buy a particle
		#[weight = T::WeightInfo::buy()]
		pub fn buy(origin, owner: T::AccountId, particle_id: ParticleIndexOf<T>, max_price: BalanceOf<T>) {
			let sender = ensure_signed(origin)?;

			ensure!(sender != owner, Error::<T>::BuyFromSelf);

			ParticlePrices::<T>::try_mutate_exists(particle_id, |price| -> DispatchResult {
				let price = price.take().ok_or(Error::<T>::NotForSale)?;

				ensure!(max_price >= price, Error::<T>::PriceTooLow);

				with_transaction_result(|| {
					NftModule::<T>::transfer(&owner, &sender, (Self::class_id(), particle_id))?;
					T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::KeepAlive)?;

					Self::deposit_event(RawEvent::ParticleSold(owner, sender, particle_id, price));

					Ok(())
				})
			})?;
		}
	}
}

impl<T: Config> Module<T> {
	#[allow(dead_code)] // used in tests and probably here in the future
	fn particles(owner: &T::AccountId, particle_id: ParticleIndexOf<T>) -> Option<Particle> {
		NftModule::<T>::tokens(Self::class_id(), particle_id).and_then(|x| {
			if x.owner == *owner {
				Some(x.data)
			} else {
				None
			}
		})
	}

	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}
}
