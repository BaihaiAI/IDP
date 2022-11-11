#!/usr/bin/env bash
set -exu
# windows powershell
# cargo b --bin idp_kernel --bin idp; cargo r --bin idp -- --listen-addr 0.0.0.0

# readlink -f not support in macOS
#script_dir=$(dirname -- $(readlink -f -- "$0"))
#repo_root=$(dirname $script_dir)
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd .. && pwd)
cd $repo_root

cd web
# In mac M1 if nodejs version > 16, then require CXXFLAGS set to C++ 17
yarn install && CXXFLAGS="--std=c++17" yarn install:terminal && yarn build:open
cd ..
if ! stat web/dist/index.html; then
    echo "web build failed"
    exit 1
fi


cd typescript-lsp
npm run install:all
cd packages/vscode-pyright
rm -rf dist pyright
npm ci && npm run package && mv dist pyright
cd ../..
cd ..

cargo b --release --bin idp --bin idp_kernel
cd target/release
curl -O -L http://baihai.cn-bj.ufileos.com/docker-build/python_packages.tgz
tar zxf python_packages.tgz
rm python_packages.tgz
cd ..
