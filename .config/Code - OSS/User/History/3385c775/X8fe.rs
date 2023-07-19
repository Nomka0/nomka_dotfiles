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


enum Message {
  Quit,
  Move {x : i32, y : i32},
  Write(String),
  ChangeColor(i32, i32, i32),
  //Todas estas son structs que perfectamente podríamos definir individualmente, pero es más conveniente
  //tenerlas acá en este enum, no solo están mejor agrupadas, sino que además, es más fácil implementar
  //métodos o funciones
}

//Con los enums también podemos implementar

impl Message {
  fn call(&self) {
    self.Write.0
  }
}

