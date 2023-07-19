fn main() {
  let mut s = String::from("hello");
  
  let prestado1 = &mut s;

  change(&mut s);
  println!("{s} , {prestado1}");
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
