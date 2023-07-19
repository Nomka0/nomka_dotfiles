/*
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    //let y = x.clone();
    //let y = &x;
/**
    let x : &str = "hello, world";
    let y = x;
*/
    println!("{},{}",x,y);
}
*/

/*
// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
*/

/*
fn main() {
    let s = give_ownership();
    println!("{:?}", s);
}

// Only modify the code below!
fn give_ownership() -> Vec<u8> {
    let s = String::from("hello, world");
    println!("{}", s);
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    _s
}
*/

/*
// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}
*/

/*
// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = (&x.0, &x.1, &x.3);
    println!("{:?}, {:?}", x, y);
}
*/

/*
fn main() {
    let s = String::from("hello, ");

    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}
*/


fn main() {
    let x = Box::new(5);

    //let mut y = Box::new(2);      // Implement this line, dont change other lines!
    
    let mut y = Box::new(2);      


    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
