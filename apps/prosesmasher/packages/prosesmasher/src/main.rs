//! User-facing `prosesmasher` binary crate.

fn main() -> std::process::ExitCode {
    prosesmasher_adapters_inbound_cli::main_entry()
}
