fn main() {
  let mut s = String::from("hello");
  

  change(&mut s);
  println!("{s}");
  let prestado1 = &mut s;
  println!("{prestado1}");

  //se pueden usar brackets para prestar varias veces el mismo valor 
  // (una vez salidos de los brackets, se termina el scope, así que no sé si sirva de mucho la vdd)

  {
    let prestado1 = &mut s;
  }
    let prestado2 = &mut s;
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
