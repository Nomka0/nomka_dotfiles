/*
// Fix the errors

//explicit discriminators
enum Number {
    Zero = 0,//aquí usé un explicit discriminator, el compilador asume que los valores sig
    //serán uno y dos
    One,
    Two,
}

enum Number1 {
    Zero,
    One = 1,// igualmente, aquí asume que elvalor anterior será 0, y el siguiente 2
    Two,
}

// C-like enum
enum Number2 {
    Zero  = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);

    println!("Success!");
} 
*/

/*
// Fill in the blank
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {
    let msg1 = Message::Move{x : 1, y : 2}; // Instantiating with x = 1, y = 2 
    let msg2 = Message::Write(String::from("hello world")); // Instantiating with "hello, world!"

    println!("Success!");
} 
*/

/*
// Fill in the blank and fix the error
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 2, y: 2};

    if let Message::Move{x,y} = msg {
        assert_eq!(x,y);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
} 
*/

/*
// Fill in the blank and fix the errors
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [//un array definido, tipo Messagem length 3
        Message::Quit,//puedo hacer un array de mensajes, con tipos del enum MEssage
        Message::Move{x:1, y:3},//una colección de distintos datos del tipo Message
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:?}", msg);

}
*/

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let n = six.unwrap() {
        println!("{:?}", n);

        println!("Success!");
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),//i es el valor contenido en x.
    }
}

