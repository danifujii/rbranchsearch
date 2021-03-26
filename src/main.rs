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
    if branches.is_empty() || selected > branches.len() - 1 {
        return Ok(())  // Nothing to do
    }
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
    let mut search = String::new();
    let mut selected_branch: usize = 0;
    let mut displayed_brances = branches.to_vec();
    loop {
        if let Ok(Event::Key(KeyEvent { code: kc, .. })) = event::read() {
            match kc {
                KeyCode::Up => {
                    if !displayed_brances.is_empty() && selected_branch > 0 {
                        update_selected_branch(&stdout, &displayed_brances, selected_branch, true)?;
                        selected_branch -= 1;
                    }
                }
                KeyCode::Down => {
                    if !displayed_brances.is_empty() && selected_branch < displayed_brances.len() - 1 {
                        update_selected_branch(&stdout, &displayed_brances, selected_branch, false)?;
                        selected_branch += 1;
                    }
                }
                KeyCode::Enter => {
                    if selected_branch < displayed_brances.len() - 1 {
                        if let Err(s) =
                            git::change_branch(branches[selected_branch].trim().to_string())
                        {
                            gui::display_closing_error(&stdout, s)?;
                        }
                    }
                    break;
                }
                KeyCode::Char(c) => {
                    if c == 'q' {
                        break;
                    } else {
                        search.push(c);
                        displayed_brances = git::get_matching_branches(&search, &branches);
                        gui::write_lines(&stdout, &displayed_brances)?;
                        selected_branch = 0;
                        draw_selected_branch(&stdout, &displayed_brances, selected_branch)?;
                    }
                }
                KeyCode::Backspace => {
                    if !search.is_empty() {
                        search.pop();
                        displayed_brances = git::get_matching_branches(&search, &branches);
                        gui::write_lines(&stdout, &displayed_brances)?;
                        selected_branch = 0;
                        draw_selected_branch(&stdout, &displayed_brances, selected_branch)?;
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
