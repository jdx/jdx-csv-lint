use std::collections::HashSet;
use std::path::PathBuf;

use crate::checks::Check;
use crate::cli::Cli;
use crate::linter::Linter;
use crate::Result;

#[derive(Default)]
pub struct LinterBuilder {
    pub filename: PathBuf,
    pub show_all: bool,
    pub checks: Vec<String>,
}

impl LinterBuilder {
    pub fn build(self) -> Result<Linter> {
        let mut linter = Linter::default();
        linter.filename = self.filename;
        linter.show_all = self.show_all;
        let checks = Check::list_keys().into_iter().collect::<HashSet<&str>>();
        for check in self.checks {
            match checks.get(check.as_str()) {
                Some(check) => {
                    linter.checks.push(check);
                }
                None => {
                    return Err(format!("Unknown check: {check}").into());
                }
            }
        }
        Ok(linter)
    }

    pub fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = show_all;
        self
    }

    pub fn from_cli(cli: &Cli, filename: PathBuf) -> Self {
        LinterBuilder {
            filename,
            show_all: cli.show_all,
            checks: if cli.check_all {
                Check::list_keys()
                    .into_iter()
                    .map(|key| key.to_string())
                    .collect()
            } else {
                cli.checks.clone()
            },
        }
    }
}
