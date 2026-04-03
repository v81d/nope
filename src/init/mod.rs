pub fn initialize_shell(shell: &str) {
    let exe = std::env::current_exe()
        .expect("Failed to get current executable path.")
        .to_string_lossy()
        .to_string();

    let raw_script = match shell {
        "bash" => include_str!("scripts/init.bash"),
        "zsh" => include_str!("scripts/init.zsh"),
        "powershell" => include_str!("scripts/init.powershell"),
        _ => panic!(
            "Unknown shell: {}. Supported: bash, zsh, powershell.",
            shell
        ),
    };

    let script = raw_script.replace("__EXE__", &exe);

    print!("{}", script);
}
