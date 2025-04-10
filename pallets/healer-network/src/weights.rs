#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use polkadot_sdk::polkadot_sdk_frame::deps::{frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}}, frame_system};
use core::marker::PhantomData;

pub trait WeightInfo {}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
}

impl WeightInfo for () {}