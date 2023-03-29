use std::thread;
use std::time::Duration;
use std::sync::mpsc; //multiple producer single consumer
use std::char;

// Concurrency = Concurrencia
// Programación concurrente con Rust
// Paralelismo
// Channels: Enviando mensajes entre threads
fn main() {
  
  let (tx, rx) = mpsc::channel(); // tx = transmisor, rx = recepto
    //chanel es un método de mpsc, y nos devulve 2 valores, un transmisor y un receptor
  let tx2 = tx.clone();

  let nombre = String::from("Julio");
  println!("Hola {}", nombre);

  // Hebra 1
  thread::spawn( move || { 
    println!("{} se unió a la partida", nombre);
    //Se uso move, se cambió el ownership, ya no se puede utilizar tx en la principal
    
    for count in 0..3 {
      let mut mensaje = String::from("Contador: ");
      mensaje.push(char::from_digit(count, 10).unwrap());
      tx.send(mensaje).unwrap();
      //send (viene del método chanel) devuleve un valor tipo Result, hay que desenvolverlo
      thread::sleep(Duration::from_secs(2));
    }
  });

  // Hebra 2
  thread::spawn( move || { 
    for count in ['a','b','c','d'].iter() {
      let mut mensaje = String::from("Letra: ");
      mensaje.push(*count);
      tx2.send(mensaje).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  //recibir mensajes
  for mensaje_recibido in rx {
    println!("recibido: {}", mensaje_recibido)
  }
}
