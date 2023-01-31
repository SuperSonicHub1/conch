/// Solely exists for sharing an error type between `conch` and `conch_macros`.
use std::error::Error;
use std::fmt;

use nu_parser::ParseError;
use nu_protocol::{ShellError, PipelineData};
use miette::{MietteHandlerOpts, RgbColors, ReportHandler};

#[derive(Debug)]
pub enum NuError {
    Parse(ParseError),
    Shell(ShellError),
}

pub type NuResult = Result<PipelineData, NuError>;

/// Mainly exists for good errors when writing Nu in the `sh!` macro. 
impl fmt::Display for NuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let handler = MietteHandlerOpts::new()
            .rgb_colors(RgbColors::Never)
            .color(false)
            .unicode(true)
            .build();

        match self {
            NuError::Parse(err) => handler.debug(err, f),
            NuError::Shell(err) => handler.debug(err, f)
        }
    }
}

/// Forwards source of the inner error.
impl Error for NuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NuError::Parse(err) => err.source(),
            NuError::Shell(err) => err.source()
        }
    }
}

impl From<ParseError> for NuError {
    fn from(error: ParseError) -> Self {
        NuError::Parse(error)
    }
}

impl From<ShellError> for NuError {
    fn from(error: ShellError) -> Self {
        NuError::Shell(error)
    }
}
