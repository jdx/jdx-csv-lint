use std::{error::Error, io, process};
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use xx::regex;

#[derive(Parser, Default)]
#[command(version, about)]
struct Cli {
    file: PathBuf,
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
        let file = File::open(&self.file)?;
        self.lint(file)
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
        println!("CSV file is valid");
        Ok(())
    }
}

fn get_header_indices(csv_headers: &csv::StringRecord, names: &[&str]) -> Vec<usize> {
    csv_headers.iter().enumerate().filter_map(|(i, h)| {
        if names.iter().any(|&name| h.eq_ignore_ascii_case(name)) {
            Some(i)
        } else {
            None
        }
    }).collect()
}

fn validate_email(email_headers: &Vec<usize>, record: &csv::StringRecord) -> Result<()> {
    for &i in email_headers {
        let email = &record[i];
        if email.is_empty() {
            continue;
        }
        regex!("^[^@]+@[^@]+$").find(email).ok_or_else(|| {
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
