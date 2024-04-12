pub fn sumar_arreglos(a1:[f32;3],a2:[f32;3])->[f32;3]{
    let mut a3:[f32;3]=[0.0,0.0,0.0];
    for i in 0..a1.len(){
        a3[i]=a1[i]+a2[i];
    }
    a3
}

#[test]
fn test_sumararreglos(){
    let a1:[f32;3]=[1.0,2.0,3.0];
    let a2:[f32;3]=[3.0,2.0,1.0];
    let a3:[f32;3]=sumar_arreglos(a1,a2);
    assert_eq!(4.0,a3[0]);
}