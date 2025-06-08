// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use scale_info::prelude::vec;
use scale_info::prelude::vec::Vec;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		sp_runtime::traits::{Hash, Member},

	};
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	// Patient Information Structure (TT_Bệnh nhân)
	#[derive(
		Encode,
		Decode,
		TypeInfo,
		CloneNoBound,
		PartialEqNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct PatientInfo<T: Config> {
		pub patient_id: u32,
		pub patient_name: Vec<u8>,
		pub date_of_birth: Vec<u8>,
		pub gender: Vec<u8>,
		pub address: Vec<u8>,
		pub phone: Vec<u8>,
		pub emergency_contact: Vec<u8>,
		pub created_at: BlockNumberFor<T>,
		pub created_by: T::AccountId,
		pub last_modified_at: BlockNumberFor<T>,
		pub last_modified_by: T::AccountId,
	}

	// Clinical Test Structure (TT_Cận lâm sàng)
	#[derive(
		Encode,
		Decode,
		TypeInfo,
		CloneNoBound,
		PartialEqNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct ClinicalTest<T: Config> {
		pub test_id: u32,
		pub patient_id: u32,
		pub doctor_id: T::AccountId,
		pub test_type: Vec<u8>,
		pub test_date: Vec<u8>,
		pub result: Vec<u8>,
		pub notes: Vec<u8>,
		pub created_at: BlockNumberFor<T>,
		pub created_by: T::AccountId,
		pub last_modified_at: BlockNumberFor<T>,
		pub last_modified_by: T::AccountId,
	}

	// Disease Progression Structure (TT_Diễn biến bệnh)
	#[derive(
		Encode,
		Decode,
		TypeInfo,
		CloneNoBound,
		PartialEqNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct DiseaseProgression<T: Config> {
		pub progression_id: u32,
		pub patient_id: u32,
		pub doctor_id: T::AccountId,
		pub visit_date: Vec<u8>,
		pub symptoms: Vec<u8>,
		pub diagnosis: Vec<u8>,
		pub treatment: Vec<u8>,
		pub prescription: Vec<u8>,
		pub next_appointment: Vec<u8>,
		pub created_at: BlockNumberFor<T>,
		pub created_by: T::AccountId,
		pub last_modified_at: BlockNumberFor<T>,
		pub last_modified_by: T::AccountId,
	}

	// Legacy MedicalRecord structure for compatibility
	#[derive(
		Encode,
		Decode,
		TypeInfo,
		CloneNoBound,
		PartialEqNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct MedicalRecord<T: Config> {
		pub(crate) record_id: u32,
		pub(crate) patient_id: u32,
		pub(crate) doctor_id: T::AccountId,
		pub(crate) record_hash: T::Hash,
		pub(crate) data_pointer: Option<Vec<u8>>,
		pub(crate) diagnosis: Vec<u8>,
		pub(crate) treatment: Vec<u8>,
		pub(crate) created_at: BlockNumberFor<T>,
		pub(crate) created_by: T::AccountId,
		pub(crate) last_modified_at: BlockNumberFor<T>,
		pub(crate) last_modified_by: T::AccountId,
	}
	
	// Storage for Patient Information
	#[pallet::storage]
	#[pallet::getter(fn patients)]
	pub type Patients<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		PatientInfo<T>,
		OptionQuery
	>;

	// Storage for mapping patient names to patient IDs for search functionality
	#[pallet::storage]
	#[pallet::getter(fn patient_name_to_ids)]
	pub type PatientNameToIds<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>, // patient_name as key
		Vec<u32>, // array of patient_ids as value
		OptionQuery
	>;

	// Storage for Clinical Tests
	#[pallet::storage]
	#[pallet::getter(fn clinical_tests)]
	pub type ClinicalTests<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		ClinicalTest<T>,
		OptionQuery
	>;

	// Storage for Disease Progressions
	#[pallet::storage]
	#[pallet::getter(fn disease_progressions)]
	pub type DiseaseProgressions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		DiseaseProgression<T>,
		OptionQuery
	>;

	// Counter storages
	#[pallet::storage]
	#[pallet::getter(fn next_patient_id)]
	pub type NextPatientId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_test_id)]
	pub type NextTestId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_progression_id)]
	pub type NextProgressionId<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Legacy storage
	#[pallet::storage]
	#[pallet::getter(fn medical_records)]
	pub type MedicalRecords<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		MedicalRecord<T>,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_record_id)]
	pub type NextRecordId<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Storage for Change History - comprehensive audit trail
	#[pallet::storage]
	#[pallet::getter(fn change_history)]
	pub type ChangeHistories<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // change_id
		ChangeHistory<T>,
		OptionQuery
	>;

	// Storage for mapping record to its changes
	#[pallet::storage]
	#[pallet::getter(fn record_changes)]
	pub type RecordChanges<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		(RecordType, u32), // (record_type, record_id)
		Blake2_128Concat,
		u32, // change_id
		(),
		OptionQuery
	>;

	// Counter for change history
	#[pallet::storage]
	#[pallet::getter(fn next_change_id)]
	pub type NextChangeId<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Change History Structure for audit trail
	#[derive(
		Encode,
		Decode,
		TypeInfo,
		CloneNoBound,
		PartialEqNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct ChangeHistory<T: Config> {
		pub change_id: u32,
		pub record_type: RecordType,
		pub record_id: u32,
		pub field_name: Vec<u8>,
		pub old_value: Option<Vec<u8>>,
		pub new_value: Vec<u8>,
		pub changed_by: T::AccountId,
		pub changed_at: BlockNumberFor<T>,
		pub operation: OperationType,
	}

	// Enum for different record types
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo, DecodeWithMemTracking)]
	pub enum RecordType {
		Patient,
		ClinicalTest,
		DiseaseProgression,
		MedicalRecord,
	}

	// Enum for different operation types
	#[derive(
		Encode,
		Decode,
		TypeInfo,
		CloneNoBound,
		PartialEqNoBound,
	)]
	pub enum OperationType {
		Create,
		Update,
		Delete,
	}

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new patient has been created.
		PatientCreated {
			patient_id: u32,
			patient_name: Vec<u8>,
		},
		/// Patient information has been updated.
		PatientUpdated {
			patient_id: u32,
			updated_by: T::AccountId,
		},
		/// Patient has been deleted.
		PatientDeleted {
			patient_id: u32,
		},
		/// A new clinical test has been created.
		ClinicalTestCreated {
			test_id: u32,
			patient_id: u32,
			doctor_id: T::AccountId,
		},
		/// Clinical test has been updated.
		ClinicalTestUpdated {
			test_id: u32,
			updated_by: T::AccountId,
		},
		/// Clinical test has been deleted.
		ClinicalTestDeleted {
			test_id: u32,
		},
		/// A new disease progression record has been created.
		DiseaseProgressionCreated {
			progression_id: u32,
			patient_id: u32,
			doctor_id: T::AccountId,
		},
		/// Disease progression has been updated.
		DiseaseProgressionUpdated {
			progression_id: u32,
			updated_by: T::AccountId,
		},
		/// Disease progression has been deleted.
		DiseaseProgressionDeleted {
			progression_id: u32,
		},
		/// A new medical record has been created (legacy).
		MedicalRecordCreated {
			record_id: u32,
			doctor_id: T::AccountId,
			patient_id: u32, // Changed from T::AccountId to u32
		},
		/// Patient found by name search.
		PatientFoundByName {
			patient_id: u32,
			patient_name: Vec<u8>,
		},
		/// Multiple patients found by name search.
		MultiplePatientsFoundByName {
			patient_ids: Vec<u32>,
			patient_name: Vec<u8>,
		},
		/// A change has been recorded in the audit trail.
		ChangeRecorded {
			change_id: u32,
			record_type: RecordType,
			record_id: u32,
			changed_by: T::AccountId,
		},
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		/// The record already exists.
		RecordAlreadyExists,
		/// Invalid record data.
		InvalidRecordData,
		/// Patient not found.
		PatientNotFound,
		/// Clinical test not found.
		ClinicalTestNotFound,
		/// Disease progression not found.
		DiseaseProgressionNotFound,
		/// Invalid patient data.
		InvalidPatientData,
		/// Invalid clinical test data.
		InvalidClinicalTestData,
		/// Invalid disease progression data.
		InvalidDiseaseProgressionData,
		/// Patient name already exists.
		PatientNameAlreadyExists,
		/// Patient not found by name.
		PatientNotFoundByName,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Patient CRUD operations
		#[pallet::weight(T::WeightInfo::create_patient())]
		pub fn create_patient(
			origin: OriginFor<T>,
			patient_name: Vec<u8>,
			date_of_birth: Vec<u8>,
			gender: Vec<u8>,
			address: Vec<u8>,
			phone: Vec<u8>,
			emergency_contact: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!patient_name.is_empty(), Error::<T>::InvalidPatientData);

			let patient_id = Self::next_patient_id();
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			let patient = PatientInfo::<T> {
				patient_id,
				patient_name: patient_name.clone(),
				date_of_birth: date_of_birth.clone(),
				gender: gender.clone(),
				address: address.clone(),
				phone: phone.clone(),
				emergency_contact: emergency_contact.clone(),
				created_at: block_number,
				created_by: who.clone(),
				last_modified_at: block_number,
				last_modified_by: who.clone(),
			};

			// Insert patient record
			Patients::<T>::insert(patient_id, patient);
			
			// Add patient_id to the name mapping array
			PatientNameToIds::<T>::mutate(&patient_name, |ids_opt| {
				match ids_opt {
					Some(ids) => ids.push(patient_id),
					None => *ids_opt = Some(vec![patient_id]),
				}
			});
			
			NextPatientId::<T>::put(patient_id + 1);

			// Record creation in audit trail
			Self::record_change(
				RecordType::Patient,
				patient_id,
				b"patient_name".to_vec(),
				None,
				patient_name.clone(),
				who.clone(),
				OperationType::Create,
			)?;
			Self::record_change(RecordType::Patient, patient_id, b"date_of_birth".to_vec(), None, date_of_birth, who.clone(), OperationType::Create)?;
			Self::record_change(RecordType::Patient, patient_id, b"gender".to_vec(), None, gender, who.clone(), OperationType::Create)?;
			Self::record_change(RecordType::Patient, patient_id, b"address".to_vec(), None, address, who.clone(), OperationType::Create)?;
			Self::record_change(RecordType::Patient, patient_id, b"phone".to_vec(), None, phone, who.clone(), OperationType::Create)?;
			Self::record_change(RecordType::Patient, patient_id, b"emergency_contact".to_vec(), None, emergency_contact, who, OperationType::Create)?;

			Self::deposit_event(Event::PatientCreated {
				patient_id,
				patient_name,
			});

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_patient())]
		pub fn update_patient(
			origin: OriginFor<T>,
			patient_id: u32,
			patient_name: Option<Vec<u8>>,
			date_of_birth: Option<Vec<u8>>,
			gender: Option<Vec<u8>>,
			address: Option<Vec<u8>>,
			phone: Option<Vec<u8>>,
			emergency_contact: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			Patients::<T>::try_mutate(patient_id, |patient_opt| -> DispatchResult {
				let patient = patient_opt.as_mut().ok_or(Error::<T>::PatientNotFound)?;

				// If updating name, handle the name mapping
				if let Some(new_name) = patient_name {
					ensure!(!new_name.is_empty(), Error::<T>::InvalidPatientData);
					
					let old_name = patient.patient_name.clone();
					
					// Record the change
					Self::record_change(
						RecordType::Patient,
						patient_id,
						b"patient_name".to_vec(),
						Some(old_name.clone()),
						new_name.clone(),
						who.clone(),
						OperationType::Update,
					)?;

					// Remove patient_id from old name mapping
					PatientNameToIds::<T>::mutate(&old_name, |ids_opt| {
						if let Some(ids) = ids_opt {
							ids.retain(|&id| id != patient_id);
							if ids.is_empty() {
								*ids_opt = None;
							}
						}
					});
					
					// Add patient_id to new name mapping
					PatientNameToIds::<T>::mutate(&new_name, |ids_opt| {
						match ids_opt {
							Some(ids) => ids.push(patient_id),
							None => *ids_opt = Some(vec![patient_id]),
						}
					});
					
					// Update patient name
					patient.patient_name = new_name;
				}
				if let Some(dob) = date_of_birth {
					Self::record_change(RecordType::Patient, patient_id, b"date_of_birth".to_vec(), Some(patient.date_of_birth.clone()), dob.clone(), who.clone(), OperationType::Update)?;
					patient.date_of_birth = dob;
				}
				if let Some(g) = gender {
					Self::record_change(RecordType::Patient, patient_id, b"gender".to_vec(), Some(patient.gender.clone()), g.clone(), who.clone(), OperationType::Update)?;
					patient.gender = g;
				}
				if let Some(addr) = address {
					Self::record_change(RecordType::Patient, patient_id, b"address".to_vec(), Some(patient.address.clone()), addr.clone(), who.clone(), OperationType::Update)?;
					patient.address = addr;
				}
				if let Some(p) = phone {
					Self::record_change(RecordType::Patient, patient_id, b"phone".to_vec(), Some(patient.phone.clone()), p.clone(), who.clone(), OperationType::Update)?;
					patient.phone = p;
				}
				if let Some(ec) = emergency_contact {
					Self::record_change(RecordType::Patient, patient_id, b"emergency_contact".to_vec(), Some(patient.emergency_contact.clone()), ec.clone(), who.clone(), OperationType::Update)?;
					patient.emergency_contact = ec;
				}

				// Update modification tracking
				patient.last_modified_at = block_number;
				patient.last_modified_by = who.clone();

				Self::deposit_event(Event::PatientUpdated { 
					patient_id,
					updated_by: who,
				});
				Ok(())
			})
		}

		#[pallet::weight(T::WeightInfo::delete_patient())]
		pub fn delete_patient(
			origin: OriginFor<T>,
			patient_id: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let patient = Patients::<T>::get(patient_id).ok_or(Error::<T>::PatientNotFound)?;

			// Record deletion in audit trail
			Self::record_change(
				RecordType::Patient,
				patient_id,
				b"deleted".to_vec(),
				Some(b"active".to_vec()),
				b"deleted".to_vec(),
				who,
				OperationType::Delete,
			)?;

			// Remove patient_id from name mapping
			PatientNameToIds::<T>::mutate(&patient.patient_name, |ids_opt| {
				if let Some(ids) = ids_opt {
					ids.retain(|&id| id != patient_id);
					if ids.is_empty() {
						*ids_opt = None;
					}
				}
			});
			
			// Remove patient record
			Patients::<T>::remove(patient_id);

			Self::deposit_event(Event::PatientDeleted { patient_id });

			Ok(())
		}

		// New function to search patient by name
		#[pallet::weight(10_000)]
		pub fn search_patient_by_name(
			origin: OriginFor<T>,
			patient_name: Vec<u8>,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let patient_ids = PatientNameToIds::<T>::get(&patient_name)
				.ok_or(Error::<T>::PatientNotFoundByName)?;

			if patient_ids.len() == 1 {
				Self::deposit_event(Event::PatientFoundByName {
					patient_id: patient_ids[0],
					patient_name,
				});
			} else {
				Self::deposit_event(Event::MultiplePatientsFoundByName {
					patient_ids,
					patient_name,
				});
			}

			Ok(())
		}

		// Clinical Test CRUD operations
		#[pallet::weight(10_000)]
		pub fn create_clinical_test(
			origin: OriginFor<T>,
			patient_id: u32,
			test_type: Vec<u8>,
			test_date: Vec<u8>,
			result: Vec<u8>,
			notes: Vec<u8>,
		) -> DispatchResult {
			let doctor_id = ensure_signed(origin)?;

			ensure!(Patients::<T>::contains_key(patient_id), Error::<T>::PatientNotFound);
			ensure!(!test_type.is_empty(), Error::<T>::InvalidClinicalTestData);

			let test_id = Self::next_test_id();
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			let clinical_test = ClinicalTest::<T> {
				test_id,
				patient_id,
				doctor_id: doctor_id.clone(),
				test_type,
				test_date,
				result,
				notes,
				created_at: block_number,
				created_by: doctor_id.clone(),
				last_modified_at: block_number,
				last_modified_by: doctor_id.clone(),
			};

			ClinicalTests::<T>::insert(test_id, clinical_test);
			NextTestId::<T>::put(test_id + 1);

			Self::deposit_event(Event::ClinicalTestCreated {
				test_id,
				patient_id,
				doctor_id,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn update_clinical_test(
			origin: OriginFor<T>,
			test_id: u32,
			test_type: Option<Vec<u8>>,
			test_date: Option<Vec<u8>>,
			result: Option<Vec<u8>>,
			notes: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			ClinicalTests::<T>::try_mutate(test_id, |test_opt| -> DispatchResult {
				let test = test_opt.as_mut().ok_or(Error::<T>::ClinicalTestNotFound)?;

				if let Some(tt) = test_type {
					test.test_type = tt;
				}
				if let Some(td) = test_date {
					test.test_date = td;
				}
				if let Some(r) = result {
					test.result = r;
				}
				if let Some(n) = notes {
					test.notes = n;
				}

				// Update modification tracking
				test.last_modified_at = block_number;
				test.last_modified_by = who.clone();

				Self::deposit_event(Event::ClinicalTestUpdated { 
					test_id,
					updated_by: who,
				});
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn delete_clinical_test(
			origin: OriginFor<T>,
			test_id: u32,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			ensure!(ClinicalTests::<T>::contains_key(test_id), Error::<T>::ClinicalTestNotFound);

			ClinicalTests::<T>::remove(test_id);

			Self::deposit_event(Event::ClinicalTestDeleted { test_id });

			Ok(())
		}

		// Disease Progression CRUD operations
		#[pallet::weight(10_000)]
		pub fn create_disease_progression(
			origin: OriginFor<T>,
			patient_id: u32,
			visit_date: Vec<u8>,
			symptoms: Vec<u8>,
			diagnosis: Vec<u8>,
			treatment: Vec<u8>,
			prescription: Vec<u8>,
			next_appointment: Vec<u8>,
		) -> DispatchResult {
			let doctor_id = ensure_signed(origin)?;

			ensure!(Patients::<T>::contains_key(patient_id), Error::<T>::PatientNotFound);
			ensure!(!visit_date.is_empty(), Error::<T>::InvalidDiseaseProgressionData);

			let progression_id = Self::next_progression_id();
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			let progression = DiseaseProgression::<T> {
				progression_id,
				patient_id,
				doctor_id: doctor_id.clone(),
				visit_date,
				symptoms,
				diagnosis,
				treatment,
				prescription,
				next_appointment,
				created_at: block_number,
				created_by: doctor_id.clone(),
				last_modified_at: block_number,
				last_modified_by: doctor_id.clone(),
			};

			DiseaseProgressions::<T>::insert(progression_id, progression);
			NextProgressionId::<T>::put(progression_id + 1);

			Self::deposit_event(Event::DiseaseProgressionCreated {
				progression_id,
				patient_id,
				doctor_id,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn update_disease_progression(
			origin: OriginFor<T>,
			progression_id: u32,
			visit_date: Option<Vec<u8>>,
			symptoms: Option<Vec<u8>>,
			diagnosis: Option<Vec<u8>>,
			treatment: Option<Vec<u8>>,
			prescription: Option<Vec<u8>>,
			next_appointment: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			DiseaseProgressions::<T>::try_mutate(progression_id, |progression_opt| -> DispatchResult {
				let progression = progression_opt.as_mut().ok_or(Error::<T>::DiseaseProgressionNotFound)?;

				if let Some(vd) = visit_date {
					progression.visit_date = vd;
				}
				if let Some(s) = symptoms {
					progression.symptoms = s;
				}
				if let Some(d) = diagnosis {
					progression.diagnosis = d;
				}
				if let Some(t) = treatment {
					progression.treatment = t;
				}
				if let Some(p) = prescription {
					progression.prescription = p;
				}
				if let Some(na) = next_appointment {
					progression.next_appointment = na;
				}

				// Update modification tracking
				progression.last_modified_at = block_number;
				progression.last_modified_by = who.clone();

				Self::deposit_event(Event::DiseaseProgressionUpdated { 
					progression_id,
					updated_by: who,
				});
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn delete_disease_progression(
			origin: OriginFor<T>,
			progression_id: u32,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			ensure!(DiseaseProgressions::<T>::contains_key(progression_id), Error::<T>::DiseaseProgressionNotFound);

			DiseaseProgressions::<T>::remove(progression_id);

			Self::deposit_event(Event::DiseaseProgressionDeleted { progression_id });

			Ok(())
		}

		// Legacy function - updated to use u32 patient_id
		#[pallet::weight(10_000)]
		pub fn create_medical_record(
			origin: OriginFor<T>,
			patient_id: u32,
			diagnosis: Vec<u8>,
			treatment: Vec<u8>,
			data_pointer: Option<Vec<u8>>,
		) -> DispatchResult {
			let doctor_id = ensure_signed(origin)?;

			// Ensure patient exists
			ensure!(Patients::<T>::contains_key(patient_id), Error::<T>::PatientNotFound);

			let record_id = Self::next_record_id();
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			let record = MedicalRecord::<T> {
				record_id,
				patient_id,
				doctor_id: doctor_id.clone(),
				record_hash: T::Hashing::hash_of(
					&(patient_id, doctor_id.clone(), diagnosis.clone(), treatment.clone(), data_pointer.clone())
				),
				data_pointer,
				diagnosis,
				treatment,
				created_at: block_number,
				created_by: doctor_id.clone(),
				last_modified_at: block_number,
				last_modified_by: doctor_id.clone(),
			};

			MedicalRecords::<T>::insert(record_id, record);
			NextRecordId::<T>::put(record_id + 1);

			Self::deposit_event(Event::MedicalRecordCreated {
				record_id,
				doctor_id,
				patient_id,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// Helper function to check if appointment is scheduled
		pub fn has_next_appointment(progression_id: u32) -> bool {
			if let Some(progression) = DiseaseProgressions::<T>::get(progression_id) {
				!progression.next_appointment.is_empty()
			} else {
				false
			}
		}

		// Helper function to clear next appointment (set to empty)
		pub fn clear_next_appointment(progression_id: u32) -> DispatchResult {
			DiseaseProgressions::<T>::try_mutate(progression_id, |progression_opt| -> DispatchResult {
				let progression = progression_opt.as_mut().ok_or(Error::<T>::DiseaseProgressionNotFound)?;
				progression.next_appointment = Vec::new(); // Empty = no appointment
				Ok(())
			})
		}

		// Query function to get only progressions WITH scheduled appointments
		pub fn get_scheduled_appointments(patient_id: u32) -> Vec<DiseaseProgression<T>> {
			DiseaseProgressions::<T>::iter()
				.filter_map(|(_, progression)| {
					if progression.patient_id == patient_id && !progression.next_appointment.is_empty() {
						Some(progression)
					} else {
						None
					}
				})
				.collect()
		}

		// Query function to get all progressions (with and without appointments)
		pub fn get_patient_progressions(patient_id: u32) -> Vec<DiseaseProgression<T>> {
			DiseaseProgressions::<T>::iter()
				.filter_map(|(_, progression)| {
					if progression.patient_id == patient_id {
						Some(progression)
					} else {
						None
					}
				})
				.collect()
		}

		// Function to record changes in audit trail
		pub fn record_change(
			record_type: RecordType,
			record_id: u32,
			field_name: Vec<u8>,
			old_value: Option<Vec<u8>>,
			new_value: Vec<u8>,
			changed_by: T::AccountId,
			operation: OperationType,
		) -> DispatchResult {
			let change_id = Self::next_change_id();
			let block_number: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

			let change = ChangeHistory::<T> {
				change_id,
				record_type: record_type.clone(),
				record_id,
				field_name,
				old_value,
				new_value,
				changed_by: changed_by.clone(),
				changed_at: block_number,
				operation,
			};

			// Store the change
			ChangeHistories::<T>::insert(change_id, change);
			// Map record to its changes
			RecordChanges::<T>::insert((record_type.clone(), record_id), change_id, ());
			NextChangeId::<T>::put(change_id + 1);

			Self::deposit_event(Event::ChangeRecorded {
				change_id,
				record_type,
				record_id,
				changed_by,
			});

			Ok(())
		}

		// Function to get all changes for a specific record
		pub fn get_record_history(record_type: RecordType, record_id: u32) -> Vec<ChangeHistory<T>> {
			RecordChanges::<T>::iter_prefix((record_type, record_id))
				.filter_map(|(change_id, _)| ChangeHistories::<T>::get(change_id))
				.collect()
		}

		// Function to get changes by a specific user
		pub fn get_changes_by_user(user: &T::AccountId) -> Vec<ChangeHistory<T>> {
			ChangeHistories::<T>::iter()
				.filter_map(|(_, change)| {
					if &change.changed_by == user {
						Some(change)
					} else {
						None
					}
				})
				.collect()
		}

		// Function to get changes within a time range
		pub fn get_changes_in_range(
			start_block: BlockNumberFor<T>,
			end_block: BlockNumberFor<T>,
		) -> Vec<ChangeHistory<T>> {
			ChangeHistories::<T>::iter()
				.filter_map(|(_, change)| {
					if change.changed_at >= start_block && change.changed_at <= end_block {
						Some(change)
					} else {
						None
					}
				})
				.collect()
		}

		// Function to get latest changes (last N changes)
		pub fn get_latest_changes(limit: u32) -> Vec<ChangeHistory<T>> {
			let current_change_id = Self::next_change_id();
			let start_id = if current_change_id > limit { current_change_id - limit } else { 0 };
			
			(start_id..current_change_id)
				.filter_map(|id| ChangeHistories::<T>::get(id))
				.collect()
		}
	}
}
