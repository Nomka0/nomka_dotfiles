fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  //también podemos descomponerlas
  let rgb_blue = black.2;

  println!("{}",rgb_blue);
}

//también tenemos tuple structs
struct Color (i32, i32, i32); //los nombres de las structs van la primera con mayuscula upper camel case
struct Point (i32, i32, i32);
