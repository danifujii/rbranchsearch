use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style, terminal, Result,
};
use std::io::{stdout, Stdout, Write};
use std::process::{Command, Output};
use std::str;

const SELECTED_INDICATOR: char = '*';

fn execute_command(cmd: String, args: Vec<&String>) -> Output {
    return Command::new(&cmd)
        .args(args)
        .output()
        .expect(&format!("Could not execute command: {}", &cmd));
}

fn get_branches() -> Vec<String> {
    let cmd = execute_command("git".to_string(), vec![&"branch".to_string()]);
    if cmd.status.success() {
        let s = match str::from_utf8(&cmd.stdout) {
            Ok(v) => v.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        return s
            .replace("*", " ")
            .split("\n")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    panic!("Failed to obtain branches: cmd unsucessful");
}

fn draw_branches(mut stdout: &Stdout, branches: &Vec<String>) -> Result<()> {
    for i in 0..branches.len() {
        queue!(
            stdout,
            style::Print(&branches[i]),
            cursor::MoveToNextLine(1)
        )?;
    }
    stdout.flush()?;
    Ok(())
}

fn draw_selected_branch(
    mut stdout: &Stdout,
    branches: &Vec<String>,
    selected: usize,
) -> Result<()> {
    let branch: String = (&branches[selected]).chars().skip(1).collect();
    let selected_branch: String = format!("{}{}", SELECTED_INDICATOR, branch);
    execute!(
        stdout,
        cursor::MoveTo(0, selected as u16),
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::Print(selected_branch)
    )?;
    Ok(())
}

fn reset_branch(mut stdout: &Stdout, branches: &Vec<String>, current: usize) -> Result<()> {
    let branch = &branches[current];
    execute!(
        stdout,
        cursor::MoveTo(0, current as u16),
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::Print(branch)
    )?;
    Ok(())
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = stdout();
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    // Initial draw
    let branches = get_branches();
    let mut selected_branch: usize = 0;
    draw_branches(&stdout, &branches)?;
    draw_selected_branch(&stdout, &branches, selected_branch)?;

    // Main loop
    loop {
        if let Ok(Event::Key(KeyEvent { code: kc, .. })) = event::read() {
            match kc {
                KeyCode::Up => {
                    if selected_branch > 0 {
                        reset_branch(&stdout, &branches, selected_branch)?;
                        selected_branch -= 1;
                        draw_selected_branch(&stdout, &branches, selected_branch)?;
                    }
                }
                KeyCode::Down => {
                    if selected_branch < branches.len() - 1 {
                        reset_branch(&stdout, &branches, selected_branch)?;
                        selected_branch += 1;
                        draw_selected_branch(&stdout, &branches, selected_branch)?;
                    }
                }
                KeyCode::Enter => {
                    let cmd = execute_command(
                        "git".to_string(),
                        vec![
                            &"checkout".to_string(),
                            &branches[selected_branch].trim().to_string(),
                        ],
                    );
                    if !cmd.status.success() {
                        terminal::disable_raw_mode()?;
                        execute!(
                            stdout,
                            terminal::Clear(terminal::ClearType::All),
                            cursor::MoveTo(0, 0),
                            style::Print(str::from_utf8(&cmd.stderr).unwrap())
                        )?;
                    }
                    break;
                }
                KeyCode::Char(c) => {
                    if c == 'q' {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    // Clean up
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;

    Ok(())
}
