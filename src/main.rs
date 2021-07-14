mod req;
mod parse;
mod file;
use req::fetch;
use parse::Hosts;

fn main() {
    let hosts = fetch("https://raw.githubusercontent.com/521xueweihan/GitHub520/main/hosts").unwrap();
    let hosts = Hosts::from(hosts.as_str());
    println!("{:#?}", hosts);
    println!("{}", hosts);
    println!("len: {}", hosts.data.len());
}
