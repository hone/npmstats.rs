extern crate chrono;
extern crate gnuplot;
#[macro_use]
extern crate serde_derive;

mod options;
mod npm_registry;

use options::Options;
use npm_registry::NpmRegistry;

use chrono::prelude::*;
use gnuplot::{AxesCommon, Figure, Caption, Color};
use gnuplot::Tick::Major;
use gnuplot::AutoOption::Fix;

fn main() {
    let options = Options::new();

    if options.cmd_downloads {
        let registry = NpmRegistry::new();
        let (x, y) = registry.downloads(&options.arg_module);
        let x_index = 0..x.len();
        let today = Local::today();
        // add year labels
        let x_major = (2015..today.year() + 1).map(|year| {
            Major(x.binary_search(&format!("{}-{}", year, 1)).unwrap(), Fix(year.to_string()))
        });

        let mut fg = Figure::new();
        fg.axes2d()
            .lines(x_index, y, &[Caption(&options.arg_module), Color("black")])
            .set_x_ticks_custom(x_major, &[], &[]);
        fg.show();
    }
}
