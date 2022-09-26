use std::{collections::HashMap, fs::File, io::{self, BufRead}, path::Path};

struct Dispenser;

fn parse_text_file() -> Vec<String> {
    let mut tribes_list = Vec::new();
    if let Ok(lines) = read_lines("config/tribes.txt") {
        for line in lines {
            if let Ok(tribe) = line {
                tribes_list.push(tribe);
            }
        }
    }
    tribes_list
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
        println!("{:?}", tribes_list);
        println!("test");
        assert_eq!("2", "1");
    }
} 