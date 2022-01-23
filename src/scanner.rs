use log::{LevelFilter, warn, debug, error, info};
use simplelog::{TermLogger, TerminalMode, Config};

use select::{document::Document, predicate::Name};


#[derive(Default)]
pub struct Scanner {
    pub web_page: String,
    elements: Vec<String>
}

// impl Default for Scanner {
//     fn default(&mut self) -> Self {
//         self.web_page = "";
//         self.elements = Vec::new();
//         self
//     }
// }

impl Scanner {
    
    pub fn find_elements(&mut self, node: &str, typeF: &str) -> &Self {
        self.elements = Vec::new();
        info!("Begining search for html elements");
        Document::from(self.web_page.as_str())
        .find(Name(node))
        .filter_map(|n| n.attr(typeF))
        .for_each(|x| self.elements.push(x.to_string()));
        info!("Search for html elements successfully");
        //println!("link: {:?}", self.elements);
        if self.elements.is_empty() {
            warn!("None of the selected elements were found in the html")
        }

        self
        
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

    pub fn filter_internal(&mut self) {

        let mut internal: Vec<String> = Vec::new();
        
        for i in self.elements.iter() {

            if !i.contains("http") {
                internal.push(i.to_string())
            }

        }

        self.elements = internal;
        println!("link: {:?}", self.elements);
        
    }
}