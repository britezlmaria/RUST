use std::io;
fn main(){
    let num1:u8=2;

    let mut data=String::new();

    println!("ingrese un numero:");
    io::stdin().read_line(&mut data).expect("error");

    let num2:u8=data.trim().parse().expect("no es un numero ");

    let suma=num1 + num2;

    println!("({} + {})² = {}",num1,num2,suma*suma);
}