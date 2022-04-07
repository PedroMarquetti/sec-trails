use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuerySubDomain {
    pub subdomains: Vec<String>,
    pub subdomain_count: i32,
}

impl QuerySubDomain {
    /// This will get 2000 subdomains for specified URL, set to only print first 5
    ///
    /// TODO: add custom range as parameter?
    pub async fn get(url: &String, key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.securitytrails.com/v1/domain/{}/subdomains?children_only=false&include_inactive=false", url);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("APIKEY", key)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<QuerySubDomain>()
            .await?;
        Ok(res)
    }
}
