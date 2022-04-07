mod modes;
use clap::Parser;
use colored::*;
use exitfailure::ExitFailure;
use modes::details;
use modes::subdomains;

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
    // let types: std::string::String = args.types.unwrap_or("_".to_string()); // gets values from -t flag... if not specified, defaults to '_' (returns String)
    // let types_vec: &Vec<&str> = &types.split(",").collect(); // turning "types" var into a Vec<&str> so it's easier to parse values
    let info = details::QueryInfo::get(&url, &key).await?;
    println!("{}{}", "Info about: ".cyan(), url.white());
    println!("\n{}", "'A Record values':\n".cyan());
    for items in info.current_dns.a.values {
        // TODO: make this async?
        println!(
            "{} {}\n{}{}\n",
            "IP address:".cyan(),
            items.ip.white(),
            "IP Org.: ".cyan(),
            items.ip_organization.white()
        );
    }
    let sub_domains = subdomains::QuerySubDomain::get(&url, &key).await?;
    println!("{}", "Subdomains of URL".cyan());
    println!(
        "{}{}",
        "Subdomain count is: ".cyan(),
        sub_domains.subdomain_count.to_string().red()
    );
    println!(
        "{}{}:\n",
        "Getting 5 items of ".yellow(),
        sub_domains.subdomain_count.to_string().red()
    );
    let mut i: i16 = 0;
    for items in sub_domains.subdomains {
        i += 1;
        println!("{}.{}", items.red(), url.yellow());
        if i == 5 {
            // is there another way to limit this?
            break;
        }
    }
    Ok(())
}
