mod details;
use clap::Parser;
// use details::QueryInfo;
use exitfailure::ExitFailure;
// use reqwest::Url;
// use serde_derive::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author = "Made by: Phlm")]
#[clap(about = "Get info on specified URL\n")]
pub struct Cli {
    /// URL you'd like to scan... Example: example.com
    #[clap(short)]
    url: String,

    /// API Key from https://securitytrails.com/
    #[clap(short)]
    key: String,
    // /// Query types to be used, separated by commas.  **not available yet**
    // ///
    // /// Example: -t dns,history
    // #[clap(short)]
    // types: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::parse();
    let url = args.url;
    let key = args.key;
    // TODO: add different types here...
    // let types = args.types; // [ ] containing request type -
    let res = details::QueryInfo::get(&url, &key).await?;
    println!("Info about: {}", url);
    println!("'A Record values':\n");
    for items in res.current_dns.a.values {
        // TODO: make this async?
        println!(
            "\tIP address: {}\n\tIP Org.: {}\n",
            items.ip, items.ip_organization
        );
    }
    Ok(())
}
