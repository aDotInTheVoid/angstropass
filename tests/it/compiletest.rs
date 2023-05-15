#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    trybuild::TestCases::new().compile_fail("tests/ui/*.rs")
}
