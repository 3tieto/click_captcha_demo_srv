_init() {
  local pwd=$(pwd)
  local dir=$(readlink -f "$BASH_SOURCE")
  cd ${dir%/*/*}/conf/docker
  local i
  for i in $@; do
    set -o allexport
    source "$i".sh
    set +o allexport
  done

  cd $pwd
}
_init host mc pg kv
unset -f _init
