fn main() {
    println!("Hello world!");
    another_function(20, 'm');
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");


    num_1 = 2;
    num_2 = 20;

    numero_sumado = suma_numeros(num_1, num_2);

}

fn suma_numeros(x : u8, y : u8){
    x + y
}

fn another_function(medida : i8, unidad_medida: char){
    println!("medida : {medida} {unidad_medida}");
}