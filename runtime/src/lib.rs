// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A minimal runtime that includes the template [`pallet`](`pallet_healer_network`).

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;
use alloc::vec::Vec;
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use polkadot_sdk::{
	frame_support::{
		traits::Get,
	},
	polkadot_sdk_frame::{
		self as frame,
		deps::{
			sp_genesis_builder,
			sp_core,
		},
		runtime::{apis, prelude::*},
	},
	*,
};

// Local pallets
pub use pallet_healer_network;
// pub use pallet_poh;

/// An index to a block.
pub type BlockNumber = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

pub use polkadot_sdk::{
	frame_support:: {
		traits::{
			Currency, EstimateNextNewSession, Imbalance, IsSubType, KeyOwnerProofSystem,
			LockIdentifier, Nothing, OnUnbalanced, ValidatorSet, VariantCountOf,
		},
		weights::{
			constants::{
				BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
			},
			IdentityFee, Weight,
		},
	}
};

/// Provides getters for genesis configuration presets.
pub mod genesis_config_presets {
	use super::*;
	use crate::{
		interface::{Balance, MinimumBalance},
		sp_keyring::AccountKeyring,
		BalancesConfig, RuntimeGenesisConfig, SudoConfig,
	};

	use alloc::{vec, vec::Vec};
	use serde_json::Value;

	/// Returns a development genesis config preset.
	pub fn development_config_genesis() -> Value {
		let endowment = <MinimumBalance as Get<Balance>>::get().max(1) * 1000;
		let config = RuntimeGenesisConfig {
			balances: BalancesConfig {
				balances: AccountKeyring::iter()
					.map(|a| (a.to_account_id(), endowment))
					.collect::<Vec<_>>(),
			},
			sudo: SudoConfig { key: Some(AccountKeyring::Alice.to_account_id()) },
			..Default::default()
		};

		serde_json::to_value(config).expect("Could not build genesis config.")
	}

	/// Get the set of the available genesis config presets.
	pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
		let patch = match id.as_ref() {
			sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
			_ => return None,
		};
		Some(
			serde_json::to_string(&patch)
				.expect("serialization to json is expected to work. qed.")
				.into_bytes(),
		)
	}

	/// List of supported presets.
	pub fn preset_names() -> Vec<PresetId> {
		vec![PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET)]
	}
}

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("healer-network-runtime"),
	impl_name: alloc::borrow::Cow::Borrowed("healer-network-runtime"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 0,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	system_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// The transaction extensions that are added to the runtime.
type TxExtension = (
	// Checks that the sender is not the zero address.
	frame_system::CheckNonZeroSender<Runtime>,
	// Checks that the runtime version is correct.
	frame_system::CheckSpecVersion<Runtime>,
	// Checks that the transaction version is correct.
	frame_system::CheckTxVersion<Runtime>,
	// Checks that the genesis hash is correct.
	frame_system::CheckGenesis<Runtime>,
	// Checks that the era is valid.
	frame_system::CheckEra<Runtime>,
	// Checks that the nonce is valid.
	frame_system::CheckNonce<Runtime>,
	// Checks that the weight is valid.
	frame_system::CheckWeight<Runtime>,
	// Ensures that the sender has enough funds to pay for the transaction
	// and deducts the fee from the sender's account.
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

// Composes the runtime by adding all the used pallets and deriving necessary types.
#[frame_construct_runtime]
mod runtime {
	/// The main runtime type.
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask
	)]
	pub struct Runtime;

	/// Mandatory system pallet that should always be included in a FRAME runtime.
	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<Runtime>;

	/// Provides a way for consensus systems to set and check the onchain time.
	#[runtime::pallet_index(1)]
	pub type Timestamp = pallet_timestamp::Pallet<Runtime>;

	/// Provides the ability to keep track of balances.
	#[runtime::pallet_index(2)]
	pub type Balances = pallet_balances::Pallet<Runtime>;

	/// Provides a way to execute privileged functions.
	#[runtime::pallet_index(3)]
	pub type Sudo = pallet_sudo::Pallet<Runtime>;

	/// Provides the ability to charge for extrinsic execution.
	#[runtime::pallet_index(4)]
	pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;

	/// A minimal pallet template.
	#[runtime::pallet_index(50)]
	pub type Template = pallet_healer_network::Pallet<Runtime>;

	// / Proof of History pallet.
	// #[runtime::pallet_index(51)]
	// pub type PoH = pallet_poh::Pallet<Runtime>;
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

/// Implements the types required for the system pallet.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
	type Version = Version;

	// Use the account data from the balances pallet
	type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
	type Nonce = u32;

	// This is needed to make pjs-apps work and send txs
	type AccountId = frame::runtime::types_common::AccountId;
}

