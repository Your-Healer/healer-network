//! Benchmarking setup for pallet-medical-record

use super::*;

#[allow(unused)]
use crate::Pallet as MedicalRecord;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_patient() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		_(
			RawOrigin::Signed(caller),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		);

		assert_eq!(MedicalRecord::<T>::next_patient_id(), 1);
	}

	#[benchmark]
	fn create_clinical_test() {
		let caller: T::AccountId = whitelisted_caller();
		
		// First create a patient
		let _ = MedicalRecord::<T>::create_patient(
			RawOrigin::Signed(caller.clone()).into(),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(caller),
			0u32,
			b"Blood Test".to_vec(),
			b"2023-01-15".to_vec(),
			b"Normal".to_vec(),
			b"All values within range".to_vec(),
		);

		assert_eq!(MedicalRecord::<T>::next_test_id(), 1);
	}

	#[benchmark]
	fn create_disease_progression() {
		let caller: T::AccountId = whitelisted_caller();
		
		// First create a patient
		let _ = MedicalRecord::<T>::create_patient(
			RawOrigin::Signed(caller.clone()).into(),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(caller),
			0u32,
			b"2023-01-15".to_vec(),
			b"Fever, headache".to_vec(),
			b"Common cold".to_vec(),
			b"Rest and fluids".to_vec(),
			b"Paracetamol 500mg".to_vec(),
			b"2023-01-22".to_vec(),
		);

		assert_eq!(MedicalRecord::<T>::next_progression_id(), 1);
	}

	impl_benchmark_test_suite!(MedicalRecord, crate::mock::new_test_ext(), crate::mock::Test);
}
