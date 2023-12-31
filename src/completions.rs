use std::result::Result;
use tuikit::prelude::Term;

use super::cursor;
use super::display;
use super::error;

pub const COMPLETIONS_DISPLAY_MAX: usize = 3;

pub struct Completion {
    display: String,
    // We can assume a String here because that is the value that will be fed to
    // the filter and execution scripts. In general it should be assumed that
    // the completion/filter/execution suite is consistent with itself and will
    // have to handle validation and deserialization between themselves. Runner
    // will stupidly hand the values along.
    value: String,
}

// Leave this as a Result - right now it's hard coded but we want error handling
// here.
// fn completions_filter<'a, E, T>(
//     completions: &'a Vec<Completion<T>>,
//     input: &'a String,
// ) -> Result<Vec<&'a Completion<T>>, E> {
//     Ok(
//         completions
//             .into_iter()
//             .filter(|x| x.display.starts_with(&input.as_str()))
//             .collect::<Vec<&Completion<T>>>(),
//     )
// }

// Leave this as a Result - right now it's hard coded but we want error handling
// here.
fn completions_get<'a>(
    _input: &'a String,
    default_value: String,
) -> Result<Vec<Completion>, error::AppError> {
    Ok(
        vec!["foo".to_string(), "bar".into(), "baz".into()]
            .into_iter()
            // .iter()
            .map(|x| Completion {
                display: x.to_string(),
                value: default_value.clone(),
            })
            .collect::<Vec<Completion>>()
    )
}

pub fn completions_print_all<'a>(
    term: &'a Term<()>,
    line: &'a String,
    cursor: &'a cursor::Cursor,
) -> Result<(), error::AppError> {
    match completions_get(&line, "hi".to_string()) {
        Ok(cs) => {
            completions_print(&term, &cs)
                .and_then(|_| {
                    display::instructions_display(&term, &cursor, cs.len() + 1)
                })
                .and_then(|_| {
                    term.present()
                        .map_err(error::AppError::TerminalPrintError)
                })
        },
        Err(e) => {
            println!("Error getting completions: {:?}", e);
            // Execution should continue even if we
            // can't get completions.
            Ok(())
        }
    }
}

fn completions_print<'a>(
    term: &'a Term<()>,
    cs: &'a Vec<Completion>,
) -> Result<usize, error::AppError> {
    let (_, r) = cs
        .into_iter()
        .fold(
            (0, Ok(usize::MIN)),
            |(i, r), x| {
                (i+1, r.and_then(|_| term.print(i+1, 0, x.display.as_str())))
            },
        );
    completions_clear_unused_lines(&term, cs.len())?;
    r.map_err(error::AppError::TerminalPrintError)
}

// If we just print the new input value and that value was shorter than the last
// time, it will remain in place, creating a kind of ghost text. We must clear
// the entire line.
fn completions_clear_unused_lines<'a>(
    term: &'a Term,
    lines_printed: usize,
) -> Result<(), error::AppError> {
    let (_, width) = term.term_size()
        .map_err(error::AppError::TerminalPrintError)
        ?;
    for i in 0..(COMPLETIONS_DISPLAY_MAX - lines_printed) {
        term.print(
            i+1 + lines_printed,
            0,
            &display::pad_right('*', width),
        )
            .map_err(error::AppError::TerminalPrintError)
            ?;
    }
    Ok(())
}
