use crate::{mock::*, Error, Event, PatientInfo, ClinicalTest, DiseaseProgression, MedicalRecord, Patients, ClinicalTests, DiseaseProgressions, MedicalRecords};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_patient_works() {
	new_test_ext().execute_with(|| {
		// Test creating a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Check that the patient was created
		let patient = Patients::<Test>::get(0).unwrap();
		assert_eq!(patient.patient_name, b"John Doe".to_vec());
		assert_eq!(patient.gender, b"Male".to_vec());

		// Check that the event was emitted
		System::assert_last_event(RuntimeEvent::MedicalRecord(Event::PatientCreated {
			patient_id: 0,
			patient_name: b"John Doe".to_vec(),
		}));
	});
}

#[test]
fn update_patient_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Update the patient
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(1),
			0,
			Some(b"John Smith".to_vec()),
			None,
			None,
			Some(b"456 Oak Ave".to_vec()),
			None,
			None,
		));

		// Check that the patient was updated
		let patient = Patients::<Test>::get(0).unwrap();
		assert_eq!(patient.patient_name, b"John Smith".to_vec());
		assert_eq!(patient.address, b"456 Oak Ave".to_vec());
	});
}

#[test]
fn delete_patient_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Delete the patient
		assert_ok!(MedicalRecord::delete_patient(RuntimeOrigin::signed(1), 0));

		// Check that the patient was deleted
		assert_eq!(Patients::<Test>::get(0), None);
	});
}

#[test]
fn create_clinical_test_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a clinical test
		assert_ok!(MedicalRecord::create_clinical_test(
			RuntimeOrigin::signed(2),
			0,
			b"Blood Test".to_vec(),
			b"2023-01-15".to_vec(),
			b"Normal".to_vec(),
			b"All values within range".to_vec(),
		));

		// Check that the clinical test was created
		let test = ClinicalTests::<Test>::get(0).unwrap();
		assert_eq!(test.patient_id, 0);
		assert_eq!(test.test_type, b"Blood Test".to_vec());
		assert_eq!(test.doctor_id, 2);
	});
}

#[test]
fn create_disease_progression_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a disease progression record
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-15".to_vec(),
			b"Fever, headache".to_vec(),
			b"Common cold".to_vec(),
			b"Rest and fluids".to_vec(),
			b"Paracetamol 500mg".to_vec(),
			b"2023-01-22".to_vec(),
		));

		// Check that the disease progression was created
		let progression = DiseaseProgressions::<Test>::get(0).unwrap();
		assert_eq!(progression.patient_id, 0);
		assert_eq!(progression.symptoms, b"Fever, headache".to_vec());
		assert_eq!(progression.doctor_id, 2);
	});
}

#[test]
fn create_patient_with_empty_name_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MedicalRecord::create_patient(
				RuntimeOrigin::signed(1),
				b"".to_vec(),
				b"1990-01-01".to_vec(),
				b"Male".to_vec(),
				b"123 Main St".to_vec(),
				b"555-1234".to_vec(),
				b"Jane Doe - 555-5678".to_vec(),
			),
			Error::<Test>::InvalidPatientData
		);
	});
}

#[test]
fn update_nonexistent_patient_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MedicalRecord::update_patient(
				RuntimeOrigin::signed(1),
				999,
				Some(b"John Smith".to_vec()),
				None,
				None,
				None,
				None,
				None,
			),
			Error::<Test>::PatientNotFound
		);
	});
}

#[test]
fn create_clinical_test_for_nonexistent_patient_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MedicalRecord::create_clinical_test(
				RuntimeOrigin::signed(2),
				999,
				b"Blood Test".to_vec(),
				b"2023-01-15".to_vec(),
				b"Normal".to_vec(),
				b"All values within range".to_vec(),
			),
			Error::<Test>::PatientNotFound
		);
	});
}

