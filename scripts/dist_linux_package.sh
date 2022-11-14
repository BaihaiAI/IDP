#!/bin/bash
set -exu

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

toolchain=$(rustup show active-toolchain)

# since some npm package require proxy, must run with proxy in host machine `cd web && npm install` before docker build
img=idp.baihai.ai/idp-note
if [[ $toolchain == *"aarch64-apple"* ]]; then 
# mac M1 would error when compile vender C/C++ code
docker build -t $img -f Dockerfile --target release --progress plain --platform linux/x86_64 .
else
docker build -t $img -f Dockerfile --target release --progress plain .
fi

dir=IDPStudio-linux-x64-v$(date +'%y%m%d-%H%M%S')
container=$(docker run -dit --rm --entrypoint bash $img)
docker cp $container:/root/IDPStudio $dir
docker stop $container

tar zcf $dir.tar.gz $dir
rm -rf $dir
