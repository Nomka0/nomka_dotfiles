/*
fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;
 
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
 }
 */

/*
fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}
*/

/*
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {

}
*/

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("{}", s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world");
}