use std::collections::{BTreeSet, HashMap};
use async_trait::async_trait;
use crate::Backend;
use crate::discovery::ServiceDiscovery;
use dns_lookup::{lookup_host, lookup_addr};

struct DNS(Box<str>);

///
/// 实现 DNS 服务发现
#[async_trait]
impl ServiceDiscovery for DNS {
    async fn discover(&self) -> pingora_error::Result<(BTreeSet<Backend>, HashMap<u64, bool>)> {
        // no readiness
        let health = HashMap::new();
        let mut tree: BTreeSet<Backend> = BTreeSet::new();
        let ips: Vec<std::net::IpAddr> = lookup_host(self.0.as_ref()).unwrap();
        for item in ips.to_vec() {
            let _b = tree.insert(Backend::new(item.to_string().as_str())?);
        }
        Ok((tree, health))
    }
}