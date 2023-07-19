fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  //también podemos descomponerlas con la misma notación de las estructuras normales
  //solo que aquí usamos los indices para acceder a los valores de la variable que usó esa struct
  let rgb_blue = black.2;

  println!("{}",rgb_blue);

  let subject = AlwaysEqual;
}

//también tenemos tuple structs
struct Color (i32, i32, i32); //los nombres de las structs van la primera con mayuscula upper camel case
struct Point (i32, i32, i32);

//Unit-like Structs
//son similares a las tuples vacias (), unit type
//esto tipo de structs es útil para en el futuro implementarle comportamientos, traits 
struct AlwaysEqual;

