enum IpAddrKind {
  V4,
  V6,
}

fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  let ip_cualquieraa = IpAddr {
    kind : IpAddrKind::V4,
    address : String::from("123456"),
  }
}

fn route(ip : IpAddrKind) {

}


struct IpAddr {
  kind : IpAddrKind,
  address : String,
}

