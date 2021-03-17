use std::result::Result;
use tuikit::error::TuikitError;
use tuikit::prelude::Term;
use tuikit::prelude::TermHeight;

mod input;
use input::Input;
use input::input_poll;

fn completions_filter<E>(
    completions: Vec<String>,
    input: String,
) -> Result<Vec<String>, E> {
    Ok(completions
        .iter()
        .map(|x| String::from(x.clone()))
        .filter(|x| x.starts_with(&input))
        .collect::<Vec<String>>())
}

fn completions_get<E>(_input: String) -> Result<Vec<String>, ()> {
    Ok(vec!["foo".to_string(), "bar".into(), "baz".into()])
}

fn completions_print_all(
    term: &Term<()>,
    line: String,
) -> Result<(), TuikitError> {
    // Compiler demands we clone here, else we try to use "moved" value for
    // completions_filter. Value is immutable. Why an error?
    match completions_get::<TuikitError>(line.clone()) {
        Ok(cs) => {
            let total = cs.len() + 1;
            completions_filter(cs, line)
                .and_then(|cs| completions_print(&term, cs))
                .and_then(|_| {
                    term.print(
                        total,
                        0,
                        "Press RET or C-j to run the highlighted appliction",
                    )
                    // Per https://github.com/lotabout/tuikit/issues/28
                    // set_cursor must be called prior to present to
                    // avoid the cursor being lost.
                }).and_then(|_| term.set_cursor(0, 0))
                  .and_then(|_| term.present())
            //     let (width, height) = term.term_size().unwrap();

            //     let attr = Attr{ fg: Color::RED, ..Attr::default() };
            //     let _ = term.print_with_attr(row, col, "Hello World! 你好！今日は。", attr);
        },
        Err(e) => {
            println!("Error getting completions: {:?}", e);
            // Execution should continue even if we
            // can't get completions.
            Ok(())
        }
    }
}

fn completions_print(
    term: &Term<()>,
    cs: Vec<String>,
) -> Result<usize, TuikitError> {
    let (_, r) = cs
        .into_iter()
        .fold(
            (0, Ok(usize::MIN)),
            |(i, r), x| (i+1, r.and_then(|_| term.print(i+1, 0, x.as_str()))),
        );
    r
}

fn command_invoke(_c: String) -> () {

}

enum Loop<E> {
    Err(E),
    Quit,
    Repeat,
}

fn main() -> Result<(), TuikitError> {
    Term::with_height(TermHeight::Percent(30)).and_then(|term: Term<()>| {
        term.present().and_then(|_| {
            let mut line = String::new();
            loop {
                match term.poll_event().and_then(|ev| {
                    match input_poll(&term, line.clone(), &ev) {
                        Input::Err(e) => Ok(Loop::Err(e)),
                        Input::ModifyCursor => Ok(Loop::Repeat),
                        Input::ModifyLine(mod_line) => {
                            line = mod_line.clone();
                            match completions_print_all(&term, mod_line) {
                                Ok(_) => Ok(Loop::Repeat),
                                Err(e) => Ok(Loop::Err(e)),
                            }
                        },
                        Input::Quit => Ok(Loop::Quit),
                        Input::Submit(c) => {
                            command_invoke(c);
                            Ok(Loop::Repeat)
                        },
                    }
                }) {
                    Ok(x) => {
                        match x {
                            Loop::Err(e) => break Err(e),
                            Loop::Quit => break Ok(()),
                            Loop::Repeat => (),
                        }
                    },
                    Err(e) => break Err(e),
                }
            }
        })
    })
    // This is redundant - main can accept a result.
    // match term_result {
    //     Ok(_) => std::process::exit(0),
    //     // TODO: Show error.
    //     Err(_) => std::process::exit(1),
    // }
}
