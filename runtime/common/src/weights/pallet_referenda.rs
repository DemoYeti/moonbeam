// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("moonbase-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_referenda
// --extrinsic=*
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: `Referenda::ReferendumCount` (r:1 w:1)
	/// Proof: `Referenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:0 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `42428`
		// Minimum execution time: 44_277_000 picoseconds.
		Weight::from_parts(45_432_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `477`
		//  Estimated: `83866`
		// Minimum execution time: 58_428_000 picoseconds.
		Weight::from_parts(59_624_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3275`
		//  Estimated: `42428`
		// Minimum execution time: 79_899_000 picoseconds.
		Weight::from_parts(81_723_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3295`
		//  Estimated: `42428`
		// Minimum execution time: 79_261_000 picoseconds.
		Weight::from_parts(81_856_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `477`
		//  Estimated: `83866`
		// Minimum execution time: 69_762_000 picoseconds.
		Weight::from_parts(70_557_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `477`
		//  Estimated: `83866`
		// Minimum execution time: 66_308_000 picoseconds.
		Weight::from_parts(67_542_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `387`
		//  Estimated: `4377`
		// Minimum execution time: 37_261_000 picoseconds.
		Weight::from_parts(37_868_000, 0)
			.saturating_add(Weight::from_parts(0, 4377))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `4377`
		// Minimum execution time: 36_492_000 picoseconds.
		Weight::from_parts(37_398_000, 0)
			.saturating_add(Weight::from_parts(0, 4377))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `83866`
		// Minimum execution time: 39_964_000 picoseconds.
		Weight::from_parts(40_804_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:1 w:0)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `786`
		//  Estimated: `83866`
		// Minimum execution time: 130_877_000 picoseconds.
		Weight::from_parts(132_485_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Referenda::TrackQueue` (r:1 w:0)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `102`
		//  Estimated: `5477`
		// Minimum execution time: 12_438_000 picoseconds.
		Weight::from_parts(12_694_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3074`
		//  Estimated: `42428`
		// Minimum execution time: 56_134_000 picoseconds.
		Weight::from_parts(58_265_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3060`
		//  Estimated: `42428`
		// Minimum execution time: 57_607_000 picoseconds.
		Weight::from_parts(60_467_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2915`
		//  Estimated: `5477`
		// Minimum execution time: 27_078_000 picoseconds.
		Weight::from_parts(28_277_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2915`
		//  Estimated: `5477`
		// Minimum execution time: 27_734_000 picoseconds.
		Weight::from_parts(28_933_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2919`
		//  Estimated: `5477`
		// Minimum execution time: 35_698_000 picoseconds.
		Weight::from_parts(37_297_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2939`
		//  Estimated: `5477`
		// Minimum execution time: 35_449_000 picoseconds.
		Weight::from_parts(37_207_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `42428`
		// Minimum execution time: 26_816_000 picoseconds.
		Weight::from_parts(27_395_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `42428`
		// Minimum execution time: 27_339_000 picoseconds.
		Weight::from_parts(27_888_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194`
		//  Estimated: `4377`
		// Minimum execution time: 17_555_000 picoseconds.
		Weight::from_parts(18_006_000, 0)
			.saturating_add(Weight::from_parts(0, 4377))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `42428`
		// Minimum execution time: 35_245_000 picoseconds.
		Weight::from_parts(35_981_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `42428`
		// Minimum execution time: 37_616_000 picoseconds.
		Weight::from_parts(38_341_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `42428`
		// Minimum execution time: 31_221_000 picoseconds.
		Weight::from_parts(31_820_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `321`
		//  Estimated: `42428`
		// Minimum execution time: 31_088_000 picoseconds.
		Weight::from_parts(31_845_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `42428`
		// Minimum execution time: 29_206_000 picoseconds.
		Weight::from_parts(29_538_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342`
		//  Estimated: `42428`
		// Minimum execution time: 29_150_000 picoseconds.
		Weight::from_parts(29_642_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342`
		//  Estimated: `83866`
		// Minimum execution time: 44_230_000 picoseconds.
		Weight::from_parts(45_409_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `42428`
		// Minimum execution time: 31_144_000 picoseconds.
		Weight::from_parts(31_751_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:0 w:1)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `4377`
		// Minimum execution time: 24_832_000 picoseconds.
		Weight::from_parts(25_304_000, 0)
			.saturating_add(Weight::from_parts(0, 4377))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:1 w:1)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `4377`
		// Minimum execution time: 19_713_000 picoseconds.
		Weight::from_parts(20_164_000, 0)
			.saturating_add(Weight::from_parts(0, 4377))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
