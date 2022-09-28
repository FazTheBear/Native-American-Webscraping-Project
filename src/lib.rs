use std::{collections::HashMap, fs::{File, self}, io::{self, BufRead, BufReader, Error}, path::Path};

#[derive(Debug)]
struct Dispenser {
    package: HashMap<String, String>,
    queries: Vec<String>
}

impl Dispenser {
    fn new(key: String) -> Dispenser {

        Dispenser {
            package: HashMap::new(),
            queries: parse_text_file().unwrap()
        }
    }

    pub fn search(&self,) {
        for query in 0..self.queries.len() {

        }
        let mut params = HashMap::<String, String>::new();
        params.insert("engine".to_string(), "google".to_string());
        params.insert("q".to_string(), key.to_string());

        sel

        let search = SerpApiSearch::google(self.p, "secret_api_key".to_string());
        
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