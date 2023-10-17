#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
if [ -d "conf" ]; then
  cd conf
  git pull
  cd ..
else
  cd ..
  git clone --depth=1 git@github.com:i18n-conf/conf.git conf
  cd $DIR
fi

if ! command -v watchexec &>/dev/null; then
  cargo install --locked watchexec-cli
fi

if ! command -v bun &>/dev/null; then
  curl -fsSL https://bun.sh/install | bash
fi

bun i

direnv allow
cd docker
direnv allow
