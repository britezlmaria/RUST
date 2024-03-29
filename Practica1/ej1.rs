use std::io;
fn main(){
    let x=5.0;
    let mut data=String::new();

    println!("ingrese un numero:");
    io::stdin().read_line(&mut data).expect("error");

    let y:f32=data.trim().parse().expect("no es un numero decimal");

    println!("{} + {} = {}",x,y,x+y);
    println!("{} * {} = {} ",x,y,x*y);
    println!("{} / {} = {} ",x,y,x/y);
    println!("{} - {} = {} ",x,y,x-y);
}