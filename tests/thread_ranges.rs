use nvtx::{range_pop, range_push};

#[test]
fn test_thread_range() {
    let expected: i32 = range_push!("Test with {text}", text = "hi");
    let actual: i32 = range_pop!();
    assert_eq!(expected, actual);
}

#[test]
fn test_nested_range() {
    let _: i32 = range_push!("");
    let expected = range_push!("Test with {text}", text = "hi");
    let actual: i32 = range_pop!();
    let _: i32 = range_pop!();
    assert_eq!(expected, actual);
}