#[test]
fn create_disease_progression_without_appointment_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a disease progression record WITHOUT next appointment (empty vec)
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-15".to_vec(),
			b"Fever, headache".to_vec(),
			b"Common cold".to_vec(),
			b"Rest and fluids".to_vec(),
			b"Paracetamol 500mg".to_vec(),
			Vec::new(), // No next appointment
		));

		// Check that the disease progression was created with no appointment
		let progression = DiseaseProgressions::<Test>::get(0).unwrap();
		assert_eq!(progression.patient_id, 0);
		assert_eq!(progression.next_appointment, Vec::<u8>::new());
		
		// Check helper function
		assert!(!MedicalRecord::has_next_appointment(0));
	});
}

#[test]
fn set_next_appointment_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a disease progression record without next appointment
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-15".to_vec(),
			b"Fever, headache".to_vec(),
			b"Common cold".to_vec(),
			b"Rest and fluids".to_vec(),
			b"Paracetamol 500mg".to_vec(),
			Vec::new(), // No appointment initially
		));

		// Set next appointment
		assert_ok!(MedicalRecord::set_next_appointment(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-22 10:00".to_vec(),
		));

		// Check that the appointment was set
		let progression = DiseaseProgressions::<Test>::get(0).unwrap();
		assert_eq!(progression.next_appointment, b"2023-01-22 10:00".to_vec());
		assert!(MedicalRecord::has_next_appointment(0));
	});
}

#[test]
fn clear_appointment_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a disease progression record with next appointment
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-15".to_vec(),
			b"Fever, headache".to_vec(),
			b"Common cold".to_vec(),
			b"Rest and fluids".to_vec(),
			b"Paracetamol 500mg".to_vec(),
			b"2023-01-22 10:00".to_vec(),
		));

		// Verify appointment is set
		assert!(MedicalRecord::has_next_appointment(0));

		// Clear the appointment
		assert_ok!(MedicalRecord::clear_appointment(RuntimeOrigin::signed(2), 0));

		// Check that the appointment was cleared
		let progression = DiseaseProgressions::<Test>::get(0).unwrap();
		assert_eq!(progression.next_appointment, Vec::<u8>::new());
		assert!(!MedicalRecord::has_next_appointment(0));
	});
}

#[test]
fn get_scheduled_appointments_filters_correctly() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create progression WITH appointment
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-15".to_vec(),
			b"Fever".to_vec(),
			b"Cold".to_vec(),
			b"Rest".to_vec(),
			b"Medicine".to_vec(),
			b"2023-01-22 10:00".to_vec(), // HAS appointment
		));

		// Create progression WITHOUT appointment
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-22".to_vec(),
			b"Better".to_vec(),
			b"Recovering".to_vec(),
			b"Continue".to_vec(),
			b"Same medicine".to_vec(),
			Vec::new(), // NO appointment
		));

		// Get scheduled appointments (should only return the first one)
		let scheduled = MedicalRecord::get_scheduled_appointments(0);
		assert_eq!(scheduled.len(), 1);
		assert_eq!(scheduled[0].next_appointment, b"2023-01-22 10:00".to_vec());

		// Get all progressions (should return both)
		let all = MedicalRecord::get_patient_progressions(0);
		assert_eq!(all.len(), 2);
	});
}

#[test]
fn create_legacy_medical_record_works() {
	new_test_ext().execute_with(|| {
		// First create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a legacy medical record
		assert_ok!(MedicalRecord::create_medical_record(
			RuntimeOrigin::signed(2),
			0u32, // Patient ID as u32
			b"Flu diagnosis".to_vec(),
			b"Rest and medication".to_vec(),
			123u32,
		));

		// Check that the medical record was created
		let record = MedicalRecords::<Test>::get(0).unwrap();
		assert_eq!(record.patient_id, 0u32);
		assert_eq!(record.doctor_id, 2);
		assert_eq!(record.diagnosis, b"Flu diagnosis".to_vec());

		// Check that the event was emitted
		System::assert_last_event(RuntimeEvent::MedicalRecord(Event::MedicalRecordCreated {
			record_id: 0,
			doctor_id: 2,
			patient_id: 0u32,
		}));
	});
}

#[test]
fn create_legacy_medical_record_for_nonexistent_patient_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MedicalRecord::create_medical_record(
				RuntimeOrigin::signed(2),
				999u32, // Nonexistent patient ID
				b"Flu diagnosis".to_vec(),
				b"Rest and medication".to_vec(),
				123u32,
			),
			Error::<Test>::PatientNotFound
		);
	});
}

