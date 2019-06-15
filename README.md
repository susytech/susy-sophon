<h1>Copyleft Notice</h1>
<p>This is a release-only, community-driven, GPL-3.0 licensed, copyleft project derived from Parity(forked from <a href="https://github.com/paritytech/parity-ethereum/">parity/ethereum</a>).  Paritytech and other third party code contributors, if any, in this repository reserves all copyrights.</p>
<h1>Contribution Credits</h1>
<p>Special Thanks to all the contributions from <strong><a href="https://github.com/paritytech/parity-ethereum/graphs/contributors">Contributor List</a></strong>. And please checkout <strong><a href="https://github.com/paritytech/parity-ethereum/commits">Commit History</a></strong> to view their work. Superstring Community values all contributions and especially appreciates those generous contributions from partiytech and other third parties, directly or indirectly.</p>
<h1>Community Statement</h1>
Superstring Community is an open research community that embraces copyleft movement and decentralized technology. All sub-organizations named after "susy" including susytech belongs to Superstring Community. Superstring Community and all its sub-organizations reserve NO copyright.
<h1>No Warranty Disclaimer</h1>
<p>USE AT YOUR OWN RISK! It is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.</p>

![Susy Sophon](docs/logo-susy-sophon.png)

<h2 align="center">The Fastest and most Advanced Sophon Client.</h2>

<p align="center"><strong><a href="https://octonion.institute/susytech/susy-sophon/releases/latest">» Download the latest release «</a></strong></p>

<p align="center"><a href="https://gitlab.superstring.io/susy/susy-sophon/commits/master" target="_blank"><img src="https://gitlab.superstring.io/susy/susy-sophon/badges/master/build.svg" /></a>
<a href="https://www.gnu.org/licenses/gpl-3.0.en.html" target="_blank"><img src="https://img.shields.io/badge/license-GPL%20v3-green.svg" /></a></p>

**Built for mission-critical use**: Miners, service providers, and exchanges need fast synchronisation and maximum uptime. Susy Sophon provides the core infrastructure essential for speedy and reliable services.

- Clean, modular codebase for easy customisation
- Advanced CLI-based client
- Minimal memory and storage footprint
- Synchronise in hours, not days with Warp Sync
- Modular for light integration into your service or product

## Technical Overview

Susy Sophon's goal is to be the fastest, lightest, and most secure Sophon client. We are developing Susy Sophon using the sophisticated and cutting-edge **Rust programming language**. Susy Sophon is licensed under the GPLv3 and can be used for all your Sophon needs.

By default, Susy Sophon runs a JSON-RPC HTTP server on port `:8545` and a Web-Sockets server on port `:8546`. This is fully configurable and supports a number of APIs.

