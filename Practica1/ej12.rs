fn main(){
    let tupla=("hola".to_string(),[1,2,3]);
    

    let mut suma=0;
    for i in tupla.1{
        suma+=i;
    }

    println!("la cadena de la tupla es:{} y la suma de los elemetnos del arreglo es:{}",tupla.0,suma);
}