#[test]
fn search_patient_by_name_works() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Search for the patient by name
		assert_ok!(MedicalRecord::search_patient_by_name(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
		));

		// Check that the search event was emitted
		System::assert_last_event(RuntimeEvent::MedicalRecord(Event::PatientFoundByName {
			patient_id: 0,
			patient_name: b"John Doe".to_vec(),
		}));
	});
}

#[test]
fn search_nonexistent_patient_by_name_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MedicalRecord::search_patient_by_name(
				RuntimeOrigin::signed(1),
				b"Nonexistent Patient".to_vec(),
			),
			Error::<Test>::PatientNotFoundByName
		);
	});
}

#[test]
fn get_patient_by_name_helper_works() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"Alice Smith".to_vec(),
			b"1985-05-15".to_vec(),
			b"Female".to_vec(),
			b"456 Oak St".to_vec(),
			b"555-9876".to_vec(),
			b"Bob Smith - 555-5432".to_vec(),
		));

		// Use helper function to get patient by name
		let patient = MedicalRecord::get_patient_by_name(b"Alice Smith").unwrap();
		assert_eq!(patient.patient_id, 0);
		assert_eq!(patient.patient_name, b"Alice Smith".to_vec());
		assert_eq!(patient.gender, b"Female".to_vec());

		// Test with nonexistent name
		assert_eq!(MedicalRecord::get_patient_by_name(b"Nonexistent"), None);
	});
}

#[test]
fn create_patient_with_duplicate_name_fails() {
	new_test_ext().execute_with(|| {
		// Create first patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Try to create another patient with same name
		assert_noop!(
			MedicalRecord::create_patient(
				RuntimeOrigin::signed(2),
				b"John Doe".to_vec(), // Same name
				b"1985-05-15".to_vec(),
				b"Male".to_vec(),
				b"456 Oak St".to_vec(),
				b"555-9876".to_vec(),
				b"Mary Doe - 555-5432".to_vec(),
			),
			Error::<Test>::PatientNameAlreadyExists
		);
	});
}

#[test]
fn update_patient_name_updates_mapping() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Update patient name
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(1),
			0,
			Some(b"John Smith".to_vec()),
			None,
			None,
			None,
			None,
			None,
		));

		// Old name should not exist
		assert!(!MedicalRecord::patient_name_exists(b"John Doe"));
		// New name should exist
		assert!(MedicalRecord::patient_name_exists(b"John Smith"));
		
		// Search by new name should work
		assert_eq!(MedicalRecord::get_patient_id_by_name(b"John Smith"), Some(0));
		// Search by old name should fail
		assert_eq!(MedicalRecord::get_patient_id_by_name(b"John Doe"), None);
	});
}

#[test]
fn update_patient_name_to_existing_name_fails() {
	new_test_ext().execute_with(|| {
		// Create first patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create second patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"Alice Smith".to_vec(),
			b"1985-05-15".to_vec(),
			b"Female".to_vec(),
			b"456 Oak St".to_vec(),
			b"555-9876".to_vec(),
			b"Bob Smith - 555-5432".to_vec(),
		));

		// Try to update second patient's name to first patient's name
		assert_noop!(
			MedicalRecord::update_patient(
				RuntimeOrigin::signed(1),
				1,
				Some(b"John Doe".to_vec()), // Already exists
				None,
				None,
				None,
				None,
				None,
			),
			Error::<Test>::PatientNameAlreadyExists
		);
	});
}

#[test]
fn delete_patient_removes_name_mapping() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Verify name mapping exists
		assert!(MedicalRecord::patient_name_exists(b"John Doe"));

		// Delete the patient
		assert_ok!(MedicalRecord::delete_patient(RuntimeOrigin::signed(1), 0));

		// Verify name mapping is removed
		assert!(!MedicalRecord::patient_name_exists(b"John Doe"));
		assert_eq!(MedicalRecord::get_patient_id_by_name(b"John Doe"), None);
	});
}

