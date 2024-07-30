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

//! Autogenerated weights for `pallet_sudo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_sudo
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_sudo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sudo::WeightInfo for WeightInfo<T> {
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `1505`
		// Minimum execution time: 8_485_000 picoseconds.
		Weight::from_parts(8_731_000, 1505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn sudo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `1505`
		// Minimum execution time: 9_300_000 picoseconds.
		Weight::from_parts(9_558_000, 1505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn sudo_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `1505`
		// Minimum execution time: 9_183_000 picoseconds.
		Weight::from_parts(9_639_000, 1505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn remove_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `1505`
		// Minimum execution time: 7_629_000 picoseconds.
		Weight::from_parts(8_047_000, 1505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
