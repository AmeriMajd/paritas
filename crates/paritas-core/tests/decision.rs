use paritas_core::Decision;

fn assert_copy<T: Copy>() {}
fn assert_debug<T: std::fmt::Debug>() {}
fn assert_equality<T: Eq>() {}

#[test]
fn decision_supports_value_semantics() {
    assert_copy::<Decision>();
    assert_debug::<Decision>();
    assert_equality::<Decision>();
}

#[test]
fn decisions_are_distinct() {
    assert_ne!(Decision::Allowed, Decision::Denied);
    assert_ne!(Decision::Denied, Decision::Unknown);
    assert_ne!(Decision::Unknown, Decision::Error);
}
