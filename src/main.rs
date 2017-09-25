#[macro_use]
extern crate serde_derive;

mod options;
mod npm_registry;

use options::Options;
use npm_registry::NpmRegistry;

fn main() {
    let options = Options::new();

    if options.cmd_downloads {
        let registry = NpmRegistry::new();
        registry.downloads(&options.arg_module)
    }
}
