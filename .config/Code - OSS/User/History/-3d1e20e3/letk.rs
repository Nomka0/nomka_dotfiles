//data types

//unsigned andd signed values

//an 8 bit size unsigned value







fn main() {
    let x: i8  = -32;
    //let y : i8 = 300;  //an overflow

    //a float type

    let f_x = -2.5241243251251;

    //bool
    let dormido = false;
    
    //chars
    let gatica = '😻';
    let letra = 'z';

    //tupla 

    let tupl : (char, i8, f64) = ('h', 69, -69.420);
    
    //ddestructurando la tupla
    let (x, y, z) = tupl;

    //también se puede acceder a la tupla por medio del indice

    println!("valor de una letra: {x.0}");
    println!("valor de una letra: {z}");
    println!("valor de una letra: {letra}");
    println!("emoji: {gatica}");
    println!("valor de dormido: {dormido}");
    println!("valor de x: {x}");
    println!("valor de f_x: {f_x}");
    
}
