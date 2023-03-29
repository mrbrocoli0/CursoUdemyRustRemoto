use std::thread;
use std::time::Duration;

// Concurrency = Concurrencia
// Programación concurrente con Rust
// Paralelismo
fn main() {
  
  let nombre = String::from("Julio");
  let nombre_clone = nombre.clone();
  //Esta variable es para poder obtener el valor de nombre (interno), porque se desasocia cuando le
  //pasamos el ownership a nombre (interno)

  println!("Hola {}", nombre);

  let join_handle = thread::spawn( move || { 
      //move cambia el ownership de la memoria por un rato a este scope de nombre (main) a nombre (interno)
    println!("{} se unió a la partida", nombre_clone);
    thread::sleep(Duration::from_millis(2000));
    //Se detiene/duerme 2 segundos
  });

  join_handle.join().unwrap();
  //Join devuelve un valor tipo Result, unwrap lo desenvuelve
  //El método join lo que hace es ESPERAR a que la hebra asociada TERMINE

  println!("¿Qué quieres hacer hoy {}?", nombre);
}
