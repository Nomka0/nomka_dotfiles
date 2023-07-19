fn main() {
    /*
    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.
    */
    
    //este es un string literal
    //está hard-codeado en el programa
    //y por lo tanto también en el 
    //binary ejecutable final
    //es más rápido eficiente, pero 
    //no es mutable
    //la referencia del tipo de dato
    //&str de esta
    //var se guarda en stack
    //la variabla, en memoria estatica
    let s = "hello";
    let t = String::from("Goodbye");
    //Este tipo de dato puede ser mutado:
    
    let mut t = String::from("Goodbye");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 = {}, s2 = {}", s1, s2);
    
    
    //en estos ejemplos podemos ver que no usamos nada de clone, o accedemos a una copia del heap.
    //la razon es sencilla: estos son datos almacenados únicamente en el stack, ya que son datos fijos.
    //e inmutables cuando los declaramos, así que rust no se tiene que preocupar por cosas como
    //el length o la capacidad; son datos sencillos guardados en stack.
    //si hicieramos lo mismo con una variable dinámica ( tipo String, como lo hicimos en el anterior
        //segmento) por ejemplo con s1 y s2, s1 se movería (move, en vez de clone, o shallow-copy)
        //a s2 en el stack, por lo cual, los datos en stack de s1 se invalidarían, y ya no podría
        //acceder a esos datos apuntados a heap, solo s2, pero el programa no correría, lanzaría el error
        //de que los datos fueron movidos de s1 a s2.
        
        //Con estos datos guardados en stack, no pasa eso, son trivialmente copiados con el trait "Copy"
        //en lugar de ser movidos e invalidados.
        
        //Rust no va a dejar usar Copy, si el dato con el que queremos usarlo o cualquiera de sus partes
        //tiene implementado el trait de Drop.
        let q = s;
        println!("q = {q}");
        let x = 2;
        let y = x;
        
        println!("x = {x}, y = {y}");
        
        /*
        Donde se puede usar el trait de Copy:
        *All the integer types, such as u32.
        *The Boolean type, bool, with values true and false.
        *All the floating-point types, such as f64.
        *The character type, char.
        *Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
        */
        
        let s = String::from("hello");  // s comes into scope
        
        takes_ownership(s);             // s's value moves into the function...
        // ... and so is no longer valid here
        

        
        let x = 5;                      // x comes into scope
        
        makes_copy(x);                  // x would move into the function,

        // but i32 is Copy, so it's okay to still
        // use x afterward
        
        
} // aquí se acaba el scope de main, por lo tanto, lo que está dentro, también deja de ser válido
    //afuera de este scope. Rust llama automáticamente a la función drop, la cual podemos usar
    //para retornar memoria allocator. Rust la llama apenas se acaba el scope de la fn donde están las var
    //POdemos usar esa fn manualmente cuando queramos, pero aquí RUst la usa automáticamente.
    
    
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    