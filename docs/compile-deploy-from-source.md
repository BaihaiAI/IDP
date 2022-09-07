---
title: Compile And Deploy IDP From Source
---

# Compile And Deploy IDP From Source

## System Requirement

IDP is written in Rust, to build IDP from source you will need to install the following tools or system packages:

SDK require:

- Rust Install with rustup
- Python version > 3.6
- nodejs version > 16 and with yarn installed
- c++/g++ version support CXXFLAGS="--std=c++14"(if you want to compile web/terminal, because node-pty require C++ and python3 to compile)

### Linux

debian/ubuntu like distributions system packages require:

> apt install build-essential python3-dev python3-pip openssl git libgit2-dev

centos/fedora like distributions system packages require:

> dnf install base-devel python3-devel python3-pip openssl-devel git libgit2-devel

### Windows

support windows version >= 7 and x86_64 arch

### macos

macos can't use system bundle's python3 because it's static linked and no dylib.

you must install python from brew or virtual env e.g. pyenv/conda

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

#### conda's python

Modify .cargo/config/toml as follows:
(note that you need to _replace_ `CONDA_PREFIX` with
the output of `echo $CONDA_PREFIX` from your terminal.)

```toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup", "-C", "link-arg=-Wl,-rpath,`CONDA_PREFIX`/lib"]
```

## compile backend

> cargo b

## compile web

```
cd web
yarn install && yarn build
```

## build docker image

require backend/web both compile

if rust backend compile in debug build

> docker build -t note -f Dockerfile --target debug .

if rust backend compile in release build

> docker build -t note -f Dockerfile --target release .
