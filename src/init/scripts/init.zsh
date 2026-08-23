[[ -t 0 ]] || return

_nope_check() {
    local cmd="$1"

    if [[ -z "$cmd" ]]; then
        return
    fi

    nope check <<< "$cmd"

    if [[ $? -ne 0 ]]; then
        local response

        read "response?Run anyway? [y/N] " </dev/tty

        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            kill -INT -$$
        fi
    fi
}

preexec_functions+=( _nope_check )
