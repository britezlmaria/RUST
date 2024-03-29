fn main(){
    const CONSTANTE:u8=2;

    let mut arreglo=[1,2,3,4,5,6];

    for i in 0..6 {
        arreglo[i]=arreglo[i]*CONSTANTE;
    }
    println!("{:?}",arreglo);
}