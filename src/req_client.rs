
mod req {
    
    //use std::{fmt::Result, iter::repeat_with};

    

    //use anyhow::{Context};
    //use openssl::stack::Stack;
    use reqwest::{Response, StatusCode};
    //use std::default;

    //use crate::Error;
    //use log::{debug, info, warn};

    #[derive(Default)]
    pub struct ReqClient {
        pub code: StatusCode,
        pub body: String,
    }
    
    impl ReqClient {
        
        pub async fn send_req(&mut self, target : &str) {
            //pretty_env_logger::init();
            info!("Request Target: {}", target);
    
            let response : Response;

            match reqwest::get(target).await  {
                Ok(resp) => response = resp,
                Err(e) => panic!("{}", e.to_string()),
            };

            //debug!("Status code: {}", response.status());

            self.code = response.status();
            self.body = response.text().await.unwrap();
            
        }
    
    }

    #[tokio::test]
    async fn send_req_successful() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/200").await;

        assert_eq!(req_client.code,  StatusCode::OK);
        

    }

    #[tokio::test]
    async fn send_req_unsuccessful() {
        
        let mut req_client: ReqClient = Default::default();
        req_client.send_req("http://127.0.0.1:8000/status/404").await;
        
        assert_eq!(req_client.code,  StatusCode::NOT_FOUND);
        

    }

}



