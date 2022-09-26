#!/bin/bash
set -exu
source ./scripts/k8s_env.sh

script_dir=$(dirname -- $(readlink -f -- "$0"))
repo_root=$(dirname $script_dir)
cd $repo_root

binary=note_storage
target=${1:-glibc}
if [ $target == "musl" ]; then
cargo b --bin $binary --target x86_64-unknown-linux-musl --release
scp -C target/x86_64-unknown-linux-musl/release/$binary ucloud:/root/
else
cargo b --bin $binary
scp -C target/debug/$binary ucloud:/root/
fi

ssh ucloud "kubectl -n $namespace exec $pod -- sudo supervisorctl stop $binary"
ssh ucloud "kubectl -n $namespace cp /root/$binary $pod:/tmp/$binary"
ssh ucloud "kubectl -n $namespace exec $pod -- sudo mv /tmp/$binary /usr/local/bin/"
ssh ucloud "kubectl -n $namespace exec $pod -- sudo supervisorctl restart $binary"
