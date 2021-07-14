use hosts::*;
use super::res::GITHUB_ASSETS;
use std::net::IpAddr;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::{
    config::*,
    error::ResolveError
};

pub struct Client {
    resolver: Resolver
}

impl std::default::Default for Client {
    fn default() -> Self {
        let resolver = Resolver::new(ResolverConfig::cloudflare_https(), ResolverOpts::default()).unwrap();
        Client {
            resolver
        }
    }
}

impl Client {
    pub fn resolve(&self, domain: &str) -> Result<Option<IpAddr>, ResolveError> {
        let response = self.resolver.lookup_ip(domain);
        match response {
            Ok(response) => {
                let mut fb_ipv6 = None;
                for item in response {
                    if item.is_ipv4() {
                        return Ok(Some(item))
                    } else if fb_ipv6.is_none() {
                        fb_ipv6 = Some(item);
                    }
                }
                Ok(fb_ipv6)
            },
            Err(e) => Err(e)
        }
    }
}

pub fn resolve_github_assets() -> Hosts{
    let client = Client::default();
    let mut hosts = Hosts::default();

    for domain in GITHUB_ASSETS {
        if let Ok(Some(ip)) = client.resolve(&domain) {
            hosts.insert(domain.to_string(), ip);
        }
    }

    hosts
}
