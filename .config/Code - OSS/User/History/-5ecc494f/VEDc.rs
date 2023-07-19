fn main() {
  let s = String::from("hello");
  let len = s.len();

  //these are the same (implicitly starting)
  let slice = &s[0..2];
  let slice = &s[..2];

  //these are the same (implicitly ending)
  let slice = &s[3..len];
  let slice = &s[3..];
  
  //entire string
  let slice = &s[3..];
  
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
// método con slices
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}
