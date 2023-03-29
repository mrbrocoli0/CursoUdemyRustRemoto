#[allow(unused_variables)] //Para que no alerte por variables no usadas

//Option

/*
enum Option<T> {
    Some (T),
    None,
}
//Todo lo de arriba ya viene predefinido en el lenguaje
*/

//Null

fn main(){
    //let nombre:Option<String> = None;
    let nombre:Option<String> = Some("Valeria1".to_string());

    match nombre{
        None => println!("Nombre no identificado"),
        Some(nombre) => println!("{}", nombre),
    }

    let nuevo = User{
        name: "Valeria2".to_string(),
        //age: Some(27),
        age: None,
    };

    println!("Nombre: {}", nuevo.name);
    let age = nuevo.getAge();

    match age{
        Some(age) => println!("Edad: {}", age),
        //None => (), //Así no hace nada
        None => println!("Edad no identificada"),
    }
}

struct User{
    name: String,
    age: Option<i32>,
}

impl User{
    fn getAge(&self) -> Option<i32>{ //Va a devolver un entero de 32 bits, ->
        self.age
        //self.age.unwrap_or_default() //Te da la edad en 0, quitar el match de arriba y sólo imprimir age
        /*
         if self.age.is_none(){
            -1
         }else{
            0
         }
         //Dar instrucciones con el resultado, si es -1 mandar error, si es 0 imprimir la fecha
         */
    }
}
