fn main() {
  let s = String::from("hello");

  //these are the same
  let slice = &s[0..2];
  let slice = &s[..2];

  
  let slice = &s[0..2];
  let slice = &s[..2];
  
}



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