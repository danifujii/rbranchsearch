use super::{git, gui};
use anyhow;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Color,
    Result,
};
use std::io::Stdout;

const DISPLAY_OFFSET: u16 = 6;
const HEADER: &str = "
|_   _ _  _  _ |_     _  _  _   _ _ |_
|_) | (_|| |(_ | |   _> (/_(_| | (_ | |
";
const HEADER_OFFSET: u16 = 4;
const HEADER_COLOR: Color = Color::DarkBlue;
const SELECTED_INDICATOR: char = '*';

pub struct Cli<'a> {
    branches: Vec<String>,
    displayed_branches: Vec<String>,
    selected_branch: usize,
    stdout: &'a Stdout,
    search: String,
}

impl<'a> Cli<'a> {
    pub fn new(branches: Vec<String>, stdout: &Stdout) -> Cli {
        let displayed_copy = branches.to_vec();
        Cli {
            branches,
            displayed_branches: displayed_copy,
            selected_branch: 0,
            stdout,
            search: String::new(),
        }
    }

    pub fn initial_draw(&self) -> Result<()> {
        self.draw_header()?;
        gui::write_lines(self.stdout, &self.branches, DISPLAY_OFFSET)?;
        self.draw_selected_branch()?;
        Ok(())
    }

    pub fn main_loop(&mut self) -> Result<()> {
        self.initial_draw()?;
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: kc,
                modifiers: km,
            })) = event::read()
            {
                match kc {
                    KeyCode::Up => {
                        self.update_selected_branch(true)?;
                    }
                    KeyCode::Down => {
                        self.update_selected_branch(false)?;
                    }
                    KeyCode::Enter => {
                        if let Some(b) = self.displayed_branches.get(self.selected_branch) {
                            if let Err(s) = git::change_branch(b) {
                                gui::display_closing_error(self.stdout, s)?;
                            }
                            break;
                        }
                    }
                    KeyCode::BackTab => {
                        if let Err(_) = self.delete_branch() {
                            break;
                        }
                    }
                    KeyCode::Char(c) => {
                        if km == KeyModifiers::CONTROL && c == 'c' {
                            break;
                        } else {
                            self.search.push(c);
                            self.update_displayed_branches()?;
                        }
                    }
                    KeyCode::Backspace => {
                        if !self.search.is_empty() {
                            self.search.pop();
                            self.update_displayed_branches()?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw_selected_branch(&self) -> Result<()> {
        if let Some(branch) = self.displayed_branches.get(self.selected_branch) {
            let branch: String = branch.chars().skip(1).collect();
            let selected_branch: String = format!("{}{}", SELECTED_INDICATOR, branch);
            gui::write_line(
                self.stdout,
                &selected_branch,
                self.selected_branch as u16 + DISPLAY_OFFSET,
            )?;
        }
        Ok(())
    }

    fn update_selected_branch(&mut self, up: bool) -> Result<()> {
        if (up && self.selected_branch == 0)
            || (!up && self.selected_branch + 1 >= self.displayed_branches.len())
        {
            return Ok(());
        }

        gui::write_line(
            self.stdout,
            &self.displayed_branches[self.selected_branch],
            self.selected_branch as u16 + DISPLAY_OFFSET,
        )?; // Reset previous selected
        self.selected_branch = if up {
            self.selected_branch - 1
        } else {
            self.selected_branch + 1
        };
        self.draw_selected_branch()?;
        Ok(())
    }

    fn draw_header(&self) -> Result<()> {
        let mut idx = 0;
        for line in HEADER.split("\n") {
            gui::write_line_with_color(self.stdout, &line.to_string(), idx, HEADER_COLOR)?;
            idx += 1;
        }
        gui::write_line_with_color(
            self.stdout,
            &String::from("Searching: "),
            HEADER_OFFSET,
            HEADER_COLOR,
        )?;
        Ok(())
    }

    fn update_displayed_branches(&mut self) -> Result<()> {
        gui::write_line_with_color(
            self.stdout,
            &format!("Searching: {}", self.search),
            HEADER_OFFSET,
            HEADER_COLOR,
        )?;
        self.displayed_branches = git::get_matching_branches(&self.search, &self.branches);
        gui::write_lines(self.stdout, &self.displayed_branches, DISPLAY_OFFSET)?;
        self.selected_branch = 0;
        if !self.displayed_branches.is_empty() {
            self.draw_selected_branch()?;
        }
        Ok(())
    }

    fn delete_branch(&mut self) -> anyhow::Result<()> {
        if let Some(del_branch) = self.displayed_branches.get(self.selected_branch) {
            if let Err(s) = git::delete_branch(del_branch) {
                gui::display_closing_error(self.stdout, s)?;
                anyhow::bail!("Could not delete branch");
            } else {
                self.branches = git::get_branches(false).unwrap();
                self.update_displayed_branches()?;
            }
        }
        Ok(())
    }
}
