[[ -t 0 ]] || return

shopt -s extdebug

_nope_check() {
    local cmd="$BASH_COMMAND"
    [[ -z "$cmd" ]] && return 0
    nope check <<< "$cmd"
    if [[ $? -ne 0 ]]; then
        read -r -p "Run anyway? [y/N] " response </dev/tty
        [[ "$response" =~ ^[Yy]$ ]] && return 0
        return 1
    fi
    return 0
}

trap '_nope_check' DEBUG
