#[derive(Debug)] //esto es para que obtenga el
struct Rectangulo {
  largo: u32,
  ancho: u32,
}

fn main() {
  
  let largo : u8 = 10;
  let ancho : u8 = 5;
  let area : u8 = calcular_area_rectangulo(largo,ancho);
  println!("el área del rectángulo es = {area} pixeles cuadrados");
  
  //ahora con tuplas
  let rect1 : (u32, u32) = (150, 320);
  println!("el area del rectangulo es de : {} pixeles cuadrados", area_v_tuplas(rect1));

  //ahora con structs

  let rect2 = Rectangulo {
    largo: 1000,
    ancho: 200,
  };

  println!("El área del rectángulo {} es: {}", stringify!(rect2), area_v_structs(&rect2));

  println!("{:?}", rect2);

}

fn calcular_area_rectangulo(largo : u8, ancho : u8) -> u8{
  let area : u8 = largo * ancho;
  area
}

fn area_v_tuplas(dimensiones : (u32, u32)) -> u32{
  dimensiones.0 * dimensiones.1
}

//se toma la estructura por referencia, para pedirla prestada, en lugar de tomar ownership
//ya que entonces después quedaría inútil esa variable
fn area_v_structs(rectangulo : &Rectangulo) -> u32 {
  rectangulo.largo * rectangulo.ancho
}