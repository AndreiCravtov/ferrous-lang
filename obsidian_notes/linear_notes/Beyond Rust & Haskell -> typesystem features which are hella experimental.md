Rust & Haskell are both pretty advanced, but some features they don't have which is experimental asf but could be cool

# Row polymorphism
Haskell has records, but those aren't tracked at the <u>type-system level</u> so are pretty inflexible BUT there is something called [row polymorphism](https://en.wikipedia.org/wiki/Row_polymorphism) implemented by languages like [pure script](https://www.purescript.org/) which makes **row-types** their own kind **and** you can do:
```haskell
data Record :: # Type -> Type

type ClosedSet = ( foo :: Foo, bar :: Bar ) -- exactly these fields
type OpenSet r = ( foo :: Foo, bar :: Bar | r ) -- anything which has atleast these fields

type ClosedRecord = Record (ClosedSet)
type OpenRecord r = Record (OpenSet r)

```
This leans heavily into [structural typing](https://en.wikipedia.org/wiki/Structural_type_system) and may interplay with [[#Refinement types]] or [[#View types]] and might even be used to enable them
- If we take the view that [[Constraint kinds thought dump|type-classes are just records of functions]] then this could make possible [Go-like structural interfaces](https://www.golang-book.com/books/intro/9) since records are now structural
- However Rust and Haskell are **very heavily nominal** so I have to make sure the interplay doesn't erode/clash with that...

# Refinement types
Refinement types are essentially [set-builder notation](https://en.wikipedia.org/wiki/Set-builder_notation) for types, meaning a _subset_ of some type for which a predicate holds 
- e.g. you can write `type Nat = Int [ i | i >= 0 ]` which roughly translates to $\mathbb{N} = \{ i \in \mathbb{Z} \ | \ i \geq 0 \}$ in logic
- it is related to "contract programming" => I think [kotlin has contracts](https://www.baeldung.com/kotlin/contracts) but idk if its related or not
Here is a [Refinement Types](https://arxiv.org/pdf/2010.07763) paper explaining it.

These might be super useful for defining [[#View types]] specifically when selecting a subset of functions of a trait. E.g. a view type on the trait's methods which filters for only those methods which take **no mutable references**. Lets say `trait FooRef = Foo[f | ref(f)]` is the subset of `Foo` which only takes `&self` references, then for `T: Foo`, we could say `&T` implements `FooRef` via (auto)dereferencing.
# View types



refinement types
path polymorphism -> [type-soundness paper](https://dl.acm.org/doi/10.1016/j.entcs.2016.06.015)
generic  paths
views
views into traits -> impl subset of trait -> structural typing (not nominal) like go