use std::cell::RefCell;
use std::net;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

async fn resolve_single(resolver: &TokioAsyncResolver, addr: &String) -> Option<net::IpAddr> {
    if let Ok(ip) = addr.parse::<net::IpAddr>() {
        return Some(ip);
    }

    let remote_addr = format!("{}.", addr);
    let res = resolver.lookup_ip(remote_addr).await.unwrap();

    match res.iter().find(|ip| ip.is_ipv4()) {
        Some(ip_v4) => Some(ip_v4),
        None => {
            if let Some(ip_v6) = res.iter().find(|ip| ip.is_ipv6()) {
                Some(ip_v6)
            } else {
                return None;
            }
        }
    }
}

pub async fn resolve(addr_list: Vec<String>, ip_list: Vec<Rc<RefCell<net::IpAddr>>>) {
    let resolver =
        async { TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()) }
            .await
            .unwrap();
    loop {
        for (i, addr) in addr_list.iter().enumerate() {
            if let Some(new_ip) = resolve_single(&resolver, addr).await {
                if new_ip != *ip_list[i].borrow() {
                    *ip_list[i].borrow_mut() = new_ip;
                }
            }
        }
        sleep(Duration::from_secs(60)).await;
    }
}
