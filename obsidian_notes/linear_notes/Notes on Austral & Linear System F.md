Ok so [Austral Lang](https://austral-lang.org/) has 
- linear types to enforce move-semantics
- region-based management (no GC)
- (lexical) borrowing of shared/immutable references like (old) Rust
- a (somewhat) thought out type-system
- typeclasses, kinds, and other goodies borrowed from functional languages like Haskell, ML, etc.

The paper [Lightweight Linear Types in System $F^{\circ}$](https://www.cis.upenn.edu/~stevez/papers/MZZ10.pdf) essentially extends **System F** with linear types.
- It also discusses how to use it to represent all sorts of shared, exclusive, whatever, references AND MORE
- It also has an extension section for encoding: polykinded products, "sum" types, fixpoints, ADTs, existential types, etc
- Maybe can be extended to be more **System FC** -like and then I can just graft on all the Haskell extensions
Then [Linear Regions Are All You Need](https://www.khoury.northeastern.edu/home/amal/papers/linrgn.pdf) creates a core language that has **linearly-typed regions** which is also translatable to **System F**
The [Region-based Resource Management and Lexical Exception Handlers in Continuation-Passing Style](https://link.springer.com/chapter/10.1007/978-3-030-99336-8_18) which has **explicit region types** and translatable to **System F**

[Quantitative program reasoning with graded modal types](https://dl.acm.org/doi/10.1145/3341714) what even is graded modal types [Granule Language](https://granule-project.github.io/granule.html)
And a followup paper [Functional Ownership through Fractional Uniqueness](https://dl.acm.org/doi/10.1145/3649848) which tries to encode ownership with the above

https://link.springer.com/chapter/10.1007/978-3-662-54434-1_20

# Some more concrete ideas
Basically I'm thinking that I can piece together a bunch of features from all sorts of proposals, into a nice little TARGET language:
- Absolutely everything is linearly-typed
	- If a type `a` is affine, then compiler generates destructor-function `a -> ()` used explicitly to drop a value
		- I still have to figure out the rules for which types are affine, and how to generate the destructor exactly - is it structural?
		- One thing to consider is if (absence of) drop-types will be visible in the surface language. On the one hand, articles like [this one](https://smallcultfollowing.com/babysteps/blog/2023/03/16/must-move-types/) have explored it and found it to be quite useful. On the other hand, they also noted that stack-unwinding is not (always) possible with non-linear types; the solution may be to mark functions (with auto-traits) as supporting/not-supporting stack-unwinding based on if they use (purely) linear types in them; and then maybe provide a method for "upgrading" non-stack-unwinding functions TO stack-unwinding functions by giving fallback parameters??
	- If a type `a` is non-linear, then compiler generates duplication-function `a -> (a,a)` used explicitly to duplicate a value
		- As with dropping, still need to figure out the rules for which types are non-linear and how to generate the duplication exactly
- Lean hardcore into "coersion proof terms" for **region types** just like System FC did for <u>equality coersions</u>
	- We can basically define the following Haskell type that represents region-coersion proofs
    ```haskell
    data (<:) :: Region -> Region -> Type where
	  Refl :: r <: r
	  Trans :: r1 <: r2 -> r2 <: r3 -> r1 <: r3
	  
    ```
	- And then any <u>explicit</u> lexical scoping can produce automatically subregion-evidence for the lexically-enclosing, e.g. 
    ```rust
    'outer: {
	  // subregioning evidence is auto-generated and (optionally) 
	  // "passed" to inner scope AKIN a function parameter
	  'inner: (wit: 'inner <: 'outer) => {
	    <..> // inner code can use `wit` for coersion operations
	  }
    }
    
    ```
    - Anything that might have a scope (functions, loops, if-statement bodies, etc.) automatically gets a (lexical) region and can be overloaded to admit auto-proofs of subregioning; for functions specifically the default region-name is same as function-name
	- I can define a bunch of (inherent/assumed) operations that can use subregion-evidence for coersions
		- One immediate example I can think of is `fn restrict_ref<'a, 'b, T>(ab_wit: 'a <: 'b, t_ref: &'b T) -> &'a T` and similarly `fn restrict_mut<'a, 'b, T>(ab_wit: 'a <: 'b, t_mut: &'b mut T) -> &'a mut T` for mutable references; this can be understood as saying IF you give a reference that is valid within the region `'b` then it is DEFINITELY a valid reference within region `'a` since `'a <: b` (i.e. `'a` is a subregion of `'b`)
		- For some of the other coersion operations I'd have to think through the semantics a little more and ensure those are sane :)
- Use [Austral-style borrow statements](https://austral-lang.org/spec/spec.html#stmt-borrow) for explicit borrow-checking, and add re-borrow statements for `&mut T => &T` conversions 
    ```rust
    fn main() {
      let val: i32 = 1;
      let useless_val_ref: &'a i32;
      'a: borrow val (val_ref: &'a i32, wit_a: 'a <: 'main) => {
		// Shared references should all be duplicatable (compiler generates function for this)
        let (val_ref, val_ref2) = dup(val_ref); // `val_ref` shaddowed

		// I could even let this reference "escape" the 'a region but it would be rather useless :)
		useless_val_ref = val_ref2; // `val_ref2` moved into `useless_val_ref`

        // I can now use reference-related functions like `fn clone<T>(&T) -> (&T,T)`
        let (val_ref, cloned_val) = clone(val_ref); // `val_ref` shaddowed again
        
        // I am NOT ALLOWED to use `val` within region 'a
        <..>
        
        // All types are linear, so pass to destructor to "use" the reference
        drop(val_ref);
        drop(wit_a);
      }
      
      // region-realiasing probably should not be supported
      // to allow for erroneous "escaping references" like the ones above
      let other_val: i32 = 2;
      'b: borrow mut val (val_mut: &'b mut i32, wit_b: 'b <: 'main) => {
        // I can now use reference-related functions like `fn memset<T>(&mut T, T) -> &mut T`
        let val_mut = memset(val_mut, other_val); // `val_mut` shaddowed
        
        // I could even (explicitly) downgrade mutable to shared references
        'c: reborrow val_mut (val_ref: &'c mut i32, wit_c: 'c <: 'b) => {
	      // I can now combine witnesses of nested scopes to create new ones
	      // with `fn trans<'a,'b,'c>('a<:'b, 'b<:'c) -> 'c<:'a`
	      let (wit_b, wit_b2) = dup(wit_b); // `wit_b` shaddowed
	      let (wit_c, wit_c2) = dup(wit_c); // `wit_c` shaddowed
	      let wit_combined: 'c <: 'main = trans<'c,'b,'main>(wit_c2, wit_b2);
	      drop(wit);
	        
	      // I am NOT ALLOWED to use `val_mut` (or `val`) within region 'c
          <..>

		  // All types are linear, so pass to destructor to "use" the reference
	      drop(val_ref);
	      drop(wit_c);
        }
	    
	    // I am NOT ALLOWED to use `val` within region 'b
        <..>
        
	    // All types are linear, so pass to destructor to "use" the reference
        drop(val_mut);
        drop(wit_b);
      }
    }
    
    ```
	- One issue might be how to interpret what `&'a T` means; it COULD mean that `&'a T` is valid within region `'a` or it COULD mean that it is a reference to a value stored in region `'a`, which is VERY DIFFERENT semantics, and would have to be ironed out!!
	- Rust's [non-lexical lifetimes](https://smallcultfollowing.com/babysteps/blog/2017/02/21/non-lexical-lifetimes-using-liveness-and-location/) might be more advanced than the current model, and [Polonius](https://smallcultfollowing.com/babysteps/blog/2023/09/22/polonius-part-1/) reformulation even-more-so; may need to update the above borrowing-mechanic to capture those intricacies OR introduce more proof-types :)) also there is [a paper on tree-borrowing](https://github.com/Vanille-N/tree-borrows/blob/master/model/treebor.pdf) which may be useful
	- It may be helpful to restrict this target language to be strictly CFG or CPS in order to more closely match MIR, and also have access to (guranteed) control-flow information at the language-level
	- One thing mentioned in the NLL blogs is the concept of "liveness" so maybe I could introduce the idea of liveness-constraints + proofs?? Might even go a step further and INVERT the linearity: nothing is a linear type except liveness-proofs
		- any operations whatsoever that "make use of" (moving, receiving, cloning, etc.) will expect a liveness-proof of the value to accompany that value; this makes those values (for all intents and purposes) linear
		- the only way to dispose of a liveness proof (use without _actually_ using it) would be to pass it to a (compiler-generated) destructor function, i.e. `drop(live_wit, value)` which would implicitly make the values bound to liveness-proofs with destructors affine
		- the only problem is how do I bind an INSTANCE of a value to this liveness proof ?? and how does this relate to "lifetime of value" vs. "lifetime of reference" distinction ?? I REALLY don't want to introduce another ad-hoc kind JUST for this, with usage akin to `let (t_val: Instance<T,B>, live_wit: Live<T,B>) = mk_t(..)` where `B` would be some kind of binding-type used to tie together an instance together with it's liveness proof - this would really suck and be really annoying :(
	- Touching on the concept of "lifetimes of values" I wonder if all express the lifetime of their values as an impicit (or explicit??) `'self` lifetime?? This might be a really dumb idea however
	- Another issue is that Rust's borrow/move checker can PARTIALLY move/borrow structures with paths (`a.b` or `a[i]`) so I need to refine this approach to be path-aware and borrow/move PATHS rather than entire values
		- This also relates to the paper [about deferred borrows](https://cfallin.org/pubs/ecoop2020_defborrow.pdf) which builds on this path-awareness to enhance it
		- And then there is [a blog building extending that paper to path generics](https://cfallin.org/blog/2024/06/12/rust-path-generics/#:~:text=The%20genericity%20over%20a%20path,(up%20the%20call%20stack).), and [further ambitious extensions](https://smallcultfollowing.com/babysteps/blog/2024/06/02/the-borrow-checker-within/)
		- Here is a paper on [typed path polymorphism](https://www.sciencedirect.com/science/article/pii/S0304397519301240) which describes operating on paths polymorphically - may be orthogonal to path-dependent types tho...
- Keep Rust's rank-2 region polymorphism (and maybe extend to rank-N region polymorphism??)
- Use something akin to Cyclone's region-handles, e.g.
    ```rust
	fn make_number<'a>(h: Handle<'a>) -> &'a mut i32 {
	  // I haven't decided if a more BROAD statement like `use <handle> { .. }` this is fine, 
	  // or a more fine-graned set of statements using `Handle<'a>` is appropriate;
	  let val_mut: &'a mut i32 = use h {
	    // within the `use` statement, the typecheker thinks the current region is 'a
	    12 // return values are "downgraded" to mutable references upon exiting the `use` scope
	  };
	  drop(h);
	  
	  val_mut
	}

    fn main() { // I could optionally get a `Handle<'main>` if I wanted to
	  'b: (wit: 'b <: 'main, h: Handle<'b>) => {
	    // now I can pass around `h` to functions that return references
	    // to objects they've allocated within this region
	    let val_mut: &'b mut i32 = make_number(h);
		// I haven't thought through if/how coersion proofs interact with handles...
		
		drop(val_mut);
		drop(h);
	  }
    }
	  
    ```
	- Cyclone also uses dynamic regions (that you can open with keys) so that might be a cool opt-in feature (with possibly dynamic-dispatch)
	- Cyclone _also_ has a special **heap** region `H` into which you can allocate values (but not clean them up); this is somewhat akin to using Rust's `Box::leak`, and could be an "escape hatch" where (conservative) garbage collectors are allowed to be used to clean up memory; might be cool to add, but kinda iffy as to how its used right now...; is it the same?? or different?? than Rust's `'static` lifetime??
