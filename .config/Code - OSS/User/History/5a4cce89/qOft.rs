fn main() {
    let number = 3;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3 nor 2");
    }
    
    //if at the right side offf aa llet statement
    
    let condition = true;
    
    let num = if condition {69} else {420};
    
    println!("num: {num}");
    
    loop {
        println!("lol");
        break;
    }
    
    let mut counter = 0;
    //so... here I'm assigning a loop to this variaable, andd when the counter reaaches 10,
    // it'lll break the lloop aandd *2 the counter
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {result}");
    
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    fn colapso() {
        println!("Ḩ̷̜̞̝̀͐G̸͉̈́̂F̴͕͈͚͇͆͝K̸̨͝J̴̰̋͗F̷̨̪̳͚̂͐G̸̖̲̈͋J̶̻͐͒̔Ḑ̶̜̒͒̊́H̷̲͖̩̊̇̊͠N̵̨̻̊̑̂̀R̵̮̞͈͋̔͆̔J̴̣̞̍͑B̶̰̜͂̎̄̂D̶̮̞̦̈́͝D̷̩̘̰͠A̸̡̧͖͚̐̑̀Ḍ̴͇͒N̴̪̤͎͠J̵̜̫̓̌͗ ̷̨̹̠̌͗̃͘Ṭ̴̪͕̉̌̅-̵̰̣͂͌͒T̵̻̓͋͛ ̷̰̭̪̑̚Î̷͖͜'̶͖͇͗͗̄͊M̶̳̃̓ͅͅ ̶̞̥̤̬̎ ̷̼̮̀͆͒F̸̤̲̣̄͛Ȗ̵̖͈͍̇͂̂Ķ̷̘̝̜͝K̴͈̉̀̔͝Ḳ̴͕̆̽̽Ḯ̸̧̐̏̄Ǹ̷̰̼̀̈́Ǵ̶̩ ̵͉̺̦̐̚D̵̩̉̈́̿͛D̷͖͐̐͘͠Ḑ̴̗͍̆̽̕͠I̸̹͙̊͋͑͛S̵͕̣̙̝̎̎G̸̩̜̤̔͝ͅR̸̛̝̠A̷̳͛̒A̸̹͈͔͆̕A̶̙̅̾͌̽A̵̬̣̖̋̎͘C̶̢̜͉͖̅̅͝É̴̘̠̓̑̉");
    }
    
    println!("End count = {count}");
    
    fn decadencia() {
        let mut tranquilidad = true; 
        let mut ansiedad = 0;
        loop {
            if tranquilidad  {
                println!("estoy bien");
            } else {
                break;
            }
            ansiedad += 1;
            if ansiedad == 100 {
                tranquilidad = false;
            }
        }
        colapso();
    }

    decadencia();

    let mut number = 10;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFT OFF !!!!!!!!!!!!");
    println!("THIS IS GROUND CONTROL TO MAJJOR TOOOOOM");

    


    fn print_array() {
        let una_coleccion = [1,2,3,4,5,6,7,8,9,10];
        index = 0;
        while index != una_coleccion.len()-1{
            let valor_array = una_coleccion[index];
            println!("the value of index {index} is: {valor_array}");
        }
    }

}
