use crossterm::{cursor, execute, queue, terminal, Result};
use std::io::{stdout, Stdout};
#[macro_use]
extern crate clap;
use clap::App;

mod cli;
mod git;
mod gui;

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

fn cleanup(mut stdout: &Stdout) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    Ok(())
}

fn quick_checkout(stdout: &Stdout, s: String, branches: Vec<String>) -> Result<()> {
    let matching = git::get_matching_branches(&s, &branches);
    if matching.is_empty() {
        gui::display_closing_error(&stdout, String::from("Could not find a matching branch"))?;
    } else if let Err(s) = git::change_branch(&matching.first().unwrap()) {
        gui::display_closing_error(&stdout, s)?;
    }
    Ok(())
}

fn update_branches(stdout: &Stdout) -> Result<()> {
    if let Err(s) = git::update_branches() {
        gui::display_closing_error(&stdout, s)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let yaml = load_yaml!("cli_opts.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let stdout = stdout();
    setup(&stdout)?;

    let get_branches_res = git::get_branches(matches.is_present("all_branches"));
    if let Ok(branches) = get_branches_res {
        if let Some(s) = matches.value_of("BRANCH") {
            quick_checkout(&stdout, s.to_string(), branches)?;
        } else if matches.is_present("update") {
            update_branches(&stdout)?;
        } else {
            let mut cli = cli::Cli::new(branches, &stdout);
            cli.main_loop()?;
        }
    } else {
        gui::display_closing_error(&stdout, get_branches_res.err().unwrap())?
    }

    cleanup(&stdout)?;
    Ok(())
}
