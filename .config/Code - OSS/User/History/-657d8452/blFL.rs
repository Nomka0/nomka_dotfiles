
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
    
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    
    println!("The value of x is {} and value of y is {}", x, y); 
}
*/

// Fix the error with the use of define_x
fn main() {
    define_x
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}