pub fn duplicar_valores(arreglo:[f32;5])->[f32;5]{
    let mut a2:[f32;5]=[0.0,0.0,0.0,0.0,0.0];
    for i in 0..arreglo.len(){
        a2[i]=arreglo[i]*2.0;
    }
    a2

}

#[test]
fn test_dup(){
    let a1:[f32;5]=[1.0,2.0,3.0,4.0,5.0];
    let a2= duplicar_valores(a1);
    assert_eq!(2.0,a2[0]);
    assert_eq!(4.0,a2[1]);
}