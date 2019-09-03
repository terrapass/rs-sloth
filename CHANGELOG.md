# `sloth` changelog

## 0.2.0 (2019-09-03)
* Added `Deref<Target = T>`, `DerefMut`, `AsRef<T>`, `AsMut<T>`, `Borrow<T>`,
`BorrowMut<T>` implementations for `Lazy<T, Eval>`.
* Added `unwrap()` method, which consumes `Lazy<T, Eval>` and extracts the evaluated `T` value.
* Deprecated `value_ref()` and `value_mut()` methods in favour of dereference operator `*`, `as_ref()` and `as_mut()`
methods.
* Updated tests and documentation to reflect changes for 0.2.0 release.
* Added `CHANGELOG.md`.

## 0.1.3 (2019-09-01)
* Moved unit-tests and `Lazy` struct implementation to separate files from `lib.rs`.

## 0.1.2 (2019-09-01)
* Updated documentation.

## 0.1.1 (2019-09-01)
* Added Cargo metadata (repository, keywords, categories, badges) to `Cargo.toml`
as well as badges to `README`.

## 0.1.0 (2019-09-01)
* Added `Lazy<T, Eval>` struct, implementing lazily initialized values.
* Added `README.md` and `README.tpl` (for cargo readme) as well as description 
and license to `Cargo.toml`.
