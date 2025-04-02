Haskell has a REALLY cool feature related to [[Constraint kinds thought dump|constraints]] called [Quantified Constraints](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/quantified_constraints.html), which in short allows for constraints like these:
```haskell
class (forall m. Monad m => Monad (t m)) => MonadTrans t where
  lift :: (Monad m) => m a -> t m a

``` 
which says that for any type-constructor `t :: (Type -> Type) -> (Type -> Type)`, the statement `MonadTrans t` holds if
- for any `m :: Type -> Type` s.t. `Monad m` holds, the statement `Monad (t m)` also holds
- the function `lift :: (Monad m) => m a -> t m a` is provided by the `MonadTrans t` instance 
Here is [a paper on it](https://homepages.inf.ed.ac.uk/wadler/papers/quantcc/quantcc.pdf), for more detail. 

If I were to compare it to a Rust feature, it would be [non-lifetime binders](https://github.com/rust-lang/rust/issues/108185) with bounds; here is an example of that:
```rust
// higher-kinded-types
struct One<T>(T); 
struct Two<T>(T); 
struct Three<T>(T);

// capture higher-kinded types with family pattern
trait Family { type Of<T>; }
struct OneFamily;
struct TwoFamily;
struct ThreeFamily;
impl Family for OneFamily { type Of<T> = One<T>; }
impl Family for TwoFamily { type Of<T> = Two<T>; } 
impl Family for ThreeFamily { type Of<T> = Three<T>; }

// some constraint, which is only met by SOME higher-kinded-type
trait Foo { fn foo(&self) { .. } }
impl<T> Foo for Two<T> {}
impl<T> Foo for Three<T> {}

// an item which works with higher-kinded containers
struct Wrapper<F: Family> { 
  int: F::Of<u32>, 
  float: F::Of<f64>,
}

impl<F: Family> Wrapper<F> {
  // Unquantified constraint-implementation
  // equivalent to `new :: (Family f) => () -> Wrapper f`
  fn new() -> Self {
    Self { int: F::Of::new(42), float: F::Of::new(4.2) }
  }

  // Quantified constraint implementation
  // equivalent to `doFoo :: (forall a. Sized a => Foo (f a)) => &(Wrapper f) -> ()`
  fn do_foo(&self)
  where
	  for<T: Sized> F::Of<T>: Foo,
  {
    self.int.foo();
    self.float.foo();
  }
}

```
