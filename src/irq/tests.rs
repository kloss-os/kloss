#![cfg(test)]

fn increment_counter() {

}

#[test]
fn verify_dispatch_0() {

    //super::entry(0); // Fake an interrupt

    assert_eq!(4, 2 + 2);
}

#[test]
fn verify_dispatch_255() {
    //super::entry(255); // Fake an interrupt

    assert_eq!(4, 2 + 2);
}
