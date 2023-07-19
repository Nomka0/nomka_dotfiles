fn main() {
   fibonacci(10);
}

fn fibonacci(limit : u128) {
   let mut fibo_1 : u128 = 0;
   let mut fibo_2 : u128 = 1;

   println!("{fibo_1}");
   println!("{fibo_2}");

   for _ in 0..limit{
      let numero_print = fibo_1 + fibo_2;
      println!("{numero_print}");
      fibo_1 = fibo_2;
      fibo_2 = numero_print;
   }
}


