use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config{
    pub fn new(args: &[String]) -> Config{
        let filename = args[1].clone();
        let query = args[2].clone();

        Config {filename, query}
    }
}

pub fn run (config: Config){
    let content = fs::read_to_string(config.filename). expect("No se puede leer el archivo");
        //println!("{}", &content[..100]);
        //Lee los primeros 100 caracteres
        let found = search(&content, &config.query);
        //Recuerda que query es una parte config, que es cuando instanciamos la struct Config

        println!("\nBuscando...\n");
        for element in found{
            println!("{}", element);
        }
}

fn search<'a>(content: &'a str, query: &str) -> Vec<&'a str>{
//Declaro el lifetime: <'lifetime>
    
    let mut resultado = Vec::new();
    
    for line in content.lines(){
        if line.contains(query){
            resultado.push(line);
        }
    }
    return resultado;
}