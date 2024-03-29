fn main(){
    let arreglo=[1,2,3,4,5];

    let mut suma=0;
    for i in arreglo {
        suma+=arreglo[i];
    }

    println!("la suma de los elemntos del arreglo es:{}",suma);
}