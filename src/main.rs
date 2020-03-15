mod azure_api_config;
mod pull_requests;
mod settings;

use chrono::prelude::*;
use std::error::Error;
use tokio;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let set = settings::read_from_file();
    println!("{:#?}", set);

    let azure_settings = set.azure.unwrap();
    let org = azure_settings.organization.unwrap();

    let azure_connection_data = azure_api_config::AzureApiConfig::new(&org);

    let (_date_from, _date_to) = calculate_dates();

    let pass: String = azure_settings.pass.unwrap();
    let auth_header: String = create_auth_header(&pass);

    pull_requests::load_pull_requests(&azure_connection_data.uris.connection_data, &auth_header).await;

    Ok(())
    //https://docs.google.com/presentation/d/1QmWRwnKzclTZFn2h6tlMyjPaQVUCR9haoJd7NiIeONA/edit#slide=id.p
}

fn create_auth_header(pass: &str) -> String {
    let secret = format!(":{}", pass);
    let secret_encoded = base64::encode(&secret);
    format!("Basic {}", secret_encoded)
}

fn calculate_dates() -> (String, String) {
    let date_to = Local::now();
    let date_from = date_to - chrono::Duration::days(30);
    (
        date_to.format("%Y-%m-%d").to_string(),
        date_from.format("%Y-%m-%d").to_string(),
    )
}
