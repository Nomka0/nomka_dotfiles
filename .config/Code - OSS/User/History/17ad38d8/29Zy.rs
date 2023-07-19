fn main() {
  let mut s = String::from("hello");
  

  change(&mut s);
  println!("{s}");
  let prestado1 = &mut s;
  println!("{prestado1}");

}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
