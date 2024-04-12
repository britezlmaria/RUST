pub fn cantidad_de_cadenas_mayor_a(strings:&[String;3],limite:i32)->i32{
    let mut cant=0;
    for i in strings{
        if i.len() as i32>limite{
            cant+=1;
        }
    }
    cant  

}


#[test]
fn testcantcadenasmayores(){
    let strings:[String;3]=["holi".to_string(),"como".to_string(),"va".to_string()];
    let count=cantidad_de_cadenas_mayor_a(&strings,3);
    assert_eq!(2,count);
}