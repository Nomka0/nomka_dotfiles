fn main() {
 let x = 5;
//shadowing el valor puede cambiar, y el tipo también
//pero solo en un scope interno, o temporalmente.

 let x = x + 1;
 {
    let x = x * 2;
    println!("el vaalor de x en el scope interno es: {x}");
 }
 
 println!("el vaalor de x es {x}");

 //caambiando el vaalor a una vaariable mutable
 let mut y = 5;
 println!("el vaalor de y mutable es {y}");
 y = 2+1;
 println!("el vaalor de y cambió a {y}");

 const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
 
 let spaces = "   ";
 let spaces = spaces.len();
}