#[test]
fn modification_tracking_works_for_patient() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		let patient = Patients::<Test>::get(0).unwrap();
		assert_eq!(patient.created_by, 1);
		assert_eq!(patient.last_modified_by, 1);
		assert_eq!(patient.created_at, patient.last_modified_at);

		// Move to next block
		System::set_block_number(2);

		// Update the patient with different account
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(2),
			0,
			Some(b"John Smith".to_vec()),
			None,
			None,
			None,
			None,
			None,
		));

		let updated_patient = Patients::<Test>::get(0).unwrap();
		assert_eq!(updated_patient.created_by, 1); // Original creator
		assert_eq!(updated_patient.last_modified_by, 2); // New modifier
		assert_eq!(updated_patient.created_at, 1); // Original creation block
		assert_eq!(updated_patient.last_modified_at, 2); // New modification block
		assert_eq!(updated_patient.patient_name, b"John Smith".to_vec());
	});
}

#[test]
fn modification_tracking_works_for_clinical_test() {
	new_test_ext().execute_with(|| {
		// Create a patient first
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a clinical test
		assert_ok!(MedicalRecord::create_clinical_test(
			RuntimeOrigin::signed(2),
			0,
			b"Blood Test".to_vec(),
			b"2023-01-15".to_vec(),
			b"Normal".to_vec(),
			b"All values within range".to_vec(),
		));

		let test = ClinicalTests::<Test>::get(0).unwrap();
		assert_eq!(test.created_by, 2);
		assert_eq!(test.last_modified_by, 2);
		assert_eq!(test.doctor_id, 2);

		// Move to next block
		System::set_block_number(2);

		// Update the test with different account
		assert_ok!(MedicalRecord::update_clinical_test(
			RuntimeOrigin::signed(3),
			0,
			Some(b"Updated Blood Test".to_vec()),
			None,
			Some(b"Abnormal".to_vec()),
			None,
		));

		let updated_test = ClinicalTests::<Test>::get(0).unwrap();
		assert_eq!(updated_test.created_by, 2); // Original creator
		assert_eq!(updated_test.last_modified_by, 3); // New modifier
		assert_eq!(updated_test.doctor_id, 2); // Original doctor unchanged
		assert_eq!(updated_test.created_at, 1);
		assert_eq!(updated_test.last_modified_at, 2);
		assert_eq!(updated_test.test_type, b"Updated Blood Test".to_vec());
		assert_eq!(updated_test.result, b"Abnormal".to_vec());
	});
}

#[test]
fn modification_tracking_works_for_disease_progression() {
	new_test_ext().execute_with(|| {
		// Create a patient first
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Create a disease progression
		assert_ok!(MedicalRecord::create_disease_progression(
			RuntimeOrigin::signed(2),
			0,
			b"2023-01-15".to_vec(),
			b"Fever, headache".to_vec(),
			b"Common cold".to_vec(),
			b"Rest and fluids".to_vec(),
			b"Paracetamol 500mg".to_vec(),
			b"2023-01-22".to_vec(),
		));

		let progression = DiseaseProgressions::<Test>::get(0).unwrap();
		assert_eq!(progression.created_by, 2);
		assert_eq!(progression.last_modified_by, 2);

		// Move to next block
		System::set_block_number(2);

		// Update the progression with different account
		assert_ok!(MedicalRecord::update_disease_progression(
			RuntimeOrigin::signed(3),
			0,
			None,
			Some(b"Feeling better".to_vec()),
			Some(b"Recovering from cold".to_vec()),
			None,
			None,
			Some(b"2023-01-29".to_vec()),
		));

		let updated_progression = DiseaseProgressions::<Test>::get(0).unwrap();
		assert_eq!(updated_progression.created_by, 2); // Original creator
		assert_eq!(updated_progression.last_modified_by, 3); // New modifier
		assert_eq!(updated_progression.doctor_id, 2); // Original doctor unchanged
		assert_eq!(updated_progression.created_at, 1);
		assert_eq!(updated_progression.last_modified_at, 2);
		assert_eq!(updated_progression.symptoms, b"Feeling better".to_vec());
		assert_eq!(updated_progression.diagnosis, b"Recovering from cold".to_vec());
		assert_eq!(updated_progression.next_appointment, b"2023-01-29".to_vec());
	});
}

