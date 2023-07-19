fn main() {
  println!("hello world");
  curismas();
}

fn curismas() {
  let dias : [&str; 12]= ["first",  "second",  "third",  "fourth",  "fifth",  "sixth",  "seventh",  "eighth",  "ninth",  "tenth",
  "eleventh",  "twelfth"];

  for element in &dias {
    println!("On the {element} day of Christmas, my true love sent to me\n");
  }
}
