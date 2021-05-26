extern crate nvtx_rs;

#[test]
fn test_thread_range() {
    let expected: i32 = nvtx_rs::range_push("Test");
    let actual: i32 = nvtx_rs::range_pop();
    assert!(
        expected == actual,
        "Actual: {}, Expected: {}",
        actual,
        expected
    );
}
