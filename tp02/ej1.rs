pub fn es_par(num: i32) ->bool{
     num%2 == 0
}

#[test]

fn test_espar(){
    let j=10;
    assert!(es_par(j));

}
