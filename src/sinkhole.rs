use std::boxed::Box;
use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use trust_dns_proto::rr::{Name, RData, Record, RecordSet, RecordType};
use trust_dns_server::authority::{Authority, Catalog, ZoneType};
use trust_dns_server::client::rr::RrKey;
use trust_dns_server::server::ServerFuture;
use trust_dns_server::store::in_memory::InMemoryAuthority;

#[tokio::main]
async fn main() {
    // Create a Catalog (a collection of DNS zones)
    let mut catalog: Catalog = Catalog::new();

    // Create a Zone (a part of the DNS namespace)
    let zone = ZoneType::Primary;
    let records: BTreeMap<RrKey, RecordSet> = BTreeMap::new();
    let origin = Name::root(); // root level

    // Create an In-Memory Authority (the DNS database)
    let mut authority = InMemoryAuthority::new(origin, records, zone, false).unwrap();

    // Add an A record to the authority for the domain we want to block
    let domain_to_block = Name::from_utf8("example.com").unwrap();
    let record = Record::from_rdata(domain_to_block, 0, RData::A(Ipv4Addr::new(0, 0, 0, 0)));

    authority.upsert(record.clone(), 0);
    catalog.upsert(authority.origin().clone(), authority);

    // Create a Server
    let mut server = ServerFuture::new(catalog);

    // Listen on a socket address
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);

    tokio::spawn(async move {
        server.listen(&[socket_addr]).await;
    });

    println!("DNS server is running!");
}
