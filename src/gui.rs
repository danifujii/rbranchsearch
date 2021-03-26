use crossterm::{cursor, execute, queue, style, terminal, Result};
use std::io::{Stdout, Write};

pub fn display_closing_error(mut stdout: &Stdout, err: String) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print(err)
    )?;
    Ok(())
}

pub fn write_line(mut stdout: &Stdout, line: &String, idx: u16) -> Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, idx),
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::Print(line)
    )?;
    Ok(())
}

pub fn write_lines(mut stdout: &Stdout, lines: &Vec<String>) -> Result<()> {
    queue!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown))?;
    for i in 0..lines.len() {
        queue!(stdout, style::Print(&lines[i]), cursor::MoveToNextLine(1))?;
    }
    stdout.flush()?;
    Ok(())
}
