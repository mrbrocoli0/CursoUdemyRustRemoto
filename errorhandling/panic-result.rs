use std::fs::File;
use std::io::ErrorKind;

//Manejo de errores
//
//Error no recuperable - ejemplo: tratar de acceder a un arreglo más allá de su límite
//Se usa: panic!
//
//Error recuperable - ejemplo: abrir un archivo con el path incorrecto
//Result<T, E>

fn main(){
    let archivo = File::open("/file/file");
    match archivo {
        Ok(file) => leer_archivo(file),
        //Err(error) => println!("Tipo de error: {:?}", error.kind()), 
        Err(error) => match error.kind(){
            ErrorKind::NotFound => println!("Archivo no encontrado"),
            other_error => println!("Error desconocido"),
        }
    }

    let archivo2 = File::open("/file2/file2").expect("El archivo no existe");
    //Con esto creo que la ejecución se queda pegada
}

fn leer_archivo(file: File){
}
