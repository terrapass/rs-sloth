use std::cell::{
    Cell,
    RefCell,
    Ref,
    RefMut
};
use std::ops::{
    Deref,
    DerefMut
};
use std::borrow::{
    Borrow,
    BorrowMut
};

//
// Constants
//

const EXPECT_VALUE_CELL_INITIALIZED:  &str = "option in value_cell must be initialized at this point";
const EXPECT_EVALUATOR_STILL_PRESENT: &str = "evaluator must still be present at this point";
const EXPECT_VALUE_CELL_PTR_NOT_NULL: &str = "value_cell as ptr must not be null";

//
// Interface
//

//
// struct Lazy<T, Eval>: Deref<Target = T> + DerefMut + AsRef<T> + Borrow<T> + BorrowMut<T>
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

//
// Trait impls
//

impl<T, Eval> Deref for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    type Target = T;

    fn deref(&self) -> &T {
        self.init_once();

        self.as_ref_impl()
    }
}

impl<T, Eval> DerefMut for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    fn deref_mut(&mut self) -> &mut T {
        self.init_once();

        self.as_mut_impl()
    }
}

impl<T, Eval> AsRef<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    fn as_ref(&self) -> &T {
        self.init_once();

        self.as_ref_impl()
    }
}

impl<T, Eval> AsMut<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    fn as_mut(&mut self) -> &mut T {
        self.init_once();

        self.as_mut_impl()
    }
}

impl<T, Eval> Borrow<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    fn borrow(&self) -> &T {
        self.init_once();

        self.as_ref_impl()
    }
}

impl<T, Eval> BorrowMut<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    fn borrow_mut(&mut self) -> &mut T {
        self.init_once();

        self.as_mut_impl()
    }
}

//
// Methods
//

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
    #[must_use]
    pub fn value_ref(&self) -> Ref<'_, T> {
        self.init_once();

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
    #[must_use]
    pub fn value_mut(&mut self) -> RefMut<'_, T> {
        self.init_once();

        // Returns a RefMut to the T instance contained within Option<T> referenced by value_cell_mut
        RefMut::map(
            self.value_cell.borrow_mut(),
            |value_option| {
                value_option.as_mut().expect(EXPECT_VALUE_CELL_INITIALIZED)
            }
        )
    }

    /// Consumes `Lazy` instance and extracts the evaluation result value.
    ///
    /// This will invoke evaluator function if none of the `value`* methods
    /// were called earlier.
    #[must_use]
    pub fn unwrap(self) -> T {
        self.init_once();

        self.value_cell.replace(None).expect(EXPECT_VALUE_CELL_INITIALIZED)
    }

    //
    // Service
    //

    fn as_ref_impl(&self) -> &T {
        unsafe {
            self.value_cell
                .as_ptr()
                .as_ref()
                .expect(EXPECT_VALUE_CELL_PTR_NOT_NULL)
                .as_ref()
                .expect(EXPECT_VALUE_CELL_INITIALIZED)
        }
    }

    fn as_mut_impl(&mut self) -> &mut T {
        unsafe {
            self.value_cell
                .as_ptr()
                .as_mut()
                .expect(EXPECT_VALUE_CELL_PTR_NOT_NULL)
                .as_mut()
                .expect(EXPECT_VALUE_CELL_INITIALIZED)
        }
    }

    fn init_once(&self) {
        if self.value_cell.borrow().is_none() {
            *self.value_cell.borrow_mut() = Some(self.evaluate());
        }
    }

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
    #[must_use]
    pub fn value(&self) -> T {
        self.init_once();

        self.value_cell.borrow().expect(EXPECT_VALUE_CELL_INITIALIZED)
    }
}
