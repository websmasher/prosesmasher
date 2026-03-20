//! Markdown parser adapter — `DocumentParser` implementation.

#[allow(dead_code)] // TODO: remove when segmenter.rs calls count_syllables
mod syllables;

use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
