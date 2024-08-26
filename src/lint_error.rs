use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LintError {
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error("Parse error: {filename}[{line}]: ({record}) {message}")]
    Check {
        filename: PathBuf,
        line: usize,
        record: String,
        message: String,
    },
}