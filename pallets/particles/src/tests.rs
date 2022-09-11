use super::*;

use crate as particles;
use std::cell::RefCell;
use sp_core::H256;
use frame_support::{
    parameter_types, assert_ok, assert_noop,
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
	testing::TestXt,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		NFT: orml_nft::{Module, Storage},
		ParticlesModule: particles::{Module, Call, Storage, Event<T>, Config},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl orml_nft::Config for Test {
    type ClassId = u32;
	type TokenId = u32;
	type ClassData = ();
	type TokenData = Particle;
}

thread_local! {
    static RANDOM_PAYLOAD: RefCell<H256> = RefCell::new(Default::default());
}

pub struct MockRandom;

impl Randomness<H256> for MockRandom {
    fn random(_subject: &[u8]) -> H256 {
        RANDOM_PAYLOAD.with(|v| *v.borrow())
    }
}

parameter_types! {
	pub const DefaultDifficulty: u32 = 3;
}

impl Config for Test {
    type Event = Event;
    type Randomness = MockRandom;
    type Currency = Balances;
    type WeightInfo = ();
}

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;

impl<LocalCall> SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

    pallet_balances::GenesisConfig::<Test>{
		balances: vec![(200, 500)],
    }.assimilate_storage(&mut t).unwrap();

    crate::GenesisConfig::default().assimilate_storage::<Test>(&mut t).unwrap();

    let mut t: sp_io::TestExternalities = t.into();

    t.execute_with(|| System::set_block_number(1) );
    t
}

fn last_event() -> Event {
    System::events().last().unwrap().event.clone()
}

#[test]
fn can_create() {
    new_test_ext().execute_with(|| {
        assert_ok!(ParticlesModule::create(Origin::signed(100)));
        assert_eq!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 100);

        let particle: Particle = Particle {
            state: NFT::tokens(ParticlesModule::class_id(), 0).unwrap().data.state,
        };

        assert_eq!(ParticlesModule::particles(&100, 0), Some(particle.clone()));
        assert_eq!(last_event(), Event::particles(crate::Event::<Test>::ParticleCreated(100, 0, particle)));

        // negative check
        let another_particle: Particle = Particle {
            state: [0u8; 16],
        };
        assert_ne!(ParticlesModule::particles(&100, 0), Some(another_particle.clone()));
        assert_ne!(last_event(), Event::particles(crate::Event::<Test>::ParticleCreated(100, 0, another_particle)));
    });
}


#[test]
fn can_transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(ParticlesModule::create(Origin::signed(100)));
        assert_eq!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 100);

        // check non-owners cannot transfer asset
        assert_noop!(ParticlesModule::transfer(Origin::signed(101), 200, 0), orml_nft::Error::<Test>::NoPermission);

        // transfer asset and check new owner
        assert_ok!(ParticlesModule::transfer(Origin::signed(100), 200, 0));
        assert_eq!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 200);
        assert_eq!(last_event(), Event::particles(RawEvent::ParticleTransferred(100, 200, 0)));

        // check previous owner has no permission
        assert_noop!(ParticlesModule::transfer(Origin::signed(100), 300, 0), orml_nft::Error::<Test>::NoPermission);
    });
}

#[test]
fn can_handle_self_transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(ParticlesModule::create(Origin::signed(100)));
        System::reset_events();

        // only one particle has been created
        assert_noop!(ParticlesModule::transfer(Origin::signed(100), 100, 1), orml_nft::Error::<Test>::TokenNotFound);

        // self transfer
        assert_ok!(ParticlesModule::transfer(Origin::signed(100), 100, 0));
        assert_eq!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 100);

        // no transfer event because no actual transfer is executed
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn cannot_transfer_unowned_particles() {
    new_test_ext().execute_with(|| {
        assert_ok!(ParticlesModule::create(Origin::signed(100)));

        assert_eq!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 100);
        assert_ne!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 200);

        assert_noop!(ParticlesModule::transfer(Origin::signed(200), 300, 0), orml_nft::Error::<Test>::NoPermission);
    });
}

#[test]
fn can_set_price() {
    new_test_ext().execute_with(|| {
        assert_ok!(ParticlesModule::create(Origin::signed(100)));

        assert_noop!(ParticlesModule::set_price(Origin::signed(200), 0, Some(10)), Error::<Test>::NotOwner);

        assert_ok!(ParticlesModule::set_price(Origin::signed(100), 0, Some(10)));

        assert_eq!(last_event(), Event::particles(RawEvent::ParticlePriceUpdated(100, 0, Some(10))));

        assert_eq!(ParticlesModule::particle_prices(0), Some(10));

        assert_ok!(ParticlesModule::set_price(Origin::signed(100), 0, None));
        assert_eq!(ParticlePrices::<Test>::contains_key(0), false);

        assert_eq!(last_event(), Event::particles(RawEvent::ParticlePriceUpdated(100, 0, None)));
    });
}

#[test]
fn can_buy() {
    new_test_ext().execute_with(|| {
        assert_ok!(ParticlesModule::create(Origin::signed(100)));

        assert_noop!(ParticlesModule::buy(Origin::signed(100), 100, 0, 10), Error::<Test>::BuyFromSelf);
        assert_noop!(ParticlesModule::buy(Origin::signed(200), 100, 1, 10), Error::<Test>::NotForSale);
        assert_noop!(ParticlesModule::buy(Origin::signed(200), 100, 0, 10), Error::<Test>::NotForSale);

        assert_ok!(ParticlesModule::set_price(Origin::signed(100), 0, Some(600)));

        assert_noop!(ParticlesModule::buy(Origin::signed(200), 100, 0, 500), Error::<Test>::PriceTooLow);

        assert_noop!(ParticlesModule::buy(Origin::signed(200), 100, 0, 600), pallet_balances::Error::<Test, _>::InsufficientBalance);

        assert_ok!(ParticlesModule::set_price(Origin::signed(100), 0, Some(400)));

        assert_ok!(ParticlesModule::buy(Origin::signed(200), 100, 0, 500));

        assert_eq!(ParticlePrices::<Test>::contains_key(0), false);
        assert_eq!(NFT::tokens(ParticlesModule::class_id(), 0).unwrap().owner, 200);
        assert_eq!(Balances::free_balance(100), 400);
        assert_eq!(Balances::free_balance(200), 100);

        assert_eq!(last_event(), Event::particles(RawEvent::ParticleSold(100, 200, 0, 400)));
    });
}
