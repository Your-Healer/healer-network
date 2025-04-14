#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame::deps::frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as Template;
    use frame_system::RawOrigin;

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}