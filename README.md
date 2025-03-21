As of now, this is entirely wishful thinking and I will probably never ever implement any of this :)

The idea for this language is to combine 4 different languages into one that I wish existed:

- The base of the language should be Rust: in syntax and semantics, and everything else would be described as "
  deviations" from Rust (as borrowed from other languages)
- I want this language to have a Go-style garbage collector and Go-style dynamic-dispatch of interfaces (aka traits)
  using a fat-pointer to store polymorphism information (rather than vtable like C++)
    + this means no need for borrow checker, lifetimes, manual memory management, etc. Essentially a garbage collected,
      performant, rich-typesystem language. (Eliminating lifetimes means lots of things become a lot less convoluted,
      implementation wise AND language-design wise.)
    + This also means no more "object safety" rules that restrict which traits can and cannot be trait-objects (i.e.
      dynamically dispatched interfaces.) But the reason why object-safety exists in Rust is because trait-objects are
      considered to be "unsized" so the compiler doesn't know how much stack-space they require for allocation. So these
      fat-pointers are gonna need to include that information somewhere I guess??
    + Although I'd still have to see what things should be considered part of the "unsafe" block? Unsafe generally means
      it can create undefined behavior, or is something simmilar. What kinds of things in Go can create this? Can Go's
      pointer arithmetic lead to UB? Should I still have a notion of mutable and immutable references/pointers? If so,
      maybe a limited version of the borrow-checked may still be needed (to ensure I don't create `&T` while mutably
      borrowing with `&mut T`) or something??
    + Speaking of mutability, maybe I can follow Kotlin's on how they approach mutability-by-default? For references,
      maybe use runtime checks for preventing concurrent access (Go-style) or even runtime borrow-checker (like Swift’s
      exclusivity checks) or maybe even `Rc<RefCell<T>>` syntax sugar?? Not quite sure...
    + Also perhaps I could merge the notion of Go's "unsafe" package and Rust's "unsafe" blocks? Something to think
      about...
    + Also Go's interfaces are nominally typed while Rust's traits (and Haskell's typeclasses) are extremely nominally
      typed, So have to reconcile this somehow...., maybe see PureScript's row polymorphism/extensible records for
      inspiration???
- I want to borrow Generics syntax from Scala, so `T[A]` instead of `T<A>` and also same for higher-kinded-types syntax
  `T[_]` instead of the hypothetical `T<_>`.
    + This also means it might be possible to design convenient "out-of-order" partial type-application syntax?? For
      example a Haskell-result-type would be `Result t e :: Type -> Type -> Type` and partially applying it would give
      us `Result t :: Type -> Type` which isn't _exactly_ super convenient and why Haskell's equivalent is `Either e t`
      instead (to partially apply the error). But potentially we could have `Result[_, _] :: Type -> Type -> Type`
      unapplied constructor type, and `Result[_, E] :: Type -> Type` would *out of order* partially apply the 2nd type
      parameter.
    + Although maybe I will have to think about this a little more, perhaps Haskell-style syntax for generics is
      superior still...? I.e. no extra brackets or weird things, just `T a b` or whatnot?
- I want to (eventually) borrow advanced Haskell-like type-system features:
    + such as function partial application, although I'd have to think a little more how I represent functions. Rust's
      approach is to have `Fn,FnMut,FnOnce` trait-hierarchy, which is a non-uniform way of representing functions.
      Ideally I would wanna move away from that..., and closer to Haskell's `(->) :: Type -> Type -> Type`
      arrow-function-type. I also have to figure out how I want to represent curried functions with special syntax?
      Maybe something like `fn foo(a: A)(b: B) -> C` or something??? Not quite too sure....
    + type-constructor partial-application, ideally with out-of-order application as described above
    + constraint-kinds (e.g. traits/typeclasses are really just HKTs of the kind `Type -> Constraint`)
    + Rank-N polymorphism, which is currently VERY limited-ly supported in Rust with lifetime higher-ranked-trait-bounds
      syntax, e.g. `where for<'a> Foo: BarTrait<'a>` or something. But I want it to be arbitrarily nested (hence Rank-N)
      and work on types. Not too sure how *exactly* I want to the syntax to be? Keeping Rust's "for" syntax is the path
      of least resistance so something like `where for<T> Foo: BarTrait<T>` or something..., but I don't know how I it
      would scale to nested bounds like these. Also presumably I would want to create implication bounds like these
      `where for<T: FooTrait> Foo: BarTrait<T>` which says, "forall T such that T:FooTrait require Foo:BarTrait<T>".
    + Optionally explicit "self" types (atleast for traits/typeclasses). In Rust, any trait will implicitly have a "
      Self" type. So `trait Foo {}` is equivalent to Haskell's `class Foo a` where the `Self` corresponds to `a`. So
      what I want to do is make that correspondence clear; the `Self` type should become simply syntax-sugar for a
      hidden generic-parameter `a` which can be optionally "explicitly desugared" although i've yet to come up with what
      that desugared syntax will be. I still haven't figured out how this should interact with "impl" blocks and the "
      Self" types of actual types, e.g. `impl Foo for MyStruct` corresponds to `instance Foo MyStruct` in haskell but
      what will this "explicitly desugared" capacity mean for what `Self` means in `impl Foo for MyStruct`?
    + Existential types, in Haskell they have existential type quantification via the isomorphism
      `(forall a. (t a -> r)) ≅ ((exists a. t a) -> r)`. Although I wonder if introducing an explicit `exists` keyword
      could be warranted - to avoid the need for an isomorphic form? Additionally, I do wonder if this is even possible?
      Rust is monormorphised at some point during compilation, but I don't know if Haskell is, or if it oscillates
      between representations until all is resolved????? I'd have to investigate this a little more....
    + GADTs like in Haskell, and more broadly, equality constraints e.g. `a ~ b`, although I'd have to figure out how I
      wanna represent this in my language. The Haskell syntax suggests to me it might be a typeclass of kind
      `(~) :: Type -> Type -> Constraint` so maybe it can be some "built-in" trait `trait Is<T> {}`?? Not too sure.
        * I also have to figure out how exactly I want GADT syntax to work. Rust's Enums are translations of Haskell's
          ADTs, so something like `data Option a = None | Some a` becomes `enum Option[A] { None, Some(a) }` but what
          about GADT syntax? Something like Haskell's
            ```haskell
            data Option a where
                None :: Option a
                Some :: a -> Option a
            ```
          should perhaps become this in my language?
            ```my-lang
            enum Option[A] {
                None: Option[A],
                Some: fn(a) -> Option[A]
            }
            ```
          This *would* fit nicely with some of existing Rust's concepts such as Tuple-like enum-variants being secretly
          also function-types. Although I'd have to figure out how I want to "finalize" what "function-types" look like.
          Right now it uses Rust's `fn(A) -> B` type syntax, but if I instead somehow generalized function types to the
          arrow-type `(->)` (i.e. if I abandon the `Fn/FnMut/FnOnce` model??) then it would become:
            ```my-lang
            enum Option[A] {
                None: Option[A],
                Some: a -> Option[A]
            }
            ```
          However, I do want to eventually add bits and pieces of Haskell's Kind-system to this language so e.g.
          `KindSignatures` or `DataKinds`, etc., so I'm not to sure how well this syntax would mesh with explicit kind
          signature like you can do in Haskell
            ```haskell
            data Option :: Type -> Type where
                None :: Option a
                Some :: a -> Option a
            ```
          If I translate to my proposed GADT syntax I get a bit of a conceptual hole, what exactly do I put here?
            ```my-lang
            enum Option ??????????? {
                None: Option[A],
                Some: a -> Option[A]
            }
            ```
        * What about NON-equality constraints also? e.g. `a !~ b` ?? would that even make any sense??? Could that even
          be done??????
    + As mentioned in the GADTs, I want to eventually borrow may of Haskell's Kind-system features. However, I don't
      know how many of these is even theoretically portable to an imperative-language. Maybe some of these features are
      gated behind the language being a pure-FP language??? Not too sure.
    + Rust's macro system seems rather powerful onto itself, but Haskell's "Template Haskell" and splicing system seems
      perhaps _more_ powerful? So I want to think about borrowing that system eventually. Maybe even have both, where
      they compliment each-other? Rust's macro-system could be used for "dumb" token-stream processing to enable things
      like parsing DSLs with foreign syntax, while the "splicing" system from Haskell could enable more
      reflection-related meta-programming capabilities? Also have to think about who's "derive" system to use? For
      Rust, "derive" is a proc-macro while for Haskell I'm not too sure how deriving works....
    + I want to borrow Haskell's "infix" notation as it seems incredibly convenient. Rust already has limited form of
      this with `+,-,*,/,...` operator overloading de-referencing to Traits, but I wonder if there's a way to to do this
      in a more systematic and generalized way? Like e.g. infix constructor types for enum variants?
    + Haskell has "do-notation" for monads, and Rust has "question-mark" syntax for `Result`, `Option` and so on.
      Perhaps I could conceptually merge these two??
- I want to use Haskell's "deferred typecheking" algorithm where they turn type-constraints into a constraint-solving
  problem in a dedicated constraint-solving compiler-internal mini-language. And then defer the solving of this
  constraint problem until later in the compilation.
- While this one has no specific ONE language I want to borrow the feature from, PERHAPS maybe adding dependent-types to
  the type-system of my language??? This may just be completely not worth it, as it might make the type-inference
  NIGHTMARISH to both use and implement?? Perhaps most of the useful features of dependent-types could be utilized by
  porting enough of Haskell's type-level features (e.g. `TypeFamilies`)?? Not quite too sure...

Basically just make sure you have Rust nightly installed: `rustup toolchain install nightly`
And Clippy installed also: https://doc.rust-lang.org/clippy/installation.html
