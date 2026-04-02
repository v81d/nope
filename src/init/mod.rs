pub fn initialize_shell(shell: &str) {
    let exe = std::env::current_exe()
        .expect("Failed to get current executable path")
        .to_string_lossy()
        .to_string();
    let script = match shell {
        "bash" => format!(
            r#"
[[ -t 0 ]] || return
shopt -s extdebug
_nope_check() {{
    local cmd="$BASH_COMMAND"
    [[ -z "$cmd" ]] && return 0
    if ! "{exe}" check "$cmd"; then
        read -r -p "Run anyway? [y/N] " response </dev/tty
        [[ "$response" =~ ^[Yy]$ ]] && return 0
        return 1
    fi
    return 0
}}
trap '_nope_check' DEBUG
"#
        ),
        "zsh" => format!(
            r#"
[[ -t 0 ]] || return
_nope_check() {{
    local cmd="$1"
    if [[ -z "$cmd" ]]; then
        return
    fi
    if ! "{exe}" check "$cmd"; then
        local response
        echo "Due to ZSH scripting limitations, the entire shell must be restarted if you want to cancel. To avoid this, use ^C instead."
        read "response?Run anyway? [y/N] " </dev/tty
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            exec zsh
        fi
    fi
}}
preexec_functions+=( _nope_check )
"#
        ),
        _ => panic!("Unknown shell: {}. Supported: bash, zsh.", shell),
    };
    print!("{}", script);
}
