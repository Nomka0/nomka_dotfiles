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

fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);
    //el slice consiste de [tipo; tamaño] para cada uno de los valores,
    //así que cada pieza consta de 8 bytes (4*2), o sea que 

    println!("Success!");
}
