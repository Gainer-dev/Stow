//! Deposits — convert vault-token in for shares of the adapter.

use soroban_sdk::{Address, Env};

use crate::error::Error;
use crate::types::Position;

/// Deposit `amount` of the vault token, minting shares to `from` at the
/// current exchange rate.
///
/// - `from.require_auth()`.
/// - Errors `Error::Paused` while paused (see `admin::set_paused`).
/// - Errors `Error::InvalidAmount` if `amount <= 0`.
/// - Errors `Error::StrategyCapExceeded` if the active strategy has a
///   nonzero deposit cap and the resulting total deployed balance would
///   exceed it.
/// - Shares minted = `accounting::convert_to_shares(amount)`, computed
///   **before** `amount` is transferred in (so the deposit itself does not
///   move the exchange rate against the depositor).
/// - Transfers `amount` in, then forwards it to the active strategy (if
///   any); with no active strategy, funds sit idle in the adapter and
///   accrue no yield until one is set.
/// - Creates the position on first deposit; increments `shares` otherwise.
/// - Emits a `deposited` event.
///
/// TODO(issue): implement. Compare with `savings-vault::flexible::deposit`
/// for the auth/validation/storage shape — the share-minting math is new to
/// this crate and has no direct analogue there.
pub fn deposit(_env: &Env, _from: Address, _amount: i128) -> Result<i128, Error> {
    unimplemented!("deposit: deposit")
}

/// Read `owner`'s position, or `Error::NotFound`.
///
/// TODO(issue): implement.
pub fn get_position(_env: &Env, _owner: Address) -> Result<Position, Error> {
    unimplemented!("deposit: get_position")
}
