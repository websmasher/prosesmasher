//! Application core — check trait, runner, and all checks.

pub mod check;
pub mod patterns;
pub mod readability;
pub mod runner;
pub mod structure;
pub mod terms;

#[cfg(test)]
pub(crate) mod test_helpers;

use prosesmasher_ports_outbound_traits as _;
