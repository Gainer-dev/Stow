//! Emergency circuit breaker — pull all funds out of the active strategy
//! back into the adapter's own idle balance, independent of the normal
//! `migrate_strategy` flow. Exists for the case where the active strategy
//! itself is misbehaving or believed compromised and admin wants funds out
//! *now*, without registering or trusting a replacement strategy first.

use soroban_sdk::{Address, Env};

use crate::error::Error;

/// Withdraw the adapter's entire deployed balance from the active strategy
/// back into the adapter's own custody, and clear `ActiveStrategy`. Does
/// **not** deregister the strategy (an admin can still inspect/re-activate
/// it after investigating). Admin-only.
///
/// - Requires `require_auth` from the current admin.
/// - Safe to call even if the strategy is unresponsive to normal calls,
///   *if* the strategy interface's withdraw entrypoint still functions —
///   this is a graceful pull, not a bypass of the strategy contract. A
///   fully unresponsive/malicious strategy that refuses to return funds is
///   out of scope for this contract to solve unilaterally; see
///   `README.md`'s "Strategy interface" trust notes.
/// - After this call, `deposit` continues to accept funds (held idle,
///   earning no yield) until an admin sets a new active strategy.
/// - Emits a `strategy_changed` event with `to: None`.
///
/// TODO(issue): implement.
pub fn emergency_withdraw_all(_env: &Env, _caller: Address) -> Result<i128, Error> {
    unimplemented!("circuit_breaker: emergency_withdraw_all")
}
