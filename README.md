As of now, this is entirely wishful thinking and I will probably never ever implement any of this :)

The idea for this language is to combine 4 different languages into one that I wish existed:

- The base of the language should be Rust: in syntax and semantics, and everything else would be described as "
  deviations" from Rust (as borrowed from other languages)
    + One thing I would LOVE to remove from this language is the "impl" keyword. For return-position "impl" it should
      ideally be replaced with existential types (because I want that feature supported eventually) and for
      argument-position "impl" you should just use a generic parameter - there is no advantage to NOT using generic
      parameters in this situation.
    + Another thing I want to do is support robust "generic-eliding". In Rust there is already "lifetime-eliding" from
      the generic position when there isn't any ambiguity e.g. `fn str_id<'a>(s: &'a str) -> &'a str { s }` can be
      elided
      to `fn str_id(s: &str) -> &str { s }`. I want something like that but for types. just like in Haskell e.g.
        ```haskell
        flip :: forall a b c. (a -> b -> c) -> b -> a -> c
        flip f b a = f a b
      
        -- the explicit quantification can be "elided", just writing this instead
        flip :: (a -> b -> c) -> b -> a -> c
        flip f b a = f a b
        ```
      So in my language `fn id<T>(t: T) -> T { t }` could be elided to `fn id(t: T) -> T { t }`. Or for a more involved
      example
        ```my-lang
        fn compose_kleisli_ltr[M, A, B, C](bs: A -> M[B])(cs: B -> M[C])(a: A) -> M[C] 
        where M: Monad { 
            ... 
        }
      
        // the explicit generics could be elided, just writing this instead
        fn compose_kleisli_ltr(bs: A -> M[B])(cs: B -> M[C])(a: A) -> M[C] 
        where M: Monad { 
            ... 
        }
        ```
      but this would HAVE to be integrated into the higher-ranked-polymorhism system somehow; this is because (as it
      stands) Haskell's "forall" in the Rank-1 position corresponds to our function's generic parameters (e.g.
      `foo[A] ...`)
      while the "forall"s in any Rank-N (N >= 2) would correspond to our higher-ranked-trait-bounds keyword `for[T]`
      which is an unnecessary conceptual fragmentation.
        * Perhaps I could re-design "generic" syntax (of functions, and maybe even data-constructors, but I don't know
          if even haskell allows this to be honest...) to be simple syntax sugar for the full "bound" expression that
          uses `for[A]` keyword under the hood? This way, both generics and `for[A]` would always correspond
          conceptually to Haskell's `forall`.
        * But I can't think of any intuitive syntax for this, since this language (as it stands) explicitly combines
          function body and function signature, while Haskell has the luxury of having them separate, so it can easily
          write something like this `flip :: forall a b c. (a -> b -> c) -> b -> a -> c` adding the quantification
          keyword into the signature itself.
        * A stack-exchange question
          that [further elaborates on Rank-N parametric polymorphism](https://softwareengineering.stackexchange.com/questions/277048/is-higher-rank-parametric-polymorphism-useful)
          and another one
          about [ad-hoc vs. parametric polymorphism](https://stackoverflow.com/questions/6730126/parametric-polymorphism-vs-ad-hoc-polymorphism).
- I want this language to have a Go-style garbage collector and Go-style dynamic-dispatch of interfaces (aka traits)
  [using a fat-pointer to store polymorphism information](https://research.swtch.com/interfaces) (rather than vtable
  like C++, although this may have downsides)
    + this means no need for borrow checker, lifetimes, manual memory management, etc. Essentially a garbage collected,
      performant, rich-typesystem language. (Eliminating lifetimes means lots of things become a lot less convoluted,
      implementation wise AND language-design wise.)
    + This also means no more "object safety" rules that restrict which traits can and cannot be trait-objects (i.e.
      dynamically dispatched interfaces.) But the reason why object-safety exists in Rust is because trait-objects are
      considered to be "unsized" so the compiler doesn't know how much stack-space they require for allocation. So these
      fat-pointers are gonna need to include that sizing information somewhere I guess?? Then it might be wize to keep
      Rust's `dyn Trait` syntax to indicate that a trait is being dynamically dispatched (and hence the runtime overhead
      is explicitly shown.) Also to do with the whole "size" of types
      business, [here is an article](https://smallcultfollowing.com/babysteps/blog/2024/04/23/dynsized-unsized/)
      describing it in more detail and
      also [must move article](https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/) which
      describes the idea of "drop-able" types being an additional trait-requirement, meaning `T: ?Drop` would be a
      requirement that enforces must-move semantics.
    + Although I'd still have to see what things should be considered part of the "unsafe" block? Unsafe generally means
      it can create undefined behavior, or is something simmilar. What kinds of things in Go can create this? Can Go's
      pointer arithmetic lead to UB? Should I still have a notion of mutable and immutable references/pointers? If so,
      maybe a limited version of the borrow-checked may still be needed (to ensure I don't create `&T` while mutably
      borrowing with `&mut T`) or something??
    + Speaking of mutability, maybe I can follow Kotlin's on how they approach mutability-by-default? For references,
      maybe use runtime checks for preventing concurrent access (Go-style) or even runtime borrow-checker (like Swift’s
      exclusivity checks) or maybe even `Rc<RefCell<T>>` syntax sugar??
        * Once idea could be to *still* have lifetimes internal to the compiler but not expose them to the user? So
          maybe have a special internal lifetime `'gc` which represents how long a GC-d reference lives?? And then all
          references are GC'd so `'a: 'gc` for all `'a`. Then that might go somewhere, not sure?
        * SEE: Cyclone is a safe dialect of C that
          has [region-based](https://www.cs.umd.edu/projects/cyclone/papers/cyclone-regions.pdf) (i.e. lifetimes) memory
          management, and garbage collection. So kinda a lil-bit like this language, maybe have a look and borrow some
          ideas of how they handle
          mutable access?
            - [Here is a blog](https://pling.jondgoodwin.com/post/cyclone/) that explores more about Rust-Cyclone
              connection. Interesting bits: apparently languages have looked for better mutable, aliasing solutions for
              decades. Ada’s limited types, C’s `restrict`, C++’s strict aliasing rules, Fortran’s argument non-aliasing
              restrictions, [Flexible Alias Protection](https://janvitek.org/pubs/ecoop98.pdf) for Java, and academic
              work on fractional permissions and (again) separation logic.
              As it turns out, linear logic is not just a valuable memory management strategy. It is also a valuable
              aliasing and data race strategy. Clean introduced uniqueness types in 1992 for IO handling and destructive
              updates. Later languages, like ATS, Alms, and Mezzo, explored this idea in different ways. The Rust team
              was in touch with these teams, and stayed abreast of a wealth
              of [research](https://www.cs.cmu.edu/~carsten/linearbib/llb.html) underway at the same time.
              Rust’s resulting ownership-based model is largely based on the mutual exclusion idea that one either has a
              single mutable reference to an object, or possibly-multiple immutable references to that object. Abiding
              by this restriction eliminates the problems cited earlier, and makes it safe to transfer mutable data
              locklessly from one thread to another. Rust’s extensive approach
              to [fearless concurrency](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html) relies on much
              more than this, but this aspect is an important contributor.
            - Cyclone seems to heavily rely on "Linear/affine types" and "linear logic" in general, as a backbone to
              their "regions", and they also have a concept called "effect-sets" closely linked to this?? Is this linked
              to dependent types somehow? From Reddit: "If you look at the Curry-Howard isomorphism then linear type
              systems correspond to those where sub-structural logic rules (eg weakening and contraction) are reflected
              in the types."
            - Something, something, move semantics??
            - They also seem to have "existential types" which I might borrow, unless its just Rust's return-position
              `impls`...
        * Here is a paper on [combining region-inference and GC](https://dl.acm.org/doi/10.1145/512529.512547)
        * Leaning in on the "linear types" thing, here a paper on uniqueness
          types: [Uniqueness Typing Simplified](https://link.springer.com/chapter/10.1007/978-3-540-85373-2_12),
          and [austral](https://github.com/austral/austral) is a systems language with linear types. There is
          also [clean](https://clean-lang.org/) language which has uniqueness types; there is
          also [Linear Haskell](https://arxiv.org/pdf/1710.09756) which extends Haskell with linear types for resource
          safety, but also Haskell's `LinearTypes` extension.
        * Leaning in on the "capabilities" or "effect-sets" thing, you could track ownership through static
          capabilities (e.g., mutable vs. immutable). For example [Pony](https://www.ponylang.io/) uses reference
          capabilities (`iso`, `val`, `ref`) to ensure data-race freedom. There is also this
          paper: [Ownership Types for Safe Programming](https://web.eecs.umich.edu/~bchandra/publications/oopsla02.pdf)
        * Could use static analyzers like Infer (Facebook) or VeriFast, to reason about memory-access permissions. Here
          is a [paper](https://www.eis.mdx.ac.uk/staffpages/r_bornat/papers/fractional_permissions.pdf) on it. HOWEVER
          the integration into my compiler may be hella difficult, and limited to specific code patterns.
        * Escape analysis (a la Go and Java) for local/stack references could be used as a part of this puzzle.
          Especially in the case
          of [Go's escape analysis](https://medium.com/@trinad536/escape-analysis-in-golang-fc81b78f3550) you can return
          a pointer to a locally-scoped allocation e.g.
            ```go
            type person struct {
              name string
              age int
            }
            func newPerson(name string) *person {
              p := person{name: name}
              p.age = 42
              return &p
            }
            ```
          so presumably the escape-analysis determines that `p` should be allocated on the heap and the returned
          reference is to that location on the heap. Somehow, this must be meshed with mutability-tracking so that we
          have `*mut T` and `*T` to denote mutable and readonly pointers.
        * Just to mention a few more random
          resources: [capability based security with effects](https://arxiv.org/abs/2005.11444), Rust's
          new [Polonius](https://github.com/rust-lang/polonius/) borrow checker,
          Rust's [Oxide](https://arxiv.org/abs/1903.00982)
          paper, [Higher ranked region inference for compile-time garbage
          collection](https://studenttheses.uu.nl/bitstream/handle/20.500.12932/33775/document.pdf) which isn't a paper
          but is a student-thesis
          instead, [Garbage-Collection Safety for Region-Based Type-Polymorphic Programs](https://dl.acm.org/doi/10.1145/3591229)
          which seems to be
          right-up-my-alley, [Integrating region memory management and tag-free generational garbage collection](https://www.cambridge.org/core/journals/journal-of-functional-programming/article/integrating-region-memory-management-and-tagfree-generational-garbage-collection/782D317A9B811CD99FA0E924A35B6A58), [Liveness-Based Garbage Collection](https://www.cl.cam.ac.uk/~am21/papers/cc14.pdf), [Safe Garbage Collection = Regions + Intensional Type Analysis](https://www.cs.princeton.edu/techreports/1999/609.pdf), [Parallelism in a Region Inference Context](https://dl.acm.org/doi/10.1145/3591256)
          probably deals with concurrency primitives in light of region-inference and Golang has pretty neat concurrency
          features like "goroutines" and "channels" which I definitely wanna borrow.
        * There is also an experimental [Effekt Language](https://effekt-lang.org/) for effect-tracking, which splits
          the type-system into [Value vs. Computation types](https://effekt-lang.org/tour/computation) which may include
          mutability via [regions](https://effekt-lang.org/tour/regions)??
            - In a similar vein of experimental languages, there
              is [Koka](https://koka-lang.github.io/koka/doc/index.html), [Links](https://links-lang.org/), [Frank](https://arxiv.org/abs/1611.09259)
            - Also papers for more complex features like rank-2 polymorphism
              like [Evidently](https://dl.acm.org/doi/10.1145/3408981) or more specifically they link to
              this [Koka paper](https://arxiv.org/abs/1406.2061), [Abstraction-safe effect handlers via tunneling](https://dl.acm.org/doi/10.1145/3290318), [Generalized Evidence Passing for Effect Handlers](https://www.microsoft.com/en-us/research/wp-content/uploads/2021/08/genev-icfp21.pdf), [Inferring Algebraic Effects](https://arxiv.org/pdf/1312.2334)
              something about dependent-type systems with algebraic types BUT MORE IMPORTANTLY it deals with GARBAGE
              COLLECTED REGIONS
              OMG!!!!, [First-Class Names for Effect Handlers](https://prg.is.titech.ac.jp/papers/pdf/oopsla2022.pdf)
              gives explicit names to handlers AND ALSO USES rank-2
              stuff!!!, [Refinement Type System for Algebraic Effects and Handlers](https://arxiv.org/abs/2307.15463)
              use refinement types to model algebraic effects I
              think, [Efficient compilation of algebraic effect handlers](https://dl.acm.org/doi/10.1145/3485479).
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
    + I will have to think more about how this interacts with Rust's turbo-fish syntax, e.g. `Foo::<i32>::bar()`?? Maybe
      best to abandon altogether, in favor of Haskell-like visible type application, e.g. `show (read @Int "5")`..., but
      will have to think more how this interacts with the rest of the type-system down-the-line??
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
      of least resistance so something like `where for[T] Foo: BarTrait[T]` or something..., but I don't know how I it
      would scale to nested bounds like these. Also presumably I would want to create implication bounds like these
      `where for[T: FooTrait] Foo: BarTrait[T]` which says, "forall T such that T:FooTrait require Foo:BarTrait[T]".
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
    + Haskell also has the idea of functional dependencies for typeclasses (Rust has limited version of this with
      associated types/GATs), but it would be cool if there was some kind of way to support this in my language. But
      the syntax for it, I can't even begin to know what it might be.
        * Naively it could be something like
            ```my-lang
            // Functional dependency: `Key` determines `Value`
            trait Lookup<Key, Value> where Key => Value {
                fn get(&self, key: Key) -> Option<Value>;
            }
            ```
          but this would suggest that `Key => Value` is some kind of "constraint" when it might not be so. I would have
          to research what Haskell does under-the-hood to represent these things.
    + Rust's macro system seems rather powerful onto itself, but Haskell's "Template Haskell" and splicing system seems
      perhaps _more_ powerful? So I want to think about borrowing that system eventually. Maybe even have both, where
      they compliment each-other? Rust's macro-system could be used for "dumb" token-stream processing to enable things
      like parsing DSLs with foreign syntax, while the "splicing" system from Haskell could enable more
      reflection-related meta-programming capabilities? Also have to think about who's "derive" system to use? For
      Rust, "derive" is a proc-macro while for Haskell I'm not too sure how deriving works....
    + I want to borrow Haskell's "infix" notation as it seems incredibly convenient. Rust already has limited form of
      this with `+,-,*,/,...` operator overloading de-referencing to Traits, but I wonder if there's a way to to do this
      in a more systematic and generalized way? Like e.g. infix constructor types for enum variants? Maybe look into how
      Kotlin/Scala do its infix notation support?? (and operator overloading in general)
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
