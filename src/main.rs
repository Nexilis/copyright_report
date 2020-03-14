mod settings;

use hyper::body;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::Value;
use std::error::Error;
use tokio;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let set = settings::read_from_file();
    println!("{:#?}", set);

    let azure_settings = set.azure.unwrap();

    let org = azure_settings.organization.unwrap();
    let base_url = "https://dev.azure.com/";
    let url_with_org = format!("{}/{}", base_url, org);

    let connection_data_uri = format!("{}/_apis/connectionData", &url_with_org);
    let _repositories_uri = format!("{}/_apis/git/repositories", &url_with_org);
    let _pull_requests_uri = format!("{}/_apis/git/pullRequests", &url_with_org);
    let _commits_uri = format!("{}/_apis/git/repositories/repo-id/commits", &url_with_org);

    let pass: String = azure_settings.pass.unwrap();
    let auth_header: String = create_auth_header(&pass);

    let authenticated_user_id = get_authenticated_user_id(&connection_data_uri, &auth_header).await?;

    println!("{:#?}", authenticated_user_id);

    Ok(())
    //https://docs.google.com/presentation/d/1QmWRwnKzclTZFn2h6tlMyjPaQVUCR9haoJd7NiIeONA/edit#slide=id.p
}

fn create_auth_header(pass: &str) -> String {
    let secret = format!(":{}", pass);
    let secret_encoded = base64::encode(&secret);
    format!("Basic {}", secret_encoded)
}

async fn get_authenticated_user_id(connection_data_uri: &str, auth_header: &str) -> Result<String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::GET)
        .uri(connection_data_uri)
        .header("Authorization", auth_header)
        .body(Body::empty())?;

    let res = client.request(req).await?;

    let body_bytes = body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");
    let body_deserialized: Value = serde_json::from_str(&body)?;
    let authenticated_user_id = body_deserialized
        .as_object()
        .unwrap()
        .get("authenticatedUser")
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();

    Ok(authenticated_user_id.to_string())
}
