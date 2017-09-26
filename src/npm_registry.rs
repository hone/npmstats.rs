extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use std::io::Read;
use self::chrono::{Datelike, Local, NaiveDate};
use self::chrono::prelude::*;
use self::reqwest::Method;

const API_BASE: &'static str = "api.npmjs.org";

#[derive(Deserialize)]
struct Downloads {
    pub downloads: Vec<DownloadDay>,
    pub start: String,
    pub end: String,
    pub package: String,
}

#[derive(Deserialize)]
struct DownloadDay {
    pub downloads: u32,
    pub day: String,
}

pub struct NpmRegistry {
    client: reqwest::Client,
    base_url: String,
}

impl NpmRegistry {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new().unwrap(),
            base_url: API_BASE.to_owned(),
        }
    }

    pub fn downloads(&self, module: &str) {
        let today = Local::today();
        for year in 2015..today.year() + 1 {
            for month in 1..13 {
                let npm_date = Local.ymd(year, month, 1);
                if npm_date <= today {
                    println!("{} {}: {}", year, month, self.monthly_downloads(module, month, year as u32));
                }
            }
        }
    }

    fn monthly_downloads(&self, module: &str, month: u32, year: u32) -> i32 {
        // https://api.npmjs.org/downloads/range/{period}[/{package}]
        let uri = format!("https://{host}/downloads/range/{year}-{month}-01:{year}-{month}-{day}/{module}",
                          host = self.base_url,
                          day = DateHelper::days_in_month(month, 2017),
                          month = month,
                          year = year,
                          module = module);
        let mut request = self.client.request(Method::Get, &uri).unwrap();
        let mut response = request.send().unwrap();

        match response.json() as self::reqwest::Result<Downloads> {
            Ok(json) => json.downloads.iter().fold(0, |acc, day| acc + day.downloads as i32),
            Err(e) => {
                println!("{}", e);
                let mut content = String::new();
                response.read_to_string(&mut content).unwrap();
                println!("{}: {}", response.status(), content);
                -1
            }
        }
    }
}

struct DateHelper;
impl DateHelper {
    pub fn days_in_month(month: u32, year: u32) -> u32 {
        NaiveDate::from_ymd_opt(year as i32, month + 1, 1).unwrap_or(NaiveDate::from_ymd(year as i32 + 1, 1, 1)).pred().day()
    }
}
