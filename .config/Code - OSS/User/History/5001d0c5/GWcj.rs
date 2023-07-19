fn main() {
   /*
   let mut vector_inicial : Vec<u128> = vec![0, 1];
   
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
   
   fibonacci(10);
   
}

fn fibonacci(limit : u128) {
   let mut fibo_1 : u128 = 0;
   let mut fibo_2 : u128 = 1;
   let mut counter : u128 = 0;

   println!("{fibo_1}");
   println!("{fibo_2}");

   for _ in 0..limit{
      let numero_print = fibo_1 + fibo_2;
      println!("{numero_print}");
      fibo_1 = fibo_2;
      fibo_2 = numero_print;
   }

}


