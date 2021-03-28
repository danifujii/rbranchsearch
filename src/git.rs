use std::collections::HashMap;
use std::process::{Command, Output};
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
    let cmd = execute_command("git".to_string(), args);
    if cmd.status.success() {
        let s = str::from_utf8(&cmd.stdout).unwrap();
        return Ok(s
            .replace("*", " ")
            .split("\n")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect());
    } else {
        Err(str::from_utf8(&cmd.stderr).unwrap().to_string())
    }
}

pub fn change_branch(branch: String) -> Result<(), String> {
    let cmd = execute_command(
        "git".to_string(),
        vec!["checkout".to_string(), branch.trim().to_string()],
    );
    return if cmd.status.success() {
        Ok(())
    } else {
        Err(str::from_utf8(&cmd.stderr).unwrap().to_string())
    };
}

fn execute_command(cmd: String, args: Vec<String>) -> Output {
    return Command::new(&cmd)
        .args(args)
        .output()
        .expect(&format!("Could not execute command: {}", &cmd));
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