#[test]
fn update_events_include_modifier_info() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Update the patient
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(2),
			0,
			Some(b"John Smith".to_vec()),
			None,
			None,
			None,
			None,
			None,
		));

		// Check that the update event includes modifier info
		System::assert_last_event(RuntimeEvent::MedicalRecord(Event::PatientUpdated {
			patient_id: 0,
			updated_by: 2,
		}));
	});
}

#[test]
fn modification_tracking_preserves_original_creator() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Multiple updates by different users
		for i in 2..=5 {
			System::set_block_number(i);
			assert_ok!(MedicalRecord::update_patient(
				RuntimeOrigin::signed(i as u64),
				0,
				None,
				None,
				None,
				Some(format!("Address {}", i).into_bytes()),
				None,
				None,
			));

			let patient = Patients::<Test>::get(0).unwrap();
			assert_eq!(patient.created_by, 1); // Always original creator
			assert_eq!(patient.last_modified_by, i as u64); // Latest modifier
			assert_eq!(patient.created_at, 1); // Original creation time
			assert_eq!(patient.last_modified_at, i); // Latest modification time
		}
	});
}

#[test]
fn comprehensive_change_history_tracking_works() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Move to next block
		System::set_block_number(2);

		// Update patient name
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(2),
			0,
			Some(b"John Smith".to_vec()),
			None,
			None,
			None,
			None,
			None,
		));

		// Move to next block
		System::set_block_number(3);

		// Update address
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(3),
			0,
			None,
			None,
			None,
			Some(b"456 Oak St".to_vec()),
			None,
			None,
		));

		// Get complete history for this patient
		let history = MedicalRecord::get_record_history(crate::RecordType::Patient, 0);
		
		// Should have 8 changes: 6 for creation + 1 name update + 1 address update
		assert_eq!(history.len(), 8);

		// Check creation changes (first 6)
		let creation_changes: Vec<_> = history.iter().filter(|h| matches!(h.operation, crate::OperationType::Create)).collect();
		assert_eq!(creation_changes.len(), 6);
		assert!(creation_changes.iter().all(|h| h.changed_by == 1));
		assert!(creation_changes.iter().all(|h| h.changed_at == 1));

		// Check update changes
		let update_changes: Vec<_> = history.iter().filter(|h| matches!(h.operation, crate::OperationType::Update)).collect();
		assert_eq!(update_changes.len(), 2);

		// Find name change
		let name_change = update_changes.iter().find(|h| h.field_name == b"patient_name").unwrap();
		assert_eq!(name_change.old_value, Some(b"John Doe".to_vec()));
		assert_eq!(name_change.new_value, b"John Smith".to_vec());
		assert_eq!(name_change.changed_by, 2);
		assert_eq!(name_change.changed_at, 2);

		// Find address change
		let address_change = update_changes.iter().find(|h| h.field_name == b"address").unwrap();
		assert_eq!(address_change.old_value, Some(b"123 Main St".to_vec()));
		assert_eq!(address_change.new_value, b"456 Oak St".to_vec());
		assert_eq!(address_change.changed_by, 3);
		assert_eq!(address_change.changed_at, 3);
	});
}

#[test]
fn get_changes_by_user_works() {
	new_test_ext().execute_with(|| {
		// Create patients with different users
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(2),
			b"Alice Smith".to_vec(),
			b"1985-05-15".to_vec(),
			b"Female".to_vec(),
			b"456 Oak St".to_vec(),
			b"555-9876".to_vec(),
			b"Bob Smith - 555-5432".to_vec(),
		));

		// Update first patient with user 2
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(2),
			0,
			Some(b"John Updated".to_vec()),
			None,
			None,
			None,
			None,
			None,
		));

		// Get changes by user 1 (should have 6 creation changes)
		let user1_changes = MedicalRecord::get_changes_by_user(&1);
		assert_eq!(user1_changes.len(), 6);
		assert!(user1_changes.iter().all(|c| matches!(c.operation, crate::OperationType::Create)));

		// Get changes by user 2 (should have 6 creation + 1 update = 7 changes)
		let user2_changes = MedicalRecord::get_changes_by_user(&2);
		assert_eq!(user2_changes.len(), 7);
		
		let user2_creates = user2_changes.iter().filter(|c| matches!(c.operation, crate::OperationType::Create)).count();
		let user2_updates = user2_changes.iter().filter(|c| matches!(c.operation, crate::OperationType::Update)).count();
		assert_eq!(user2_creates, 6);
		assert_eq!(user2_updates, 1);
	});
}

