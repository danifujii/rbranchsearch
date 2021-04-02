use std::collections::HashMap;
use std::process::{Command};
use std::str;

pub fn get_matching_branches(search: &String, branches: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut positions = HashMap::new();
    for s in branches {
        if let Some(i) = s.find(search) {
            positions.insert(s, i);
            result.push(s.to_string());
        }
    }
    result.sort_by(|a, b| positions.get(a).unwrap().cmp(positions.get(b).unwrap()));
    result
}

pub fn get_branches(all: bool) -> Result<Vec<String>, String> {
    let mut args = vec!["branch".to_string()];
    if all {
        args.push(String::from("-a"));
    }
    let stdout = execute_command("git", args)?;
    let s = str::from_utf8(&stdout).unwrap();
    Ok(s.replace("*", " ")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

pub fn change_branch(branch: &String) -> Result<(), String> {
    execute_command(
        "git",
        vec![String::from("checkout"), String::from(branch.trim())],
    )?;
    Ok(())
}

pub fn delete_branch(branch: &String) -> Result<(), String> {
    execute_command(
        "git",
        vec![
            String::from("branch"),
            String::from("-d"),
            String::from(branch.trim()),
        ],
    )?;
    Ok(())
}

fn execute_command(cmd: &str, args: Vec<String>) -> Result<Vec<u8>, String> {
    let cmd = Command::new(cmd)
        .args(args)
        .output()
        .expect(&format!("Could not execute command: {}", &cmd));
    if cmd.status.success() {
        Ok(cmd.stdout)
    } else {
        Err(str::from_utf8(&cmd.stderr).unwrap().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_branches() {
        let result = get_matching_branches(
            &String::from("ab"),
            &vec![String::from("cab"), String::from("ab"), String::from("d")],
        );
        assert_eq!(result, vec!["ab", "cab"]);
    }

    #[test]
    fn test_no_matching_branches() {
        let result = get_matching_branches(&String::from("ab"), &vec![String::from("d")]);
        assert_eq!(result, [] as [&str; 0]);
    }

    #[test]
    fn test_empty() {
        let result = get_matching_branches(
            &String::from(""),
            &vec![String::from("a"), String::from("b")],
        );
        assert_eq!(result, vec!["a", "b"]);
    }
}
