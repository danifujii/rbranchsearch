use crossterm::{cursor, execute, queue, style, style::Color, terminal, Result};
use std::io::{Stdout, Write};

pub fn display_closing_error(mut stdout: &Stdout, err: String) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print(err),
        cursor::MoveToNextLine(1)
    )?;
    Ok(())
}

pub fn write_line(mut stdout: &Stdout, line: &String, idx: u16) -> Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, idx),
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::Print(line),
        cursor::MoveToNextLine(1),
    )?;
    Ok(())
}

pub fn write_line_with_color(mut stdout: &Stdout, line: &String, idx: u16, color: Color) -> Result<()> {
    execute!(stdout, style::SetForegroundColor(color))?;
    write_line(&stdout, line, idx)?;
    execute!(stdout, style::ResetColor)?;
    Ok(())
}

pub fn write_lines(mut stdout: &Stdout, lines: &Vec<String>, offset: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(0, offset),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )?;
    for i in 0..lines.len() {
        queue!(stdout, style::Print(&lines[i]), cursor::MoveToNextLine(1))?;
    }
    stdout.flush()?;
    Ok(())
}
