//Tuples

//1. 🌟 Elements in a tuple can have different types. Tuple's type signature is (T1, T2, ...), where T1, T2 are the types of tuple's members.


/*
fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members // lol no sabía esto
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
*/

//2. Members can be extracted from the tuple using indexing.

/*
// Make it work
fn main() {
    let t = ("i", "am", "sunface"); //en las tuples no es necesario  definir loss tipos
    //no como en los arrays
    assert_eq!(t.2, "sunface");

    println!("Success!");
}
*/

//3. Long tuples cannot be printed


// Fix the error
fn main() {

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let range_vec: Vec<_> = too_long_tuple.0..=too_long_tuple.9;
    println!("too long tuple: {:?}", range_vec);
}

