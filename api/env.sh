#!/usr/bin/env bash

dir=$(readlink -f "$BASH_SOURCE")
dir=${dir%/*}
cd ${dir%/*/*}/conf
set -o allexport
source srv/api.sh
set +o allexport
