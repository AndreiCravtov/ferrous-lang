Unlike **Rust** and **C++** which must <u>monomorphize</u> (copy+paste) implementations for each generic/template substitution, **Swift** is able to compile a generic function into a single implementation that can handle <u>every substitution dynamically</u>; here is the full article: [How Swift Achieved Dynamic Linking Where Rust Couldn't](https://faultlore.com/blah/swift-abi/#polymorphic-generics).

But basically:
But basically:
-> Swift can create (and supply) type’s _value witness table_ at runtime
-> Which contains size, alignment, stride, extra inhabitants, and so on
-> Code is compiled to be able to work with with generic types using **dynamic sizing info**
-> Laying out things **inline** can actually be done **dynamically** with relative ease
	-> Memory allocators and pointers **don’t care** about **static** layouts
	-> As long as you have all the relevant **value witness tables** everything works fine, just with more **dynamic** values than usual
-> The real major problem is stack allocations: LLVM really doesn’t like dynamic stack allocations
	-> [alloca](https://llvm.org/docs/LangRef.html#alloca-instruction) exists but it is **messy**
	-> you'd have to do special handling for function-calling-arguments + many optimizations potentially missed
	-> Swift devs managed to get it working all the time for **resilient layout** (but still hairy asf and PITA)
-> Protocol implementations (in other languages: traits, interfaces, typeclasses) can also be **resolved dynamically** via type’s _protocol witness tables_
	-> very similar to how Haskell compiles typeclass-constraints into dictionary-passing (optionally at runtime) 
-> Existentials (in the return position) are tricky for **dynamic stack allocators**
	-> the **size of the return-type** is not known until <u>AFTER</u> the function returns
	-> but the stack-allocation for the return-type needs to happen BEFORE the function call
	-> really messes up [alloca](https://llvm.org/docs/LangRef.html#alloca-instruction), so with existentials involved **boxing of return-values is forced**
-> Also **associated types** in <u>function signatures</u> **still** prevent existentials from being created
	-> it creates fundamental type system problems <u>unrelated</u> to ABI
	-> e.g. every instance of `MyProtocol` could have a **different associated type**, and you <u>can't let them get mixed up</u>
	-> Swift could use **path-dependent types** to deal with this, but thats for another time...
-> **Associated types** are <u>fine</u> for **normal** polymorphic code: generics enforce that every instance has the _same_ type, which is the only issue with them in existentials