If you run into problems while using Susy Sophon, check out the [wiki for documentation](https://wiki.superstring.io/), feel free to [file an issue in this repository](https://octonion.institute/susytech/susy-sophon/issues/new), or hop on our [Gitter](https://gitter.im/susytech/susy) or [Riot](https://riot.im/app/#/group/+susy:matrix.superstring.io) chat room to ask a question. We are glad to help! **For security-critical issues**, please refer to the security policy outlined in [SECURITY.md](SECURITY.md).

Susy Sophon's current beta-release is 2.1. You can download it at [the releases page](https://octonion.institute/susytech/susy-sophon/releases) or follow the instructions below to build from source. Please, mind the [CHANGELOG.md](CHANGELOG.md) for a list of all changes between different versions.

## Build Dependencies

Susy Sophon requires **latest stable Rust version** to build.

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have `rustup`, you can install it like this:

- Linux:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

  Susy Sophon also requires `gcc`, `g++`, `libudev-dev`, `pkg-config`, `file`, `make`, and `cmake` packages to be installed.

- OSX:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

  `clang` is required. It comes with Xcode command line tools or can be installed with homebrew.

- Windows
  Make sure you have Visual Studio 2015 with C++ support installed. Next, download and run the `rustup` installer from
  https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe, start "VS2015 x64 Native Tools Command Prompt", and use the following command to install and set up the `msvc` toolchain:
  ```bash
  $ rustup default stable-x86_64-pc-windows-msvc
  ```

Once you have `rustup` installed, then you need to install:
* [Perl](https://www.perl.org)
* [Yasm](https://yasm.tortall.net)

Make sure that these binaries are in your `PATH`. After that, you should be able to build Susy Sophon from source.

## Build from Source Code

```bash
# download Susy Sophon code
$ git clone https://octonion.institute/susytech/susy-sophon
$ cd susy-sophon

# build in release mode
$ cargo build --release --features final
```

This produces an executable in the `./target/release` subdirectory.

Note: if cargo fails to parse manifest try:

```bash
$ ~/.cargo/bin/cargo build --release
```

Note, when compiling a crate and you receive errors, it's in most cases your outdated version of Rust, or some of your crates have to be recompiled. Cleaning the repository will most likely solve the issue if you are on the latest stable version of Rust, try:

```bash
$ cargo clean
```

This always compiles the latest nightly builds. If you want to build stable or beta, do a

```bash
$ git checkout stable
```

or

```bash
$ git checkout beta
```

## Simple One-Line Installer for Mac and Linux

```bash
bash <(curl https://get.superstring.io -L)
```

The one-line installer always defaults to the latest beta release. To install a stable release, run:

```bash
bash <(curl https://get.superstring.io -L) -r stable
```

## Start Susy Sophon

### Manually

To start Susy Sophon manually, just run

```bash
$ ./target/release/susy
```

so Susy Sophon begins syncing the Sophon blockchain.

### Using `systemd` service file

To start Susy Sophon as a regular user using `systemd` init:

1. Copy `./scripts/susy.service` to your
`systemd` user directory (usually `~/.config/systemd/user`).
2. Copy release to bin folder, write `sudo install ./target/release/susy /usr/bin/susy`
3. To configure Susy Sophon, write a `/etc/susy/config.toml` config file, see [Configuring Susy Sophon](https://susytech.github.io/wiki/Configuring-Susy) for details.

## Susy Sophon toolchain

In addition to the Susy Sophon client, there are additional tools in this repository available:

- [svmbin](https://octonion.institute/susytech/susy-sophon/src/branch/master/svmbin/) - SVM implementation for Susy Sophon.
- [sofabi](https://octonion.institute/susytech/sofabi) - Susy Sophon function calls encoding.
- [sofstore](https://octonion.institute/susytech/susy-sophon/src/branch/master/accounts/sofstore) - Susy Sophon key management.
- [sofkey](https://octonion.institute/susytech/susy-sophon/src/branch/master/accounts/sofkey) - Susy Sophon keys generator.
- [whisper](https://octonion.institute/susytech/susy-sophon/src/branch/master/whisper/) - Implementation of Whisper-v2 PoC.

## Join the chat!

Questions? Get in touch with us on Gitter:
[![Gitter: Susy](https://img.shields.io/badge/gitter-susy-4AB495.svg)](https://gitter.im/susytech/susy)
[![Gitter: Susy.js](https://img.shields.io/badge/gitter-susy.js-4AB495.svg)](https://gitter.im/susytech/susy.js)
[![Gitter: Susy/Miners](https://img.shields.io/badge/gitter-susy/miners-4AB495.svg)](https://gitter.im/susytech/susy/miners)
[![Gitter: Susy-PoA](https://img.shields.io/badge/gitter-susy--poa-4AB495.svg)](https://gitter.im/susytech/susy-poa)

Alternatively, join our community on Matrix:
[![Riot: +Susy](https://img.shields.io/badge/riot-%2Bsusy%3Amatrix.superstring.io-orange.svg)](https://riot.im/app/#/group/+susy:matrix.superstring.io)

## Documentation

Official website: http://superstring.io

Be sure to [check out our wiki](https://wiki.superstring.io) for more information.
