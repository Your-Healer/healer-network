//! Benchmarking setup for pallet-poh
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use polkadot_sdk::frame_benchmarking::v2::* as frame_benchmarking;

#[benchmarks]
mod benchmarks {
	use super::*;
	#[cfg(test)]
	use crate::pallet::Pallet as Template;
	use polkadot_sdk::frame_system::RawOrigin;

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
