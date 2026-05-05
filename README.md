# libc-cfg

[![MIT licensed][license-badge]][license-url]
[![Unsafe Forbidden][unsafe-forbidden-badge]][unsafe-forbidden-url]
[![crates.io](https://img.shields.io/crates/v/libc-cfg.svg)](https://crates.io/crates/libc-cfg)
[![docs.rs](https://docs.rs/libc-cfg/badge.svg)](https://docs.rs/libc-cfg)

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: ./LICENSE
[unsafe-forbidden-badge]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-forbidden-url]: https://github.com/rust-secure-code/safety-dance/

A codegen tool for extracting `cfg` expressions from the [`libc`](https://github.com/rust-lang/libc) crate.

## Usage

```bash
cargo run --features binary -- --libc /path/to/libc '^pthread_'
```

## Contributing

+ [Development Guide](./CONTRIBUTING.md)
