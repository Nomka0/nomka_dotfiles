
fn f_to_c(temp_f : i32) -> i32 {
    (temp_f - 32) * 5/9
}

fn c_to_f(temp_c : i32) -> i32 {
    (temp_c * 9/5) + 32
}

fn main() {
    let fahrenheit : i32 = 32;
    println!("Temperatura en fahrenheit: {fahrenheit}");
    let transformado_celsius = f_to_c(fahrenheit);
    println!("Temperatura convertida en grados celcius: {transformado_celsius}\n");

}
