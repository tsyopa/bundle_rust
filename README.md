## Usage

```bash
# in this repository directory
cargo build --release
cp target/release/bundle_rust ~/.cargo/bin

# then inside some rust project directory
bundle_rust ./src/main.rs
# bundled file is available at ./target/main.rs
```
