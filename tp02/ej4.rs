pub fn cantidad_impares(arreglo: [i32;5])-> i32{
    let mut cant=0;
    for elemento in arreglo{
        if elemento%2 !=0{
            cant+=elemento;
        }
    }
    cant
}


#[test]
fn test_impares(){
    let arreglo:[i32;5]=[1,2,3,4,5];
    assert_eq!(9,cantidad_impares(arreglo));
}