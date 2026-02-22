use core::cmp::min;
use std::{env, io::Error};

use crossterm::event::{
    Event::{self, Key},
    KeyCode::{self},
    KeyEvent, KeyEventKind, KeyModifiers, read,
};

mod terminal;
mod view;
use crate::editor::{
    terminal::{Position, Size},
    view::View,
};
use terminal::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default, Debug)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
pub struct Editor {
    view: View,
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_content) = args.get(1) {
            self.view.load(file_content);
        }
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            state,
            kind: KeyEventKind::Press,
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                x = x.saturating_sub(1);
            }
            KeyCode::Down => {
                x = min(height.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::Left => {
                y = y.saturating_sub(1);
            }
            KeyCode::Right => {
                y = min(width.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
