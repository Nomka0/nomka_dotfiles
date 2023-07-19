//Este archivo toma únicamente los conceptos de structs e implementación del proyecto de rectangle_area

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
//esta unción retorn un booleano, indicando si el ancho es positivo, o no.
  fn ancho_positivo(&self) -> bool {
    self.ancho > 0 //esto va a retornar algo ya que no pusimos ";", sencillamente, si se cumple esa condición,
    //entonces va a retornar true, sino false
  }
}

fn main() {
  let rect1 = Rectangulo {
    largo : 5,
    ancho : 2,
  };
  
  let rect2 = Rectangulo {
    largo : 5,
    ancho : 2,
  };
  
  let rect3 = Rectangulo {
    largo : 5,
    ancho : 2,
  };
  
  //se usan las implementaciones
  //implementación de área
  println!("El área del rectángulo (impl) {} es: {}", stringify!(rect3), rect3.area());
  //ancho positivo?
  println!("El rectángulo tiene un ancho positivo; eso es {}", rect3.ancho_positivo());
}
