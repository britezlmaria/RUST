pub fn incrementar(num: &mut f32){
    *num+=1.0;
}

#[test]
fn test_incrementar(){
    let mut num=5.0;
    incrementar(&mut num);
    assert_eq!(6.0,num);
}