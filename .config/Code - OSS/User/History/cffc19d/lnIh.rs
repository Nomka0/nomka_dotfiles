fn main() {
  //una tupla
  let tupla : (&str, u8, char) = ("soy un slice string", 1, 'a');
  let (x, y, z) = tupla;
  println!("{:?}",tupla);
  
  println!("primer elemento de tupla: {x}");
  println!("segundo elemento de tupla: {y}");
  println!("tercer elemento de tupla: {z}");
  
  //aquí creamos un usuario, como le colocamos el mut, como en cualquier otra var, es mutable
  let mut user1 = Usuario{
    activo: true,
    id: 283023,
    nombre: String::from("Lady"),
    sexo: 'F',
    correo: String::from("correo@empresa.com"),
    tipo: String::from("Empleado"),
    conteo_entrada : 1,
  };

  //una instancia de estructura puede ser mutable, eso significa que todos los elementos de la struct son mutables.
  //Sin embargo, no se pueden declarar partes individuales de la struct como mutables, únicamente la struct entera.

  user1.conteo_entrada += 1;

  //modificamos correo

  user1.correo = String::from("otrocorreo@empresa.com");

  println!("conteo de entradas: {}\ncorreo: {}", user1.conteo_entrada, user1.correo);
  
  //Struct Update Syntax
  //quiero crear otro usuario
  //pero quiero que teng algunas de las características de user1, bueno puedo usar esta sintaxis
  // es sencillo, simplemente instancio las propiedades de la estructura que quiero que tenga user2
  //y las que ya tiene user1 y se las quiero pasar a user2, 

}

//las estructuras son como las tuplas, solo que más flexibles, también tenemos datos,
//pero los podemos nombrar, y además el orden no importa, también podemos ponerle nombre a nuestra estructura y
//asignarle métodos y traits (de esto último aún no estoy tan seguro).
struct Usuario {
  //estos se llaman fields
  activo : bool,
  id: u32,
  nombre: String,
  sexo: char,
  correo: String,
  tipo: String,
  conteo_entrada : u32,
}


//Struct dentro de función y
//field init shorthand
fn build_employee(id : u32, nombre : String, sexo : char, correo: String, tipo : String) -> Usuario {
  Usuario {
    activo : true,
    id,
    nombre, //como los nombres de las parametros de entrada y los de la struct son iguales, se puede usar
    //esta sintaxis de field init shorthand, en la que simplemente escribimos el nombre común de la var y yap
    sexo,
    correo,
    tipo,
    conteo_entrada: 1,
  }
}
