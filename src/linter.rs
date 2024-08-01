use std::io;
use std::path::PathBuf;

use itertools::Itertools;
use log::{debug, error};

use crate::checks::Check;
use crate::lint_error::LintError;
use crate::Result;

#[derive(Default)]
pub struct Linter {
    pub filename: PathBuf,
    pub show_all: bool,
    pub checks: Vec<&'static str>,
    pub errors: Vec<LintError>,

    headers: csv::StringRecord,
    line: u64,
    header_checks: Vec<Vec<&'static str>>,
}

impl Linter {
    pub fn run<R: io::Read>(&mut self, rdr: R) -> Result<()> {
        let mut rdr = csv::Reader::from_reader(rdr);
        self.headers = rdr.headers()?.clone();
        self.header_checks = self.init_header_checks();
        self.line += 1;
        for result in rdr.records() {
            self.line += 1;
            let err = match result {
                Ok(record) => {
                    if let Err(err) = self.lint_record(&record) {
                        Some(LintError::Check {
                            filename: self.filename.clone(),
                            line: self.line,
                            record: display_record(&record),
                            message: err.to_string(),
                        })
                    } else {
                        None
                    }
                }
                Err(err) => Some(err.into()),
            };
            if let Some(err) = err {
                error!("{err}");
                self.errors.push(err);
            }
        }
        if !self.errors.is_empty() {
            return Err(format!("{} is invalid", self.filename.display()).into());
        }
        Ok(())
    }

    /// creates a list of checks for each header of the csv
    /// filters the checks to only include the ones that are set on self.checks
    fn init_header_checks(&self) -> Vec<Vec<&'static str>> {
        self.headers
            .iter()
            .map(|k| Check::get_for_column(k).iter()
                .filter(|c| self.checks.contains(&c.key))
                .map(|c| c.key).collect())
            .collect()
    }

    fn lint_record(&mut self, record: &csv::StringRecord) -> Result<()> {
        debug!("{}", self.display_record(record));
        if self.show_all {
            println!("{}", self.display_record(record));
        }
        for (i, check_keys) in self.header_checks.iter().enumerate() {
            for check_key in check_keys {
                let check = Check::get(check_key).unwrap();
                let data = record.get(i).unwrap();
                check.run(data)?;
            }
        }
        Ok(())
    }

    fn display_record(&self, record: &csv::StringRecord) -> String {
        format!("{filename}[{line}]: {record}",
                filename = self.filename.display(),
                line = self.line,
                record = display_record(record),
        )
    }
}

fn display_record(record: &csv::StringRecord) -> String {
    record.iter().join(",")
}


#[cfg(test)]
mod tests {
    use crate::linter_builder::LinterBuilder;

    use super::*;

    #[test]
    fn test_lint() -> Result<()> {
        run(r#"a,b,c
1,2,3
4,5,6"#)
    }

    #[test]
    fn test_lint_empty() -> Result<()> {
        run("")
    }

    #[test]
    fn test_lint_empty_line() -> Result<()> {
        run("\n")
    }

    #[test]
    fn test_lint_valid_email() -> Result<()> {
        run(r#"id,email,phone
1,foo@example.com,1234567890"#)
    }

    #[test]
    fn test_lint_invalid_email() -> Result<()> {
        let res = run(r#"id,email,phone
1,foo,1234567890"#);
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_lint_empty_email() -> Result<()> {
        run(r#"id,email_address,phone
1,,1234567890"#)
    }

    fn run(data: &str) -> Result<()> {
        let rdr = io::Cursor::new(data);
        let mut linter = LinterBuilder::default().build()?;
        linter.checks = vec!["email"];
        linter.run(rdr)
    }
}
