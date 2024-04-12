pub fn multiplicar_valores(arreglo: &mut[i32;3],num:i32){
    for i in 0..arreglo.len(){
        arreglo[i]=arreglo[i]*num;
    }
    
}


#[test]
fn testmultval(){
    let mut arreglo:[i32;3]=[1,2,3];
    multiplicar_valores(&mut arreglo, 3);
    assert_eq!(3,arreglo[0]);
}