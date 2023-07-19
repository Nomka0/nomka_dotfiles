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

  //una instancia de estructura puede ser mutable, eso significa que todos los 

  user1.conteo_entrada += 1;

  //modificamos correo

  user1.correo = String::from("otrocorreo@empresa.com");

  println!("conteo de entradas: {}\n correo: {}", user1.conteo_entrada, user1.correo);
  
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