// Implements the types required for the balances pallet.
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
	// Store the accounts in the system pallet -- this is a common practice in FRAME-based runtimes.
	// See `pallet_balances::Account` for more information.
	type AccountStore = System;
	
	type Balance = u128;
	type ExistentialDeposit = ConstU128<10>;
}

// Implements the types required for the sudo pallet.
#[derive_impl(pallet_sudo::config_preludes::TestDefaultConfig)]
impl pallet_sudo::Config for Runtime {
}

// Implements the types required for the transaction payment pallet.
#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig)]
impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
	// Setting fee as independent of the weight of the extrinsic for demo purposes
	type WeightToFee = NoFee<<Self as pallet_balances::Config>::Balance>;
	// Setting fee as fixed for any length of the call data for demo purposes
	type LengthToFee = FixedFee<1, <Self as pallet_balances::Config>::Balance>;
}

// Implements the types required for the template pallet.
impl pallet_healer_network::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_healer_network::weights::SubstrateWeight<Runtime>;
}

// Implements the types required for the PoH pallet.
// impl pallet_poh::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Hasher = <Runtime as pallet_poh::Config>::Hasher;
// 	type Time = <Runtime as pallet_poh::Config>::Time;
// 	type Hash = <Runtime as pallet_poh::Config>::Hash;
// 	type WeightInfo = pallet_poh::weights::SubstrateWeight<Runtime>;
// }

// Implements the types required for the timestamp pallet. This is required for PJS.
#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
impl pallet_timestamp::Config for Runtime {}

type Block = frame::runtime::types_common::BlockOf<Runtime, TxExtension>;
type Header = HeaderFor<Runtime>;

type RuntimeExecutive =
	Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

impl_runtime_apis! {
	impl apis::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			RuntimeExecutive::execute_block(block)
		}

		fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
			RuntimeExecutive::initialize_block(header)
		}
	}
	impl apis::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl apis::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
			RuntimeExecutive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> HeaderFor<Runtime> {
			RuntimeExecutive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: InherentData,
		) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: ExtrinsicFor<Runtime>,
			block_hash: <Runtime as frame_system::Config>::Hash,
		) -> TransactionValidity {
			RuntimeExecutive::validate_transaction(source, tx, block_hash)
		}
	}

	impl apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &HeaderFor<Runtime>) {
			RuntimeExecutive::offchain_worker(header)
		}
	}

	impl apis::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, apis::KeyTypeId)>> {
			Default::default()
		}
	}

	impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
		fn account_nonce(account: interface::AccountId) -> interface::Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		interface::Balance,
	> for Runtime {
		fn query_info(uxt: ExtrinsicFor<Runtime>, len: u32) -> RuntimeDispatchInfo<interface::Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: ExtrinsicFor<Runtime>, len: u32) -> FeeDetails<interface::Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> interface::Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> interface::Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl apis::GenesisBuilder<Block> for Runtime {
		fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
			build_state::<RuntimeGenesisConfig>(config)
		}

		fn get_preset(id: &Option<PresetId>) -> Option<Vec<u8>> {
			get_preset::<RuntimeGenesisConfig>(id, self::genesis_config_presets::get_preset)
		}

		fn preset_names() -> Vec<PresetId> {
			self::genesis_config_presets::preset_names()
		}
	}
}

/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
///
/// Other types should preferably be private.
// TODO: this should be standardized in some way, see:
// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
pub mod interface {
	use super::Runtime;
	use polkadot_sdk::{polkadot_sdk_frame as frame, *};

	pub type Block = super::Block;
	pub use frame::runtime::types_common::OpaqueBlock;
	pub type AccountId = <Runtime as frame_system::Config>::AccountId;
	pub type Nonce = <Runtime as frame_system::Config>::Nonce;
	pub type Hash = <Runtime as frame_system::Config>::Hash;
	pub type Balance = <Runtime as pallet_balances::Config>::Balance;
	pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
