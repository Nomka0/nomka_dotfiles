fn main() {
  println!("hello world");
  curismas();
}

fn curismas() {
  let dias : [&str; 12] = ["first",  "second",  "third",  "fourth",  "fifth",  "sixth",  "seventh",  "eighth",  "ninth",  "tenth",
  "eleventh",  "twelfth"];

  let obsequios : [&str; 12] = ["A partridge in a pear tree",  "Two turtle doves",  "Three french hens",
    "Four calling birds",  "Five golden rings",  "Six geese a-laying",  "Seven swans a-swimming",  "Eight maids a-milking",  "",  "",  "",  ""];

  for element in &dias {
    println!("On the {element} day of Christmas, my true love sent to me\n");
  }
}
