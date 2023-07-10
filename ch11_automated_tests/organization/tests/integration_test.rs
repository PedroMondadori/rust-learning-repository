use organization;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, organization::add(2, 2));
}
