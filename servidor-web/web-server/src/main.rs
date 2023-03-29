use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs; //Biblioteca File System

fn main() {
    // Iniciar el servidor

    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(&address).unwrap();
    println!("Servidor iniciado en {}", &address);

    // Escuchar por conexiones

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      handle_connection(stream);
    }
}

// Manejar conexiones

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  
  stream.read(&mut buffer).unwrap();
  //Es &mut porque en el método read está implementado así, y al querer replicarlo te solicita que también sea así

  println!("Stream recibido!");
  println!("{}", String::from_utf8_lossy(&buffer[..]));
  //Esto muestra el detalle de HTML

  let get = b"GET / HTTP/1.1";//127.0.0.1:8000/index
  //Aquí hace algo con la dirección pero no lo entendí

  if buffer.starts_with(get) {
    send_index(stream);
    //Si lo encuentra va a la función a leer el archivo
  } else {
    send_not_found(stream);
    //Si no lo encuentra va a la función e imprime el error
  }
}

fn build_response(content: String) -> String {
  format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
    content.len(),
    content
  )
}

//Responder al cliente

fn send_index(mut stream: TcpStream) {
  let contents = fs::read_to_string("index.html").unwrap();
  //Lee el archivo HTML
  stream.write(build_response(contents).as_bytes()).unwrap();
  stream.flush().unwrap();
  //Limpia la salida estándar
}

fn send_not_found(mut stream: TcpStream) {
  let contents = fs::read_to_string("404.html").unwrap();
  //Lee el archivo HTML
  stream.write(build_response(contents).as_bytes()).unwrap();
  stream.flush().unwrap();
    //Limpia la salida estándar
}
