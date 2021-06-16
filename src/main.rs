use std::result::Result;
use tuikit::error::TuikitError;
use tuikit::prelude::Term;
use tuikit::prelude::TermHeight;

mod cursor;
use cursor::Cursor;
mod display;
mod error;
mod input;
use input::InputEvent;
use input::input_poll;

pub const COMPLETIONS_DISPLAY_MAX: usize = 3;

// Leave this as a Result - right now it's hard coded but we want error handling
// here.
fn completions_filter<'a, E>(
    completions: &'a Vec<String>,
    input: &'a String,
) -> Result<Vec<String>, E> {
    Ok(completions
        .iter()
        .map(|x| String::from(x.clone()))
        .filter(|x| x.starts_with(&input.as_str()))
        .collect::<Vec<String>>())
}

// Leave this as a Result - right now it's hard coded but we want error handling
// here.
fn completions_get<'a, E>(_input: &'a String) -> Result<Vec<String>, ()> {
    Ok(vec!["foo".to_string(), "bar".into(), "baz".into()])
}

fn completions_print_all<'a>(
    term: &'a Term<()>,
    line: &'a String,
    cursor: &'a Cursor,
) -> Result<(), error::AppError> {
    match completions_get::<TuikitError>(&line) {
        Ok(cs) => {
            completions_filter(&cs, &line)
                .and_then(|cs| completions_print(&term, &cs))
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
    cs: &'a Vec<String>,
) -> Result<usize, error::AppError> {
    let (_, r) = cs
        .into_iter()
        .fold(
            (0, Ok(usize::MIN)),
            |(i, r), x| (i+1, r.and_then(|_| term.print(i+1, 0, x.as_str()))),
        );
    completions_clear_unused_lines(&term, &cs)?;
    r.map_err(error::AppError::TerminalPrintError)
}

fn completions_clear_unused_lines<'a>(
    term: &'a Term,
    cs: &'a Vec<String>,
) -> Result<(), error::AppError> {
    let (_, width) = term.term_size()
        .map_err(error::AppError::TerminalPrintError)
        ?;
    // If we just print the new input value and that value was shorter than the
    // last time, it will remain in place, creating a kind of ghost text. We
    // must clear the entire line.
    let lines_printed = cs.len();
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

fn command_invoke(_c: String) -> () {

}

enum Loop {
    Quit,
    Repeat,
}

fn input_loop(term: &Term<()>) -> Result<(), error::AppError> {
    let mut line = String::new();
    let mut cursor = Cursor { x: 0, y: 0 };
    loop {
        let event = term
            .poll_event()
            .map_err(error::AppError::TerminalEventError)
            .and_then(|ev| {
                match input_poll(&term, &cursor, &line, &ev)? {
                    // InputEvent::Err(e) => Ok(Loop::Err(e)),
                    // InputEvent::Modify(_, _) => Ok(Loop::Repeat),
                    InputEvent::Noop => Ok(Loop::Repeat),
                    InputEvent::Modify(mod_line, mod_cursor) => {
                        line = mod_line;
                        cursor = mod_cursor.clone();
                        display::prompt_display(&term, &line, &cursor)?;
                        let result = completions_print_all(
                            &term,
                            &line,
                            &cursor,
                        ).map(|_| Loop::Repeat);
                        // Per https://github.com/lotabout/tuikit/issues/28
                        // set_cursor must be called prior to present to
                        // avoid the cursor being lost.
                        term.set_cursor(0, display::PROMPT.len() + cursor.x)
                            .map_err(error::AppError::TerminalPrintError)
                            ?;
                        result
                    },
                    InputEvent::Quit => Ok(Loop::Quit),
                    InputEvent::Submit(c) => {
                        command_invoke(c);
                        Ok(Loop::Repeat)
                    },
                }
            });
        // Use match here so we can use loop semantics such as break, which are
        // not available inside of closures/lambdas.
        match event {
            Ok(x) => {
                match x {
                    Loop::Quit => break Ok(()),
                    Loop::Repeat => (),
                }
            },
            Err(e) => break Err(e),
        }
    }
}

fn main() -> Result<(), error::AppError> {
    Term::with_height(TermHeight::Percent(30))
        .map_err(error::AppError::TerminalPrintError)
        .and_then(|term: Term<()>| {
            term
                .present()
                .map_err(error::AppError::TerminalPrintError)?;
            input_loop(&term)
        })
}
