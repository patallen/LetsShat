use std::io::{self, Write};

use termion::clear;
use termion::color;
use termion::cursor;

use std::cmp;
use std::str;

type DrawResult = Result<(), io::Error>;

fn calculate_sidebar_width() -> u16 {
    let full_width = term_width();
    return cmp::max(full_width / 7, 20);
}

fn term_width() -> u16 {
    termion::terminal_size().unwrap().0
}

fn term_height() -> u16 {
    termion::terminal_size().unwrap().1
}

fn hide_cursor<W: Write>(screen: &mut W) -> DrawResult {
    write!(screen, "{}", cursor::Hide)
}

fn show_cursor<W: Write>(screen: &mut W) -> DrawResult {
    write!(screen, "{}", cursor::Show)
}

fn flush<W: Write>(screen: &mut W) -> DrawResult {
    screen.flush()
}

fn reset_colors<W: Write>(screen: &mut W) -> DrawResult {
    write!(
        screen,
        "{}{}",
        color::Fg(color::Reset),
        color::Bg(color::Reset)
    )
}

fn goto<W: Write>(screen: &mut W, coords: (u16, u16)) -> DrawResult {
    write!(screen, "{}", cursor::Goto(coords.0, coords.1))
}

fn color_line<W: Write>(screen: &mut W, color: &color::Color) -> DrawResult {
    write!(screen, "{}{}", color::Bg(color), clear::CurrentLine)
}

fn draw_command_buffer<W: Write>(screen: &mut W, buffer: &[u8]) -> DrawResult {
    goto(screen, (1, term_height()))?;
    write!(screen, "{}", clear::CurrentLine)?;
    write!(screen, "cmd: ")?;
    screen.write(buffer)?;
    reset_colors(screen)
}

fn draw_legend<W: Write>(screen: &mut W) -> DrawResult {
    goto(screen, (1, 1))?;
    color_line(screen, &color::Magenta)?;
    write!(screen, "[ESC]: Quit")?;
    reset_colors(screen)
}

fn draw_body<W: Write>(screen: &mut W, buffer: &[u8]) -> DrawResult {
    let start = calculate_sidebar_width() + 1;

    debug!("{:?}", buffer);
    for (line_no, string) in str::from_utf8(buffer)
        .unwrap()
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .enumerate()
    {
        goto(screen, (start, line_no as u16 + 2))?;
        write!(
            screen,
            "{}{}[Me]: {}{}",
            color::Bg(color::Rgb(10, 10, 10)),
            color::Fg(color::Magenta),
            string,
            clear::UntilNewline,
        )?;
    }
    reset_colors(screen)?;
    Ok(())
}

fn draw_sidebar<W: Write>(screen: &mut W /* items: &Vec<SidebarItem>*/) -> DrawResult {
    let width = calculate_sidebar_width();
    let height = term_height();

    let spaces = (0..width).map(|_| " ").collect::<String>();

    // Paint sidebar
    write!(screen, "{}", color::Bg(color::Rgb(20, 20, 20)))?;

    for line_no in 2..height {
        goto(screen, (0, line_no))?;
        write!(screen, "{}", spaces)?;
    }

    reset_colors(screen)
}

pub fn redraw<W: Write>(screen: &mut W, body_buff: &[u8], cmd_buff: &[u8]) -> DrawResult {
    hide_cursor(screen)?;
    draw_sidebar(screen)?;
    draw_body(screen, body_buff)?;
    draw_legend(screen)?;
    draw_command_buffer(screen, cmd_buff)?;
    show_cursor(screen)?;
    flush(screen)
}
