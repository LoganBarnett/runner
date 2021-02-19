// TODO: Use explicit imports.
use tuikit::prelude::*;

fn main() {
    let term: Term<()> = Term::with_height(TermHeight::Percent(30)).unwrap();

    let _ = term.print(0, 0, "Type application name, ESC to quit: ");
    let _ = term.present();
    let mut line = String::new();

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();
        let _ = term.print(0, 0, "Type application name, ESC to quit: ");


        match ev {
            Event::Key(Key::ESC) => break,
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Backspace) => {
                line.pop();
            }
            Event::Key(Key::Char(ch)) => {
                line.push(ch);
                term.print(
                    0,
                    0,
                    format!("Type application name, ESC to quit: {}", line).as_str(),
                );
            }
            _ => {}
        }
        let completions = [ "foo", "bar", "baz" ];
        let filtered_completions = completions
            .into_iter()
            .map(|x| String::from(x.clone()))
            .filter(|x| x.starts_with(&line))
            .collect::<Vec<String>>();
        for i in 0..filtered_completions.len() {
            term.print(i+1, 0, filtered_completions[i].as_str());
        }
        term.print(completions.len()+1, 0, "Press RET or C-j to run the highlighted appliction");
    //     let (width, height) = term.term_size().unwrap();

    //     let attr = Attr{ fg: Color::RED, ..Attr::default() };
    //     let _ = term.print_with_attr(row, col, "Hello World! 你好！今日は。", attr);

        // Per https://github.com/lotabout/tuikit/issues/28 set_cursor must be
        // called prior to present to avoid the cursor being lost.
        let _ = term.set_cursor(0, 0);
        let _ = term.present();
    }
}
