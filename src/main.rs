use error_chain::error_chain;   
use clap::Parser;


#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    let res = reqwest::get(args.url)
    .await?
    .text()
    .await?;
    
    println!("{}", res.as_str());
    
    //   Document::from(res.as_str())
    //     .find(Name("a"))
    //     .filter_map(|n| n.attr("href"))
    //     .for_each(|x| println!("{}", x));
    
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
