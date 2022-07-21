use std::io::{Write, Stdout, stdout};
use crossterm::{
    queue,
    execute,
    event::{KeyEvent, KeyModifiers, KeyCode, Event, read},
    style::{Print, Color, Color::Rgb, Stylize, StyledContent},
    cursor,
    terminal::{self, enable_raw_mode, ClearType}
};

extern crate term_size;

use gitko_render::{DrawScreen, Line, ScreenSize, Position, Style};
use gitko_common::num;

pub static mut MAX_WIDTH: i32   = 0;
pub static mut MAX_HEIGHT: i32  = 0;

static HIGHLIGHT_COLOR: Color = Rgb { r: 50, g: 50, b: 50 };

pub fn screen_factory(size: ScreenSize, position: Position) -> Box<dyn DrawScreen> {
    Box::new(CrosstermWindow::new(size, position))
}

pub fn init() {
    enable_raw_mode().unwrap();
    let (cols, rows) = term_size::dimensions().unwrap();
    unsafe {
        MAX_HEIGHT = rows as i32;
        MAX_WIDTH  = cols as i32;
    }
}

pub fn exit() {
    terminal::disable_raw_mode().unwrap();
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All)
    ).unwrap();
}

pub struct CrosstermWindow {
    lines: Vec<Line>,
    height: i32,
    width: i32,
    screen_start: Position,
    cursor_position: Position,
    cursor_shown: bool,
    stdout: Stdout
}

impl CrosstermWindow {
    pub fn new(size: ScreenSize, position: Position) -> CrosstermWindow {
        let (cols, rows) = term_size::dimensions().unwrap();

        let mut crossterm_window = CrosstermWindow {
            lines: vec![],
            height: rows as i32,
            width: cols as i32,
            screen_start: Position::default(),
            cursor_position: Position::default(),
            cursor_shown: true,
            stdout: stdout()
        };

        if size.lines != 0 && size.cols != 0 {
            crossterm_window.resize(size);
        }

        if position != Position::default() {
            crossterm_window.screen_start = position;
        }

        crossterm_window
    }

    fn screen_start(&self) -> (u16, u16) {
        let x = self.screen_start.x.try_into().unwrap();
        let y = self.screen_start.y.try_into().unwrap();
        (x, y)
    }

    fn cursor_position(&self) -> (u16, u16) {
        let x = (self.screen_start.x + self.cursor_position.x).try_into().unwrap();
        let y = (self.screen_start.y + self.cursor_position.y).try_into().unwrap();
        (x, y)
    }

    fn create_screen_data(&self) -> Vec<Vec<StyledContent<String>>> {
        let mut lines: Vec<Vec<StyledContent<String>>> = vec![];
        let mut split_part: Option<StyledContent<String>> = None;

        for line in &self.lines {
            let mut styled_line: Vec<StyledContent<String>> = vec![];

            if let Some(split) = split_part {
                styled_line.push(split);
                split_part = None;
            }

            'parts:
            for (j, part) in line.parts.iter().enumerate() {
                let mut output_str = part.value.clone().stylize();

                for style in &part.styles {
                    match style {
                        Style::Underlined => {
                            output_str = output_str.underlined();
                        },
                        Style::Bold => {
                            output_str = output_str.bold();
                        },
                        Style::Painted (foreground, background) => {
                            let foreground_color = Rgb {
                                r: foreground.0,
                                g: foreground.1,
                                b: foreground.2
                            };

                            let background_color = Rgb {
                                r: background.0,
                                g: background.1,
                                b: background.2
                            };

                            output_str = output_str
                                .with(foreground_color)
                                .on(background_color);
                        },
                        _ => { }
                    }
                }

                let output = part.value.clone();
                let current_len = output.len();
                let previous_length: usize = line
                    .parts[..j]
                    .iter()
                    .map(|p| p.value.len())
                    .sum();

                // Checks if the current part is over the screen width.
                let over_width = previous_length + current_len > self.width as usize;

                // If the current part is over the screen width, split it apart
                // and render the second part into the next line.
                if over_width {
                    let idx = current_len - ((previous_length + current_len) - self.width as usize);

                    let first  = &output[..idx];
                    let second = &output[idx..];

                    let style  = output_str.style().clone();
                    styled_line.push(StyledContent::new(style, first.to_string()));
                    split_part = Some(StyledContent::new(style, second.to_string()));
                    break 'parts
                } else {
                    styled_line.push(output_str);
                }
            }

            lines.push(styled_line);
        }

        lines
    }
}

