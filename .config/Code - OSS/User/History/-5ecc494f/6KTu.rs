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
  //ya que s_recortado


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
          return &s[0..i];//retorna la string HASTA la posición (i) donde encontró un espacio
      }
  }

  &s[..]// si no encuentra ningún espacio, retorna todo el string
}
