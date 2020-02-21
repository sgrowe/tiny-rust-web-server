# Tiny Rust web server example

This repo contains a [tiny web server](src/main.rs) using [warp](https://github.com/seanmonstar/warp) that can be [built using Docker](Dockerfile).

The final docker image is only 2.66MB in size thanks to Rust's `x86_64-unknown-linux-musl` compile target (so we get a statically linked executable) and enabling "link-time optimisations" in our [Cargo.toml](Cargo.toml):

```toml
[profile.release]
lto = true
```
