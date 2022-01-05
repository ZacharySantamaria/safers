use structopt::StructOpt;
use std::fmt;

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
    let content: Vec<Website> = Vec::new();
    read_from_file(content, args);
}

fn read_from_file(mut content: Vec<Website>, cli: Cli) {
    let file = std::fs::read_to_string(&cli.path).expect("could not read file");
    println!("Loading contents of file");

    for line in file.lines() {
        let website = line.split_whitespace().next().unwrap_or("").to_string();
        let username = line.split_whitespace().next().unwrap_or("").to_string();
        let password = line.split_whitespace().next().unwrap_or("").to_string();

        content.push(Website {
            website: website,
            username: username,
            password: password,
        });
    }
}

fn print_content(content: Vec<Website>) {
    for website in content {
        println!("{}", website);
    }
}
