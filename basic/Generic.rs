#[allow(unused_variables)] //Para que no alerte por variables no usadas

fn main(){

    //Generics

    let pointA = Point{
        x: 0.6,
        y: 9.1,
    };

    println!("{}, {}",pointA.x, pointA.y);

    let pointB = Point{
        x: 2,
        y: 5.3,
    };

    println!("{}, {}",pointB.x, pointB.y);

    let pointC = Point{
        x: "Valeria".to_string(),
        y: "Bruno".to_string(),
    };

    println!("{}, {}",pointC.x, pointC.y);
}

struct Point<T, V>{

    x:T,
    y:V,
    //Si pones sólo T en Point (Point<T>), x & y deben de ser del mismo tipo, no importa cuál
}
