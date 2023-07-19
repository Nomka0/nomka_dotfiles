fn main() {
 let x = 5;

 let x = x + 1;
 {
    let x = x * 2;
    println!("el vaalor de x en el scope interno es: {x}");
 }
 
 println!("el vaalor de x es {x}")
}