enum IpAddrKind {
  V4(String),
  V6(String),
}

fn main() {
  let four = IpAddrKind::V4(Strig::from("1.22.112.33"));
  let six = IpAddrKind::V6(Strig::from("::1"));

}

fn route(ip : IpAddrKind) {

}



