pub fn es_primo(num :i32)->bool{
    let mut cant=0;
    for i in 1..num+1{
        if num%i ==0{
            cant+=1;
        }
        if cant >2{
            break;
        }
    }
    cant<=2
}

#[test]
fn test_esprimo(){
    let data=7;
    assert!(es_primo(data))
}