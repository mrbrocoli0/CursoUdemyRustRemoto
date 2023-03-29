fn main(){

    //Tipado estático: chequeo de los tipos de datos se hace en tiempo de compilación-- Rust, Java
    //Tipado dinámico: Tiempo de ejecución -- Python, JavaScript}

    //Integer
    let x:i8 = 23; //Signed
    let y:u8 = 40; //Unsigned
    let z:i8 = -40;

    //Integer literals
    let decimal = 10.56;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 01101111;

    //Float point
    let float1 =5.0;
    let float32: f32 = 12.456;

    //Boolean type
    let verdadero = true;
    let falso:bool = false;

    //Character
    let caracter = 'a'; //Unicode
    
    //Compound type
    
    //Tuplas
    let tupla = ('h', 23, -45, 0.222);
    let tupla2: (char, u8, f32) = ('b', 55, 4.345);

    let (x, y, z, k) = tupla; //En el orden en el que asignó arriba van a tomar el valor corresponidente
    
    println!("El valor de x es: {}", x);
    println!("El segundo valor de tupla es: {}", tupla.1);

    //Array
    let arreglo = [1, 2, 3, 4, 5];
    println!("El segundo valor de tupla es: {}", arreglo[1]);

    let arreglo2:[char;5] = ['a', 'b', 'c', 'd', 'e'];
    println!("El segundo valor de tupla es: {}", arreglo2[1]);

    //Strings
    let nombre = "Valeria Fdz";
    println!("Nombre: {nombre}");
    let nombre2: String = "Bruno Rom".to_string();
    println!("Nombre2: {nombre2}");
    let mut domicilio = String::new();
    domicilio = "mi casa".to_string();
    println!("Domicilio: {domicilio}");

}
