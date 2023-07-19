fn main() {
  let mut s = String::from("hello");
  

  change(&mut s);
  println!("{s}");
  let prestado1 = &mut s;
  println!("{prestado1}");

  //se pueden usar brackets para prestar varias veces el mismo valor 
  // (una vez salidos de los brackets, se termina el scope, así que no sé si sirva de mucho la vdd)

  {
    let prestado1 = &mut s;
  }
    let prestado2 = &s;
    let prestado3 = &s; //aquí no hay problema, se puede pedir prestado varias veces sin mutar
    //let pretado 4 = &mut s //no se puede pedir prestado mutado, si ya se pidió prestado 
//antes mutado

    println!("{prestado2},{prestado3}");
    let prestado4 = &mut s; //aquí sí puedo pedir prestado, porque ya los valores que presté 
    //en prestado 2 y 3 los devolví, los usé en el print anterior, así que ya se puede usar
    //otra vez sin problemas
    println!("{prestado4}");

    let string_cualquiera = dangle();
    println!("{string_cualquiera}");

}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

//este dangle no va a funcionar, está retornando un &String, pero cuando se termine el scope
//de la función, entonces se le hace deallocate a s, así que la referencia va a estar apuntando
// a una String invalida, porque se le hizo deallocate

/*
fn dangle() -> &String {
  let s = String::from("hello");

  &s
}
*/

//en su lugar, simplemente se devuelve la String, en lugar de la referencia
//obviamente la ownership es movida de s, y esa ownership se retorna y se asigna a quien sea 
//que use esa funcioón como statement

fn dangle() -> String {
  let s = String::from("soy una string cualquiera");

  s
}
