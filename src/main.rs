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
        let colors = ["black", "red", "blue", "brown", "yellow"];
        let registry = NpmRegistry::new();
        let (dates, downloads) = registry.downloads(&options.arg_module);
        let today = Local::today();
        // add year labels
        let x_major = (2015..today.year() + 1).map(|year| {
            Major(dates.binary_search(&format!("{}-{}", year, 1)).unwrap(), Fix(year.to_string()))
        });

        let mut fg = Figure::new();
        // can't borrow fg mutably twice, so force it out of scope
        {
            let mut axes2d = fg.axes2d()
                .set_x_ticks_custom(x_major, &[], &[]);
            let mut color_index = 0;
            for (module, counts) in downloads {
                let dates_index = 0..dates.len();
                axes2d.lines(dates_index, counts, &[Caption(&module), Color(colors[color_index])]);
                color_index += 1;
            }
        }
        fg.show();
    }
}
