pub fn reemplazar_pares(numbers :&mut[i32;3]){
    for i in 0..numbers.len(){
        if numbers[i]%2 ==0{
            numbers[i]=-1;
        }
    }
}

#[test]
fn testreemp(){
    let mut arreglo:[i32;3]=[5,2,3];
    reemplazar_pares(&mut arreglo);
    assert_eq!(-1,arreglo[1]);
}