#[test]
fn get_changes_in_time_range_works() {
	new_test_ext().execute_with(|| {
		// Block 1: Create patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Block 2: Update patient
		System::set_block_number(2);
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(2),
			0,
			Some(b"John Smith".to_vec()),
			None,
			None,
			None,
			None,
			None,
		));

		// Block 3: Another update
		System::set_block_number(3);
		assert_ok!(MedicalRecord::update_patient(
			RuntimeOrigin::signed(3),
			0,
			None,
			None,
			None,
			Some(b"New Address".to_vec()),
			None,
			None,
		));

		// Get changes from block 1 to 2
		let range_changes = MedicalRecord::get_changes_in_range(1, 2);
		assert_eq!(range_changes.len(), 7); // 6 creates + 1 update

		// Get changes only from block 3
		let block3_changes = MedicalRecord::get_changes_in_range(3, 3);
		assert_eq!(block3_changes.len(), 1); // 1 address update
		assert_eq!(block3_changes[0].field_name, b"address");
	});
}

#[test]
fn get_latest_changes_works() {
	new_test_ext().execute_with(|| {
		// Create multiple patients to generate many changes
		for i in 1..=3 {
			assert_ok!(MedicalRecord::create_patient(
				RuntimeOrigin::signed(i),
				format!("Patient {}", i).into_bytes(),
				b"1990-01-01".to_vec(),
				b"Male".to_vec(),
				b"123 Main St".to_vec(),
				b"555-1234".to_vec(),
				b"Emergency Contact".to_vec(),
			));
		}

		// Total changes should be 3 * 6 = 18
		let total_changes = MedicalRecord::next_change_id();
		assert_eq!(total_changes, 18);

		// Get latest 5 changes
		let latest_5 = MedicalRecord::get_latest_changes(5);
		assert_eq!(latest_5.len(), 5);
		
		// Get latest 10 changes
		let latest_10 = MedicalRecord::get_latest_changes(10);
		assert_eq!(latest_10.len(), 10);

		// Get all changes (more than exist)
		let all_changes = MedicalRecord::get_latest_changes(50);
		assert_eq!(all_changes.len(), 18);
	});
}

#[test]
fn delete_patient_records_audit_trail() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Delete the patient
		assert_ok!(MedicalRecord::delete_patient(RuntimeOrigin::signed(2), 0));

		// Check audit trail includes deletion
		let history = MedicalRecord::get_record_history(crate::RecordType::Patient, 0);
		let delete_changes: Vec<_> = history.iter().filter(|h| matches!(h.operation, crate::OperationType::Delete)).collect();
		assert_eq!(delete_changes.len(), 1);
		
		let delete_change = delete_changes[0];
		assert_eq!(delete_change.field_name, b"deleted");
		assert_eq!(delete_change.old_value, Some(b"active".to_vec()));
		assert_eq!(delete_change.new_value, b"deleted".to_vec());
		assert_eq!(delete_change.changed_by, 2);
	});
}

#[test]
fn change_events_are_emitted() {
	new_test_ext().execute_with(|| {
		// Create a patient
		assert_ok!(MedicalRecord::create_patient(
			RuntimeOrigin::signed(1),
			b"John Doe".to_vec(),
			b"1990-01-01".to_vec(),
			b"Male".to_vec(),
			b"123 Main St".to_vec(),
			b"555-1234".to_vec(),
			b"Jane Doe - 555-5678".to_vec(),
		));

		// Check that change events were emitted (6 for each field)
		let events = System::events();
		let change_events: Vec<_> = events.iter()
			.filter(|e| matches!(e.event, RuntimeEvent::MedicalRecord(crate::Event::ChangeRecorded { .. })))
			.collect();
		assert_eq!(change_events.len(), 6);
	});
}