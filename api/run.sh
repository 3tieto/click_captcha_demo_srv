#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -e

./build.sh

source pid.sh
lsof -i :$PORT | awk 'NR>1 {print $2}' | xargs kill -9

set -x

run="exec bun run "

if [ ! -n "$1" ]; then
  $run ./lib/index.js
else
  $run ./${@:1}
fi
