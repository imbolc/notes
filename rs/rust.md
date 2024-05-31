Rust
====
```sh
cargo install --locked cargo-expand cargo-tree cargo-sort cargo-watch cargo-spellcheck
```

Rustup
------
- install rust stable: `rustup install stable`
- install nightly: `rustup install nightly`
- add some components: `rustup component add rustfmt clippy rust-src rust-analyzer`
- add some components to nightly: `rustup component add rustfmt --toolchain nightly`
- update rust: `rustup update`
- check current version: `rustc --version`
- set global default toolchain: `rustup default nightly`
- set default toolchain for a directory: `rustup override set nightly`
- set rust version for a directory: `rustup override set 1.30.0`
- offline std lib documentation: `rustup doc --std`
- add a target: `rustup target add x86_64-unknown-linux-musl`

Cargo
-----

- create a new project: `cargo new my_project`
- check for errors: `cargo check`
- build in dev mode: `cargo build`
- build in dev mode and run: `cargo run`
- build in prod mode: `cargo build --release`
- use a particular toolchain: `cargo +nightly bench`

Clippy
------
```bash
cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
```


Depending on private github repos
---------------------------------
Add to `~/.cargo/config`:
```toml
[net]
git-fetch-with-cli = true
```
Add the dependency into `Cargo.toml` as:
```toml
[dependencies.bar]
git = "ssh://git@github.com/user/lib.git"
tag = "1.1.0"
package = "foo"
```
In the example it's also renamed from `foo` to `bar`
