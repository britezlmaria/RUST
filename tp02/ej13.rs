pub fn ordenar_nombres(names:&mut[String;3]){
    names.sort();
}

#[test]
fn test_ordenar(){
    let mut nombres=["felipe".to_string(),"vicente".to_string(),"carla".to_string()];
    ordenar_nombres(&mut nombres);

    assert_eq!("felipe",nombres[1]);
}