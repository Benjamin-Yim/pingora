use std::collections::{BTreeSet, HashMap};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use async_trait::async_trait;
use crate::Backend;
use crate::discovery::ServiceDiscovery;
use dns_lookup::{lookup_host};
use hickory_client::client::{Client, SyncClient};
use hickory_client::error::ClientResult;
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RData, Record, RecordType};
use hickory_client::udp::UdpClientConnection;
use rand::{Rng};
use crate::selection::SelectionAlgorithm;

struct DNS(String);

static DNS_ADDR_ENV: &str = "DNS_ADDR";
static GOOGLE_DNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);

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
        let mut dnsAddr: Vec<SocketAddr> = vec![];

        if let Ok(result) = env::var(DNS_ADDR_ENV) {
            let splits = result.split(",");
            for item in splits {
                match item.parse() {
                    Ok(addr) => {
                        dnsAddr.push(addr);
                    }
                    _ => {}
                }
            }
        }

        if dnsAddr.is_empty() {
            dnsAddr.push(GOOGLE_DNS)
        }

        let mut rng = rand::thread_rng();
        let index;
        if dnsAddr.len() > 1 {
            index = rng.gen_range(0..dnsAddr.len());
        } else {
            index = 0
        }
        // random request dns addr
        let address = dnsAddr[index];
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let name = Name::from_str(self.0.as_ref()).unwrap();
        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
        let answers: &[Record] = response.answers();
        for x in answers {
            if let Some(RData::A(ref remoteAddr)) = x.data() {
                if let Ok(value) = Backend::new(remoteAddr.to_string().as_str()) {
                    tree.insert(value);
                }
            }
        }
        Ok((tree, health))
    }
}