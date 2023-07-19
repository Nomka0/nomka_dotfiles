fn main() {
  let mut s = String::from("hello world");
  let len = s.len();

  //these are the same (implicitly starting)
  let slice = &s[0..2];
  let slice = &s[..2];

  //these are the same (implicitly ending)
  let slice = &s[3..len];
  let slice = &s[3..];
  
  //entire string
  let slice = &s[3..];
  
  let s_recortada = first_word(&s);


  println!("s_recortada: {s_recortada}");
  s.clear();//este método ÚNICAMENTE puede ser usado después de que s_recortado haya sido usado
  //ya que s_recortado ya lo pidió prestado como referencia inmutable, y aquí lo queremos mutar
  //solo podemos hacer esto, hasta que s recortada, ya no lo tenga prestado

  let soy_inmutable = "inmutable"; //esto se declara implicitamente como &str
  //qué quiere decir? que es una referencia INMUTABLE de string slice. todas las strings declaradas
  //así son inmutables

  //slices también funcionan con otros datos, como las arrays, bueno después de todo, las strings son arrays de chars.

  let un_array = [1,2,3,4,5,6,7];

  let pedazo_array = &un_array[..3];

  println!("{:?}", pedazo_array); //estos arrays tienen un traid de debug, así que para debuggearlos
  //y que se formateewn bien en el debug, se pone el :?, y se escribe la variable a imprimir
  //afuera
  assert_eq!(pedazo_array, &[2, 3]);//estamos comparando que pedazo array sea igual a
  //ese arreglo que pusimos, si es igual, el programa continua, sino lanzará un error el compilador
  //indicando en qué no son iguales

}


/* método sin slices
fn first_word(s: &String) -> usize { //tratamos de buscar la primera palabra de una string
  let bytes = s.as_bytes();//con este método convertimos la string en bytes

  for (i, &item) in bytes.iter().enumerate() {//.enumerate() retorna una tupla tipo (index, ref. del elemento)
    //recorreremos cada tupla que suelte el enumerate
      if item == b' ' {//si el elemento (b es el literal de bytes) tiene un caracter de espacio ->
          return i;//retorna el index nomás
      }
  }

  s.len()//si no se cumple la función del espacio, pim, se retorna el lenghth de s
}
*/

/*
// método con slices
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];//retorna la string HASTA la posición (i) donde encontró un espacio
      }
  }

  &s[..]// si no encuentra ningún espacio, retorna todo el string
}
*/

// método con slices NOTACIÓN &str (string slice) como parametro
//al declarar como parametro de entrada &str en lugar de &String
//la función va a ser más flexible, ya que tenemos la posibilidad de meter Strings, o pedazos de string 
// con la notación [a..b]
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];//retorna la string HASTA la posición (i) donde encontró un espacio
      }
  }

  &s[..]// si no encuentra ningún espacio, retorna todo el string
}