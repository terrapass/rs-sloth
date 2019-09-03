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

/// Contains a value of some type `T`, lazily evaluated using a parameterless
/// function or a closure (`FnOnce() -> T`) passed to [`Lazy::new()`](struct.Lazy.html#method.new).
/// 
/// This type provides pointer-like interface to the evaluation result,
/// implementing [`Deref`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html),
/// [`AsRef`](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html),
/// [`Borrow`](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html)
/// and their `Mut` counterparts.
/// 
/// The result will be evaluated the first time it is accessed via any of `Lazy`'s methods.
/// 
/// # Accessing evaluated value
/// 
/// A `Lazy` value can be acessed in any of the following ways:
/// ```
/// use sloth::Lazy;
/// 
/// let mut lazy_value = Lazy::new(|| 2019);
/// 
/// let value: i32 = *lazy_value;
/// let value_ref: &i32 = lazy_value.as_ref();
/// let value_mut: &mut i32 = lazy_value.as_mut();
/// 
/// let value_copy: i32 = lazy_value.value(); // only for types implementing Copy
/// 
/// let value: i32 = lazy_value.unwrap(); // consumes lazy_value
/// ```
/// 
/// Due to [`Deref` coercion](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion)
/// `T`'s methods may be called directly on [`Lazy<T, Eval>`](struct.Lazy.html), while references to `Lazy`
/// are coerced to references to `T`:
/// ```
/// use sloth::Lazy;
/// 
/// fn print_string_len(string: &String) {
///     println!("{} has length {}", string, string.len());
/// }
/// 
/// let mut lazy_string = Lazy::new(|| String::from("lorem "));
/// 
/// lazy_string.push_str("ipsum"); // can call T's methods
/// 
/// print_string_len(&lazy_string); // can pass as &T function param
///
/// ```
/// 
/// # Laziness
/// 
/// The evaluator function will not be called more than once,
/// regardless of how many times the value is accessed:
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
/// assert_eq!(*lazy_value, 25);
/// 
/// let another_value = *lazy_value + *lazy_value;
/// 
/// assert_eq!(evaluator_called_times, 1);
/// ```
/// 
/// If a `Lazy` value is never dereferenced and none of its methods are called,
/// its evaluator function will not be invoked at all.
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

    /// Immutable dereference, allowing access to the contained value.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    fn deref(&self) -> &T {
        self.as_ref_impl()
    }
}

impl<T, Eval> DerefMut for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    /// Mutable dereference, allowing access to the contained value.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut_impl()
    }
}

impl<T, Eval> AsRef<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    /// Immutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    fn as_ref(&self) -> &T {
        self.as_ref_impl()
    }
}

impl<T, Eval> AsMut<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    /// Mutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    fn as_mut(&mut self) -> &mut T {
        self.as_mut_impl()
    }
}

impl<T, Eval> Borrow<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    /// Immutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    fn borrow(&self) -> &T {
        self.as_ref_impl()
    }
}

impl<T, Eval> BorrowMut<T> for Lazy<T, Eval>
    where Eval: FnOnce() -> T
{
    /// Mutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    fn borrow_mut(&mut self) -> &mut T {
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
    /// `evaluator` will be invoked only the first time this instance
    /// is dereferenced or one of its methods is invoked.
    pub fn new(evaluator: Eval) -> Self {
        Self{
            evaluator_cell: Cell::new(Some(evaluator)),
            value_cell:     RefCell::new(None)
        }
    }

    /// Immutably borrows the evaluation result.
    /// 
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    /// 
    /// **`value_ref()` will be removed in sloth 0.3.0. [`as_ref()` or immutable * dereference](struct.Lazy.html#implementations) should be used instead.**
    #[must_use]
    #[deprecated(since = "0.2.0", note = "will be removed in sloth 0.3.0; please use as_ref() or * deref operator instead")]
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
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    /// 
    /// **`value_mut()` will be removed in sloth 0.3.0. [`as_mut()` or mutable * dereference](struct.Lazy.html#implementations) should be used instead.**
    #[must_use]
    #[deprecated(since = "0.2.0", note = "will be removed in sloth 0.3.0; please use as_mut() or * deref operator instead")]
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

    /// Consumes this [`Lazy<T, Eval>`](struct.Lazy.html) instance and extracts the evaluation result value.
    ///
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    #[must_use]
    pub fn unwrap(self) -> T {
        self.init_once();

        self.value_cell.replace(None).expect(EXPECT_VALUE_CELL_INITIALIZED)
    }

    //
    // Service
    //

    fn as_ref_impl(&self) -> &T {
        self.init_once();

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
        self.init_once();

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
    /// This will invoke evaluator function if none of the methods
    /// or `*` deref operator were previously used.
    #[must_use]
    pub fn value(&self) -> T {
        self.init_once();

        self.value_cell.borrow().expect(EXPECT_VALUE_CELL_INITIALIZED)
    }
}
