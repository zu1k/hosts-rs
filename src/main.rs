#![windows_subsystem = "windows"]

mod req;
mod parse;
mod file;
use req::*;
use parse::Hosts;
use file::*;

fn main() {
    let hosts = fetch(GITHUB_520).unwrap();
    let hosts = Hosts::from(hosts);

    match mod_hosts_file(hosts.to_string()) {
        Ok(_) => println!("Success!"),
        Err(err) => eprintln!("Error: {}", err)
    }
}
