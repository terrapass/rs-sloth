//! This crate provides a generic `Lazy<T, Eval>` wrapper struct for lazy initialization.
//! It can be used for expensive-to-calculate `T` values to ensure that the evaluation logic runs
//! only once and only if needed.
//! 
//! For example:
//! ```
//! use sloth::Lazy;
//! 
//! fn get_expensive_string() -> String {
//!     // do something expensive here to obtain the result,
//!     // such as read and process file contents
//!
//!     String::from("some expensive string we got from a file or something")
//! }
//! 
//! fn get_expensive_i32() -> i32 {
//!     // do something expensive here to calculate the result,
//!     // such as build a supercomputer and wait 7.5 million years
//!
//!     42
//! }
//! 
//! let lazy_string = Lazy::new(get_expensive_string);
//! let lazy_i32 = Lazy::new(get_expensive_i32);
//! 
//! //...
//! 
//! let must_use_string = true;
//! 
//! //...
//! 
//! if must_use_string {
//!     println!("Expensive string is: {}", *lazy_string.value_ref());
//!     println!("It has length: {}", (*lazy_string.value_ref()).len());
//!
//!     // get_expensive_string() has been called only once,
//!     // get_expensive_i32() has not been called at all
//! } else {
//!     println!("Expensive int is: {}", lazy_i32.value());
//!     println!("It is{} divisible by 6", if lazy_i32.value() % 6 == 0 { "" } else { " not" });
//! 
//!     // get_expensive_string() has not been called,
//!     // get_expensive_i32() has been called only once
//! }
//! 
//! ```

use std::cell::{
    Cell,
    RefCell,
    Ref,
    RefMut
};

//
// Constants
//

const EXPECT_VALUE_CELL_INITIALIZED:  &str = "option in value_cell must be initialized at this point";
const EXPECT_EVALUATOR_STILL_PRESENT: &str = "evaluator must still be present at this point";

//
// Interface
//

//
// Lazy<T, Eval>
//

/// Represents a value of type `T`, lazily evaluated using a parameterless
/// function or a closure (`FnOnce() -> T`) passed to `Lazy::new()`.
/// 
/// The value within may be referenced using `value_ref()` or `value_mut()` methods.
/// For types implementing Copy, a copy of the contained value may be obtained using `value()`.
/// 
/// The evaluator function will not be called more than once.
/// If none of `value()`, `value_ref()` and `value_mut()` methods are used,
/// the evaluator function will never be called at all.
/// 
/// # Examples
/// Lazily converting a string to upper case:
/// ```
/// use sloth::Lazy;
/// 
/// let some_str = "the quick brown fox jumps over the lazy dog";
/// let lazy_upper_str = Lazy::new(|| some_str.to_uppercase());
/// 
/// assert_eq!(
///     *lazy_upper_str.value_ref(),
///     "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"
/// );
/// ```
/// Regardless of how many times the value is accessed, the evaluator function
/// is only called once:
/// ```
/// use sloth::Lazy;
/// 
/// let mut evaluator_called_times = 0;
/// 
/// let lazy_value = Lazy::new(|| {
///     evaluator_called_times += 1;
///     25
/// });
/// 
/// assert_eq!(lazy_value.value(), 25);
/// 
/// let another_value = lazy_value.value() + lazy_value.value();
/// 
/// assert_eq!(evaluator_called_times, 1);
/// ```
pub struct Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    evaluator_cell: Cell<Option<Eval>>,
    value_cell:     RefCell<Option<T>>
}

impl<T, Eval> Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    //
    // Interface
    //

    pub fn new(evaluator: Eval) -> Self {
        Self{
            evaluator_cell: Cell::new(Some(evaluator)),
            value_cell:     RefCell::new(None)
        }
    }

    pub fn value_ref(&self) -> Ref<'_, T> {
        if self.value_cell.borrow().is_none() {
            *self.value_cell.borrow_mut() = Some(self.evaluate());
        }

        // Returns a Ref to the T instance contained within Option<T> referenced by value_cell
        Ref::map(
            self.value_cell.borrow(),
            |value_option| {
                value_option.as_ref().expect(EXPECT_VALUE_CELL_INITIALIZED)
            }
        )
    }

    pub fn value_mut(&mut self) -> RefMut<'_, T> {
        let mut value_cell_mut = self.value_cell.borrow_mut();

        if value_cell_mut.is_none() {
            *value_cell_mut = Some(self.evaluate());
        }

        // Returns a RefMut to the T instance contained within Option<T> referenced by value_cell_mut
        RefMut::map(
            value_cell_mut,
            |value_option| {
                value_option.as_mut().expect(EXPECT_VALUE_CELL_INITIALIZED)
            }
        )
    }

    //
    // Service
    //

    fn evaluate(&self) -> T {
        let evaluator = self.evaluator_cell
            .take()
            .expect(EXPECT_EVALUATOR_STILL_PRESENT);

        evaluator()
    }
}

impl<T, Eval> Lazy<T, Eval>
    where T:    Copy,
          Eval: FnOnce() -> T
{
    #[allow(dead_code)]
    pub fn value(&self) -> T {
        if self.value_cell.borrow().is_none() {
            *self.value_cell.borrow_mut() = Some(self.evaluate());
        }

        self.value_cell.borrow().expect(EXPECT_VALUE_CELL_INITIALIZED)
    }
}

//
// Unit tests
//

#[cfg(test)]
mod tests {
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
}
