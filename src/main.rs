//#[macro_use] extern crate log;

//use req_client::req::ReqClient;

use error_chain::error_chain;   
use log::{LevelFilter, warn, debug, error, info};
use simplelog::{TermLogger, TerminalMode, Config};
use clap::Parser;
use req_client::ReqClient;

mod req_client;


#[tokio::main]
async fn main() -> Result<()> {

    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Stdout).unwrap();
    info!("Web Hunter Starting...");

    let args = Args::parse();

    info!("arguments url: {}", args.url);

   let mut req_client: ReqClient = Default::default();
   req_client.send_req(&args.url).await;
   req_client.find_links();
        
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short = 'u', long = "url")]
    url: String

}

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}
