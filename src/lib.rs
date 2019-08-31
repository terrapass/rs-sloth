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
// LazyEval<T, Eval>
//

pub struct LazyEval<T, Eval>
    where Eval: FnOnce() -> T
{
    evaluator_cell: Cell<Option<Eval>>,
    value_cell:     RefCell<Option<T>>
}

impl<T, Eval> LazyEval<T, Eval>
    where Eval: FnOnce() -> T
{
    //
    // Interface
    //

    pub fn from(evaluator: Eval) -> Self {
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

    #[allow(dead_code)]
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

impl<T, Eval> LazyEval<T, Eval>
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
    fn lazy_eval_int_value_retrieval() {
        let mut lazy_value = LazyEval::from(|| 5 + 5);

        assert_eq!(lazy_value.value(), 10);
        assert_eq!(*lazy_value.value_ref(), 10);
        assert_eq!(*lazy_value.value_mut(), 10);
    }

    #[test]
    fn lazy_eval_int_value_modification() {
        let mut lazy_value = LazyEval::from(|| -1);

        *lazy_value.value_mut() = 42;

        assert_eq!(lazy_value.value(), 42);
        assert_eq!(*lazy_value.value_ref(), 42);
        assert_eq!(*lazy_value.value_mut(), 42);
    }

    #[test]
    fn lazy_eval_str_value_retrieval() {
        let mut lazy_value = LazyEval::from(|| "some str");

        assert_eq!(lazy_value.value(), "some str");
        assert_eq!(*lazy_value.value_ref(), "some str");
        assert_eq!(*lazy_value.value_mut(), "some str");
    }

    #[test]
    fn lazy_eval_str_value_modification() {
        let mut lazy_value = LazyEval::from(|| "initial str");

        *lazy_value.value_mut() = "new str";

        assert_eq!(lazy_value.value(), "new str");
        assert_eq!(*lazy_value.value_ref(), "new str");
        assert_eq!(*lazy_value.value_mut(), "new str");
    }

    #[test]
    fn lazy_eval_string_value_retrieval() {
        let mut lazy_value = LazyEval::from(|| "some string".to_string());

        assert_eq!(*lazy_value.value_ref(), "some string".to_string());
        assert_eq!(*lazy_value.value_mut(), "some string".to_string());
    }

    #[test]
    fn lazy_eval_string_value_modification() {
        let mut lazy_value = LazyEval::from(|| "initial string".to_string());

        *lazy_value.value_mut() = "new string".to_string();

        assert_eq!(*lazy_value.value_ref(), "new string".to_string());
        assert_eq!(*lazy_value.value_mut(), "new string".to_string());
    }

    #[test]
    fn lazy_eval_evaluator_called_once() {
        let mut evaluator_call_count = 0;

        let mut lazy_value = LazyEval::from(|| {
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
