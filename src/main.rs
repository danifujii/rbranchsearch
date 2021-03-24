use crossterm::{cursor, style::Print, terminal, ErrorKind, QueueableCommand};
use std::io::{stdout, Write};
use std::process::Command;
use std::str;

fn get_branches() -> Vec<String> {
    let cmd = Command::new("git")
        .arg("branch")
        .output()
        .expect("Failed to obtain branches: could not execute command");

    if cmd.status.success() {
        let s = match str::from_utf8(&cmd.stdout) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        return s
            .replace("*", " ")
            .split("\n")
            .map(|s| s.to_string())
            .collect();
    }
    panic!("Failed to obtain branches: cmd unsucessful");
}

fn main() -> Result<(), ErrorKind> {
    let mut stdout = stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    let branches = get_branches();
    for i in 0..branches.len() {
        stdout
            .queue(cursor::MoveTo(0, i as u16))?
            .queue(Print(&branches[i]))?;
    }

    stdout.flush()?;

    Ok(())
}
