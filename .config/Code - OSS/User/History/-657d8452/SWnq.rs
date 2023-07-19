
//1. 🌟 A variable can be used only if it has been initialized.
// Fix the error below with least amount of modification to the code
/*
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/

//2. 🌟 Use mut to mark a variable as mutable.

/*
fn main() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}
*/


// Fix the error below with least amount of modification
/*
fn main() {
    let x: i32 = 10;
    let y: i32 = 14;
        {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
        }
    println!("The value of x is {} and value of y is {}", x, y); 
}
*/

/*
// Fix the error with the use of define_x
fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String{
    let x = "hello".to_string();
    x
}
*/

/*
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
*/

/*
// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    //x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
*/

/*
fn main() {
    let _x = 1; 
}
*/
// Warning: unused variable: `x`

/*
fn main() {
    let un_float :f32 = 0.1+0.2;
    assert!(un_float==0.3);

    let un_float = 0.1_f32+0.2f32;
    assert!(un_float==0.3);


    println!("Success!");
}
*/

/*
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 97..=122 {
        println!("{}",c);
    }
}*/

/*
// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
*/
/*
// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1_u32 + 2 == 3);

    // Integer subtraction
    assert!(1_i32 - 2 == -1);
    assert!(1_i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    //Esta línea realiza la operación AND (y lógico) entre los
    //números binarios 0b0011 y 0b0101. El resultado es 0b0001,
    //que es 1 en decimal. El formato de impresión {:04b} formatea 
    //el resultado como un número binario de 4 dígitos.
    el resultado como un número binario de 4 dígitos.
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
*/