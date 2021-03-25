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
    let branches = git::get_branches();
    let mut selected_branch: usize = 0;
    gui::write_lines(&stdout, &branches)?;
    draw_selected_branch(&stdout, &branches, selected_branch)?;

    // Main loop
    loop {
        if let Ok(Event::Key(KeyEvent { code: kc, .. })) = event::read() {
            match kc {
                KeyCode::Up => {
                    if selected_branch > 0 {
                        gui::write_line(
                            &stdout,
                            &branches[selected_branch],
                            selected_branch as u16,
                        )?;
                        selected_branch -= 1;
                        draw_selected_branch(&stdout, &branches, selected_branch)?;
                    }
                }
                KeyCode::Down => {
                    if selected_branch < branches.len() - 1 {
                        gui::write_line(
                            &stdout,
                            &branches[selected_branch],
                            selected_branch as u16,
                        )?;
                        selected_branch += 1;
                        draw_selected_branch(&stdout, &branches, selected_branch)?;
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

    // Clean up
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;

    Ok(())
}
