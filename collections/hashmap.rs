use std::collections::HashMap;

#[allow(unused_variables)]

fn main(){

    //Collections
    //HashMap - Deben de ser del mismo tipo
    let mut puntaje: HashMap<String, i32> = HashMap::new();
    puntaje.insert("Equipo1".to_string(), 20);
    puntaje.insert(String::from("Equipo2"), 30);

    puntaje.entry(String::from("Equipo1")).or_insert(40); //Esto sólo funciona si falta alguno de los valores

    for (key, value) in &puntaje{
        println!("Equipo: {}, Id: {}", key, value);
    }

    let ptos = puntaje.get("Equipo3"); // Si es diferente a Equipo1 y Equipo2 va a poner que el equipo no existe

    match ptos{
        Some(ptos) => println!("El equipo es: {}", ptos),
        None => println!("El equipo no existe"),
    }
}
