use clase_testing::basic;

#[test]
fn int_sumar_ok() {
    let result = basic::sumar(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn int_dividir_ok() {
    let result = basic::dividir(12, 2);
    assert_eq!(result, Ok(6));
}
