//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as AiPrediction;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	impl_benchmark_test_suite!(AiPrediction, crate::mock::new_test_ext(), crate::mock::Test);
}
