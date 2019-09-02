use super::*;

use std::borrow::{
    Borrow,
    BorrowMut
};

//
// Macros
//

macro_rules! lazy_all_ref_assert_eq {
    ($lazy_value:expr, $expected:expr) => {
            assert_eq!(*$lazy_value, $expected);
            assert_eq!(*$lazy_value.as_ref(), $expected);
            assert_eq!(*$lazy_value.as_mut(), $expected);
            assert_eq!(*$lazy_value.value_ref(), $expected);
            assert_eq!(*$lazy_value.value_mut(), $expected);

            assert!(is_borrow_eq(&$lazy_value, &$expected));
    };
}

//
// Tests
//

#[test]
fn lazy_int_value_retrieval() {
    let mut lazy_value = Lazy::new(|| 5 + 5);

    lazy_all_ref_assert_eq!(lazy_value, 10);
    assert_eq!(lazy_value.value(), 10);
}

#[test]
fn lazy_int_value_modification_value_mut() {
    let mut lazy_value = Lazy::new(|| -1);

    *lazy_value.value_mut() = 42;

    lazy_all_ref_assert_eq!(lazy_value, 42);
    assert_eq!(lazy_value.value(), 42);
}

#[test]
fn lazy_int_value_modification_deref_mut() {
    let mut lazy_value = Lazy::new(|| -1);

    *lazy_value = 42;

    lazy_all_ref_assert_eq!(lazy_value, 42);
    assert_eq!(lazy_value.value(), 42);
}

#[test]
fn lazy_str_value_retrieval() {
    let mut lazy_value = Lazy::new(|| "some str");

    lazy_all_ref_assert_eq!(lazy_value, "some str");
    assert_eq!(lazy_value.value(), "some str");
}

#[test]
fn lazy_str_value_modification_value_mut() {
    let mut lazy_value = Lazy::new(|| "initial str");

    *lazy_value.value_mut() = "new str";

    lazy_all_ref_assert_eq!(lazy_value, "new str");
    assert_eq!(lazy_value.value(), "new str");
}

#[test]
fn lazy_str_value_modification_deref_mut() {
    let mut lazy_value = Lazy::new(|| "initial str");

    *lazy_value = "new str";

    lazy_all_ref_assert_eq!(lazy_value, "new str");
    assert_eq!(lazy_value.value(), "new str");
}

#[test]
fn lazy_string_value_retrieval() {
    let mut lazy_value = Lazy::new(|| "some string".to_string());

    lazy_all_ref_assert_eq!(lazy_value, "some string".to_string());
}

#[test]
fn lazy_string_value_modification_value_mut() {
    let mut lazy_value = Lazy::new(|| "initial string".to_string());

    *lazy_value.value_mut() = "new string".to_string();

    lazy_all_ref_assert_eq!(lazy_value, "new string".to_string());
}

#[test]
fn lazy_string_value_modification_deref_mut() {
    let mut lazy_value = Lazy::new(|| "initial string".to_string());

    *lazy_value = "new string".to_string();

    lazy_all_ref_assert_eq!(lazy_value, "new string".to_string());
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
#[allow(unused_must_use)]
fn lazy_evaluator_called_once() {
    let mut evaluator_call_count = 0;

    let mut lazy_value = Lazy::new(|| {
        evaluator_call_count += 1;
        150
    });

    *lazy_value;
    lazy_value.as_ref();
    *lazy_value.as_mut() = 150;
    let _: i32 = *lazy_value.borrow();
    *lazy_value.borrow_mut() = 250;
    lazy_value.value();
    lazy_value.value();
    *lazy_value.value_mut() = 200;
    lazy_value.value();

    assert_eq!(evaluator_call_count, 1);
}

#[test]
#[allow(unused_must_use)]
fn lazy_value_drop_if_used() {
    let mut was_value_dropped = false;

    {
        let lazy_value = Lazy::new(|| SomethingDroppable{was_dropped: &mut was_value_dropped});

        lazy_value.value_ref();
    }

    assert!(was_value_dropped);
}

#[test]
#[allow(unused_variables)]
fn lazy_value_no_drop_if_unused() {
    let mut was_value_dropped = false;

    {
        let lazy_value = Lazy::new(|| SomethingDroppable{was_dropped: &mut was_value_dropped});
    }

    assert!(!was_value_dropped);
}

//
// Service
//

fn is_borrow_eq<T, Q>(borrowed: &Q, expected: &T) -> bool
    where   T: PartialEq,
            Q: Borrow<T>
{
    borrowed.borrow() == expected.borrow()
}

//
// Service types
//

struct SomethingDroppable<'a> {
    was_dropped: &'a mut bool
}

impl Drop for SomethingDroppable<'_> {
    fn drop(&mut self) {
        assert!(!*self.was_dropped, "was_dropped must initially be false");

        *self.was_dropped = true;
    }
}
