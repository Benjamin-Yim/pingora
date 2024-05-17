
use pingora_core::prelude::{background_service, Opt};
use pingora_core::server::Server;
use pingora_load_balancing::health_check::TcpHealthCheck;
use pingora_load_balancing::LoadBalancer;
use pingora_proxy::http_proxy_service;

mod gateway;
mod modules;

fn main() { 
    let mut server = Server::new(Some(Opt::default())).unwrap();

    server.bootstrap();

    let mut upstream = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();

    let hc = TcpHealthCheck::new();

    upstream.set_health_check(hc);
    upstream.health_check_frequency = Some(std::time::Duration::from_secs(1));

    let background = background_service("health check", upstream);
    let upstreams = background.task();
    let mut lb = http_proxy_service(&server.configuration, gateway::http_proxy::LB(upstreams));


    lb.add_tcp("0.0.0.0:6188");

    server.add_service(background);
    server.add_service(lb);

    server.run_forever();
}
