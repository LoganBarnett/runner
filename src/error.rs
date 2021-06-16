
#[derive(Debug)]
pub enum AppError {
    InputUnknownError(tuikit::prelude::Event),
    TerminalEventError(tuikit::error::TuikitError),
    TerminalPrintError(tuikit::error::TuikitError),
}
