//! Markdown parser adapter — `DocumentParser` implementation.

mod syllables;
#[allow(dead_code)] // TODO: remove when markdown.rs calls segment_paragraph
mod segmenter;

use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
