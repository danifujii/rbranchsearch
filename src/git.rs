use std::process::{Command, Output};
use std::str;

pub fn get_branches() -> Vec<String> {
    let cmd = execute_command("git".to_string(), vec![&"branch".to_string()]);
    if cmd.status.success() {
        let s = str::from_utf8(&cmd.stdout).unwrap();
        return s
            .replace("*", " ")
            .split("\n")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    panic!("Failed to obtain branches: cmd unsucessful");
}

pub fn change_branch(branch: String) -> Result<(), String> {
    let cmd = execute_command("git".to_string(), vec![&"checkout".to_string(), &branch]);
    return if cmd.status.success() {
        Ok(())
    } else {
        Err(str::from_utf8(&cmd.stderr).unwrap().to_string())
    };
}

fn execute_command(cmd: String, args: Vec<&String>) -> Output {
    return Command::new(&cmd)
        .args(args)
        .output()
        .expect(&format!("Could not execute command: {}", &cmd));
}
