Haskell uses [System FC](https://www.microsoft.com/en-us/research/wp-content/uploads/2007/01/tldi22-sulzmann-with-appendix.pdf) as its **core calculus**, and all extensions (GADTs, type-families, kind polymorphism, datatype promotion, etc.) are **conservative (i think??) extensions** of **System FC**. And System FC is itself an extension of [System $\ds F_{\omega}$](https://en.wikipedia.org/wiki/System_F#System_F%CF%89) (I think.)

Here is [System FC with Explicit Kind Equality](https://www.cis.upenn.edu/~sweirich/nokinds-extended.pdf), basically its is **dependent kinds** calculus, i.e. **kinds** can depend on **types**

Here is a [Haskell researcher's website](https://richarde.dev/pubs.html), to track interesting papers:
- [Kinds Are Calling Conventions](https://richarde.dev/papers/2020/kinds-are-cc/kinds-are-cc.pdf) talks about the <u>difficulties of runtime-representation of highly-polymorphic code</u>, and designs intermediate language that allows for efficient static compilation, while still supporting flexible polymorphism !!!!! NICEEEEEE

[Lightweight higher-kinded polymorphism](https://www.cl.cam.ac.uk/~jdy22/papers/lightweight-higher-kinded-polymorphism.pdf) paper on using [Defunctionalization](https://en.wikipedia.org/wiki/Defunctionalization) to reduce higher-order terms (functions/constructors) with first-order terms

Here is [a Reddit comment](https://www.reddit.com/r/haskell/comments/29onpf/comment/cioauo4/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) that talks about **keeping monomorphism** AND STILL HAVING *super higher-ranked polymorphism a-la Haskell* by **passing around type-descriptors** to functions. Here is a [stack exchange question](https://cstheory.stackexchange.com/questions/37523/higher-rank-polymorphism-over-unboxed-types) on the same thing. Instead of generating specialized code for each type (monomorphization), generic functions receive **type descriptors** at runtime. These descriptors encode:
- **Type layout**: Size, alignment, and memory representation (crucial for unboxed values).
- **Type operations**: Functions/methods like `map` for a functor, `pure`/`>>=` for a monad, or typeclass/trait implementations.
- **Higher-kinded structure**: Information about type constructors (e.g., `Maybe`, `List`, or user-defined HKTs).
Then for examples where these descriptors CAN BE ERASED they ARE ERASED
- Therefore after this "elaboration and optimization" process, most of the items which are monomorphizable are monomorphized
- A small subset of cases (existentials, polymorphic recursion, rank-n types, etc.) will require dynamic dispatch (i.e. type descriptor passing) and can be opt-in via a keyword