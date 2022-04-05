mod modes;
use clap::Parser;
use colored::*;
use exitfailure::ExitFailure;
use modes::details;

#[derive(Parser)]
#[clap(author = "Made by: Phlm")]
#[clap(about = "Get info on specified URL\n")]
pub struct Cli {
    // argParser (Clap)
    /// URL you'd like to scan... Example: example.com
    #[clap(short)]
    url: String,

    /// API Key from https://securitytrails.com/
    #[clap(short)]
    key: String,
    /// Query types to be used, separated by commas.
    ///
    /// Available modes:
    ///
    /// >details (default if -t is not specified)
    ///
    /// Example: -t dns,history
    #[clap(short)]
    types: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::parse();
    let url = args.url;
    let key = args.key;
    // TODO: add different types here...
    let types: std::string::String = args.types.unwrap_or("_".to_string()); // gets values from -t flag... if not specified, defaults to '_' (returns String)
    let types_vec: &Vec<&str> = &types.split(",").collect(); // turning "types" var into a Vec<&str> so it's easier to par
    let res = details::QueryInfo::get(&url, &key).await?;
    println!("{}{}", "Info about: ".cyan(), url.white());
    println!("\n{}", "'A Record values':\n".cyan());
    for items in res.current_dns.a.values {
        // TODO: make this async?
        println!(
            "{} {}\n{}{}\n",
            "IP address:".cyan(),
            items.ip.white(),
            "IP Org.: ".cyan(),
            items.ip_organization.white()
        );
    }
    Ok(())
}
