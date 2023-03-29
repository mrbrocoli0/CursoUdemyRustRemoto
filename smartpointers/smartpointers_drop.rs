use std::ops::Deref;

// Smart pointers = punteros inteligentes
fn main() {
  // Drop trait: que hacer cuando la instancia sale del scope
  let x = 5;
  let y = MiCaja::new(x);
  //Aquí sólo implementa el struct de MiCaja
  drop(y);
  //Adios mundo cruel, se imprime aquí
  //Sólo puedes hacer drop de las variables que son de tipo MiCaja
  println!("hola");
}

struct MiCaja<T>(T);

impl<T> MiCaja<T> {
  fn new(x:T) -> MiCaja<T> {
    MiCaja(x)
  }
}

impl<T> Deref for MiCaja<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target { 
    &self.0
  }
}

impl<T> Drop for MiCaja<T> {
  fn drop(&mut self) { 
    println!("Adios mundo cruel!");
  }
}
