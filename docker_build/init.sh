#!/usr/bin/env bash
set -exu
script_dir=$(dirname -- $(readlink -f -- "$0"))
pip install $script_dir/store/12345/projects/6789/notebooks/baihai_aid-1.2.2-py3-none-any.whl
cd $script_dir/store/12345/projects/6789/notebooks/
git init