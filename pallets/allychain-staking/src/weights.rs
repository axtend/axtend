// Copyright 2019-2022 PureStake Inc.
// This file is part of Axtend.

// Axtend is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axtend is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axtend.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for allychain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-19, STEPS: `[32, ]`, REPEAT: 64, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/axtend
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// allychain_staking
// --extrinsic
// *
// --steps
// 32
// --repeat
// 64
// --raw
// --template=./benchmarking/frame-weight-template.hbs
// --output
// /tmp/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for allychain_staking.
pub trait WeightInfo {
	fn hotfix_remove_delegation_requests(x: u32) -> Weight;
	fn hotfix_update_candidate_pool_value(x: u32) -> Weight;
	fn set_staking_expectations() -> Weight;
	fn set_inflation() -> Weight;
	fn set_allychain_bond_account() -> Weight;
	fn set_allychain_bond_reserve_percent() -> Weight;
	fn set_total_selected() -> Weight;
	fn set_collator_commission() -> Weight;
	fn set_blocks_per_round() -> Weight;
	fn join_candidates(x: u32) -> Weight;
	fn schedule_leave_candidates(x: u32) -> Weight;
	fn execute_leave_candidates(x: u32) -> Weight;
	fn cancel_leave_candidates(x: u32) -> Weight;
	fn go_offline() -> Weight;
	fn go_online() -> Weight;
	fn candidate_bond_more() -> Weight;
	fn schedule_candidate_bond_less() -> Weight;
	fn execute_candidate_bond_less() -> Weight;
	fn cancel_candidate_bond_less() -> Weight;
	fn delegate(x: u32, y: u32) -> Weight;
	fn schedule_leave_delegators() -> Weight;
	fn execute_leave_delegators(x: u32) -> Weight;
	fn cancel_leave_delegators() -> Weight;
	fn schedule_revoke_delegation() -> Weight;
	fn delegator_bond_more() -> Weight;
	fn schedule_delegator_bond_less() -> Weight;
	fn execute_revoke_delegation() -> Weight;
	fn execute_delegator_bond_less() -> Weight;
	fn cancel_revoke_delegation() -> Weight;
	fn cancel_delegator_bond_less() -> Weight;
	fn round_transition_on_initialize(x: u32, y: u32) -> Weight;
	fn base_on_initialize() -> Weight;
	fn pay_one_collator_reward(y: u32) -> Weight;
}

