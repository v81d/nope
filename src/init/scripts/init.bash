[[ -t 0 ]] || return

shopt -s extdebug

_nope_check() {
    local cmd="$BASH_COMMAND"
    [[ -z "$cmd" ]] && return 0
    if ! "__EXE__" check "$cmd"; then
        read -r -p "Run anyway? [y/N] " response </dev/tty
        [[ "$response" =~ ^[Yy]$ ]] && return 0
        return 1
    fi
    return 0
}

trap '_nope_check' DEBUG
