//! Strategy registry.
//!
//! A "strategy" is an external contract this adapter can route idle vault
//! funds into (e.g. a lending-market wrapper). Exactly one registered
//! strategy is ever "active" at a time — the one `deposit` and `harvest`
//! interact with. Others may stay registered (e.g. mid-migration) but never
//! receive funds. See `README.md`'s "Strategy interface" section for the
//! entrypoints a strategy contract must expose to be usable here.

use soroban_sdk::{Address, Env, String, Vec};

use crate::error::Error;
use crate::types::StrategyInfo;

/// Register a new strategy. Admin-only.
///
/// - Requires `require_auth` from the current admin.
/// - Errors `Error::StrategyAlreadyRegistered` if `address` is already
///   registered under a different id.
/// - Does **not** make the strategy active — call `set_active_strategy`
///   separately. This split lets an admin register and sanity-check a
///   strategy before routing real funds to it.
/// - Emits a `strategy_registered` event.
///
/// TODO(issue): implement.
pub fn register_strategy(
    _env: &Env,
    _caller: Address,
    _address: Address,
    _name: String,
) -> Result<u64, Error> {
    unimplemented!("strategy: register_strategy")
}

/// Deregister a strategy. Admin-only.
///
/// - Requires `require_auth` from the current admin.
/// - Errors `Error::StrategyActive` if `strategy_id` is the currently active
///   strategy — migrate away from it first via `migrate_strategy`.
/// - A deregistered strategy's id can never be re-registered or reactivated;
///   `deregistered_at` is permanent.
/// - Emits a `strategy_deregistered` event.
///
/// TODO(issue): implement.
pub fn deregister_strategy(_env: &Env, _caller: Address, _strategy_id: u64) -> Result<(), Error> {
    unimplemented!("strategy: deregister_strategy")
}

/// Set the active strategy when there is currently none (first activation
/// only — funds are not moved because there is nothing to move from).
/// Admin-only.
///
/// - Requires `require_auth` from the current admin.
/// - Errors `Error::StrategyAlreadyActive` if a strategy is already active
///   — use `migrate_strategy` to switch between two active strategies so
///   funds are moved atomically rather than stranded.
/// - Emits a `strategy_changed` event with `from: None`.
///
/// TODO(issue): implement.
pub fn set_active_strategy(_env: &Env, _caller: Address, _strategy_id: u64) -> Result<(), Error> {
    unimplemented!("strategy: set_active_strategy")
}

/// Move all deployed funds from the current active strategy to
/// `new_strategy_id` and make it active. Admin-only.
///
/// - Requires `require_auth` from the current admin.
/// - Withdraws the adapter's full balance from the old strategy, deposits it
///   into the new one.
/// - Must preserve `total_assets()` (module the old strategy's own
///   withdrawal fees/slippage, if any — see the "Strategy interface" doc for
///   how those are surfaced and accounted for).
/// - Emits a `strategy_changed` event with both `from` and `to` ids.
///
/// TODO(issue): implement.
pub fn migrate_strategy(_env: &Env, _caller: Address, _new_strategy_id: u64) -> Result<(), Error> {
    unimplemented!("strategy: migrate_strategy")
}

/// Set a per-strategy deposit cap, in vault-token stroops. `0` means
/// unlimited. Admin-only.
///
/// - Requires `require_auth` from the current admin.
/// - Enforced in `deposit` against the *active* strategy's cap only; a
///   non-active strategy's cap has no live effect.
///
/// TODO(issue): implement.
pub fn set_strategy_deposit_cap(
    _env: &Env,
    _caller: Address,
    _strategy_id: u64,
    _cap: i128,
) -> Result<(), Error> {
    unimplemented!("strategy: set_strategy_deposit_cap")
}

/// Read a strategy by id, or `Error::StrategyNotFound`.
///
/// TODO(issue): implement.
pub fn get_strategy(_env: &Env, _strategy_id: u64) -> Result<StrategyInfo, Error> {
    unimplemented!("strategy: get_strategy")
}

/// List all registered strategies (including deregistered ones — check
/// `deregistered_at` to filter).
///
/// TODO(issue): implement. Note: iterates `1..=NextStrategyId`; fine at
/// expected strategy counts (low single digits) but do not reuse this
/// pattern for anything with unbounded cardinality (e.g. positions).
pub fn list_strategies(_env: &Env) -> Vec<StrategyInfo> {
    unimplemented!("strategy: list_strategies")
}
