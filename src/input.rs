/*******************************************************************************
 * Manage input here.
 ******************************************************************************/
use tuikit::error::TuikitError;
use tuikit::prelude::Event;
use tuikit::prelude::Key;
use tuikit::prelude::Term;

use super::cursor::Cursor;

#[derive(Debug)]
pub enum InputError {
    TerminalPrintError(TuikitError),
}

pub enum InputEvent {
    Modify(String, Cursor),
    Quit,
    Submit(String),
}

pub fn input_poll(
    term: &Term<()>,
    cursor: Cursor,
    line: String,
    ev: &Event,
) -> Result<InputEvent, InputError> {
    term.clear()
        .and_then(|_| {
            let prompt =  "Type application name, ESC to quit: ";
            // Instead of using cursor.y here, it is used to highlight candidates.
            term.print(cursor.x + prompt.len(), 0, prompt)
        })
        .map_err(InputError::TerminalPrintError)?;
    match ev {
        Event::Key(Key::ESC) => Ok(InputEvent::Quit),
        Event::Key(Key::Ctrl('c')) => Ok(InputEvent::Quit),
        Event::Key(Key::Enter) => Ok(InputEvent::Submit(line)),
        Event::Key(Key::Backspace) => {
            Ok(InputEvent::Modify(
                line.chars().take(cursor.x).skip(1).collect(),
                cursor.back(),
            ))
        },
        Event::Key(Key::Ctrl('a')) => {
            Ok(InputEvent::Modify(line, cursor.home()))
        },
        Event::Key(Key::Char(ch)) => {
            let begin: String = line.chars().take(cursor.x).collect();
            let end: String = line.chars().skip(cursor.x).collect();
            // This seems like a very expensive concat (or extend, in
            // Rust parlance), but I had trouble with extend.
            let mod_line = format!("{}{}{}", begin, ch, end);
            term.print(
                0,
                0,
                format!(
                    "Type application name, ESC to quit: {}",
                    mod_line,
                ).as_str(),
            )
                .map(|_| InputEvent::Modify(
                    mod_line.clone(),
                    cursor.forward(mod_line),
                ))
                .map_err(InputError::TerminalPrintError)
        },
        _ => Ok(InputEvent::Modify(line, cursor)),
    }
}
