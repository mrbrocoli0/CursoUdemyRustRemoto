fn main(){
    
    let _contenido: impl Future<Output = String> = leer_archivo("foo.txt");
    //println!("{_contenido}");

}

//A las funciones async les ponemos ese sufijo antes del fn
async fn leer_archivo(nombre_archivo: &str) -> String{
    //En todas las funciones async lo que se va a devolver es un Future
    println!("Leyendo el nombre del archivo {nombre_archivo}...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    "Hola".to_string()
}