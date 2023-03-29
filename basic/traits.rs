fn main(){
    //Trait =  rasgo

    let julio = Humano;
    println!("{}", julio.di_hola());
    println!("{}", Humano::idioma());

    let pelusa = Gato;
    println!("{}", pelusa.di_hola());
    println!("{}", Gato::idioma());

}
struct Humano;
struct Gato;

//El formato de trait es cammel case
trait Hablar{
    fn di_hola(&self) -> String;
    fn idioma() -> String{
        "No tengo idioma".to_string()
    }
}

impl Hablar for Humano{
    fn di_hola(&self) -> String {
        "Hola amigos".to_string()
    }
    fn idioma() -> String{
        "Hablo humano".to_string()
    }
}

impl Hablar for Gato{
    fn di_hola(&self) -> String {
        "Miau".to_string()
    }
}
