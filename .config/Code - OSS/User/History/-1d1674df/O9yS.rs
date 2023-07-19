fn main() {
  println!("hello world");
  curismas();
}

fn curismas() {
  let dias : [&str; 12] = ["first",  "second",  "third",  "fourth",  "fifth",  "sixth",  "seventh",  "eighth",  "ninth",  "tenth",
  "eleventh",  "twelfth"];

  let obsequios : [&str; 12] = ["A partridge in a pear tree",  "Two turtle doves",  "Three french hens",
    "Four calling birds",  "Five golden rings",  "Six geese a-laying",  "Seven swans a-swimming",
      "Eight maids a-milking",  "Nine ladies dancing",  "Ten lords a-leaping",  "Eleven pipers piping", 
      "Twelve drummers drumming"];

  let mut contador_obsequios = 0;
  for element in &dias {
    println!("On the {element} day of Christmas, my true love sent to me\n");
  }
}
