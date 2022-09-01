#!/bin/bash
set -exu

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

toolchain=$(rustup show active-toolchain)

img=idp-note-dist
if [[ $toolchain == *"aarch64-apple"* ]]; then 
# mac M1 would error when compile vender C/C++ code
docker build -t $img -f Dockerfile --target release --progress plain --platform linux/x86_64 .
else
docker build -t $img -f Dockerfile --target release --progress plain .
fi

dir=IDPStudio-linux-x64-v$(date +'%y%m%d-%H%M%S')
container="$img-tmp"
docker run -dit --rm --name $container --entrypoint bash $img
docker cp $container:/root/IDPStudio $dir
docker stop $container

tar zcf $dir.tar.gz $dir
rm -rf $dir
