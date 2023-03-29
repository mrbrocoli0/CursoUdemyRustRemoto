fn main(){

    macro_rules! tres{
        () => {
            2 + 1
        };
    }

    macro_rules! suma{
        ($x: expr) => {
            $x + 1
        };
    }

    macro_rules! mi_vector{
        ($($x: expr), *) => {
        //($x: expr, *) => {
            //Instrucciones
            { // Esta es una función sin nombre
                let mut vec = Vec::new();
                $(
                    vec.push($x);
                )*
                vec
                //Aquí devuelve vec
            }
        };
    }

    macro_rules! nueva_funcion{
        ($name: ident) => {
            //Macro que crea función con el nombre que yo le pase
            fn $name() {
                println!("Hola soy la función: {:?}()", stringify!($name));
            }
        };
    } 

    println!("Hola");
    println!("Macro: {}", tres!());
    println!("Macro: {}", suma!(200));

    let vector = mi_vector!(1,2,3);
    println!("Mi vector: {:?}", vector);
    nueva_funcion!(saludar);
    saludar();
}
