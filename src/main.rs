use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, terminal, Result,
};
use std::io::{stdout, Stdout};

mod git;
mod gui;

const SELECTED_INDICATOR: char = '*';

fn draw_selected_branch(stdout: &Stdout, branches: &Vec<String>, selected: usize) -> Result<()> {
    let branch: String = (&branches[selected]).chars().skip(1).collect();
    let selected_branch: String = format!("{}{}", SELECTED_INDICATOR, branch);
    gui::write_line(&stdout, &selected_branch, selected as u16)?;
    Ok(())
}

fn update_selected_branch(
    stdout: &Stdout,
    branches: &Vec<String>,
    selected: usize,
    up: bool,
) -> Result<()> {
    gui::write_line(&stdout, &branches[selected], selected as u16)?; // Reset previous selected
    let new_selected = if up { selected - 1 } else { selected + 1 };
    draw_selected_branch(&stdout, &branches, new_selected)?;
    Ok(())
}

fn setup(mut stdout: &Stdout, branches: &Vec<String>) -> Result<()> {
    terminal::enable_raw_mode()?;
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;
    gui::write_lines(&stdout, &branches)?;
    draw_selected_branch(&stdout, &branches, 0)?;
    Ok(())
}

fn main_loop(stdout: &Stdout, branches: &Vec<String>) -> Result<()> {
    let mut selected_branch: usize = 0;
    loop {
        if let Ok(Event::Key(KeyEvent { code: kc, .. })) = event::read() {
            match kc {
                KeyCode::Up => {
                    if selected_branch > 0 {
                        update_selected_branch(&stdout, &branches, selected_branch, true)?;
                        selected_branch -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_branch < branches.len() - 1 {
                        update_selected_branch(&stdout, &branches, selected_branch, false)?;
                        selected_branch += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Err(s) = git::change_branch(branches[selected_branch].trim().to_string())
                    {
                        gui::display_closing_error(&stdout, s)?;
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
    Ok(())
}

fn cleanup(mut stdout: &Stdout) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    Ok(())
}

fn main() -> Result<()> {
    let branches = git::get_branches();
    let stdout = stdout();
    setup(&stdout, &branches)?;
    main_loop(&stdout, &branches)?;
    cleanup(&stdout)?;
    Ok(())
}
