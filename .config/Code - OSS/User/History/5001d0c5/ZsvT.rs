

let mut fibo_1 = 0;
let mut fibo_2 = 1;
let mut counter = 0;

fn fibonacci() {
   if counter != 6 {
      let numero_print = fibo_1 + fibo_2;
      println!("número {counter} en la secuencia fibonacci: {numero_print}"); 
   }
   else {
      fibonacci();
   }
}


fn main() {
   /*
   let mut vector_inicial : Vec<u32> = vec![0, 1];
   
   for numbers in vector_inicial.iter() {
      println!("{numbers}");
   }
   
   let mut counter = 1;
   for numbers in vector_inicial.iter_mut() {
      let mut numero_sumado = numbers + vector_inicial[counter];
      vector_inicial.push(numero_sumado);
      counter += 1;
   }
   */
   
   
}
