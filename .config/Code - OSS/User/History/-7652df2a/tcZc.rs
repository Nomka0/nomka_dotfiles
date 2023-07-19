#[derive(Debug)] //esto es para que obtenga el trait de debug y así podamos usar ":?" en println!
struct Rectangulo {
  largo: u32,
  ancho: u32,
}

impl Rectangulo {
  fn area(&self) -> u32 { //así como hicimos en la función fuera de esta implementación "rectangulo : &Rectangulo"
  //lamamos a una referencia de Triangulo, en este caso estamos haciendo exacatamente lo mismo.
  //&self es una abreviatura para self : &Self, se puede escribir de esa forma para que sea más corto.
  //el tipo self se refiere a la structura en donde estamos aplicando el método de la implementación, en este caso
  //es Rectangulo. sellf puede ser borrowed inmutablemente (como aquí; &self) omutablemente: &mut self
    self.ancho * self.largo
  }
  fn ancho_positivo
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
  let scale = 2;
  let rect2 = Rectangulo {
    largo: dbg!(1000 * scale),
    ancho: 200,
  };

  //ahora con un método implementado en la propia struct de rectangulo

  let rect3 = Rectangulo {
    largo : 5,
    ancho : 2,
  };

  println!("El área del rectángulo (impl) {} es: {}", stringify!(rect3), rect3.area());


  println!("El área del rectángulo {} es: {}", stringify!(rect2), area_v_structs(&rect2));

  println!("{:?}", rect2); //se imprime rect2 con el trait de debug por medio de println!

  dbg!(&rect2); //ahora se imprimie por dbg!, la diferencia es que este macro toma ownership del 
  //parametro de entrada, y luego la retorna, la devuelve, aquí lo pasamos por referencia, porque no 
  //queremos que tome ownership. la otra diferencia es que imprime el valor enconsola
  //por meedio de stderr, cuando println lo hace por medio de stdout

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