---
title: Compile And Deploy IDP From Source
---

# Compile And Deploy IDP From Source

## System Requirements

You will need Git, Rustup, Python 3, the Node.js active LTS (v16.17.0), yarn, and npm (v8+). You may need to make `python3`(v3.8+) the default if Python 2.7 is default for your OS. Also, if you don't have anything named `python` on your machine and only have python3, you will need something like `python-is-python3`

### Linux additional build dependencies

If you are using Ubuntu/Debian, additionally install:

```shell
> apt install build-essential python3-dev python3-pip openssl git libgit2-dev
```
If you are using Fedora/Centos, additionally install:

```shell
> dnf install base-devel python3-devel python3-pip openssl-devel git libgit2-devel
```

### Windows additional build dependencies

require x86_64-pc-windows-msvc target, can't compile with windows-gnu target

Follow [Visual Studio guide to install msvc](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools)

### macOS

macOS can't use system bundle's python3 because it's static linked and no dylib, you can install python3 with dylib from conda/miniconda3/brew.

copy .cargo/config.toml and edit it to where your python installed
```shell
> cp .cargo/config.toml.example .cargo/config.toml
```

#### Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

#### homebrew's python

edit .cargo/config.toml

```toml
[target.x86_64-apple-darwin]
rustflags = ["-L", "/opt/homebrew/lib/", "-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]
[env]
PYO3_PYTHON="/opt/homebrew/bin/python3"
```

#### macOS conda's python

Modify .cargo/config.toml as follows:
(note that you need to _replace_ `CONDA_PREFIX` with
the output of `echo $CONDA_PREFIX` from your terminal.)

```toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup", "-C", "link-arg=-Wl,-rpath,`CONDA_PREFIX`/lib"]
```

## compile and run

```shell
./scripts/build.sh
./target/release/idp
```

---

## build docker image
```shell
> docker build -t note -f Dockerfile --target release .
```
