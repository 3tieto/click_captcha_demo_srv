export RUSTFLAGS='--cfg reqwest_unstable'
export RUST_LOG=debug,supervisor=warn,hyper=warn,rustls=warn,h2=warn,tower=warn,reqwest=warn

_init() {
  local pwd=$(pwd)
  local dir=$(readlink -f "$BASH_SOURCE")
  cd ${dir%/*/*/*/*}/conf/
  local i
  for i in $@; do
    set -o allexport
    source "$i".sh
    set +o allexport
  done

  cd $pwd
}
_init env/ipv6_proxy
unset -f _init
