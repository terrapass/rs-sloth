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

/// Represents a value of some type `T`, lazily evaluated using a parameterless
/// function or a closure (`FnOnce() -> T`) passed to [`Lazy::new()`](struct.Lazy.html#method.new).
/// 
/// The evaluated value may be referenced using [`value_ref()`](struct.Lazy.html#method.value_ref)
/// or [`value_mut()`](struct.Lazy.html#method.value_mut) methods.
/// For types implementing `Copy`, a copy of the contained value may be obtained
/// using [`value()`](struct.Lazy.html#method.value).
/// 
/// The evaluator function will not be called more than once.
/// If none of [`value()`](struct.Lazy.html#method.value),
/// [`value_ref()`](struct.Lazy.html#method.value_ref)
/// and [`value_mut()`](struct.Lazy.html#method.value_mut) methods are used,
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

    /// Constructs a lazy `T` instance, whose value, if needed, will later be
    /// obtained from `evaluator` and cached.
    /// 
    /// `evaluator` will be invoked only the first time any one of
    /// [`value()`](struct.Lazy.html#method.value),
    /// [`value_ref()`](struct.Lazy.html#method.value_ref)
    /// or [`value_mut()`](struct.Lazy.html#method.value_mut)
    /// methods is called.
    pub fn new(evaluator: Eval) -> Self {
        Self{
            evaluator_cell: Cell::new(Some(evaluator)),
            value_cell:     RefCell::new(None)
        }
    }

    /// Immutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the `value`* methods
    /// were called earlier.
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

    /// Mutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the `value`* methods
    /// were called earlier.
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
    /// Returns a copy of the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the `value`* methods
    /// were called earlier.
    pub fn value(&self) -> T {
        if self.value_cell.borrow().is_none() {
            *self.value_cell.borrow_mut() = Some(self.evaluate());
        }

        self.value_cell.borrow().expect(EXPECT_VALUE_CELL_INITIALIZED)
    }
}
