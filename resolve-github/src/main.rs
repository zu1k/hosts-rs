mod dns;
use dns::*;

fn main() {
    let hosts = resolve_github_assets();
    println!("{:#?}", hosts);
}
