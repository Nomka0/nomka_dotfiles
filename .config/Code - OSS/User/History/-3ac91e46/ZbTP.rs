fn main() {
    println!("Hello world!");
    another_function(20, 'm');
}

fn another_function(medida : i8, unidad_medida: char){
    println!("valor del parametro : {medida} {unidad_medida}");
}