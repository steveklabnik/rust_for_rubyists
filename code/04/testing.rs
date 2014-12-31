#[test] 
fn this_tests_code() { 
    panic!("Fail!");
}

#[test]
fn this_tests_more_code() {
    let (x,y) = (1i, 2i);
    assert!(x == y);
}