/// Weights for allychain_staking using the Axlib node and recommended hardware.
pub struct AxlibWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AxlibWeight<T> {
	fn hotfix_remove_delegation_requests(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 3_000
			.saturating_add((8_132_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	fn hotfix_update_candidate_pool_value(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 147_000
			.saturating_add((26_825_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_staking_expectations() -> Weight {
		(20_719_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_inflation() -> Weight {
		(63_011_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_allychain_bond_account() -> Weight {
		(20_434_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_allychain_bond_reserve_percent() -> Weight {
		(19_239_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_total_selected() -> Weight {
		(18_402_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_collator_commission() -> Weight {
		(18_178_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_blocks_per_round() -> Weight {
		(65_939_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn join_candidates(x: u32) -> Weight {
		(80_619_000 as Weight) // Standard Error: 1_000
			.saturating_add((107_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn schedule_leave_candidates(x: u32) -> Weight {
		(50_933_000 as Weight) // Standard Error: 1_000
			.saturating_add((108_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn execute_leave_candidates(x: u32) -> Weight {
		(8_634_000 as Weight) // Standard Error: 6_000
			.saturating_add((26_979_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn cancel_leave_candidates(x: u32) -> Weight {
		(43_482_000 as Weight) // Standard Error: 0
			.saturating_add((111_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn go_offline() -> Weight {
		(30_778_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn go_online() -> Weight {
		(31_178_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn candidate_bond_more() -> Weight {
		(53_492_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn schedule_candidate_bond_less() -> Weight {
		(29_393_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn execute_candidate_bond_less() -> Weight {
		(62_395_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn cancel_candidate_bond_less() -> Weight {
		(25_564_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn delegate(x: u32, y: u32) -> Weight {
		(103_760_000 as Weight) // Standard Error: 12_000
			.saturating_add((198_000 as Weight).saturating_mul(x as Weight)) // Standard Error: 3000
			.saturating_add((112_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn schedule_leave_delegators() -> Weight {
		(30_908_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn execute_leave_delegators(x: u32) -> Weight {
		(1_091_000 as Weight) // Standard Error: 14_000
			.saturating_add((37_192_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn cancel_leave_delegators() -> Weight {
		(26_796_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn schedule_revoke_delegation() -> Weight {
		(37_580_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn delegator_bond_more() -> Weight {
		(65_757_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn schedule_delegator_bond_less() -> Weight {
		(70_859_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn execute_revoke_delegation() -> Weight {
		(87_836_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn execute_delegator_bond_less() -> Weight {
		(80_983_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn cancel_revoke_delegation() -> Weight {
		(37_923_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn cancel_delegator_bond_less() -> Weight {
		(70_813_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn round_transition_on_initialize(x: u32, y: u32) -> Weight {
		(0 as Weight) // Standard Error: 1_560_000
			.saturating_add((62_210_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 4_000
			.saturating_add((208_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(168 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(159 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	fn base_on_initialize() -> Weight {
		(4_913_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn pay_one_collator_reward(y: u32) -> Weight {
		(44_854_000 as Weight) // Standard Error: 4_000
			.saturating_add((16_772_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn hotfix_remove_delegation_requests(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 3_000
			.saturating_add((8_132_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	fn hotfix_update_candidate_pool_value(x: u32) -> Weight {
		(0 as Weight) // Standard Error: 147_000
			.saturating_add((26_825_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_staking_expectations() -> Weight {
		(20_719_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_inflation() -> Weight {
		(63_011_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_allychain_bond_account() -> Weight {
		(20_434_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_allychain_bond_reserve_percent() -> Weight {
		(19_239_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_total_selected() -> Weight {
		(18_402_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_collator_commission() -> Weight {
		(18_178_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_blocks_per_round() -> Weight {
		(65_939_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn join_candidates(x: u32) -> Weight {
		(80_619_000 as Weight) // Standard Error: 1_000
			.saturating_add((107_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn schedule_leave_candidates(x: u32) -> Weight {
		(50_933_000 as Weight) // Standard Error: 1_000
			.saturating_add((108_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn execute_leave_candidates(x: u32) -> Weight {
		(8_634_000 as Weight) // Standard Error: 6_000
			.saturating_add((26_979_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn cancel_leave_candidates(x: u32) -> Weight {
		(43_482_000 as Weight) // Standard Error: 0
			.saturating_add((111_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn go_offline() -> Weight {
		(30_778_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn go_online() -> Weight {
		(31_178_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn candidate_bond_more() -> Weight {
		(53_492_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn schedule_candidate_bond_less() -> Weight {
		(29_393_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn execute_candidate_bond_less() -> Weight {
		(62_395_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn cancel_candidate_bond_less() -> Weight {
		(25_564_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn delegate(x: u32, y: u32) -> Weight {
		(103_760_000 as Weight) // Standard Error: 12_000
			.saturating_add((198_000 as Weight).saturating_mul(x as Weight)) // Standard Error: 3000
			.saturating_add((112_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn schedule_leave_delegators() -> Weight {
		(30_908_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn execute_leave_delegators(x: u32) -> Weight {
		(1_091_000 as Weight) // Standard Error: 14_000
			.saturating_add((37_192_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
	fn cancel_leave_delegators() -> Weight {
		(26_796_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn schedule_revoke_delegation() -> Weight {
		(37_580_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn delegator_bond_more() -> Weight {
		(65_757_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn schedule_delegator_bond_less() -> Weight {
		(70_859_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn execute_revoke_delegation() -> Weight {
		(87_836_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn execute_delegator_bond_less() -> Weight {
		(80_983_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn cancel_revoke_delegation() -> Weight {
		(37_923_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn cancel_delegator_bond_less() -> Weight {
		(70_813_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn round_transition_on_initialize(x: u32, y: u32) -> Weight {
		(0 as Weight) // Standard Error: 1_560_000
			.saturating_add((62_210_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 4_000
			.saturating_add((208_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(168 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(159 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	fn base_on_initialize() -> Weight {
		(4_913_000 as Weight).saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn pay_one_collator_reward(y: u32) -> Weight {
		(44_854_000 as Weight) // Standard Error: 4_000
			.saturating_add((16_772_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
}
