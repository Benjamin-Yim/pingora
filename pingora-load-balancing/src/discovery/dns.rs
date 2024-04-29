use crate::discovery::ServiceDiscovery;
use crate::Backend;
use async_trait::async_trait;
use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RData, Record, RecordType};
use hickory_client::udp::UdpClientConnection;
use pingora_error::Error;
use pingora_error::ErrorType::Custom;
use rand::Rng;
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

pub struct DNS {
    domain: Box<str>,
    resolver: Vec<SocketAddr>,
    port: u32,
}

static DNS_ADDR_ENV: &str = "DNS_ADDR";
static GOOGLE_DNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);

impl DNS {
    ///
    /// # Example
    ///     Dns::new("www.example.com:8088")
    /// if this domain port haven't set ,default 80
    pub fn new(service: &str) -> pingora_error::Result<Self, Error> {
        let mut resolver = vec![];
        if service.is_empty() {
            return Err(*Error::new(Custom("the service is empty is illegal")));
        }
        // from ENV get dns addr
        if let Ok(result) = env::var(DNS_ADDR_ENV) {
            result
                .split(",")
                .into_iter()
                .for_each(|item| resolver.push(item.parse().unwrap()));
        } else {
            // default
            resolver.push(GOOGLE_DNS);
        }
        let addr: Vec<&str> = service.split(":").map(|field| field.trim()).collect();
        if addr.len() == 1 {
            return Ok(DNS {
                domain: service.into(),
                resolver,
                port: 80,
            });
        }

        let port = addr.get(1).unwrap().parse().unwrap();
        return Ok(DNS {
            domain: service.into(),
            resolver,
            port,
        });
    }

    ///
    /// new DNS
    /// # Example
    /// let _ = DNS::new("www.example.com",vec!["8.8.8.8:53"])
    ///
    pub fn new_with_resolve(
        service: &str,
        resolver: Vec<String>,
    ) -> pingora_error::Result<Self, Error> {
        if service.is_empty() {
            return Err(*Error::new(Custom("the service is empty is illegal")));
        }
        if resolver.is_empty() {
            return Err(*Error::new(Custom("the resolver is empty is illegal")));
        }
        let resolver = resolver.iter().map(|addr| addr.parse().unwrap()).collect();
        let addr: Vec<&str> = service.split(":").map(|field| field.trim()).collect();
        if addr.len() == 1 {
            return Ok(DNS {
                domain: service.into(),
                resolver,
                port: 80,
            });
        }

        let port = addr.get(1).unwrap().parse().unwrap();
        return Ok(DNS {
            domain: service.into(),
            resolver,
            port,
        });
    }

    fn insert_item(
        &self,
        remote_addr: &str,
        health: &mut HashMap<u64, bool>,
        tree: &mut BTreeSet<Backend>,
    ) {
        let addr = format!("{}:{}", remote_addr, self.port);
        let value = Backend::new(addr.as_str()).unwrap();
        health.insert(value.hash_key(), true);
        tree.insert(value);
    }
}

///
/// 实现 DNS 服务发现
#[async_trait]
impl ServiceDiscovery for DNS {
    async fn discover(&self) -> pingora_error::Result<(BTreeSet<Backend>, HashMap<u64, bool>)> {
        // no readiness
        let mut health: HashMap<u64, bool> = HashMap::new();
        let mut tree: BTreeSet<Backend> = BTreeSet::new();
        // random dns address
        let mut rng = rand::thread_rng();
        let index;
        if self.resolver.len() > 1 {
            index = rng.gen_range(0..self.resolver.len());
        } else {
            index = 0
        }
        // random request dns addr
        let address = self.resolver[index];
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let name = Name::from_str(self.domain.as_ref()).unwrap();
        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
        let answers: &[Record] = response.answers();
        for x in answers {
            match x.data() {
                Some(RData::A(ref remote_addr)) => {
                    self.insert_item(remote_addr.to_string().as_str(), &mut health, &mut tree)
                }
                Some(RData::AAAA(ref remote_addr)) => {
                    self.insert_item(remote_addr.to_string().as_str(), &mut health, &mut tree)
                }
                _ => {}
            }
        }
        Ok((tree, health))
    }
}
