
fn f_to_c(temp_f : f32) -> f32 {
    (temp_f - 32) * 5/9
}

fn c_to_f(temp_c : f32) -> f32 {
    (temp_c * 9/5) + 32
}

fn main() {
    let fahrenheit : f32 = 32.0;
    println!("Temperatura en fahrenheit: {fahrenheit}");
    let transformado_celsius = f_to_c(fahrenheit);
    println!("Temperatura convertida en grados celcius: {transformado_celsius}\n");

    let fahrenheit : f32 = 32.0;
    println!("Temperatura en fahrenheit: {fahrenheit}");
    let transformado_fahrenheit = c_to_f(celsius);
    println!("Temperatura convertida en grados celcius: {transformado_fahrenheit}\n");

}
