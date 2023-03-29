fn main(){
    let edad: Option<i32> = Some(20);
    match edad{
        Some(value) => println!("Edad: {}", value),
        _ => () //Cualquier otro valor de Some, en enum sirve más porque hay más valores
    }

    //Lo mismo que arriba pero en menos código con if-let
    if let Some(value) = edad{
        println!("Edad: {}", value);
    }

    //while
    
    let mut mensajes_no_leidos:Option<i32> = Some(15);
    //let mut mensajes_no_leidos = Option::<i32>::Some(100);

    loop{
        match mensajes_no_leidos{
            Some(value) => {
                if value > 0{
                    println!("Tienes {} mensajes no leidos", value);
                    mensajes_no_leidos = Some(value-1);
                }else{
                    println!("No tienes mensajes nuevos");
                    mensajes_no_leidos = None;
                }
            }
            None => { break; }
        }
    }

    //Lo mismo que arriba pero en menos código con while-let
    
    mensajes_no_leidos = Some(5);

    while let Some(value) = mensajes_no_leidos{
        if value > 0{
            println!("Tienes {} mensajes no leidos", value);
            mensajes_no_leidos = Some(value-1);
        }else{
            println!("No tienes mensajes nuevos");
            mensajes_no_leidos = None;
        }
    }
}
