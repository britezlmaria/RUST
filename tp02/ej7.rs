pub fn cantidad_mayores(arreglo:[i32;3],limite:i32)->i32{
    let mut cant=0;
    for i in 0..arreglo.len(){
        if arreglo[i]>limite{
            cant+=1;
        }
    }
    cant
}

#[test]
fn test_mayores(){
    let arreglo:[i32;3]=[4,5,2];
    

    assert_eq!(2,cantidad_mayores(arreglo,2));
}