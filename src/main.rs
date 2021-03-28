use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Color,
    terminal, Result,
};
use std::io::{stdout, Stdout};
#[macro_use]
extern crate clap;
use clap::App;

mod git;
mod gui;

const SELECTED_INDICATOR: char = '*';
const DISPLAY_OFFSET: u16 = 6;
const HEADER: &str = "
|_   _ _  _  _ |_     _  _  _   _ _ |_
|_) | (_|| |(_ | |   _> (/_(_| | (_ | |
";
const HEADER_OFFSET: u16 = 4;
const HEADER_COLOR: Color = Color::DarkBlue;

fn draw_selected_branch(stdout: &Stdout, branches: &Vec<String>, selected: usize) -> Result<()> {
    if branches.is_empty() || selected > branches.len() - 1 {
        return Ok(()); // Nothing to do
    }
    let branch: String = (&branches[selected]).chars().skip(1).collect();
    let selected_branch: String = format!("{}{}", SELECTED_INDICATOR, branch);
    gui::write_line(&stdout, &selected_branch, selected as u16 + DISPLAY_OFFSET)?;
    Ok(())
}

fn update_selected_branch(
    stdout: &Stdout,
    branches: &Vec<String>,
    selected: usize,
    up: bool,
) -> Result<()> {
    gui::write_line(
        &stdout,
        &branches[selected],
        selected as u16 + DISPLAY_OFFSET,
    )?; // Reset previous selected
    let new_selected = if up { selected - 1 } else { selected + 1 };
    draw_selected_branch(&stdout, &branches, new_selected)?;
    Ok(())
}

fn draw_header(stdout: &Stdout) -> Result<()> {
    let mut idx = 0;
    for line in HEADER.split("\n") {
        gui::write_line_with_color(&stdout, &line.to_string(), idx, HEADER_COLOR)?;
        idx += 1;
    }
    gui::write_line_with_color(
        &stdout,
        &String::from("Searching: "),
        HEADER_OFFSET,
        HEADER_COLOR,
    )?;
    Ok(())
}

fn setup(mut stdout: &Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

fn initial_draw(stdout: &Stdout, branches: &Vec<String>) -> Result<()> {
    draw_header(stdout)?;
    gui::write_lines(&stdout, &branches, DISPLAY_OFFSET)?;
    draw_selected_branch(&stdout, &branches, 0)?;
    Ok(())
}

fn main_loop(stdout: &Stdout, branches: &Vec<String>) -> Result<()> {
    let mut search = String::new();
    let mut selected_branch: usize = 0;
    let mut displayed_branches = branches.to_vec();
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: kc,
            modifiers: km,
        })) = event::read()
        {
            match kc {
                KeyCode::Up => {
                    if !displayed_branches.is_empty() && selected_branch > 0 {
                        update_selected_branch(
                            &stdout,
                            &displayed_branches,
                            selected_branch,
                            true,
                        )?;
                        selected_branch -= 1;
                    }
                }
                KeyCode::Down => {
                    if !displayed_branches.is_empty()
                        && selected_branch < displayed_branches.len() - 1
                    {
                        update_selected_branch(
                            &stdout,
                            &displayed_branches,
                            selected_branch,
                            false,
                        )?;
                        selected_branch += 1;
                    }
                }
                KeyCode::Enter => {
                    if !displayed_branches.is_empty()
                        && selected_branch < displayed_branches.len() - 1
                    {
                        if let Err(s) = git::change_branch(branches[selected_branch].to_string()) {
                            gui::display_closing_error(&stdout, s)?;
                        }
                        break;
                    }
                }
                KeyCode::Char(c) => {
                    if c == 'c' && km == KeyModifiers::CONTROL {
                        break;
                    } else {
                        search.push(c);
                        gui::write_line_with_color(
                            &stdout,
                            &format!("Searching: {}", search),
                            HEADER_OFFSET,
                            HEADER_COLOR,
                        )?;
                        displayed_branches = git::get_matching_branches(&search, &branches);
                        gui::write_lines(&stdout, &displayed_branches, DISPLAY_OFFSET)?;
                        selected_branch = 0;
                        draw_selected_branch(&stdout, &displayed_branches, selected_branch)?;
                    }
                }
                KeyCode::Backspace => {
                    if !search.is_empty() {
                        search.pop();
                        gui::write_line_with_color(
                            &stdout,
                            &format!("Searching: {}", search),
                            HEADER_OFFSET,
                            HEADER_COLOR,
                        )?;

                        displayed_branches = git::get_matching_branches(&search, &branches);
                        gui::write_lines(&stdout, &displayed_branches, DISPLAY_OFFSET)?;
                        selected_branch = 0;
                        draw_selected_branch(&stdout, &displayed_branches, selected_branch)?;
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
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let branches = git::get_branches(matches.is_present("all_branches"));
    let stdout = stdout();

    setup(&stdout)?;

    if let Some(s) = matches.value_of("BRANCH") {
        let matching = git::get_matching_branches(&s.to_string(), &branches);
        if matching.is_empty() {
            gui::display_closing_error(&stdout, String::from("Could not find a matching branch"))?;
        } else if let Err(s) = git::change_branch(matching.first().unwrap().to_string()) {
            gui::display_closing_error(&stdout, s)?;
        }
    } else {
        initial_draw(&stdout, &branches)?;
        main_loop(&stdout, &branches)?;
    }

    cleanup(&stdout)?;
    Ok(())
}
