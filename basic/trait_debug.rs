#[derive(Debug)]

struct User {
  nombre: String,
  edad: i32,
}

fn main() {
  let usuario = User {
    nombre: "Julio Andres".to_string(),
    edad: 20,
  };

  //println!("{}", usuario);
  println!("{:?}", usuario);
}
