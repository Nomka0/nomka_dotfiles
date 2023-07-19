enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn main() {
  let home = IpAddrKind::V4(127, 0, 0, 1);
  let loopback = IpAddrKind::V6(String::from("::1"));

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
  V4(Ipv4Addr),//esto puede aalmacenar lo que queramos, incluso structs, u otros enums
  V6(Ipv6Addr),
}


enum Messsage {
  Quit,
  Move {x : i32, y : i32},
  Write(String),
  ChangeColor(i32, i32, i32),
  //Todas estas son structs que perfectamente podríamos definir individualmente 
}

