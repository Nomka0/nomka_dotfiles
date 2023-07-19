fn main() {
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
    let obsequio_actual = obsequios[contador_obsequios];
    println!("On the {element} day of Christmas, my true love sent to me");
    println!("{obsequio_actual}\n");
    contador_obsequios += 1;
  }
}
