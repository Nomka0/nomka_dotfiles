//Slices
//los slices pueden tener un length desoncocido en tiempo de compilacion, a diferecia de
//los arrayas, que tenemos que saberlos todo el tiempo

/*
// Fix the errors, DON'T add new lines!
//siempre usar & para no causar errores
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;

    println!("Success!");
}
*/

/*
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);
    //el slice consiste de [tipo; tamaño] para cada uno de los valores,
    //así que cada pieza consta de 8 bytes (4*2), o sea que como son dos elementos entotal en el
    //slice de nombre "slice", entonces es (4*2) * 2, o sea 16.

    println!("Success!");
}
*/

/*
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice : &[i32]= &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
*/

/*
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
*/

/*
fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}
*/


// Fix errors
fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want know more, this is called `Deref coercion`. 
    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &str) -> &str {
    &s[..1]
}
