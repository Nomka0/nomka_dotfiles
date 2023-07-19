
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


fn main() {
    let un_float :f32 = 0.1+0.2;
    assert!(un_float==0.3);

    println!("Success!");
}

