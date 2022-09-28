use std::{collections::HashMap, fs::{File, self}, io::{self, BufRead, BufReader, Error}, path::Path};

struct Dispenser {
    key: String,
    tribes_list: Vec<String>
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
    use crate::{parse_text_file};

    #[test]
    fn parse_text_file_test() {
        let tribes_list = parse_text_file();
        println!("{:?}", tribes_list.unwrap());
        assert_eq!("2", "1");
    }
} 