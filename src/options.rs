extern crate docopt;

use self::docopt::Docopt;

const USAGE: &'static str = "
NPM Download Stats.

Usage:
  npmstats downloads <module>...
";

#[derive(Debug, Deserialize)]
pub struct Options {
    pub cmd_downloads: bool,
    pub arg_module: Vec<String>,
}

impl Options {
    pub fn new() -> Self {
        Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
    }
}
