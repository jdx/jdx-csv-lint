use clap::Parser;
use std::path::PathBuf;
use log::info;
use crate::linter_builder::LinterBuilder;
use crate::Result;

/// A CSV file linter
///
/// This linter reads a CSV file and checks for common errors.
/// It can be run from the command line.
#[derive(Parser, Default)]
#[command(version, about, after_long_help = AFTER_LONG_HELP, verbatim_doc_comment)]
pub(crate) struct Cli {
    /// The CSV file(s) to lint
    #[clap(required = true, number_of_values = 1)]
    files: Vec<PathBuf>,
    /// Display all records, not just the ones with errors
    #[clap(long, hide = true)]
    pub(crate) show_all: bool,

    /// Check all rules, not just the ones that are enabled
    #[clap(short = 'a', long)]
    pub(crate) check_all: bool,

    /// The checks to enable
    #[clap(short, long, conflicts_with = "check_all")]
    pub(crate) checks: Vec<String>,
}

impl Cli {
    pub(crate) fn run(&self) -> Result<()> {
        for filename in &self.files {
            let mut linter = LinterBuilder::from_cli(self, filename.clone()).build()?;
            linter.run(xx::file::open(filename)?)?;
            info!("{} is valid", filename.display());
        }
        Ok(())
    }
}

static AFTER_LONG_HELP: &str = color_print::cstr!(r#"<bold><underline>Environment Variables:</underline></bold>

    JDX_CSV_LINT_LOG_LEVEL: The log level to use (default: info)
                            Options: trace, debug, info, warn, error

<bold><underline>Checks:</underline></bold>
    email: Check for valid email addresses

<bold><underline>Examples:</underline></bold>

    $ <bold>jdx-csv-lint</bold> examples/data/good.csv
    [ERROR jdx-csv-lint] examples/data/good.csv is valid

    $ <bold>jdx-csv-lint</bold> examples/data/bad.csv
    [ERROR jdx-csv-lint] examples/data/bad.csv is invalid
    Invalid email address: foo@bar@baz.com
"#);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_csv() -> Result<()> {
        let cli = Cli {
            files: vec![PathBuf::from("examples/data/good.csv")],
            ..Default::default()
        };
        cli.run()
    }

    #[test]
    fn test_bad_csv() -> Result<()> {
        let cli = Cli {
            files: vec![PathBuf::from("examples/data/bad.csv")],
            ..Default::default()
        };
        assert!(cli.run().is_err());
        Ok(())
    }
}
