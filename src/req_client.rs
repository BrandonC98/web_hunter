

    use log::{LevelFilter, warn, debug, error};
    use simplelog::{TermLogger, TerminalMode, Config};
    use indicatif::ProgressBar;
    //use std::{fmt::Result, iter::repeat_with};

    

    //use anyhow::{Context};
    //use openssl::stack::Stack;
    use reqwest::{Response, StatusCode, Body};
    use tokio::select;
    //use std::default;


    #[derive(Default)]
    pub struct ReqClient {
        pub code: StatusCode,
        pub body: String,
    }
    
    impl ReqClient {
        
        pub async fn send_req(&mut self, target : &str) {

            TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
            debug!("Web Hunter Starting...");
    
            let response : Response;
            warn!("This a warning");
            match reqwest::get(target).await  {
                Ok(resp) => response = resp,
                Err(e) => panic!("{}", e.to_string()),
            };
            //debug!("Status code: {}", response.status());

            self.code = response.status();
            self.body = response.text().await.unwrap();
            
        }

        pub fn find_links(&mut self) {
            let mut webpage = Vec::new();
            webpage = self.body.split_whitespace().map(str::to_string).collect();
            let loading = ProgressBar::new(webpage.len() as u64);
            
            for i in webpage.iter() {
                
                if !i.contains("href=") {
                    loading.inc(1);
                    continue;
                }

                
                loading.inc(1);
            }
            
            //println!("Link: {}", i);
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




