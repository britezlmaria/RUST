use std::io;
fn main(){

    const CAD: &str= "probando";
    let mut data=String::new();

    println!("ingrese un caracter:");
    io::stdin().read_line(&mut data).expect("error");

    let c:char=data.trim().parse().expect("no es un numero ");

    let mut cant=0;
    for i in CAD.chars(){
        if i==c{
            cant+=1;
        }
    }

    println!("la cantidad de veces que se repite el caracter{} en la cadena es:{}",c,cant);
}