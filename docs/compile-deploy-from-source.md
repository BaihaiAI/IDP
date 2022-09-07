---
title: Compile And Deploy IDP From Source
---

# Compile And Deploy IDP From Source

## System Requirements

You will need Git, Rustup, Python 3, the Node.js active LTS (v16+), yarn, and npm (v8+ but < 8.6). You may need to make `python3`(v3.8+) the default if Python 2.7 is default for your OS. Also, if you don't have anything named `python` on your machine and only have python3, you will need something like `python-is-python3`

### Linux additional build dependencies

If you are using Ubuntu/Debian, additionally install:

> apt install build-essential python3-dev python3-pip openssl git libgit2-dev

If you are using Fedora/Centos, additionally install:

> dnf install base-devel python3-devel python3-pip openssl-devel git libgit2-devel

### Windows additional build dependencies

Install Visual Studio or the Microsoft C++ Build Tools

### macos

macos can't use system bundle's python3 because it's static linked and no dylib, you can install python3 with dylib from conda/miniconda3/brew.

copy .cargo/config.toml and edit it to where your python installed

> cp .cargo/config.toml.example .cargo/config.toml

#### homebrew's python

edit .cargo/config.toml to these and `brew install python3`

```toml
[target.x86_64-apple-darwin]
rustflags = ["-L", "/opt/homebrew/lib/", "-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]
[env]
PYO3_PYTHON="/opt/homebrew/bin/python3"
```

#### macos conda config

Modify .cargo/config/toml as follows:
(note that you need to _replace_ `CONDA_PREFIX` with
the output of `echo $CONDA_PREFIX` from your terminal.)

```toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup", "-C", "link-arg=-Wl,-rpath,`CONDA_PREFIX`/lib"]
```

## compile Rust backend

> cargo b

## compile web

```
cd web
yarn install && yarn build
```

## build docker image

> docker build -t note -f Dockerfile --target release .
