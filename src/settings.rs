use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Settings {
    azure: Option<AzureSettings>,
    report: Option<ReportSettings>,
}

#[derive(Deserialize, Debug)]
struct AzureSettings {
    user: Option<String>,
    pass: Option<String>,
    organization: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ReportSettings {
    full_name: Option<String>,
    date_from: Option<String>,
}

pub fn read_from_file() -> Settings {
    let file = fs::read_to_string("Settings.toml").unwrap();
    toml::from_str(&file).unwrap()
}
