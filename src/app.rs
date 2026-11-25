//! `ratatui` stuff, implementing the (T)UI

use std::{io, mem};

use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::{
    eval::EvalParse,
    parse::ParseTokens,
    tokenize::{ParseError, Token, TokensToString, tokenize},
};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub raw: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode
    mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,

    /// From `tokenize::tokenize()`
    /// The `Result<_,_>` from trying to _tokenize_ input-string.
    pub tokens: Result<Vec<Token>, ParseError>,
    /// From `eval::evaluate()`
    /// The result of the equation (from `f64` to `String`)
    eval: Result<String, String>,
    current_message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,

            tokens: Ok(Vec::new()),
            eval: Ok(String::new()),
            current_message: String::new(),
        }
    }

    /// If in `InputMode::Normal` and `character_index` > 0, then move left.
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, c: char) {
        let index = self.byte_index();
        self.raw.insert(index, c);
        self.move_cursor_right();

        self.update_message();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.raw
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.raw.len())
    }

    /// match `InputMode`
    /// - `InputMode::Insert` =>
    ///   - Behave like regular backspace.
    /// - `InputMode::Normal` =>
    ///   - Behave like (n)vim 'x' key.
    ///     ie: Delete character at cursor and keep position
    ///     (unless you deleted right-most character, where you would move left)
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.raw.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.raw.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.raw = before_char_to_delete.chain(after_char_to_delete).collect();

            // Behave like (n)vim when deleting characters
            if self.mode == InputMode::Insert || self.character_index > self.raw.len() {
                self.move_cursor_left();
            }

            self.update_message();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.raw.chars().count())
    }

    /// Push a message to `self.messages` and clear the 2 other text-boxes (Input & Equation).
    pub fn submit_message(&mut self) {
        self.raw.clear();
        self.character_index = 0;

        self.messages.push(mem::take(&mut self.current_message));
        self.eval = Ok(String::new());
        self.tokens = Ok(Vec::new());
    }

    /// # TODO:
    /// - [ ] Scroll _history_ in _normal_ mode with _j|k_ keys
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<Self> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                // universal exit on <CTRL+c>
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    return Ok(self);
                }

                match self.mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i' | 'a') => {
                            self.mode = InputMode::Insert;
                            // NOTE Fixes bug where cursor would leave an empty space if you
                            //   enter Insert-mode with an empty input.
                            if self.character_index == 1 {
                                self.move_cursor_left();
                            }
                        }
                        KeyCode::Enter | KeyCode::Char('o') => self.submit_message(),
                        KeyCode::Char('x') => self.delete_char(),
                        KeyCode::Char('h') => self.move_cursor_left(),
                        KeyCode::Char('l') => self.move_cursor_right(),
                        KeyCode::Char('q') => {
                            return Ok(self);
                        }
                        _ => {}
                    },
                    InputMode::Insert if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(c) => self.enter_char(c),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Insert => {}
                }
            }
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, equation_area, messages_area] = frame.area().layout(&layout);

        let (msg, style) = match self.mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "a | i".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Insert => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.raw.as_str())
            .style(match self.mode {
                InputMode::Normal => Style::default(),
                InputMode::Insert => match self.tokens {
                    Ok(_) => Style::default().fg(Color::Green),
                    Err(_) => Style::default().fg(Color::Red),
                },
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        let equation = Paragraph::new(self.current_message.as_str())
            .style(match self.eval {
                Ok(_) => Style::default().fg(Color::Green),
                Err(_) => Style::default().fg(Color::Red),
            })
            .block(Block::bordered().title("Equation"));
        frame.render_widget(equation, equation_area);

        // Cursor shenanigans
        #[expect(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x
                + self.character_index as u16
                + if self.character_index.gt(&0)
                    && let InputMode::Normal = self.mode
                {
                    0
                } else {
                    1
                },
            // Move one line down, from the border to the input line
            // self.messages.push(self.parsed.clone());
            input_area.y + 1,
        ));

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{}: {m}", i + 1)));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    }

    fn parse_eval(&mut self) {
        self.eval = match &self.tokens {
            Ok(v) => match v.clone().parse().eval() {
                Ok(n) => Ok(n.to_string()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        };
    }

    fn update_message(&mut self) {
        self.tokens = tokenize(&self.raw);
        self.parse_eval();
        let message = match &self.tokens {
            Ok(t) => t.to_string(),
            Err(_) => self.raw.clone(),
        };
        self.current_message = message
            + " = "
            + match &self.eval {
                Ok(s) | Err(s) => s,
            };
    }
}
