pub struct AzureApiConfig {
    pub uris: Uris,
}

pub struct Uris {
    pub connection_data: String,
    pub repositories: String,
    pub pull_requests: String,
    pub commits: String,
}

impl AzureApiConfig {
    #[inline(always)]
    pub fn new(organization: &str) -> AzureApiConfig {
        let base_url = "https://dev.azure.com/";
        let url_with_org = format!("{}/{}", base_url, organization);

        AzureApiConfig {
            uris: Uris {
                connection_data: format!("{}/_apis/connectionData", &url_with_org),
                repositories: format!("{}/_apis/git/repositories", &url_with_org),
                pull_requests: format!("{}/_apis/git/pullRequests", &url_with_org),
                commits: format!("{}/_apis/git/repositories/repo-id/commits", &url_with_org),
            },
        }
    }
}