impl DrawScreen for CrosstermWindow {
    fn set_data(&mut self, lines: Vec<Line>) {
        if lines.len() == 0 {
            self.lines = vec![Line::plain("")]
        } else {
            self.lines = lines;
        }
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn resize(&mut self, new_size: ScreenSize) {
        let columns = new_size.cols.try_into().unwrap();
        let rows    = new_size.lines.try_into().unwrap();

        terminal::SetSize(columns, rows);

        self.height = new_size.lines;
        self.width  = new_size.cols;
    }

    fn show_cursor(&mut self, show: bool) {
        self.cursor_shown = show;
        if show {
            execute!(self.stdout, cursor::Show).unwrap();
        } else {
            execute!(self.stdout, cursor::Hide).unwrap();
        }
    }

    fn get_cursor_line(&self) -> String {
        let index = self.cursor_position.y as usize;
        if index >= self.lines.len() {
            return "".to_owned()
        }

        self.lines[index].value()
    }

    fn queue_update(&mut self) {
        let (start_x, start_y)   = self.screen_start();
        let (cursor_x, cursor_y) = self.cursor_position();

        queue!(
            self.stdout,
            cursor::MoveTo(start_x, start_y),
            terminal::Clear(ClearType::FromCursorDown)
        ).unwrap();

        let screen_data = self.create_screen_data();
        for (line_number, styled_line) in screen_data.iter().enumerate() {
            let cursor_line = line_number as i32 == self.cursor_position.y && self.cursor_shown;

            for (part_idx, part) in styled_line.iter().enumerate() {
                if cursor_line {
                    queue!(self.stdout, Print(part.clone().on(HIGHLIGHT_COLOR)))
                        .unwrap();
                } else {
                    queue!(self.stdout, Print(part))
                        .unwrap();
                }

                // output_str contains only the text context, so it is
                // required to highlight the rest of the line as well.
                let last_part = part_idx == styled_line.len() - 1;
                if cursor_line && last_part {
                    let line_length: usize = styled_line
                        .iter()
                        .map(|p| p.content().len())
                        .sum();

                    if line_length >= self.width  as usize {
                        continue
                    }

                    let filler = format!(
                        "{text:<width$}",
                        text  = "",
                        width = self.width as usize - line_length
                    );

                    queue!(self.stdout, Print(filler.on(HIGHLIGHT_COLOR)))
                        .unwrap();
                }
            }

            queue!(self.stdout, cursor::MoveToNextLine(1))
                .unwrap();
        }

        // We highlight the cursor line while looping through the lines,
        // but if the cursor is beyond the lines, we still need to do it.
        if self.cursor_shown && self.cursor_position.y as usize >= self.lines.len() {
            let filler = format!(
                "{text:<width$}",
                text  = "",
                width = self.width as usize
            );

            queue!(
                self.stdout,
                cursor::MoveTo(cursor_x, cursor_y),
                Print(filler.on(HIGHLIGHT_COLOR)),
                cursor::MoveTo(start_x, start_y),
            ).unwrap();
        }

        queue!(self.stdout, cursor::MoveTo(cursor_x, cursor_y))
            .unwrap();
    }

    fn refresh(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn clear(&mut self) {
        let (x, y) = self.cursor_position();
        execute!(
            self.stdout,
            cursor::MoveTo(x, y + 1),
            terminal::Clear(ClearType::FromCursorDown)
        ).unwrap();
    }

    // Returns the delta between the attempted cursor
    // position move and actual end position.
    // This is the value which the data needs to be
    // scrolled by.
    // TODO: pretty confusing, need better way.
    fn move_cursor(&mut self, position: Position) -> (i32, Position) {
        let y = num::clamp(position.y, 0, self.height - 1);
        let x = num::clamp(position.x, 0, self.width - 1);

        let delta = position.y - y;

        let cursor_movement = position.y - self.cursor_position.y;
        let move_by = cursor_movement.abs().try_into().unwrap();

        if cursor_movement > 0 {
            queue!(self.stdout, cursor::MoveToNextLine(move_by))
                .unwrap();
        } else {
            queue!(self.stdout, cursor::MoveToPreviousLine(move_by))
                .unwrap();
        }

        self.cursor_position = Position { x: x.into(), y: y.into() };

        (delta, self.cursor_position)
    }

    fn set_cursor(&mut self, position: Position) {
        let x = (self.screen_start.x + position.x).try_into().unwrap();
        let y = (self.screen_start.y + position.y).try_into().unwrap();

        queue!(self.stdout, cursor::MoveTo(x, y)).unwrap();
    }

    fn listen_input(&self) -> i32 {
        loop {
            match read().unwrap() {
                Event::Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => {
                    match code {
                        KeyCode::Char('d') => return 4,
                        KeyCode::Char('u') => return 21,
                        KeyCode::Char(c)   => return c as i32,
                        _ => { }
                    }
                },
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Enter   => return 10,
                        KeyCode::Esc     => return 27,
                        KeyCode::Char(c) => return c as i32,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn listen(&mut self) {
        'input_loop:
        loop {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Esc     => break 'input_loop,
                        KeyCode::Enter   => break 'input_loop,
                        KeyCode::Char(c) => {
                            let mut line = self.get_cursor_line();
                            let index = self.cursor_position.x as usize;

                            line.insert(index, c);
                            self.lines[0] = Line::plain(&line);

                            self.cursor_position.move_right(1);
                            let (x, y) = self.cursor_position();

                            execute!(
                                self.stdout,
                                terminal::Clear(ClearType::CurrentLine),
                                cursor::MoveLeft(line.len() as u16 - 1),
                                Print(line),
                                cursor::MoveTo(x, y)
                            ).unwrap();
                        },
                        KeyCode::Backspace =>  {
                            let mut line = self.get_cursor_line();
                            if line.is_empty() { continue }

                            self.cursor_position.move_left(1);
                            let index = self.cursor_position.x as usize;

                            line.remove(index);
                            self.lines[0] = Line::plain(&line);

                            let (x, y) = self.cursor_position();
                            execute!(
                                self.stdout,
                                terminal::Clear(ClearType::CurrentLine),
                                cursor::MoveLeft(line.len() as u16 + 1),
                                Print(line),
                                cursor::MoveTo(x, y)
                            ).unwrap();
                        },
                        KeyCode::Delete => {
                            let mut line = self.get_cursor_line();
                            if line.is_empty() { continue }

                            let index = self.cursor_position.x as usize;
                            if index >= line.len() { continue }

                            line.remove(index);
                            self.lines[0] = Line::plain(&line);

                            let (x, y) = self.cursor_position();
                            execute!(
                                self.stdout,
                                terminal::Clear(ClearType::CurrentLine),
                                cursor::MoveLeft(line.len() as u16 + 1),
                                Print(line),
                                cursor::MoveTo(x, y),
                            ).unwrap();
                        },
                        KeyCode::Left  => {
                            let next = self.cursor_position.x;
                            if next > 0 {
                                self.cursor_position.move_left(1);
                                execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
                            }
                        }
                        KeyCode::Right => {
                            let line = self.get_cursor_line();

                            let next = self.cursor_position.x + 1;
                            if next <= line.len() as i32 {
                                self.cursor_position.move_right(1);
                                execute!(self.stdout, cursor::MoveRight(1)).unwrap();
                            }
                        }
                        _ => {  }
                    }
                }
                _ => {  }
            }
        }
    }
}

impl Drop for CrosstermWindow {
    fn drop(&mut self) {
        execute!(self.stdout, cursor::Show).unwrap();
    }
}
