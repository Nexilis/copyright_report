use crate::azure_api_config::*;
use hyper::body;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug)]
struct PullRequest {
    // TODO: create remaining fields (use code in line 57-61 to see the response)
    closedDate: String,
    codeReviewId: u32,
    creationDate: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PullRequestsResponse {
    count: u32,
    value: Vec<PullRequest>
}

pub async fn load_pull_requests(cfg: &AzureApiConfig, auth_header: &str) -> Result<String> {
    let user_id = get_authenticated_user_id(&cfg.uris.connection_data, &auth_header)
        .await
        .unwrap();
    println!("Authenticated User Id: {:#?}", user_id);

    // TODO: one https is probably enough
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let creator_uri = format!(
        "{}?status=All&creatorId={}&$top=1",
        &cfg.uris.pull_requests, &user_id
    );
    // TODO: GET with reviewerId uri "{}?status=All&reviewerId={}&$top=1",

    println!("{}", creator_uri);

    let req = Request::builder()
        .method(Method::GET)
        .uri(creator_uri)
        .header("Authorization", auth_header)
        .body(Body::empty())?;

    let res = client.request(req).await?;
    // TODO: extract body deserialization
    let body_bytes = body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");

    let pull_requests: PullRequestsResponse = serde_json::from_str(&body)?;
    println!("{:#?}", pull_requests);
    // TODO: filter PRs following: if status == "completed"  closedDate in range date_from date_to else creationDate in rage date_from date_to

    // let body_deserialized: Value = serde_json::from_str(&body)?;
    // let res_body = body_deserialized.as_object().unwrap();
    // let response_arr = res_body.get("value").unwrap();
    // println!("{:#?}", response_arr);

    Ok(user_id.to_string())
}

async fn get_authenticated_user_id(connection_data_uri: &str, auth_header: &str) -> Result<String> {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

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
