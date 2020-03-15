use chrono::prelude::*;

pub struct AzureApiConfig {
    pub uris: Uris,
    /// e.g. 2020-12-31
    pub date_from: String,
    /// e.g. 2020-12-31
    pub date_to: String,
}

pub struct Uris {
    pub connection_data: String,
    pub repositories: String,
    pub pull_requests: String,
    pub commits: String,
}

impl AzureApiConfig {
    #[inline(always)]
    pub fn new(organization: &str, date_from_str: &str) -> AzureApiConfig {
        let base_url = "https://dev.azure.com/";
        let url_with_org = format!("{}/{}", base_url, organization);
        let (date_from, date_to) = calculate_dates(&date_from_str);
        println!("Dates: {} - {}", date_from, date_to);

        AzureApiConfig {
            uris: Uris {
                connection_data: format!("{}/_apis/connectionData", &url_with_org),
                repositories: format!("{}/_apis/git/repositories", &url_with_org),
                pull_requests: format!("{}/_apis/git/pullRequests", &url_with_org),
                commits: format!("{}/_apis/git/repositories/repo-id/commits", &url_with_org),
            },
            date_from: date_from.to_string(),
            date_to: date_to.to_string(),
        }
    }
}

fn calculate_dates(date_from_str: &str) -> (String, String) {
    let date_from = DateTime::parse_from_rfc3339(date_from_str)
        .expect("date_from needs to be formatted according to RFC3339");
    let date_to = Local::now();
    (
        date_from.format("%Y-%m-%d").to_string(),
        date_to.format("%Y-%m-%d").to_string(),
    )
}
