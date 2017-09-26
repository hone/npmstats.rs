extern crate gnuplot;
#[macro_use]
extern crate serde_derive;

mod options;
mod npm_registry;

use options::Options;
use npm_registry::NpmRegistry;

use gnuplot::{AxesCommon, Figure, Caption, Color};
use gnuplot::Tick::{Major, Minor};
use gnuplot::AutoOption::Fix;

fn main() {
    let options = Options::new();

    if options.cmd_downloads {
        let colors = ["black", "red", "blue", "brown", "green"];
        let registry = NpmRegistry::new();
        let (dates, downloads) = registry.downloads(&options.arg_module);
        // add year labels
        let mut date_index = -1;
        let x_major = dates.iter().map(|date| {
            let parts: Vec<&str> = date.split("-").collect();
            let year = parts[0];
            let month = parts[1];
            date_index += 1;

            if month == "1" {
                Major(date_index, Fix(year.to_string()))
            } else {
                Minor(date_index)
            }
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
