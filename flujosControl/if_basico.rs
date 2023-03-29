fn main(){
    let mut var = 5;
    
    //If
    if var == 5{
        println!("Es cinco: {}", var);
    }else if var == 3{
        println!("Es tres: {}", var);
    }else{
        println!("No se que número es");
    }

    let resul = if var > 5 { 100 } else { 0 };
    println!("Resul: {}", resul);
    
    //Loop
    loop {
        println!("loop");
        if var == 10{
            break;
        }
        var += 1;
    }
    //println!("var: {}", var);

    //While
    while var < 15{
        println!("var: {}", var);
        var += 1;
    }

    //For
    let arreglo = [1, 2, 3, 4, 5];
    for elemento in arreglo.iter() {
        println!("Elemento: {}", elemento);
    }

    for ggg in 1..3{
        println!("{}", ggg);
    }
}
