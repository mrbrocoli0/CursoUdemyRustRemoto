fn main() {
    println!("Hello, world!");
    let precio = 3000;

    //println!("{}", format!("${}", precio)); //Esto manda warning con clippy
    println!("${}", precio);

    if precio > 1000 {
        //Algo
    } else if precio > 0{
        //Otra cosa
    }
    //Con esyo clippy marca error, pero si compila, clippy es para que escribas mejor código
}
