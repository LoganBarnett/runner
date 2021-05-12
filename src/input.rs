/*******************************************************************************
 * Manage input here.
 ******************************************************************************/
use tuikit::error::TuikitError;
use tuikit::prelude::Event;
use tuikit::prelude::Key;
use tuikit::prelude::Term;

use super::cursor::Cursor;

pub enum InputEvent<E> {
    Err(E),
    Modify(String, Cursor),
    Quit,
    Submit(String),
}

pub fn input_poll(
    term: &Term<()>,
    cursor: Cursor,
    line: String,
    ev: &Event,
) -> InputEvent<TuikitError> {
    let chain = term.clear().and_then(|_| {
        let prompt =  "Type application name, ESC to quit: ";
        // Instead of using cursor.y here, it is used to highlight candidates.
        term.print(cursor.x + prompt.len(), 0, prompt)
    })
        ;
    match chain {
        Ok(_) => {
            match ev {
                Event::Key(Key::ESC) => InputEvent::Quit,
                Event::Key(Key::Ctrl('c')) => InputEvent::Quit,
                Event::Key(Key::Enter) => InputEvent::Submit(line),
                Event::Key(Key::Backspace) => {
                    InputEvent::Modify(
                        line.chars().take(cursor.x).skip(1).collect(),
                        cursor.back(),
                    )
                },
                Event::Key(Key::Ctrl('a')) => {
                    InputEvent::Modify(line, cursor.home())
                },
                Event::Key(Key::Char(ch)) => {
                    let begin: String = line.chars().take(cursor.x).collect();
                    let end: String = line.chars().skip(cursor.x).collect();
                    // This seems like a very expensive concat (or extend, in
                    // Rust parlance), but I had trouble with extend.
                    let mod_line = format!("{}{}{}", begin, ch, end);
                    match term.print(
                        0,
                        0,
                        format!(
                            "Type application name, ESC to quit: {}",
                            mod_line,
                        ).as_str(),
                    ) {
                        Ok(_) => {
                            InputEvent::Modify(
                                mod_line.clone(),
                                cursor.forward(mod_line),
                            )
                        },
                        Err(e) => InputEvent::Err(e),
                    }
                },
                _ => InputEvent::Modify(line, cursor),
            }
        },
        Err(e) => InputEvent::Err(e),
    }
}
