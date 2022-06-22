// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_nomination_pools
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_nomination_pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=./frame/nomination-pools/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nomination_pools.
pub trait WeightInfo {
	fn join() -> Weight;
	fn bond_extra_transfer() -> Weight;
	fn bond_extra_reward() -> Weight;
	fn claim_payout() -> Weight;
	fn unbond() -> Weight;
	fn pool_withdraw_unbonded(s: u32, ) -> Weight;
	fn withdraw_unbonded_update(s: u32, ) -> Weight;
	fn withdraw_unbonded_kill(s: u32, ) -> Weight;
	fn create() -> Weight;
	fn nominate(n: u32, ) -> Weight;
	fn set_state() -> Weight;
	fn set_metadata(n: u32, ) -> Weight;
	fn set_configs() -> Weight;
	fn update_roles() -> Weight;
}

/// Weights for pallet_nomination_pools using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: NominationPools MinJoinBond (r:1 w:0)
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:0)
	// Storage: System Account (r:2 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: BagsList ListBags (r:2 w:2)
	fn join() -> Weight {
		(124_114_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: BagsList ListBags (r:2 w:2)
	fn bond_extra_transfer() -> Weight {
		(115_734_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:2 w:2)
	fn bond_extra_reward() -> Weight {
		(127_873_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_payout() -> Weight {
		(51_510_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListBags (r:2 w:2)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	fn unbond() -> Weight {
		(120_744_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(18 as Weight))
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn pool_withdraw_unbonded(s: u32, ) -> Weight {
		(41_267_000 as Weight)
			// Standard Error: 0
			.saturating_add((53_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(82_234_000 as Weight)
			// Standard Error: 0
			.saturating_add((48_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(142_699_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(19 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: NominationPools MinCreateBond (r:1 w:0)
	// Storage: NominationPools MinJoinBond (r:1 w:0)
	// Storage: NominationPools MaxPools (r:1 w:0)
	// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools LastPoolId (r:1 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn create() -> Weight {
		(133_702_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(22 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListNodes (r:1 w:1)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	fn nominate(n: u32, ) -> Weight {
		(46_887_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((2_277_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:0)
	fn set_state() -> Weight {
		(24_030_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: NominationPools Metadata (r:1 w:1)
	// Storage: NominationPools CounterForMetadata (r:1 w:1)
	fn set_metadata(n: u32, ) -> Weight {
		(11_336_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: NominationPools MinJoinBond (r:0 w:1)
	// Storage: NominationPools MaxPoolMembers (r:0 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:0 w:1)
	// Storage: NominationPools MinCreateBond (r:0 w:1)
	// Storage: NominationPools MaxPools (r:0 w:1)
	fn set_configs() -> Weight {
		(2_927_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	fn update_roles() -> Weight {
		(19_184_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: NominationPools MinJoinBond (r:1 w:0)
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:0)
	// Storage: System Account (r:2 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: BagsList ListBags (r:2 w:2)
	fn join() -> Weight {
		(124_114_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(17 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: BagsList ListBags (r:2 w:2)
	fn bond_extra_transfer() -> Weight {
		(115_734_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:2 w:2)
	fn bond_extra_reward() -> Weight {
		(127_873_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_payout() -> Weight {
		(51_510_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListBags (r:2 w:2)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	fn unbond() -> Weight {
		(120_744_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(18 as Weight))
			.saturating_add(RocksDbWeight::get().writes(13 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn pool_withdraw_unbonded(s: u32, ) -> Weight {
		(41_267_000 as Weight)
			// Standard Error: 0
			.saturating_add((53_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(82_234_000 as Weight)
			// Standard Error: 0
			.saturating_add((48_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(142_699_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(19 as Weight))
			.saturating_add(RocksDbWeight::get().writes(16 as Weight))
	}
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: NominationPools MinCreateBond (r:1 w:0)
	// Storage: NominationPools MinJoinBond (r:1 w:0)
	// Storage: NominationPools MaxPools (r:1 w:0)
	// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	// Storage: NominationPools PoolMembers (r:1 w:1)
	// Storage: NominationPools LastPoolId (r:1 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: NominationPools RewardPools (r:1 w:1)
	// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn create() -> Weight {
		(133_702_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(22 as Weight))
			.saturating_add(RocksDbWeight::get().writes(15 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListNodes (r:1 w:1)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	fn nominate(n: u32, ) -> Weight {
		(46_887_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((2_277_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:0)
	fn set_state() -> Weight {
		(24_030_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:0)
	// Storage: NominationPools Metadata (r:1 w:1)
	// Storage: NominationPools CounterForMetadata (r:1 w:1)
	fn set_metadata(n: u32, ) -> Weight {
		(11_336_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: NominationPools MinJoinBond (r:0 w:1)
	// Storage: NominationPools MaxPoolMembers (r:0 w:1)
	// Storage: NominationPools MaxPoolMembersPerPool (r:0 w:1)
	// Storage: NominationPools MinCreateBond (r:0 w:1)
	// Storage: NominationPools MaxPools (r:0 w:1)
	fn set_configs() -> Weight {
		(2_927_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: NominationPools BondedPools (r:1 w:1)
	fn update_roles() -> Weight {
		(19_184_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
