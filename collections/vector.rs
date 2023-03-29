#[allow(unused_variables)]
fn main(){
    /*
    Collections (básicas):
        -vector
        -strings
        -hashmap
    */

    //Vectores: Guardar valores uno a uno al lado del otro, debe ser el mismo tipo de datp
    
    let v: Vec<i32> = Vec::new(); //Forma de inicializar vectores
    //let v = vec![1, 2, 3]; //Otra forma de inicializar vectores
    aa();
     
}

fn aa (){
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(9);
    let ind = 0;

    match v.get(ind) { //Lo que va a adentro de get es el índice: 0,1,...
        Some(valor) => println!("Valor del elemento con índice[{}]: {}", ind, valor),
        None => ()
    }

    for i in &v{ //El & significa que es por referencia
        println!("{}", i);
    }

    for i in &mut v { //¿Por qué el & va en el mut?
        *i += 10;
    }

    for i in &v{ //El & significa que es por referencia
        println!("{}", i);
    }

    enum Mensaje{
        TEXTO(String),
        ERROR(i32),
    }

    let mensajes = vec![Mensaje::TEXTO("hola".to_string()), Mensaje::ERROR(404)];
    for m in &mensajes{
        match m{
            Mensaje::TEXTO(texto) => println!("Texto: {}", texto), //De dónde sale la variables textp?
            Mensaje::ERROR(error) => println!("Error: {}", error),
        }
    }
}
