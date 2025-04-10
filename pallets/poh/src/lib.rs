//! Proof of History (PoH) pallet.
//!
//! This pallet implements a simple Proof of History mechanism that allows
//! recording and verifying sequential proofs of data existence.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::prelude::vec::Vec;
use frame::prelude::*;

use polkadot_sdk::{
	polkadot_sdk_frame as frame,
	frame_support::{
		traits::Time,
		sp_runtime::traits::Hash,
	}
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: polkadot_sdk::frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

		/// The hash function used for proof generation
		type Hasher: Hash<Output = <Self as pallet::Config>::Hash>;
		
		/// Time interface
		type Time: Time;
		
		/// Hash type used for the proof
		type Hash: Member + Parameter + MaybeSerializeDeserialize + Ord + Default + Copy + frame::deps::scale_info::TypeInfo + MaxEncodedLen;

		type WeightInfo: WeightInfo;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, frame::deps::scale_info::TypeInfo, MaxEncodedLen, Debug)]
	pub struct ProofRecord<Hash, BlockNumber, Moment, AccountId> {
		/// The data hash included in this proof
		pub data_hash: Hash,
		/// The previous proof hash for chain verification
		pub previous_hash: Hash,
		/// The combined hash representing this proof
		pub proof_hash: Hash,
		/// Block number when this proof was created
		pub block_number: BlockNumber,
		/// Timestamp when this proof was created
		pub timestamp: Moment,
		/// Account that submitted the data
		pub submitter: AccountId,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proof_count)]
	/// The total number of proofs created
	pub type ProofCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	/// Storage for all proof records
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		<T as pallet::Config>::Hash, // Proof hash as key
		ProofRecord<<T as pallet::Config>::Hash, BlockNumberFor<T>, <<T as Config>::Time as Time>::Moment, T::AccountId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn latest_proof)]
	/// The hash of the latest proof for quick reference
	pub type LatestProof<T: Config> = StorageValue<_, <T as pallet::Config>::Hash, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn data_to_proof)]
	/// Maps data hashes to their corresponding proof hashes
	pub type DataToProof<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		<T as pallet::Config>::Hash, // Data hash
		<T as pallet::Config>::Hash, // Proof hash
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new proof has been created.
		/// [proof_hash, data_hash, block_number]
		ProofCreated(<T as pallet::Config>::Hash, <T as pallet::Config>::Hash, BlockNumberFor<T>),
		/// A proof has been verified successfully.
		/// [proof_hash, verifier]
		ProofVerified(<T as pallet::Config>::Hash, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The proof hash already exists
		ProofAlreadyExists,
		/// The proof does not exist
		ProofNotFound,
		/// Invalid proof chain
		InvalidProofChain,
		/// No previous proof exists to chain from
		NoPreviousProof,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Submit data to create a new proof of history record
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::submit_data())]
		pub fn submit_data(origin: OriginFor<T>, data: Vec<u8>) -> DispatchResult {
			let submitter = ensure_signed(origin)?;
			
			// Hash the input data
			let data_hash = T::Hasher::hash(&data);
			
			// Check if this data has already been included in a proof
			ensure!(!DataToProof::<T>::contains_key(data_hash), Error::<T>::ProofAlreadyExists);
			
			// Get the previous proof hash or create initial value if this is the first proof
			let previous_hash = Self::latest_proof().unwrap_or_else(|| <T as pallet::Config>::Hash::default());
			
			// Get current block number and timestamp
			let block_number = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
			let now = T::Time::now();
			
			// Combine previous hash with data hash and other parameters to create a new proof
			let mut combined = data_hash.encode();
			combined.extend_from_slice(&previous_hash.encode());
			combined.extend_from_slice(&block_number.encode());
			combined.extend_from_slice(&now.encode());
			
			// Create the new proof hash
			let proof_hash = T::Hasher::hash(&combined);
			
			// Create proof record
			let proof_record = ProofRecord {
				data_hash,
				previous_hash,
				proof_hash,
				block_number,
				timestamp: now,
				submitter: submitter.clone(),
			};
			
			// Increment proof count
			let count = Self::proof_count();
			ProofCount::<T>::put(count + 1);
			
			// Store proof and mappings
			Proofs::<T>::insert(proof_hash, proof_record);
			DataToProof::<T>::insert(data_hash, proof_hash);
			LatestProof::<T>::put(proof_hash);
			
			// Emit event
			Self::deposit_event(Event::ProofCreated(proof_hash, data_hash, block_number));
			
			Ok(())
		}
		
		/// Verify a proof record
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::verify_proof())]
		pub fn verify_proof(origin: OriginFor<T>, proof_hash: <T as pallet::Config>::Hash) -> DispatchResult {
			let verifier = ensure_signed(origin)?;
			
			// Check if proof exists
			ensure!(Proofs::<T>::contains_key(proof_hash), Error::<T>::ProofNotFound);
			
			let proof = Proofs::<T>::get(proof_hash).unwrap();
			
			// If it's not the initial proof, validate the chain
			if proof.previous_hash != <T as pallet::Config>::Hash::default() {
				// Verify that the previous proof exists
				ensure!(
					Proofs::<T>::contains_key(proof.previous_hash),
					Error::<T>::InvalidProofChain
				);
			}
			
			// Recreate the hash to validate
			let mut combined = proof.data_hash.encode();
			combined.extend_from_slice(&proof.previous_hash.encode());
			combined.extend_from_slice(&proof.block_number.encode());
			combined.extend_from_slice(&proof.timestamp.encode());
			
			let calculated_hash = T::Hasher::hash(&combined);
			
			// Verify hash matches the stored proof hash
			ensure!(
				calculated_hash == proof_hash,
				Error::<T>::InvalidProofChain
			);
			
			// Emit success event
			Self::deposit_event(Event::ProofVerified(proof_hash, verifier));
			
			Ok(())
		}
		
		/// Get a data's proof history
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::get_proof_by_data())]
		pub fn get_proof_by_data(origin: OriginFor<T>, data_hash: <T as pallet::Config>::Hash) -> DispatchResult {
			let _caller = ensure_signed(origin)?;
			
			// Check if data has a proof
			ensure!(
				DataToProof::<T>::contains_key(data_hash),
				Error::<T>::ProofNotFound
			);
			
			// This function just verifies the data exists in the system
			// The actual proof can be queried from storage
			
			Ok(())
		}
	}
}
