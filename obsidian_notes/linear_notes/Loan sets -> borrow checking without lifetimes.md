Here is an article on [borrow-checking without lifetimes](https://smallcultfollowing.com/babysteps/blog/2024/03/04/borrow-checking-without-lifetimes/), which proposes to replace **lifetimes** (possibly even [[Notes on Austral & Linear System F#Some more concrete ideas|regions]]) with a **set of loans** like `shared(a.b.c)` or `mut(x)` which is called the **origin**. Here is a post explaining polonius -> [Polonius revisited](https://smallcultfollowing.com/babysteps/blog/2023/09/29/polonius-part-2/)

Here is an quick summary of the new type-system with loans/origins
```rust
// a place is either a variable, or a (nested) path
// on that variable in case it is a struct;
// (maybe even allow indexed paths like `variable[i]` ??)
//
// Since runtime values (like variables) must be usable in the type-system
// then maybe we can create a new *kind* called `Place` and all runtime values
// which represent places will automatically be promoted to some unique type
// of the kind `Place` that corresponds to this runtime-value?
// (kinda simmilar to how `DataKinds` promotes types-to-kinds in Haskell,
// and data-constructors (technically runtime values) to types)
Place = variable(.field)*

// A loan represents borrowing a place mutably OR immutably
Loan = shared(Place) 
     | mut(Place)

// An origin is a set of loans, it describes where the reference may have come from
Origin = { Loan }

// Generics can be either types or origins, which means hypothetically if
// I have the kinds `Type` and `Origin` then a generic is simply polymorphism
// of the form `forall (t:Type) (o:Origin). T` which is polymorphic in both kinds
Generic = Type | Origin

Type = TypeName <Generic*> 
     | &Origin Type 
     | &Origin mut Type 
     
TypeName = u32 (for now I'll ignore the rest of the scalars) 
         | () (unit type, don't worry about tuples) 
         | StructName 
         | EnumName 
         | UnionName 

```
And here is a simple program that uses this new system
```rust
// we could say that `counter` is a runtime value
// but is also LIFTED to the type `counter: Place` of *kind* place
let mut counter: u32 = 22; 

// so we can ALIAS that type for greater clarity
type Counter: Place = counter;

// then we can say that `shared(..)` and `mut(..)` are
// kind-level constructors creating `Loan`-kinded types from
// `Place`-kinded types
type LoanCounter: Loan = shared(Counter); // equivalent to `shared(counter)`

// then we can say that `{..}` is a kind-level constructor
// for creating `Origin`-kinded types from a set of `Loan`-kinded types
type OriginCounter: Origin = { LoanCounter }; // equivalent to `{shared(counter)}`

// So now a referene can be constructed using an origin (loan-set) type
let p: &OriginCounter u32 = &counter; // equivalent to `&{shared(counter)} u32`

// now you could see the error in action: `p` is live
// and its origin-set contains `counter` thus this mutation is NOT allowed
counter += 1; // Error: cannot mutate `counter` while `p` is live 
println!("{}", p);

```