fn main() -> std::process::ExitCode {
    eprintln!(
        "error: this prosesmasher binary is a crates.io stub.\n\
         The real prosesmasher is distributed as prebuilt GitHub release artifacts.\n\
         \n\
         Install with cargo-binstall:\n\
             cargo binstall prosesmasher\n\
         \n\
         Or build from source:\n\
             cargo install --git https://github.com/websmasher/prosesmasher prosesmasher\n\
         \n\
         See https://github.com/websmasher/prosesmasher for details.\n"
    );
    std::process::ExitCode::from(1)
}
