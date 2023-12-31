use std::result::Result;
use tuikit::prelude::Term;
use tuikit::prelude::TermHeight;

mod completions;
mod cursor;
use cursor::Cursor;
mod display;
mod error;
mod input;
use input::InputEvent;
use input::input_poll;

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
                    InputEvent::Noop => Ok(Loop::Repeat),
                    InputEvent::Modify(mod_line, mod_cursor) => {
                        line = mod_line;
                        cursor = mod_cursor.clone();
                        display::prompt_display(&term, &line, &cursor)?;
                        let result = completions::completions_print_all(
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
