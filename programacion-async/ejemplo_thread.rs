fn main(){
    
    std::thread::spawn( || {
        let _contenido: String = leer_archivo("foo.txt");
        println!("{_contenido}");
    });

    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn leer_archivo(nombre_archivo: &str) -> String{
    println!("Leyendo el nombre del archivo {nombre_archivo}");
    std::thread::sleep(std::time::Duration::from_secs(2));
    "Hola".to_string()
}
