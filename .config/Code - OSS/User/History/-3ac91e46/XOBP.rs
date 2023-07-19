fn main() {
    println!("Hello world!");
    another_function(20, 'm');
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");


    let num_1 = 2;
    let num_2 = 20;

    let numero_sumado = suma_numeros(num_1, num_2);

    println!("la suma de num_1 y num_ es: {numero_sumado}")

}

fn suma_numeros(x : u8, y : u8) -> u8{
    x
}

fn another_function(medida : i8, unidad_medida: char){
    println!("medida : {medida} {unidad_medida}");
}