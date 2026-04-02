[[ -t 0 ]] || return

_nope_check() {
    local cmd="$1"

    if [[ -z "$cmd" ]]; then
        return
    fi

    if ! "__EXE__" check "$cmd"; then
        local response

        echo "Due to ZSH scripting limitations, the entire shell will be restarted if you cancel. To avoid this, use ^C instead."
        read "response?Run anyway? [y/N] " </dev/tty

        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            exec zsh
        fi
    fi
}

preexec_functions+=( _nope_check )
