fn main(){
    mostrar_bienvenida();
    //pasa_parametros();
    let nro = pasa_parametros(8);
    println!("Tu número es: {}", nro);

    let nro2 = {
        10
    };
    println!("Tu número es: {}", nro2);
    saludar("Valeria".to_string());
}

fn mostrar_bienvenida(){
    println!("Bienvenidos a todos!");
}

//fn pasa_parametros(numero:u8){
    //println!("El número es: {numero}");
fn pasa_parametros(nro: i32) -> i32{
    8
    //nro + 4
}

fn saludar(nombre: String){
    println!("Hola {}", nombre);
}
