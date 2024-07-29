use std::{env, error::Error, io, process};
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use xx::regex;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    file: PathBuf,
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
        lint(file)
    }
}

fn lint<R: io::Read>(rdr: R) -> Result<()> {
    let debug = env::var("JDX_CSV_LINT_DEBUG").is_ok();

    let mut rdr = csv::Reader::from_reader(rdr);
    let headers = rdr.headers()?;
    let email_headers = headers.iter().enumerate().filter_map(|(i, h)| {
        if h.eq_ignore_ascii_case("email") || h.eq_ignore_ascii_case("email_address") {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    for result in rdr.records() {
        let record = result?;
        for &i in &email_headers {
            let email = &record[i];
            regex!("^[^@]+@[^@]+$").find(email).ok_or_else(|| {
                format!("Invalid email address: {}", email)
            })?;
        }
        // TODO: use log crate
        if debug {
            println!("{:?}", record);
        }
    }
    println!("CSV file is valid");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint() {
        let res = run(r#"a,b,c
1,2,3
4,5,6"#);
        assert!(res.is_ok());
    }

    #[test]
    fn test_lint_empty() {
        let res = run("");
        assert!(res.is_ok());
    }

    #[test]
    fn test_lint_empty_line() {
        let res = run("\n");
        assert!(res.is_ok());
    }

    #[test]
    fn test_lint_valid_email() {
        let res = run(r#"id,email,phone
1,foo@example.com,1234567890"#);
        assert!(res.is_ok());
    }

    #[test]
    fn test_lint_invalid_email() {
        let res = run(r#"id,email,phone
1,foo,1234567890"#);
        assert!(res.is_err());
    }

    fn run(data: &str) -> Result<()> {
        let rdr = io::Cursor::new(data);
        lint(rdr)
    }
}
