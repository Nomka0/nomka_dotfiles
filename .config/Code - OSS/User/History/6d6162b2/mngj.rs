/*
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
*/


/*
// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }
 */

 /*
 // Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 
    println!("Success!");
 */

 /*
fn main() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
 
    println!("Success!");
 }
 */

/*
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
*/

/*
fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x : i32, y: i32) -> i32{
    x + y
}
*/

/*
fn main() {
    print();
 }
 
 // Replace i32 with another type
 fn print() -> () { //implicitamente todas las funciones devuelven "()"
    println!("Success!");
 }
*/

/*
// Solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("este programa nunca va a funcionar");
}
*/

/*
// Solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! { // "!" it's called the never type, an invalid type, abrupt termination
//or an neverending loop.
    // Implement this function, don't modify the fn signatures
    loop {
        println!("Ella no te ama.");
    }
}
*/

/*
//diverging functions

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    //panic!("paniquea");
    /*loop{
    
    }
    */
    loop {
        let _z = 3+1;
    }
}
*/


fn main() {
    // FILL in the blank
    let b = 1;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
