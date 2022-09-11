use super::*;

use sp_std::prelude::*;
use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use orml_nft::Module as NftModule;
use crate::Module as ParticlesModule;

benchmarks! {
	create {
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	transfer {
		let caller = whitelisted_caller();
		let to = account("to", 0, 0);

		let particle_id = NftModule::<T>::mint(&caller, Module::<T>::class_id(), Vec::new(), Particle(Default::default()))?;

	}: _(RawOrigin::Signed(caller), to, particle_id)

	set_price {
		let caller = whitelisted_caller();

		let particle_id = NftModule::<T>::mint(&caller, Module::<T>::class_id(), Vec::new(), Particle(Default::default()))?;

	}: _(RawOrigin::Signed(caller), particle_id, Some(100u32.into()))

	buy {
		let caller = whitelisted_caller();
		let seller = account("seller", 0, 0);

		let _ = T::Currency::make_free_balance_be(&caller, 1000u32.into());

		let particle_id = NftModule::<T>::mint(&seller, Module::<T>::class_id(), Vec::new(), Particle(Default::default()))?;
		ParticlesModule::<T>::set_price(RawOrigin::Signed(seller.clone()).into(), particle_id, Some(500u32.into()))?;

	}: _(RawOrigin::Signed(caller), seller, particle_id, 500u32.into())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create::<Test>());
			assert_ok!(test_benchmark_transfer::<Test>());
			assert_ok!(test_benchmark_set_price::<Test>());
			assert_ok!(test_benchmark_buy::<Test>());
		});
	}
}
