pub fn cantidad_en_rango(arreglo:[i32;3],s:i32,inf:i32)->i32{
    let mut cant=0;
    for i in 0..arreglo.len(){
        if arreglo[i]>=inf && arreglo[i]<=s{
            cant+=1;
        }
    }
    cant
}

#[test]
fn test_cantenrango(){
    let arreglo:[i32;3]=[5,7,3];
    let d=cantidad_en_rango(arreglo,6,2);
    assert_eq!(2,d);
}