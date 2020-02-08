use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub azure: Option<AzureSettings>,
    pub report: Option<ReportSettings>,
}

#[derive(Deserialize, Debug)]
pub struct AzureSettings {
    pub user: Option<String>,
    pub pass: Option<String>,
    pub organization: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ReportSettings {
    pub full_name: Option<String>,
    pub date_from: Option<String>,
}

pub fn read_from_file() -> Settings {
    let file = fs::read_to_string("Settings.toml").unwrap();
    toml::from_str(&file).unwrap()
}
