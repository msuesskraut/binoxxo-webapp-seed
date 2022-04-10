

# binoxxo-webapp-seed

Binoxxo Single Page Webapp using Seed.

See webapp at: [https://msuesskraut.github.io/binoxxo/](https://msuesskraut.github.io/binoxxo/)

Based on:

* [rust](https://www.rust-lang.org/)
* [seed](https://seed-rs.org/)
* [wasm](https://webassembly.org/)
* [Binoxxo crate](https://crates.io/crates/binoxxo)
* [trunk](https://trunkrs.dev/)

## Build & run

Install `wasm` target and `trunk`:

    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk


Run tests and serve page with `trunk`:

    cargo test
    trunk serve --open

## Publish

Again use `trunk`:

    cargo test
    trunk build --release --public-url /binoxxo

Copy files from `/dist` to a server into a `/binoxxo` directory.

## License

[MIT](LICENSE)
