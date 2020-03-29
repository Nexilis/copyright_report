mod azure_api_config;
mod pull_requests;
mod settings;

use std::error::Error;
use tokio;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let set = settings::read_from_file();
    println!("{:#?}", set);

    let azure_settings = set.azure.unwrap();
    let org = azure_settings.organization.unwrap();
    let date_from = set.report.unwrap().date_from.unwrap();

    let azure_api_config = azure_api_config::AzureApiConfig::new(&org, &date_from);

    let pass: String = azure_settings.pass.unwrap();
    let auth_header: String = create_auth_header(&pass);

    let _ = pull_requests::load_pull_requests(&azure_api_config, &auth_header).await?;

    Ok(())
    //https://docs.google.com/presentation/d/1QmWRwnKzclTZFn2h6tlMyjPaQVUCR9haoJd7NiIeONA/edit#slide=id.p
}

fn create_auth_header(pass: &str) -> String {
    let secret = format!(":{}", pass);
    let secret_encoded = base64::encode(&secret);
    format!("Basic {}", secret_encoded)
}
