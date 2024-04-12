pub fn longitud_de_cadenas(cad: &[String;3])->[i32;3]{
    let mut long:[i32;3]=[0,0,0];
    for i in 0..cad.len(){
        long[i]=cad[i].len() as i32;
    }
    long
}

#[test]
fn test_long(){
    let ar1:[String;3]=["hola".to_string(),"como".to_string(),"andas".to_string()];
    let long:[i32;3]=longitud_de_cadenas(&ar1);
    assert_eq!(4,long[0]);
}