

    use log::{LevelFilter, warn, debug, error, info};
    use select::document::Document;
    use simplelog::{TermLogger, TerminalMode, Config};
    use indicatif::ProgressBar;
    //use std::{fmt::Result, iter::repeat_with};
    use select::predicate::Name;
    use url::{Position, Url, form_urlencoded::parse};
    use error_chain::error_chain;


    

    //use anyhow::{Context};
    //use openssl::stack::Stack;
    use reqwest::{Response, StatusCode, Body};
    use tokio::select;
    //use std::default;


    #[derive(Default)]
    pub struct ReqClient {
        pub code: StatusCode,
        pub body: String,
        elements: Vec<String>,
        base_url: String
    }
    
    error_chain! {
        foreign_links {
            ReqError(reqwest::Error);
            IoError(std::io::Error);
            UrlParseError(url::ParseError);
            JoinError(tokio::task::JoinError);
        }
      }

    impl ReqClient {


        pub async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
            let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
            let base_url =
              base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
            Ok(base_url)
          }
        
        pub async fn send_req(&mut self, target : &str) {
    
            let response : Response;
            let url = Url::parse(target);
            
            info!("Making request to: {}", target);
            match reqwest::get(target).await  {
                Ok(resp) => response = resp,
                Err(e) => panic!("{}", e.to_string()),
            };
            debug!("status code: {}", response.status());
            info!("Successful request to: {}", target);

            let document = Document::from(response.text().await?.as_str());

            self.code = response.status();
            self.body = response.text().await.unwrap();
            let m = get_base_url(&url, &document);
            
            debug!("response body character count: {}", self.body.len());
            
        }


        pub fn find_links(&mut self) {

            self.elements = Vec::new();
            info!("Begining search for html elements");
            Document::from(self.body.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| self.elements.push(x.to_string()));
            info!("Search for html elements successfully");
            //println!("link: {:?}", self.elements);
            if self.elements.is_empty() {
                warn!("None of the selected elements were found in the html")
            }
        
        }

        pub fn filter_external(&mut self) {

            let mut external: Vec<String> = Vec::new();
            
            for i in self.elements.iter() {

                if i.contains("http") {
                    external.push(i.to_string())
                }

            }

            self.elements = external;
            println!("link: {:?}", self.elements);
            
        }
    
    }

    #[tokio::test]
    async fn send_req_200() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/200").await;

        assert_eq!(req_client.code,  StatusCode::OK);
        

    }

    #[tokio::test]
    async fn send_req_202() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/202").await;

        assert_eq!(req_client.code,  StatusCode::ACCEPTED);
        

    }

    #[tokio::test]
    async fn send_req_400() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/400").await;
        
        assert_eq!(req_client.code,  StatusCode::BAD_REQUEST);
        

    }

    #[tokio::test]
    async fn send_req_404() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/404").await;
        
        assert_eq!(req_client.code,  StatusCode::NOT_FOUND);
        

    }

    #[tokio::test]
    async fn send_req_500() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/500").await;
        
        assert_eq!(req_client.code,  StatusCode::INTERNAL_SERVER_ERROR);
        

    }

    #[tokio::test]
    async fn send_req_502() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/502").await;
        
        assert_eq!(req_client.code,  StatusCode::BAD_GATEWAY);
        

    }




