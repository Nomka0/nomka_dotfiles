enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn main() {
  let home = IpAddrKind::V4(Strig::from(127, 0, 0, 1));
  let loopback = IpAddrKind::V6(Strig::from("::1"));

}

fn route(ip : IpAddrKind) {

}


struct Ipv4Addr {
  // --snip--
}

struct Ipv6Addr {
  // --snip--
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

