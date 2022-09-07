#!/bin/bash
set -exu
source scripts/k8s_env.sh

cargo b --bin kernel_manage --bin idp_kernel
scp -C target/debug/kernel_manage ucloud:/root/
scp -C target/debug/idp_kernel ucloud:/root/kernel_py38

ssh ucloud "kubectl -n $namespace exec $pod -- bash -c 'pkill kernel_py38 || true'"
ssh ucloud "kubectl -n $namespace exec $pod -- bash -c 'pkill kernel_py39 || true'"
ssh ucloud "kubectl -n $namespace exec $pod -- sudo supervisorctl stop kernel_manage"
for binary in kernel_manage; do
    ssh ucloud "kubectl -n $namespace cp /root/$binary $pod:/tmp/"
    ssh ucloud "kubectl -n $namespace exec $pod -- sudo mv /tmp/$binary /usr/local/bin/"
done

ssh ucloud "kubectl -n $namespace exec $pod -- sudo supervisorctl restart kernel_manage"
