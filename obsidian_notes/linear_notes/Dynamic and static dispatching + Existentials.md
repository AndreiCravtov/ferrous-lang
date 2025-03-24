In Haskell (and other higher-level languages) some level of **dynamic-dispatching** or **runtime checks** are NECESSARY to allow some of the **"statically undecidable"** instances to still be used. For example, here is a Haskell program which uses polymorphic recursion and thus fails to find a compile-time instance
```haskell
nested :: Show a => Int -> a -> String
nested 0 x       = show x
nested n x | n>0 = nested (n-1) [x]

main = do
  n <- getLine
  putStrLn (nested (read n) 'a')

```
in Haskell this compiles _just fine_ but in Rust the equivalent code **reaches recursion limit while instantiating** and fails to compile, while explicitly opting-in to dynamic dispatch in Rust with trait-objects and the `dyn` keyword allows the code to finally compile.
```rust
// error: reached the recursion limit while instantiating `nested::<[[...; 1]; 1]>`
fn nested<T: Debug>(n: u32, t: T) -> String {  
    match n {  
        0 => format!("{t:?}"),  
        _ => nested(n - 1, [t]),  
    }  
}

// however this here dispatches just as expected, and works exactly like Haskell would  
fn nested(n: u32, t: Box<dyn Debug>) -> String {  
    match n {  
        0 => format!("{t:?}"),  
        _ => nested(n - 1, Box::new([t])),  
    }  
}

```
For my own language, the idea  of "trait objects" is a rather limiting form of dynamic-dispatching of typeclasses/traits. So perhaps instead I could have "opt-in" dynamic dispatch like this:
```my-lang
fn nested[T: dyn Debug](n: u32, t: T) -> String {  
    match n {  
        0 => format!("{t:?}"),  
        _ => nested(n - 1, [t]),
    }  
}

```
which could be read as generic `T` is bound by trait `Debug`, the implementation of which is permitted to be resolved at runtime (hence dynamic-dispatch.) So more generally, if my compiler **reaches recursion limit while instantiating** some kind of type or locating an instance for it, then it checks for the `dyn` keyword on that bound and if present it falls back to dynamic-dispatch, otherwise the recursion-limit is hit. This means dynamic-dispatch is always "opt-in" and explicit, but also tightly integrated with the type-system.

The only problem is, atleast in Rust's case, the recursion limit error isn't encountered until the function is _used_ meaning the recursion-limit is triggered at the _call site_, so I'd have to somehow detect this error BEFORE the callsite even becomes an issue, and report it to the user? The process of monomorphizing caused Rust to want to generate `nested::<[[...; 1]; 1]>` even-though at the callsite the "mono"morphized version of my function `nested(23, "something")` should ideally be equivalent to `nested::<&'static str>(23, "something")` where the generic-type-parameter is the concrete type `&'static str`.
I can explore the formalism of my hypothetical `T: dyn FooTrait` by comparing it to desugaring of typeclasses in Haskell:
```haskell
-- all this desugars into records (and functions thereof) 
-- being passed around as hidden arguments
class Show a where
  show :: a -> String

instance Show Char where
  show = ??

instance Show a => Show [a] where
  show = ??

nested :: Show a => Int -> a -> String 
nested 0 x = show x 
nested n x | n>0 = nested (n-1) [x] 

main = do 
  n <- getLine 
  putStrLn (nested (read n) 'a')

--- the desugared version looks like this
data Show a = Show { show :: a -> String }

$fShowChar :: Show Char
$fShowChar = Show { show = ?? }

$fShowList :: Show a -> Show [a]
$fShowList d = ??

nested :: Show a -> Int -> a -> String 
nested d 0 x = d.show x -- record dot-overloading syntax 
nested d n x | n>0 = nested ($fShowList d) (n-1) [x]

main = do 
  n <- getLine 
  putStrLn (nested $fShowChar (read n) 'a')

```
So then for constraints _not_ labeled `dyn` the default strategy would be full monomorphizing like in Rust (and hence recursion-limit errors) while `dyn` would default the strategy to runtime dynamic dispatch (by rewriting the functions to add the extra parameter) and that would avoid monomorphizing
```rust
// lets mirror the Haskell typeclass definition somewhat
trait Debug {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

impl Debug for &'_ str {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { ... }
}

impl<T, const N: usize> Debug for [T; N] {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { ... }
}

fn nested<T: dyn Debug>(n: u32, t: T) -> String {  
    match n {  
        0 => format!("{t:?}"),  
        _ => nested(n - 1, [t]),  
    }  
}

// now we try to create the "desugared" dynamic-dispatch parameters
// I don't know if these will always be generated and for `dyn` and non-`dyn` bounds alike
// and then simply aggressively inlined/optimized if the dynamic-dispatch is truly not needed?
// will have to decide later...
struct Debug<A: ?Sized> { // The first type parameter of typeclasses corresponds to `Self` of traits
  fmt: fn(&A, &mut Formatter<'_>) -> Result<(), Error>, // use function-pointer syntax, this is probably horribly inefficient though
}

const fn r#fDebugStr<'a>() -> Debug<&'a str> {  
  Debug { // this just references the current "fmt" implementation for now
    fmt: <&'a str>::fmt  
  }  
}

// not sure how to model the "function" constraint instances, should they just take references?  
const fn r#fDebugArray<T, const N: usize>(debug: &Debug<T>) -> &Debug<[T; N]> {  
    // not even sure what to do here.........  
    // if this requires calling non-const methods under the hood, then the entire function becomes non-constant :((    todo!()  
}

// convenience for modeling with Rust's current system
struct WithDebug<'a, T: ?Sized>(&'a DynDebug<T>, T);  
  
impl<T: ?Sized> Debug for WithDebug<'_, T> {  
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {  
    (self.0.fmt)(&self.1, f)  
  }  
}

// example usage: `nested(&r#fDebugStr(), 3, "foo")`
fn nested<T>(d: &DynDebug<T>, n: u32, t: T) -> String {  
  match n {  
    0 => format!("{:?}", WithDebug(&d, t)),  
    _ => nested(r#fDebugArray(d), n - 1, [t]),  
  }  
}
```
The problem here I guess is that, should ALL (`dyn`/`non-dyn`) constraints be uniformly lowered like how I did above? If so, it is _crucial_ to be able to recover the monomorphized version of a non-`dyn` constraint via aggressive POSTFACTUM optimizing and inlining, and I just don't know if thats true or not. Alternatively, we could have each trait mechanize its dyn/non-dyn counterparts seperately?? although its conceptually difficult to think about.

# Existentials
Whatever solution I find to this `dyn` problem, it could probably be directly extended to support existentials. Namely, constrained existential types will _have_ to have runtime-dictionary proofs meaning they are _necessarily_ going to be `dyn`?
For existentials, I want something better than Rust's `dyn Trait` because its rather limited. Lets imagine an extended GADT-like syntax for my new language:
```my-lang
enum AdvancedOption[A] {
  None: AdvancedOption[A],
  Some: A -> AdvancedOption[A]
  // equivalent to Haskell's `WithDisplay :: forall d a. Show d => d -> a -> AdvancedOption a`
  WithDispaly: for<D: dyn Display> D -> (A -> D) -> AdvancedOption[A]
}
```
Then clearly here I have to use the `dyn` bound `D: dyn Display` for my existential `D` as the type is runtime determined. And yet, its so much more functional/useful than rusts simple `dyn Trait` even if they both use dynamic-dispatch under-the-hood. The only thing close to this that is "static" is opaque return-position impls, which are _kind of_ like existentials in a limited way, I guess?
