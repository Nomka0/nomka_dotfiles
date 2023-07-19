fn main() {
 let x = 5;
//shadowing el valor puede cambiar, y el tipo también
//pero solo en un scope interno, o temporalmente.
//al momento de hacerle print al x, saldrá ssu
//vaalor original
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

 //unaa constante. la notación es caps con underscore
 //
 const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
 println!("tres horas en un segundo: {THREE_HOURS_IN_SECONDS}");
 let spaces = "   ";
 //caaambiaa de string aa int (en mut no sse puedde)
 let spaces = spaces.len();
}