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


// Make it work
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");

    println!("Success!");
}

 