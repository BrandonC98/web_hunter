//#[macro_use] extern crate log;

//use req_client::req::ReqClient;

use error_chain::error_chain;   
use log::{LevelFilter, warn, debug, error, info};
use simplelog::{TermLogger, TerminalMode, Config};
use clap::Parser;
use req_client::ReqClient;

use crate::scanner::Scanner;

mod req_client;
mod scanner;


#[tokio::main]
async fn main() -> Result<()> {

    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Stdout).unwrap();
    info!("Web Hunter Starting...");

    let args = Args::parse();

    info!("arguments url: {}", args.url);

   let mut req_client: ReqClient = Default::default();
   req_client.send_req(&args.url).await;
//    req_client.find_links();
//    req_client.filter_external();

    let mut scanner: Scanner = Default::default();
    scanner.web_page = req_client.body;

    scanner.find_elements("a", "href");
    scanner.filter_internal();

        
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short = 't', long = "target")]
    url: String

}

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}
