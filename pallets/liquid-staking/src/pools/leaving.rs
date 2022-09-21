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

use super::*;

pub fn shares_to_stake<T: Config>(
	candidate: &CandidateGen<T>,
	shares: &T::Balance,
) -> Result<T::Balance, Error<T>> {
	let total_staked = LeavingSharesTotalStaked::<T>::get(candidate);
	let supply = LeavingSharesSupply::<T>::get(candidate);
	ensure!(!supply.is_zero(), Error::NoOneIsStaking);

	shares
		.mul_div(total_staked, supply)
		.ok_or(Error::MathOverflow)
}

pub fn stake_to_shares<T: Config>(
	candidate: &CandidateGen<T>,
	stake: &T::Balance,
) -> Result<T::Balance, Error<T>> {
	let total_staked = LeavingSharesTotalStaked::<T>::get(candidate);
	let supply = LeavingSharesSupply::<T>::get(candidate);
	ensure!(!total_staked.is_zero(), Error::NoOneIsStaking);

	stake
		.mul_div(supply, total_staked)
		.ok_or(Error::MathOverflow)
}

/// Add stake in the leaving pool of this Candidate.
/// Accept stake instead of shares since we want to deal with rounding.
/// Returns the amount of shares created.
fn add_stake<T: Config>(
	candidate: CandidateGen<T>,
	delegator: Delegator<T>,
	stake: T::Balance,
) -> Result<T::Balance, Error<T>> {
	ensure!(!Zero::is_zero(&stake), Error::StakeMustBeNonZero);

	let shares_supply = LeavingSharesSupply::<T>::get(&candidate);

	let shares = if Zero::is_zero(&shares_supply) {
		stake // By default 1 share = 1 coin
	} else {
		// Number of shares might be rounded down / corresponds to slightly less stake.
		// But since we want to put all stake in the leaving state, we will not correct this
		// and spread the rounding among all leaving delegators.
		stake_to_shares(&candidate, &stake)?
	};

	super::add_staked::<T, LeavingSharesSupply<T>, LeavingShares<T>, LeavingSharesTotalStaked<T>>(
		&candidate, &delegator, shares, stake,
	)?;

	Ok(shares)
}

/// Remove shares from the leaving pool of this Candidate.
/// Accept shares since the leaving queue deal with shares to support slashing.
/// Returns value of removed shares.
fn sub_shares<T: Config>(
	candidate: CandidateGen<T>,
	delegator: Delegator<T>,
	shares: T::Balance,
) -> Result<T::Balance, Error<T>> {
	ensure!(!Zero::is_zero(&shares), Error::StakeMustBeNonZero);

	let stake = shares_to_stake(&candidate, &shares)?;

	super::sub_staked::<T, LeavingSharesSupply<T>, LeavingShares<T>, LeavingSharesTotalStaked<T>>(
		&candidate, &delegator, shares, stake,
	)?;

	Ok(stake)
}

pub(crate) fn register_leaving<T: Config>(
	candidate: CandidateGen<T>,
	delegator: Delegator<T>,
	stake: T::Balance,
) -> Result<(), Error<T>> {
	let leaving_shares = add_stake::<T>(candidate.clone(), delegator.clone(), stake)?;

	let block_number = frame_system::Pallet::<T>::block_number();

	let already_leaving_shares = LeavingRequests::<T>::get((&candidate, &delegator, block_number));

	let new_leaving_shares = already_leaving_shares
		.checked_add(&leaving_shares)
		.ok_or(Error::MathOverflow)?;

	LeavingRequests::<T>::insert((&candidate, &delegator, block_number), new_leaving_shares);

	Pallet::<T>::deposit_event(Event::<T>::RegisteredLeaving {
		candidate,
		delegator,
		stake,
		leaving_shares,
		total_leaving_shares: new_leaving_shares,
	});

	Ok(())
}

pub(crate) fn execute_leaving<T: Config>(
	candidate: CandidateGen<T>,
	delegator: Delegator<T>,
	at_block: T::BlockNumber,
) -> Result<T::Balance, Error<T>> {
	let block_number = frame_system::Pallet::<T>::block_number();

	let release_block_number = at_block
		.checked_add(&T::LeavingDelay::get())
		.ok_or(Error::MathOverflow)?;

	ensure!(
		block_number >= release_block_number,
		Error::TryingToLeaveTooSoon
	);

	let shares = LeavingRequests::<T>::get((&candidate, &delegator, at_block));
	let stake = sub_shares(candidate.clone(), delegator.clone(), shares)?;

	LeavingRequests::<T>::remove((&candidate, &delegator, at_block));

	Pallet::<T>::deposit_event(Event::<T>::ExecutedLeaving {
		candidate,
		delegator,
		stake,
		leaving_shares: shares,
		requested_at: at_block,
	});

	Ok(stake)
}

pub(crate) fn cancel_leaving<T: Config>(
	candidate: CandidateGen<T>,
	delegator: Delegator<T>,
	at_block: T::BlockNumber,
) -> Result<T::Balance, Error<T>> {
	let shares = LeavingRequests::<T>::get((&candidate, &delegator, at_block));
	let stake = sub_shares(candidate.clone(), delegator.clone(), shares)?;

	LeavingRequests::<T>::remove((&candidate, &delegator, at_block));

	Pallet::<T>::deposit_event(Event::<T>::CanceledLeaving {
		candidate,
		delegator,
		stake,
		leaving_shares: shares,
		requested_at: at_block,
	});

	Ok(stake)
}

pub fn shares<T: Config>(candidate: &CandidateGen<T>, delegator: &Delegator<T>) -> T::Balance {
	LeavingShares::<T>::get(candidate, delegator)
}

pub fn stake<T: Config>(
	candidate: &CandidateGen<T>,
	delegator: &Delegator<T>,
) -> Result<T::Balance, Error<T>> {
	let shares = shares::<T>(candidate, delegator);

	if shares.is_zero() {
		return Ok(Zero::zero());
	}

	shares_to_stake(candidate, &shares)
}
