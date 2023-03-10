<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-dapps/multisig-wallet" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# Multisig wallet

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]
[![Docs][docs_badge]][docs_href]

[build_badge]: https://img.shields.io/github/actions/workflow/status/gear-dapps/multisig-wallet/build.yml?label=Build
[build_href]: https://github.com/gear-dapps/multisig-wallet/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/gear-dapps/multisig-wallet/blob/master/LICENSE

[docs_badge]: https://img.shields.io/badge/docs-online-5023dd
[docs_href]: https://dapps.gear.rs/multisig_wallet

Multisignature wallets are cryptocurrency wallets that require one or more private keys to sign and send a transaction.

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-dapps/multisig-wallet/releases).

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

## License

The source code is licensed under the [MIT license](LICENSE).
