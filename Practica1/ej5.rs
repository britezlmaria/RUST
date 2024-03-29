use std::io;
fn main(){
    let mut cadena="hola".to_string();

    let mut data=String::new();

    println!("ingrese una cadena:");
    io::stdin().read_line(&mut data).expect("error");

    cadena= cadena +&data;

    println!("cadena:{} ",cadena.to_uppercase());

}