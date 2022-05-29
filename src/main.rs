use std::fmt;
use structopt::StructOpt;

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
    let mut content: Vec<Website> = Vec::new();
    read_from_file(&mut content, args);
    print_content(&content)
}

fn read_from_file(content: &mut Vec<Website>, cli: Cli) {
    let file = std::fs::read_to_string(&cli.path).expect("could not read file");
    println!("Loading contents of file");

    for line in file.lines() {
        // println!("{}", line);
        let mut words = line.split_whitespace();
        let website = words.next().unwrap_or("").to_string();
        let username = words.next().unwrap_or("").to_string();
        let password = words.next().unwrap_or("").to_string();

        content.push(Website {
            website: website,
            username: username,
            password: password,
        });
    }
}

fn print_content(content: &Vec<Website>) {
    for website in content {
        println!("{}", website);
    }
}

#[cfg(test)]
mod tests {
    use crate::read_from_file;
    use std::path::PathBuf;
    use crate::Website;
    use crate::Cli;


    #[test]
    fn test_reading_file1() {
        let args = Cli { path: PathBuf::from("./tests/input1.txt") };
        let mut content: Vec<Website> = Vec::new();
        read_from_file(&mut content, args);
        assert!(content[0].username == "user".to_string() && content[0].password == "pass".to_string() 
            && content[0].website == "web".to_string())
    }

    #[test]
    fn test_reading_file2() {
        let mut count = 0;
        let args = Cli { path: PathBuf::from("./tests/input2.txt") };
        let mut content: Vec<Website> = Vec::new();
        read_from_file(&mut content, args);
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
}


