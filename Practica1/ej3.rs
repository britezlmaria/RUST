use std::io;
fn main(){
    let b1=false;

    let mut data=String::new();

    println!("ingrese un boolean:");
    io::stdin().read_line(&mut data).expect("error");

    let b2:bool=data.trim().parse().expect("no es un boolean");

    println!("{} AND {} = {} ",b1,b2,b1 & b2);
    println!("{} OR {} = {} ",b1,b2,b1 | b2);
}