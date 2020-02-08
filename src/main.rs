mod settings;
use std::error::Error;
use hyper::body;
use hyper::{Client, Request, Method, Body};
use hyper_tls::HttpsConnector;
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
    let _pull_requests_uri = format!("{}/_apis/git/pullrequests", &url_with_org);
    let _commits_uri = format!("{}/_apis/git/repositories/repo-id/commits", &url_with_org);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let pass = azure_settings.pass.unwrap();
    let secret = format!(":{}", pass);
    let secret_encoded = base64::encode(&secret);
    let auth_header = format!("Basic {}", secret_encoded);

    let req = Request::builder()
        .method(Method::GET)
        .uri(connection_data_uri)
        .header("Authorization", auth_header)
        .body(Body::empty())?;

    let res = client.request(req).await?;

    let body_bytes = body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");
    println!("Body: {}", body);

    Ok(())
    //https://docs.google.com/presentation/d/1QmWRwnKzclTZFn2h6tlMyjPaQVUCR9haoJd7NiIeONA/edit#slide=id.p
}
