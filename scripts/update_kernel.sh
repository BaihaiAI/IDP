#!/bin/bash
set -exu
source scripts/k8s_env.sh

#export PYO3_PYTHON=/home/miniconda3/envs/py39/bin/python
cargo b --bin kernel_manage
scp -C target/debug/kernel_manage $remote:/
#scp -C target/debug/idp_kernel $remote:/kernel_py38

ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl stop kernel_manage"
for binary in kernel_manage; do
    ssh $remote "kubectl -n $namespace cp /$binary $pod:/tmp/"
    ssh $remote "kubectl -n $namespace exec $pod -- sudo mv /tmp/$binary /usr/local/bin/"
done

ssh $remote "kubectl -n $namespace exec $pod -- sudo supervisorctl restart kernel_manage"
