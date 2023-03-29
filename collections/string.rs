#[allow(unused_variables)]

fn main(){
    /*
    Collections
    
    String y String slice = colleción de caracteres, en u8´s (u es de unsigned)

    slice = referencia a una contigua secuencia de elementos en un collection

    String se guarda en el heap
    String slice: stack, referencia al heap, string literal guardado en el código binario

    */
    let nombre_String = String::from("Valeria");
    let nombre_String_slice = "Julio";

    let nombre_conv = nombre_String_slice.to_string(); //Convierte de string slice a String
    
    let  nombre_con2 = &nombre_String[..nombre_String.len()]; //Si nombre_String no es mutable ya no se puede utilizar porque ya se está referenciando a nombre_con2 en esta línea
}
