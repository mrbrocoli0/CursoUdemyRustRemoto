//Structs
fn main(){
    //let mut user = Usuario{
    let mut user = Usuario{
        nombre: String::from("Valeria"),
        //email: "aaa@hotmail.com",
        email: String::from("aaa@hotmail.com"),
        edad: 27,
        activo: true,
    };

    println!("Usuario: {}, Edad: {}", user.nombre, user.edad);

    user.activo = false;

    //nuevo_usuario(String::from("Bruno"), String::from("bruno@example.com"));
    //Brueno --> nuevo usuario
    
    let user1 = nuevo_usuario(String::from("Bruno"), String::from("bruno@example.com"));

    println!("Usuario: {}, Email: {}", user1.nombre, user1.email);

    let user2 = nuevo_usuario(String::from("Aranza"), String::from("aaa@example.com"));

    println!("Usuario: {}, Email: {}", user2.nombre, user2.email);

    let user3 = Usuario{
        nombre: String::from("Ximena"),
        edad: 30,
        ..user1
        //Con ..user1 el valor de los demás campo se los pone a user3

    };
    
    println!("Usuario: {}, Email: {}, Edad {}", user3.nombre, user3.email, user3.edad);

    //tuple structs, puedes tener structs sin especificar el nombre de los campos
    
    struct Point(i32, i32, i32);

    let PointA = Point(22, 55, 345);

    user = Usuario{
        nombre: String::from("Valeria Mutable"),
        //email: "aaa@hotmail.com",
        email: String::from("mutable@hotmail.com"),
        edad: 30,
        activo: false,
    };

    println!("Usuario: {}, Edad: {}", user.nombre, user.edad);

}

struct Usuario{
    nombre: String,
    email: String,
    edad: i32,
    activo: bool
}
fn nuevo_usuario(nombre: String, email: String) -> Usuario{
    return Usuario{
        nombre: nombre,
        email: email,
        edad: 100,
        activo: true,
    };
}
