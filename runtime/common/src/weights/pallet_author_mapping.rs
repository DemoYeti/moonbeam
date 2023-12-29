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
//! Autogenerated weights for `pallet_author_mapping`
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
// --pallet=pallet_author_mapping
// --extrinsic=*
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_author_mapping`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_author_mapping::WeightInfo for WeightInfo<T> {
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `AuthorMapping::NimbusLookup` (r:0 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `3841`
		// Minimum execution time: 37_889_000 picoseconds.
		Weight::from_parts(38_869_000, 0)
			.saturating_add(Weight::from_parts(0, 3841))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:2 w:2)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::NimbusLookup` (r:0 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `6265`
		// Minimum execution time: 23_616_000 picoseconds.
		Weight::from_parts(24_084_000, 0)
			.saturating_add(Weight::from_parts(0, 6265))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `AuthorMapping::NimbusLookup` (r:0 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn clear_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `453`
		//  Estimated: `3918`
		// Minimum execution time: 39_104_000 picoseconds.
		Weight::from_parts(40_002_000, 0)
			.saturating_add(Weight::from_parts(0, 3918))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AuthorMapping::NimbusLookup` (r:1 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn remove_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `547`
		//  Estimated: `4012`
		// Minimum execution time: 43_496_000 picoseconds.
		Weight::from_parts(44_154_000, 0)
			.saturating_add(Weight::from_parts(0, 4012))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AuthorMapping::NimbusLookup` (r:1 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432`
		//  Estimated: `3897`
		// Minimum execution time: 41_679_000 picoseconds.
		Weight::from_parts(42_285_000, 0)
			.saturating_add(Weight::from_parts(0, 3897))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
