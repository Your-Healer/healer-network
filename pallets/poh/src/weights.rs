#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use polkadot_sdk::polkadot_sdk_frame::deps::{
    frame_support::{
        traits::Get,
        weights::{
            Weight,
            constants::RocksDbWeight
        }
    },
    frame_system
};
use core::marker::PhantomData;

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