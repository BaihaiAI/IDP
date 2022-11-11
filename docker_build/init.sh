#!/usr/bin/env bash
set -exu
script_dir=$(dirname -- $(readlink -f -- "$0"))
cd $script_dir/store/12345/projects/6789/notebooks/
git init