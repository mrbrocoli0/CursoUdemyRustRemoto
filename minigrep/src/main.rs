use std::env;
use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    //Te da la dirección de este archivo, además cuando ejecutas en cargo te da el nombre del
    //archivo y la query, osea los datos que instrodujiste, el comando es:
    //cargo run archivo.txt string_a_buscar

    let config = Config::new(&args);

    println!("Archivo: {}", config.filename);
    println!("Buscar: {}", config.query);

    minigrep::run(config);

}
