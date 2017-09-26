extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use std::collections::HashMap;
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

    pub fn downloads(&self, modules: &Vec<String>) -> (Vec<String>, HashMap<String, Vec<i32>>) {
        let mut dates = Vec::new();
        let mut downloads: HashMap<String, Vec<i32>> = HashMap::new();
        let today = Local::today();
        for year in 2015..today.year() + 1 {
            for month in 1..13 {
                let npm_date = Local.ymd(year, month, 1);
                if npm_date <= today {
                    dates.push(format!("{}-{}", year, month));
                    let monthly = self.monthly_downloads(modules, month, year as u32);
                    for (module, download_count) in monthly {
                        let mut entry = downloads.entry(module).or_insert(Vec::new());
                        entry.push(download_count);
                    }
                    //println!("{} {}: {}", year, month, self.monthly_downloads(module, month, year as u32));
                }
            }
        }

        (dates, downloads)
    }

    fn monthly_downloads(&self, modules: &Vec<String>, month: u32, year: u32) -> HashMap<String, i32> {
        // https://api.npmjs.org/downloads/range/{period}[/{package}]
        let uri = format!("https://{host}/downloads/range/{year}-{month}-01:{year}-{month}-{day}/{modules}",
                          host = self.base_url,
                          day = DateHelper::days_in_month(month, 2017),
                          month = month,
                          year = year,
                          modules = modules.join(","));
        let mut request = self.client.request(Method::Get, &uri).unwrap();
        let mut response = request.send().unwrap();
        let mut result: HashMap<String, i32> = HashMap::new();

        match response.json() as self::reqwest::Result<self::serde_json::Value> {
            Ok(json) => {
                for module in modules {
                    let value = json[module].to_string();
                    let downloads: Downloads = self::serde_json::from_str(&value).unwrap();
                    let total = downloads.downloads.iter().fold(0, |acc, day| {
                        acc + day.downloads as i32
                    });

                    result.insert(module.to_owned(), total);
                }

                result
            },
            Err(e) => {
                println!("{}", e);
                let mut content = String::new();
                response.read_to_string(&mut content).unwrap();
                println!("{}: {}", response.status(), content);
                for ref module in modules {
                    result.insert(module.to_string(), -1);
                }

                result
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
