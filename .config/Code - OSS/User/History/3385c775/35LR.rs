enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn main() {
  let home = IpAddrKind::V4(Strig::from());
  let loopback = IpAddrKind::V6(Strig::from("::1"));

}

fn route(ip : IpAddrKind) {

}



