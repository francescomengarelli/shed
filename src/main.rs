//! `shed` command-line entry point.

use std::io::{self, Write};
use std::process::ExitCode;

const USAGE: &str = "\
Usage: shed [OPTIONS]

Options:
  -h, --help     Print help
  -V, --version  Print version
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Error::Usage(arg)) => {
            eprintln!("shed: unrecognized argument '{arg}'\n\n{USAGE}");
            ExitCode::from(2)
        }
        Err(Error::Io(err)) => {
            // A closed pipe (e.g. `shed | head`) is not worth reporting.
            if err.kind() != io::ErrorKind::BrokenPipe {
                eprintln!("shed: {err}");
            }
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug)]
enum Error {
    Usage(String),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

fn run(args: &[String], out: &mut impl Write) -> Result<(), Error> {
    match args.first().map(String::as_str) {
        None => writeln!(out, "Hello, world!")?,
        Some("-h" | "--help") => write!(out, "{USAGE}")?,
        Some("-V" | "--version") => writeln!(out, "shed {}", env!("CARGO_PKG_VERSION"))?,
        Some(other) => return Err(Error::Usage(other.to_owned())),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output(args: &[&str]) -> Result<String, Error> {
        let args: Vec<String> = args.iter().map(|&a| a.to_owned()).collect();
        let mut buf = Vec::new();
        run(&args, &mut buf)?;
        Ok(String::from_utf8(buf).unwrap_or_default())
    }

    #[test]
    fn prints_hello_world() {
        assert_eq!(output(&[]).ok().as_deref(), Some("Hello, world!\n"));
    }

    #[test]
    fn prints_version() {
        let out = output(&["--version"]).unwrap_or_default();
        assert!(out.starts_with("shed "));
    }

    #[test]
    fn rejects_unknown_argument() {
        assert!(matches!(output(&["--nope"]), Err(Error::Usage(_))));
    }
}
