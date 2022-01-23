

    use log::{LevelFilter, warn, debug, error, info};
    use select::document::Document;
    use simplelog::{TermLogger, TerminalMode, Config};
    use indicatif::ProgressBar;
    //use std::{fmt::Result, iter::repeat_with};
    use select::predicate::Name;

    

    //use anyhow::{Context};
    //use openssl::stack::Stack;
    use reqwest::{Response, StatusCode, Body};
    use tokio::select;
    //use std::default;


    #[derive(Default)]
    pub struct ReqClient {
        pub code: StatusCode,
        pub body: String,
        elements: Vec<String>
    }
    
    impl ReqClient {
        
        pub async fn send_req(&mut self, target : &str) {

            //TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    
            let response : Response;
            
            info!("Making request to: {}", target);
            match reqwest::get(target).await  {
                Ok(resp) => response = resp,
                Err(e) => panic!("{}", e.to_string()),
            };
            debug!("status code: {}", response.status());
            info!("Successful request to: {}", target);

            self.code = response.status();
            self.body = response.text().await.unwrap();
            debug!("response body character count: {}", self.body.len());
            
        }


        pub fn find_links(&mut self) {

            self.elements = Vec::new();

            Document::from(self.body.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| self.elements.push(x.to_string()));

            //println!("link: {:?}", self.elements);
        
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




