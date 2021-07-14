use std::collections::HashMap;
use std::vec::Vec;
use std::net::IpAddr;
use std::str::FromStr;

type HostItem = Vec<String>;
type HostMap = HashMap<String, HostItem>;

#[derive(Debug)]
pub struct Hosts {
    pub data: HostMap
}

impl From<&str> for Hosts {
    fn from(text: &str) -> Self {
        let mut result = HostMap::new();

        for line in text.lines() {
            if line.len() < 10 || line.starts_with("#") {
                continue;
            }

            let mut parts = line.split_whitespace();
            match parts.next() {
                Some(ip) => {
                    match IpAddr::from_str(ip) {
                        Ok(ip) => {
                            let ip = ip.to_string();
                            let mut domains = HostItem::new();
                            while let Some(domain) = parts.next() {
                                domains.push(domain.into())
                            }
                            if domains.len() > 0 {
                                let origin_domains = result.entry(ip).or_default();
                                origin_domains.append(&mut domains);
                            }
                        },
                        Err(_) => continue
                    }
                },
                None => continue
            }
        };

        Hosts {
            data: result
        }
    }
}

impl std::fmt::Display for Hosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ip, domains) in self.data.iter() {
            f.write_str(format!("{}\t\t{}\n", ip, domains.join(" ")).as_str())?;
        }
        Ok(())
    }
}
