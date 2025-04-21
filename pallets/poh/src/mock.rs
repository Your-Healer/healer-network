use crate as pallet_poh;

use frame_support::{
	pallet_prelude::*,
	derive_impl,
	traits::Time,
};

use sp_runtime::{
	BuildStorage,
	traits::BlakeTwo256,
};

use sp_core::{Hasher, H256};

type Block = frame_system::mocking::MockBlock<Test>;

// type Moment = <frame_support::traits::Time>::Moment;

#[frame_support::runtime]
mod runtime {
	// The main runtime
	#[runtime::runtime]
	// Runtime Types to be generated
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Test;

	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<Test>;

	#[runtime::pallet_index(1)]
	pub type PoH = pallet_poh::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

// Mock time implementation
pub struct MockTime;

impl Time for MockTime {
	type Moment = u32;
	
	fn now() -> Self::Moment {
		0 // Return a constant value for testing
	}
}

impl pallet_poh::Config for Test {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = ();

	/// The hash function used for proof generation
	type Hasher = BlakeTwo256;
		
	/// Time interface
	type Time = MockTime;
	// type Time = ();
	
	/// Hash type used for the proof
	type Hash = H256;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
