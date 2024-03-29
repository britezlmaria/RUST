use std::io;
fn main(){
    let arreglo=["uno","dos","tres","cuatro","cinco"];

    let mut data= String::new();
    println!("ingrese una cadena:");
    io::stdin().read_line(&mut data).expect("error");

    let cadena=data.trim().to_string();

    for cad in arreglo{
        if cad==cadena{
            println!("cadena encontrada");
            break;
        }
    }
    
}