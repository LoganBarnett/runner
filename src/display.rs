use tuikit::prelude::Term;

use super::error;
use super::cursor;

pub const PROMPT: &str = &"Type application name, ESC to quit: ";

pub fn prompt_display<'a>(
    term: &tuikit::prelude::Term<()>,
    input: &'a String,
    cursor: &cursor::Cursor,
) -> Result<(), error::AppError> {
    // If we just print the new input value and that value was shorter than the
    // last time, it will remain in place, creating a kind of ghost text. We
    // must clear the entire line.
    let (_, width) = term.term_size()
        .map_err(error::AppError::TerminalPrintError)
        ?;
    let prompt = format!(
        "{}{}{}",
        PROMPT,
        input,
        pad_right('_', width),
    );
    term.print(0, 0, prompt.as_str())
        .map_err(error::AppError::TerminalPrintError)
        .and_then(|_| {
            term.present()
                .map_err(error::AppError::TerminalPrintError)
        })
        .and_then(|_| {
            term
                .set_cursor(0, PROMPT.len() + cursor.x)
                .map_err(error::AppError::TerminalPrintError)
        })
        .map(|_| ())
}

pub fn instructions_display<'a>(
    term: &'a Term<()>,
    cursor: &'a cursor::Cursor,
    total: usize,
) -> Result<(), error::AppError> {
    term.print(
        total,
        0,
        format!(
            "Press RET or C-j to run the highlighted appliction {}",
            cursor,
        ).as_str(),
    )
        .map_err(error::AppError::TerminalPrintError)
        .map(|_| ())
}

pub fn pad_right<'a>(c: char, amount: usize) -> String {
    let mut pad = String::with_capacity(amount);
    for _ in 0..(amount - 1) {
        pad.push(c);
    }
    pad
}
