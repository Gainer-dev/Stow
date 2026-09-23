//! Event topic registry.
//!
//! The off-chain indexer decodes contract events by topic. This module is
//! the canonical, compile-checked registry of topic name strings; the full
//! schema — data fields, types, encoding, and stability guarantees — is
//! documented in `README.md` under "Event schema". Keep the two in sync:
//! any change here (new topic, renamed topic) must be reflected there.
//!
//! Schema version: [`EVENT_SCHEMA_VERSION`]. Bump it whenever a change is
//! breaking for a decoder built against the previous version — see the
//! README's "Stability guarantees" for the exact rules.

/// Schema version for the event topics defined below, documented in
/// `README.md` ("Event schema"). A decoder should read this from the
/// `init` event's data payload to confirm it matches what it was built
/// against before trusting subsequent events on a given contract instance.
pub const EVENT_SCHEMA_VERSION: u32 = 1;

// --- lifecycle -------------------------------------------------------------
/// Emitted once, at the end of a successful `initialize` call.
pub const TOPIC_INIT: &str = "init";
/// Emitted at the end of a successful `set_admin` call.
pub const TOPIC_ADMIN_SET: &str = "admin_set";
/// Emitted at the end of a successful `set_paused` call.
pub const TOPIC_PAUSED_CHANGED: &str = "paused_changed";
/// Emitted at the end of a successful `upgrade` call.
pub const TOPIC_UPGRADED: &str = "upgraded";

// --- deposit / withdraw ------------------------------------------------------
/// Emitted at the end of a successful `deposit` call.
pub const TOPIC_DEPOSITED: &str = "deposited";
/// Emitted at the end of a successful `request_withdraw` call.
pub const TOPIC_WITHDRAW_REQUESTED: &str = "withdraw_requested";
/// Emitted at the end of a successful `claim_withdraw` call.
pub const TOPIC_WITHDRAW_CLAIMED: &str = "withdraw_claimed";
/// Emitted at the end of a successful `cancel_withdraw` call.
pub const TOPIC_WITHDRAW_CANCELLED: &str = "withdraw_cancelled";

// --- strategy ----------------------------------------------------------------
/// Emitted at the end of a successful `register_strategy` call.
pub const TOPIC_STRATEGY_REGISTERED: &str = "strategy_registered";
/// Emitted at the end of a successful `deregister_strategy` call.
pub const TOPIC_STRATEGY_DEREGISTERED: &str = "strategy_deregistered";
/// Emitted at the end of a successful `set_active_strategy` or
/// `migrate_strategy` call.
pub const TOPIC_STRATEGY_CHANGED: &str = "strategy_changed";

// --- harvest / fees ------------------------------------------------------------
/// Emitted at the end of a successful `harvest` call.
pub const TOPIC_HARVESTED: &str = "harvested";
/// Emitted at the end of a successful `withdraw_fees` call.
pub const TOPIC_FEE_COLLECTED: &str = "fee_collected";
