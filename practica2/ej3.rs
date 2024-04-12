pub fn suma_pares(arreglo: [i32;5])-> i32{
    let mut cant=0;
    for elemento in arreglo{
        if elemento%2 ==0{
            cant+=elemento;
        }
    }
    cant
}


#[test]

fn test_suma(){
    let arreglo:[i32;5]=[1,2,3,4,5];
    assert_eq!(6,suma_pares(arreglo));
}