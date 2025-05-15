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

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::UnixTime,
	};
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
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

		type TimeProvider: UnixTime;

		/// Appointment Status.
		type RuntimeAppointmentStatus: From<AppointmentStatus> + Into<AppointmentStatus>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
	pub enum AppointmentStatus {
		/// The appointment is scheduled and pending.
		Scheduled,
		/// The appointment is completed.
		Completed,
		/// The appointment is cancelled.
		Cancelled,
	}
	impl Default for AppointmentStatus {
		fn default() -> Self {
			AppointmentStatus::Scheduled
		}
	}

	#[derive(
		Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct Appointment<T: Config> {
		pub(crate) appointment_id: u32,
		pub(crate) patient_id: T::AccountId,
		pub(crate) doctor_id: T::AccountId,
		pub(crate) scheduled_time: u64,
		pub(crate) status: Option<AppointmentStatus>,
		pub(crate) created_at: u64,
		pub(crate) updated_at: u64,
	}
	
	#[pallet::storage]
	#[pallet::getter(fn appointments)]
	pub type Appointments<T> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Appointment ID as key
		Appointment<T>, // Appointment details as value
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_appointment_id)]
	pub type NextAppointmentId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new appointment has been created.
		AppointmentBooked {
			/// The ID of the appointment.
			appointment_id: u32,
			/// The account who booked the appointment.
			patient_id: T::AccountId,
			/// The account of the doctor.
			doctor_id: T::AccountId,
			/// The scheduled time of the appointment.
			scheduled_time: u64,
		},

		/// A appointments has been updated.
		AppointmentUpdated {
			/// The ID of the appointment.
			appointment_id: u32,
			// / The new status of the appointment.
			// status: AppointmentStatus,
		},

		/// A appointment has been cancelled.
		AppointmentCancelled {
			/// The ID of the appointment.
			appointment_id: u32,
			/// The account who cancelled the appointment.
			patient_id: T::AccountId,
		},
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,

		/// The appointment not found.
		AppointmentNotFound,

		/// The appointment already exists.
		AppointmentAlreadyExists,

		/// The appointment is already completed.
		AppointmentAlreadyCompleted,

		/// Invalid appointment time.
		InvalidAppointmentTime,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_medical_appointment(
			origin: OriginFor<T>,
			patient_id: T::AccountId,
			doctor_id: T::AccountId,
			scheduled_time: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let time: u64 = <T as Config>::TimeProvider::now().as_secs();

			// Ensure the appointment time is in the future
			ensure!(scheduled_time > time, Error::<T>::InvalidAppointmentTime);

			let appointment_id = Self::next_appointment_id();

			// Check if the appointment already exists
			ensure!(!Appointments::<T>::contains_key(appointment_id), Error::<T>::AppointmentAlreadyExists);

			let new_appointment = Appointment {
				appointment_id,
				patient_id: patient_id.clone(),
				doctor_id: doctor_id.clone(),
				scheduled_time,
				status: Some(AppointmentStatus::default()),
				created_at: time,
				// <frame_system::Pallet<T>>::block_number()
				updated_at: time,
			};

			Appointments::<T>::insert(appointment_id, new_appointment);
			NextAppointmentId::<T>::put(appointment_id + 1);

			Self::deposit_event(Event::AppointmentBooked {
				appointment_id,
				patient_id,
				doctor_id,
				scheduled_time,
			});

			Ok(())
		}
	}
}
