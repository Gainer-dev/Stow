//! Harvest — report yield (or loss) from the active strategy and move the
//! exchange rate accordingly.

use soroban_sdk::{Address, Env};

use crate::error::Error;

/// Report the active strategy's current yield and update accounting.
///
/// - Callable by anyone (permissionless "keeper" pattern — no `require_auth`
///   check on `caller` beyond passing it through to the `harvested` event
///   for attribution). Rate-limited by [`check_harvest_interval`] instead of
///   an allowlist, so no single keeper is a liveness dependency.
/// - Errors `Error::Paused` while paused.
/// - Compares the strategy's reported balance against the adapter's tracked
///   deployed balance:
///   - **Positive** delta (yield): apply [`apply_performance_fee`], then
///     grow `total_assets()` by the post-fee remainder. Depositors' shares
///     are unchanged; `exchange_rate` rises.
///   - **Negative** delta (loss): shrink `total_assets()` by the full loss.
///     No fee is charged on a loss — see `apply_performance_fee` doc for
///     why that matters.
/// - Emits a `harvested` event with the signed delta and fee taken (`0` on
///   a loss).
///
/// TODO(issue): implement.
pub fn harvest(_env: &Env, _caller: Address) -> Result<i128, Error> {
    unimplemented!("harvest: harvest")
}

/// Apply the configured performance fee to a **positive** yield amount,
/// crediting the fee to `DataKey::FeesAccrued` and returning the
/// depositor-facing remainder.
///
/// Must only ever be called with `yield_amount > 0`. Charging a fee on a
/// loss (or on a wash — yield that merely offsets a prior loss) would let
/// the treasury extract value that was never actually earned, at
/// depositors' expense.
///
/// TODO(issue): implement.
pub fn apply_performance_fee(_env: &Env, _yield_amount: i128) -> Result<i128, Error> {
    unimplemented!("harvest: apply_performance_fee")
}

/// Guard for `harvest`: errors `Error::HarvestTooSoon` if fewer than
/// `admin::harvest_interval()` seconds have elapsed since
/// `DataKey::LastHarvestAt`. Exists so a griefer can't spam `harvest` calls
/// to burn the adapter's ledger-write budget; a legitimate keeper only needs
/// to call it a few times a day at most.
///
/// TODO(issue): implement.
pub fn check_harvest_interval(_env: &Env) -> Result<(), Error> {
    unimplemented!("harvest: check_harvest_interval")
}
