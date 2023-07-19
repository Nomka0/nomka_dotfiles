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
   
   fibonacci(200);
   
}

fn fibonacci(limit : u32) {
   let mut fibo_1 : u32 = 0;
   let mut fibo_2 : u32 = 1;
   let mut counter : u32 = 0;

   println!("{fibo_1}");
   println!("{fibo_2}");

   for _ in 0..limit{
      let numero_print = fibo_1 + fibo_2;
      println!("{numero_print}");
      fibo_1 = fibo_2;
      fibo_2 = numero_print;
   }

}


