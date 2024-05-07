use spin_test_sdk::{
    bindings::{
        fermyon::spin_test_virt::{http_handler, key_value},
        wasi::http,
    },
    spin_test,
};

#[spin_test]
fn first_test() {
    let a = 12;
    assert_eq!(12, a);
}
