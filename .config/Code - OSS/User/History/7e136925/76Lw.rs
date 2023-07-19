use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    //stdin() tira un Result, que puede causar 
    //errores, así que, para tratar ese error
    //se agrega el expect()
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //acá también tira un result, entonces
    //también se pone expect()    
    let guess : u32 = guess
                      .trim()
                      .parse()
                      .expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!! (that's what she said)"),
        Ordering::Greater => println!("Too high!!"),
        Ordering::Equal => println!("You win!!"),
    }

}


