use super::*;

//
// Tests
//

#[test]
fn lazy_int_value_retrieval() {
    let mut lazy_value = Lazy::new(|| 5 + 5);

    assert_eq!(lazy_value.value(), 10);
    assert_eq!(*lazy_value.value_ref(), 10);
    assert_eq!(*lazy_value.value_mut(), 10);
}

#[test]
fn lazy_int_value_modification() {
    let mut lazy_value = Lazy::new(|| -1);

    *lazy_value.value_mut() = 42;

    assert_eq!(lazy_value.value(), 42);
    assert_eq!(*lazy_value.value_ref(), 42);
    assert_eq!(*lazy_value.value_mut(), 42);
}

#[test]
fn lazy_str_value_retrieval() {
    let mut lazy_value = Lazy::new(|| "some str");

    assert_eq!(lazy_value.value(), "some str");
    assert_eq!(*lazy_value.value_ref(), "some str");
    assert_eq!(*lazy_value.value_mut(), "some str");
}

#[test]
fn lazy_str_value_modification() {
    let mut lazy_value = Lazy::new(|| "initial str");

    *lazy_value.value_mut() = "new str";

    assert_eq!(lazy_value.value(), "new str");
    assert_eq!(*lazy_value.value_ref(), "new str");
    assert_eq!(*lazy_value.value_mut(), "new str");
}

#[test]
fn lazy_string_value_retrieval() {
    let mut lazy_value = Lazy::new(|| "some string".to_string());

    assert_eq!(*lazy_value.value_ref(), "some string".to_string());
    assert_eq!(*lazy_value.value_mut(), "some string".to_string());
}

#[test]
fn lazy_string_value_modification() {
    let mut lazy_value = Lazy::new(|| "initial string".to_string());

    *lazy_value.value_mut() = "new string".to_string();

    assert_eq!(*lazy_value.value_ref(), "new string".to_string());
    assert_eq!(*lazy_value.value_mut(), "new string".to_string());
}

#[test]
#[allow(unused_variables)]
fn lazy_evaluator_never_called_if_unused() {
    let mut evaluator_call_count = 0;

    let lazy_value = Lazy::new(|| {
        evaluator_call_count += 1;
        25
    });

    assert_eq!(evaluator_call_count, 0);
}

#[test]
fn lazy_evaluator_called_once() {
    let mut evaluator_call_count = 0;

    let mut lazy_value = Lazy::new(|| {
        evaluator_call_count += 1;
        150
    });

    lazy_value.value();
    lazy_value.value();
    *lazy_value.value_mut() = 200;
    lazy_value.value();

    assert_eq!(evaluator_call_count, 1);
}