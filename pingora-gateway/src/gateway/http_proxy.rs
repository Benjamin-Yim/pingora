use std::sync::Arc;
use async_trait::async_trait;
use pingora_core::prelude::HttpPeer;
use pingora_http::RequestHeader;
use pingora_load_balancing::LoadBalancer;
use pingora_load_balancing::selection::RoundRobin;
use pingora_proxy::{ProxyHttp, Session};

pub struct LB(pub Arc<LoadBalancer<RoundRobin>>);


#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> pingora_core::Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"",256).unwrap();
        println!("upstream endpoint {upstream:?}");

        let endpoint = Box::new(HttpPeer::new(upstream,true , "1.1.1.1".to_string()));
        Ok(endpoint)
    }

    async fn upstream_request_filter(&self, _session: &mut Session, upstream_request: &mut RequestHeader, _ctx: &mut Self::CTX) -> pingora_core::Result<()> where Self::CTX: Send + Sync {
        upstream_request.insert_header("Host","1.1.1.1").unwrap();
        Ok(())
    }
}