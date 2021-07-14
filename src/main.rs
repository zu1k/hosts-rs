mod req;
mod parse;
mod file;
use req::*;
use parse::Hosts;

fn main() {
    let hosts = fetch(GITHUB_520).unwrap();
    let hosts = Hosts::from(hosts.as_str());
    println!("{}", hosts);
}
