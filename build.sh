cargo build || exit 1
cargo build --profile release || exit 1
cargo build --target x86_64-pc-windows-gnu || exit 1
cargo build --profile release --target x86_64-pc-windows-gnu || exit 1