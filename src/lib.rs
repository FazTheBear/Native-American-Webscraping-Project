use std::{collections::HashMap, fs::{File, self}, io::{self, BufRead, BufReader, Error}, path::Path, ops::Deref};
use serde_json::{Value, json};
use serpapi_search_rust::serp_api_search::SerpApiSearch;

#[derive(Debug)]
pub struct Dispenser {
    key: String,
    queries: Vec<String>
}


impl Dispenser {
    ///generates a new Dispenser object,
    pub fn new(key_file: &str, queries_file: &str) -> Dispenser {
        Dispenser {
            key: parse_text_file(key_file).unwrap().join(""),
            queries: parse_text_file(queries_file).unwrap()
        }
    }
    /// Returns the organic results from a SerpApiSearch
    async fn get_oresults(search: SerpApiSearch) -> Result<Value, Box<dyn std::error::Error>> {
        let results = search.json().await?;

        Ok(results["organic_results"].clone())
    }
    
    pub async fn tribe_search(&self, amt_queries_sent: Option<i32>) -> Vec<Result<Value, Box<dyn std::error::Error>>>{
        let mut tribe_info = Vec::new();
        for query_index in 0..amt_queries_sent.unwrap_or(self.queries.len() as i32) {
            let mut params = HashMap::<String, String>::new();
            params.insert("engine".to_string(), "google".to_string());
            params.insert("q".to_string(), self.queries.get(query_index as usize).unwrap().to_string());

            let search = SerpApiSearch::google(params, (*self.key).to_string());
            
            tribe_info.push(Self::get_oresults(search).await);
        }

        tribe_info
    }


}

fn parse_text_file(name: &str) -> Result<Vec<String>, Error> {
    let mut tribes_list = Vec::new();

    //Preformatting path so that it leads to config
    let mut path = "src/config/".to_owned();
    path.push_str(name.clone());
    println!("{}", path);

    let contents = File::open(path)?;
    let reader = BufReader::new(contents);

    for line in reader.lines().filter(|x| x.as_deref().unwrap().len() > 0) {
        tribes_list.push(line?);
    }
    Ok(tribes_list)
}

pub fn return_parameters(tribe_name: String) -> HashMap::<String, String> {
  let mut params = HashMap::<String, String>::new();
  params.insert("engine".to_string(), "google".to_string());
  params.insert("q".to_string(), tribe_name.to_string());
  params
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};
    use crate::{parse_text_file, Dispenser};

    #[test]
    fn return_key() {
        let key_list = parse_text_file("tests.txt");
        let key = &key_list.unwrap()[0];
        assert_eq!(key, "ThisIsATest")
    }

    ///Sends an actual api call to the API, uses up an actual request, use sparingly!
    /// Will fail Always!
    #[tokio::test]
    async fn test_api_call() {
        let dispenser = Dispenser::new("key.txt", "tribes.txt");
        let results = dispenser.tribe_search(Some(1)).await;

        for value in results {
            println!("{:?}", value.unwrap().to_string());
        }
        
    }
    #[tokio::test]
    async fn parse_api_call() {
        let example_response_string = "{\"about_page_link\":\"https://www.google.com/search?q=About+https://www.astribe.com/&tbm=ilp&ilps=ADNMCi3IbCd8HtgQm7HLpl1hJyWRoQyOWQ\",\"about_this_result\":{\"keywords\":[\"absentee\",\"shawnee\",\"tribe\",\"indians\",\"oklahoma\"],\"languages\":[\"English\"],\"regions\":[\"the United States\"],\"source\":{\"description\":\"astribe.com was first indexed by Google more than 10 years ago\",\"security\":\"secure\",\"source_info_link\":\"https://www.astribe.com/\"}},\"cached_page_link\":\"https://webcache.googleusercontent.com/search?q=cache:RURZLeUMJDMJ:https://www.astribe.com/&cd=1&hl=en&ct=clnk&gl=us\",\"displayed_link\":\"https://www.astribe.com\",\"link\":\"https://www.astribe.com/\"}";

        let ex_value: Value = serde_json::from_str(example_response_string).unwrap();
        
        println!("{}", ex_value["about_page_link"]);


    }
} 