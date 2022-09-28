use std::{collections::HashMap, fs::{File, self}, io::{self, BufRead, BufReader, Error}, path::Path, ops::Deref};
use serpapi_search_rust::serp_api_search::SerpApiSearch;

#[derive(Debug)]
struct Dispenser {
    key: String,
    queries: Vec<String>
}

impl Dispenser {
    fn new(key: String) -> Dispenser {

        Dispenser {
            key: key,
            queries: parse_text_file().unwrap()
        }
    }
    /// Return the organic results from the search
    fn get_oresults(search: SerpApiSearch) {
    
    }
    
    pub fn search(&self, amt_queries_sent: Option<i32>) -> Result<(), Error>{
        for query in 0..amt_queries_sent.unwrap_or(self.queries.len() as i32) {
            let mut params = HashMap::<String, String>::new();
            params.insert("engine".to_string(), "google".to_string());
            params.insert("q".to_string(), query.to_string());

            let search = SerpApiSearch::google(params, self.key);
            Self::get_oresults(search)
        }


        Ok(())
    }


}

fn parse_text_file() -> Result<Vec<String>, Error> {
    let mut tribes_list = Vec::new();
    let contents = File::open("tribes.txt")?;
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
    use crate::{parse_text_file, Dispenser};

    #[test]
    fn parse_text_file_test() {
        let tribes_list = parse_text_file();
        println!("{:?}", tribes_list.unwrap());
        assert_eq!("2", "1");
    }
    #[test]
    fn create_Dispenser () {
        let key = String::from("I'm a random key!");
        let dispenser = Dispenser::new(key);
        
        println!("{:?}", dispenser);
        assert_eq!("1", "2");
        
    }
} 