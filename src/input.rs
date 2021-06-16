/*******************************************************************************
 * Manage input here.
 ******************************************************************************/
use tuikit::prelude::Event;
use tuikit::prelude::Key;
use tuikit::prelude::Term;

use super::display;
use super::error;
use super::cursor::Cursor;

pub enum InputEvent {
    Modify(String, Cursor),
    Noop,
    Quit,
    Submit(String),
}

pub fn input_poll<'a>(
    term: &'a Term<()>,
    cursor: &'a Cursor,
    line: &'a String,
    ev: &'a Event,
) -> Result<InputEvent, error::AppError> {
    let _  = display::prompt_display(&term, &line, &cursor)
        .map(|_|
             term
             .present()
             .map_err(error::AppError::TerminalPrintError)
        )
        ?;
    match ev {
        Event::Key(Key::ESC) => Ok(InputEvent::Quit),
        Event::Key(Key::Ctrl('c')) => Ok(InputEvent::Quit),
        Event::Key(Key::Enter) => Ok(InputEvent::Submit(line.to_string())),
        Event::Key(Key::Backspace) => {
            Ok(InputEvent::Modify(
                if cursor.x == 0 {
                    line.to_string()
                } else {
                    format!(
                        "{}{}",
                        line.chars().take(cursor.x - 1).collect::<String>(),
                        line.chars().skip(cursor.x).collect::<String>(),
                    )
                },
                cursor.back(),
            ))
        },
        Event::Key(Key::Ctrl('a')) => {
            Ok(InputEvent::Modify(line.to_string(), cursor.home()))
        },
        Event::Restarted => {
            Ok(InputEvent::Noop)
        },
        Event::Key(Key::Char(ch)) => {
            let begin: String = line.chars().take(cursor.x).collect();
            let end: String = line.chars().skip(cursor.x).collect();
            // This seems like a very expensive concat (or extend, in
            // Rust parlance), but I had trouble with extend.
            let mod_line = format!("{}{}{}", begin, ch, end);
            Ok(InputEvent::Modify(mod_line.clone(), cursor.forward(&mod_line)))
        },
        x => Err(error::AppError::InputUnknownError(x.clone())),

    }
}
