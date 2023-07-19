
fn f_to_c(temp_f : i32) -> i32 {
    (temp_f - 32) * 5/9
}

fn main() {
    let fahrenheit : i32 = 32;
    pritn!("Temperatura en fahrenheit: {fahrenheit}");
    let transformado_celsius = f_to_c(fahrenheit);
    pritn!("la temperatura");
}
