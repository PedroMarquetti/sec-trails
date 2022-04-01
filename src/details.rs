use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryInfo {
    pub apex_domain: String,
    pub current_dns: CurrentDns,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentDns {
    pub a: AValues,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AValues {
    pub values: Vec<Values>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Values {
    pub ip: String,
    pub ip_organization: String,
}

impl QueryInfo {
    pub async fn get(url: &String, key: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://api.securitytrails.com/v1/domain/{}", url);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("APIKEY", key)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<QueryInfo>()
            .await?;
        Ok(res)
    }
}
