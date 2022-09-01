#!/bin/bash
set -exu

source ./scripts/k8s_env.sh
ssh -t $remote_hostname "kubectl -n $namespace exec -i $pod -- bash -c 'tail -f /var/log/*.log'"
