//Este archivo toma únicamente los conceptos de structs e implementación del proyecto de rectangle_area

#[derive(Debug)] //esto es para que obtenga el trait de debug y así podamos usar ":?" en println!
struct Rectangulo {
  largo: u32,
  ancho: u32,
}

//todas las funciones dentro de este impl son associated functions.
impl Rectangulo {
  fn area(&self) -> u32 { //así como hicimos en la función fuera de esta implementación "rectangulo : &Rectangulo"
  //lamamos a una referencia de Triangulo, en este caso estamos haciendo exacatamente lo mismo.
  //&self es una abreviatura para self : &Self, se puede escribir de esa forma para que sea más corto.
  //el tipo self se refiere a la structura en donde estamos aplicando el método de la implementación, en este caso
  //es Rectangulo. sellf puede ser borrowed inmutablemente (como aquí; &self) omutablemente: &mut self
  self.ancho * self.largo
}
//este método retorn un booleano, indicando si el ancho es positivo, o no.
  fn ancho_positivo(&self) -> bool {
    self.ancho > 0 //esto va a retornar algo ya que no pusimos ";", sencillamente, si se cumple esa condición,
    //entonces va a retornar true, sino false
  }
//este método nos lo pidieron realizar en el libro, es para saber si un rectángulo cabe en otro
  fn can_hold(&self, other : &Rectangulo) -> bool {
    other.largo < self.largo  &&  other.ancho < self.ancho
  }
//NOTA SUPER HIPER MEGA IMPORTANTE: &self es lo que va antes del "." cuando se invoca el método
//ej: si tengo can_hold(&self, other : &Rectangulo) entonces este sería el valor que tomaría self
//el que va de primero aquí: rect1.can_hold(rect2) y "rect2" sería "other",
//si el método solo tiene &self, entonces no iría nada dentro de la función cuando se invoca
//solo la variable que se usa antes del punto ".". EJ: rect1.area(), rect1 sería &self
}

fn main() {
  let rect1 = Rectangulo {
    largo : 30,
    ancho : 50,
  };
  
  let rect2 = Rectangulo {
    largo : 10,
    ancho : 40,
  };
  
  let rect3 = Rectangulo {
    largo : 60,
    ancho : 45,
  };
  
  //se usan las implementaciones
  //implementación de área
  println!("El área del rectángulo (impl) {} es: {}", stringify!(rect3), rect3.area());
  //ancho positivo?
  println!("El rectángulo tiene un ancho positivo; eso es {}", rect3.ancho_positivo());
  println!("rect1 puede contener a rect2? {}", rect1.can_hold(&rect2));
  println!("rect1 puede contener a rect3? {}", rect1.can_hold(&rect3));
}
