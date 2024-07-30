use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use std::{error::Error, io, process};
use xx::regex;

#[derive(Parser, Default)]
#[command(version, about)]
struct Cli {
    #[clap(required = true, number_of_values = 1)]
    files: Vec<PathBuf>,
    #[clap(long)]
    show_all: bool,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = cli.run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

impl Cli {
    fn run(&self) -> Result<()> {
        for filename in &self.files {
            let file = File::open(&filename)?;
            self.lint(file)?;
            println!("[jdx-csv-lint] {} is valid", filename.display());
        }
        Ok(())
    }

    fn lint<R: io::Read>(&self, rdr: R) -> Result<()> {
        let mut rdr = csv::Reader::from_reader(rdr);
        let headers = rdr.headers()?;
        let email_headers = get_header_indices(headers, &["email", "email_address"]);
        for result in rdr.records() {
            let record = result?;
            // TODO: use log crate
            if self.show_all {
                println!("{:?}", record);
            }
            validate_email(&email_headers, &record)?;
        }
        Ok(())
    }
}

fn get_header_indices(csv_headers: &csv::StringRecord, names: &[&str]) -> Vec<usize> {
    csv_headers
        .iter()
        .enumerate()
        .filter_map(|(i, h)| {
            if names.iter().any(|&name| h.eq_ignore_ascii_case(name)) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn validate_email(email_headers: &Vec<usize>, record: &csv::StringRecord) -> Result<()> {
    for &i in email_headers {
        let email = &record[i];
        if email.is_empty() {
            continue;
        }
        // https://emailregex.com
        regex!(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).find(email).ok_or_else(|| {
            format!("Invalid email address: {}", email)
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() -> Result<()> {
        let cli = Cli {
            file: PathBuf::from("examples/data/test.csv"),
            ..Default::default()
        };
        cli.run()
    }

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
        let cli = Cli {
            file: PathBuf::from("test.csv"),
            ..Default::default()
        };
        cli.lint(rdr)
    }
}
