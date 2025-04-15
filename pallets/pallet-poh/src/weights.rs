// Executed Command:
// ../../target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_template
// --extrinsic
// *
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --output
// pallets/template/src/weights.rs
// --template
// ../../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame::deps::{frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}}, frame_system};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
    fn submit_data() -> Weight;
    fn verify_proof() -> Weight;
    fn get_proof_by_data() -> Weight;
}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn submit_data() -> Weight {
        Weight::from_parts(10_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn verify_proof() -> Weight {
        Weight::from_parts(5_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn get_proof_by_data() -> Weight {
        Weight::from_parts(5_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn submit_data() -> Weight {
        Weight::from_parts(10_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn verify_proof() -> Weight {
        Weight::from_parts(5_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn get_proof_by_data() -> Weight {
        Weight::from_parts(5_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}