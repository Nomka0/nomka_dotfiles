fn main() {
 let x = 5;
//shadowing
 let x = x + 1;
 {
    let x = x * 2;
    println!("el vaalor de x en el scope interno es: {x}");
 }
 
 println!("el vaalor de x es {x}");

 let mut y = 5;
 println!("el vaalor de y mutable es {y}");
 y = 2+1
 println!("el vaalor de y cambió a {y}");

 
 let spaces = "   ";
 let spaces = spaces.len();
}