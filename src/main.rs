/*
This is intended to be a rust only CLI password manager. The idea is to have it all within this main 
file since much of the code base will be fairly small. 

Implementation: The idea is to have a very basic version of a password manager. This means
storing much of the contents in plain text. I will eventually change this but for now, 
this is just a small project. It will store the contents in this order.

EXAMPLE
(WEBSITE) (USER) (PASS)

In a file not yet decided upon. To load this file it will retreive the contents and use a 
struct called Website then have a vector of Websites that stores all the data. The idea is
once the user is ready to quit it will save the data to the file.

Considerations: Possibly use a database to store the content. The idea was to have it self-hosting
that way you only need to carry around the Database file then just load it when you download this
from Github. 

WORKING ON: UPDATE and DELETE
*/

use std::fmt;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

// Using structOpt to parse args into struct
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Website {
    website: String,
    username: String,
    password: String,
}

impl fmt::Display for Website {
    // Added this for testing purposes it will display as `WEBSITE USERNAME PASSWORD`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.website, self.username, self.password)
    }
}

fn main() {
    let args = Cli::from_args();
    let directory_path = args.path;
    let mut content: Vec<Website> = Vec::new();
    load_data_from_file(&mut content, &directory_path);

    
}

fn add_data_to_file(content: &mut Vec<Website>, path: &PathBuf) {
    let mut fileRef = OpenOptions::new().append(true).open(path).expect("Unable to open file");
    fileRef.write_all("\nwebsite username password".as_bytes()).expect("write failed");

}

fn load_data_from_file(content: &mut Vec<Website>, path: &PathBuf) -> i64 {
    let file = std::fs::read_to_string(&path).expect("could not read file");
    let mut amount_of_entries = 0;

    for line in file.lines() {
        let mut words = line.split_whitespace();
        let website = words.next().unwrap_or("").to_string();
        let username = words.next().unwrap_or("").to_string();
        let password = words.next().unwrap_or("").to_string();

        content.push(Website {
            website: website,
            username: username,
            password: password,
        });

        amount_of_entries += 1
    }
    
    return amount_of_entries;
}

fn print_content(content: &Vec<Website>) {
    for website in content {
        println!("{}", website);
    }
}

#[cfg(test)]
mod tests {
    use crate::load_data_from_file;
    use std::path::PathBuf;
    use crate::Website;
    use crate::Cli;
    use crate::add_data_to_file;
    use crate::print_content;
    use crate::delete_websites;


    #[test]
    fn test_reading_file1() {
        let args = Cli { path: PathBuf::from("./tests/input1.txt") };
        let mut content: Vec<Website> = Vec::new();
        let entry_count = load_data_from_file(&mut content, &args.path);
        assert!(content[0].username == "user".to_string() && content[0].password == "pass".to_string() 
            && content[0].website == "web".to_string() && entry_count == 1)
    }

    #[test]
    fn test_reading_file2() {
        let mut count = 0;
        let args = Cli { path: PathBuf::from("./tests/input2.txt") };
        let mut content: Vec<Website> = Vec::new();
        load_data_from_file(&mut content, &args.path);
        let file = std::fs::read_to_string(PathBuf::from("./tests/input2.txt")).expect("could not read file");

        for line in file.lines() {
            let mut words = line.split_whitespace();
            let website = words.next().unwrap_or("").to_string();
            let username = words.next().unwrap_or("").to_string();
            let password = words.next().unwrap_or("").to_string();

            if website != content[count].website || username != content[count].username || password != content[count].password {
                assert!(false)
            }
            count += 1;
        }

        assert!(true)
    }

    #[test]
    fn test_adding_entry(){
        let args = Cli { path: PathBuf::from("./tests/input2.txt") };
        let mut content: Vec<Website> = Vec::new();
        
        load_data_from_file(&mut content, &args.path);
        content.push(Website{website: "web".to_string(), username: "user1".to_string(), password: "pass1".to_string()});
        add_data_to_file(&mut content, &args.path);
        let entry_count = load_data_from_file(&mut content, &args.path);
        // print_content(&content);
        assert!(content[0].username == "user".to_string() && content[0].password == "pass".to_string() 
            && content[0].website == "web".to_string() && content[1].username == "user1".to_string() && content[1].password == "pass1".to_string() 
            && content[1].website == "web1".to_string() && entry_count == 5)
        
    }
}


