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


}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
