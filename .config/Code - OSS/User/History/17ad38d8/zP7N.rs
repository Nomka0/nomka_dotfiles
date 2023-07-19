fn main() {
  let mut s = String::from("hello");
  

  change(&mut s);
  println!("{s}");
  let prestado1 = &mut s;
  println!("{prestado1}");

  //se pueden usar brackets para prestar varias veces el mismo valor 
  // ()

}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
