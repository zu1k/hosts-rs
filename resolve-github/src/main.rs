mod dns;
use dns::*;
use std::{fs, io::Write};

fn main() {
    let hosts = resolve_github_assets();
    if let Ok(mut file) = fs::File::create("hosts") {
        match file.write_all(hosts.to_string().as_bytes()) {
            Ok(_) => println!("Success"),
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        println!("{:#?}", hosts);
    }
}
