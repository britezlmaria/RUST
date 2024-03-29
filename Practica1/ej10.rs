fn main(){
    let arreglo1=[1,2,3,4,5];
    let arreglo2=[5,4,3,2,1];

    let mut resultado=[0,0,0,0,0];

    for i in 0..6{
        resultado[i]=arreglo1[i]+arreglo2[i];
    }

    println!("{:?}",resultado);
}