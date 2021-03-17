/*******************************************************************************
 * Manage input here.
 ******************************************************************************/
use tuikit::error::TuikitError;
use tuikit::prelude::Event;
use tuikit::prelude::Key;
use tuikit::prelude::Term;

pub struct Cursor {
    x: i32,
    y: i32,
}

pub enum Input<E> {
    Err(E),
    ModifyLine(String),
    ModifyCursor,
    // Modify(String, Cursor),
    Quit,
    Submit(String),
}

pub fn input_poll(
    term: &Term<()>,
    line: String,
    ev: &Event,
) -> Input<TuikitError> {
    let chain = term.clear().and_then(|_| {
        term.print(0, 0, "Type application name, ESC to quit: ")
    })
        ;
    match chain {
        Ok(_) => {
            match ev {
                Event::Key(Key::ESC) => Input::Quit,
                Event::Key(Key::Ctrl('c')) => Input::Quit,
                Event::Key(Key::Enter) => Input::Submit(line),
                Event::Key(Key::Backspace) => {
                    match line.get(0..(line.len() - 1)) {
                        Some(l) => Input::ModifyLine(l.to_string()),
                        None => Input::ModifyLine("".to_string()),
                    }
                },
                Event::Key(Key::Ctrl('a')) => Input::ModifyCursor,
                Event::Key(Key::Char(ch)) => {
                    let mod_line = line + &ch.to_string();
                    match term.print(
                        0,
                        0,
                        format!(
                            "Type application name, ESC to quit: {}",
                            mod_line,
                        ).as_str(),
                    ) {
                        Ok(_) => Input::ModifyLine(mod_line),
                        Err(e) => Input::Err(e),
                    }
                },
                _ => Input::ModifyLine(line),
            }
        },
        Err(e) => Input::Err(e),
    }
}
