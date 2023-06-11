# Zo Syntax Spec

## Overview

- Type expressions
- Variant constructors
- Variants
- Matching
- Functions
- Function application
- Foralls
- Universes

## Type expressions

### Peano Nat:

```zo
(
    ind

    // Type: THIS MUST BE A UNIVERSE LITERAL.
    Type0

    // Name: THIS MUST BE A STRING LITERAL.
    "Nat"

    // Index types
    ()

    // Variant constructors
    (
        // The DB index stack is
        // 0 => self_type_constructor: Type0

        // Variant constructor syntax:
        // (variant_constructor_param_types index_args)

        // zero: self_type_constructor
        (() ())

        // succ: forall(pred: self_type_constructor) -> self_type_constructor
        ((0) ())
    )
)
```

As you can see, the universe level and name
must be literals.
This ultimately means there are no universe-level-polymorphic
or name-polymorphic expressions in Zo.

### Equal:

```
(
    fun

    // Decreasing param index. THIS MUST BE A NUMBER LITERAL OR `nonrec`.
    // This is equal to the arity
    // If the fun is non-recursive, write `nonrec`.
    nonrec

    // Param types
    (Type0 0)

    // Return type
    (
        for

        // Param types
            // DB index stack is
            // 0 => x: T
            // 1 => T: Type0
        (
            1 // y: T
        )

        // Return type
        Type0
    )

    // Body
    (
        ind

        Type0

        "Eq"

        // Index types
        (
            // The DB index stack is
            // 0 => self_fun (inaccessible) : forall(T': Type0, x': T') ->
            //                    (forall(y': T') -> Type0)
            //      Note this is inaccessible since the fun is
            //      declared as non-recursive.
            // 1 => x: T
            // 2 => T: Type0

            2 // y: T
        )

        // Variant constructors
        (
                // The DB index stack is
                // 0 => self_type_constructor: forall(y': T) -> Type0
                // 1 => self_fun (inaccessible) : forall(T': Type0, x': T') ->
                //                    (forall(y': T') -> Type0)
                // 2 => x: T
                // 3 => T: Type0
                //
                // Note that indices are not added to the DB stack.

            // refl: self_type_constructor(x)
            // In other words
            // refl: Eq(T, x)[x]
            (() (2))
        )
    )
)
```

### Boolean:

```zo
(
    ind

    Type0

    "Bool"

    ()

    (
        // true
        (() ())

        // false
        (() ())
    )
)
```

### List:

```zo
(
    fun

    nonrec

    // Param types
    (Type0)

    // Return type
    Type0

    // Body
    (
        ind

        Type0

        "List"

        ()

        (
            // DB index stack is
            // 0 => self_type_constructor = List(T): Type0
            // 1 => self_fun: forall(T': Type0) -> Type0
            // 2 => T: Type0

            // nil: self_type_constructor
            // In other words,
            // nil: List(T)
            (() ())

            // cons: forall(car: T, cdr: self_type_constructor) -> self_type_constructor
            // In other words,
            // cons: forall(card: T, cdr: List(T)) -> List(T)
            ((2 0) ())
        )
    )
)
```

### Less than or equal (parameterized)

```zozen
let Nat = ...
let succ = ...

return
(
    fun

    nonrec

    // Param types
    (Nat)

    // Return type
    Type0

    // Body
    (
        ind

        Type0

        "Le"

        // Index types
        (Nat)

        // Variants
        (
            // DB index stack is
            // 0 => self_type_constructor: forall(rhs: Nat) -> Type0
            // 1 => self_fun (inaccessible)
            // 2 => lhs: Nat

            // refl
            (() (2))

            // step
            (
                // Variant constructor param types
                (
                    Nat // rhs_pred: Nat

                        // DB index stack is
                        // 0 => rhs_pred: Nat
                        // 1 => self_type_constructor: forall(rhs: Nat) -> Type0
                        // 2 => self_fun (inaccessible)
                        // 3 => lhs: Nat

                    (1 0) // lhs_le_rhs_pred: Le(lhs)[rhs_pred]
                )

                // Index args

                    // DB index stack is
                    // 0 => lhs_le_rhs_pred: Le(lhs)[rhs_pred]
                    // 1 => rhs_pred: Nat
                    // 2 => self_type_constructor: forall(rhs: Nat) -> Type0
                    // 3 => self_fun (inaccessible)
                    // 4 => lhs: Nat

                ((succ 1))
            )
        )
    )
)
```

## Variant constructors

### `Nat.zero`:

```zo
(
    vcon

    // Type
    (
        ind

        // Type
        Type0

        // Name
        "Nat"

        // Index types
        ()

        // Variants
        (
            // The DB index stack is
            // 0 => self_type_constructor: Type0

            // Variant syntax: (variant_constructor_param_types index_args)

            // zero: self_type_constructor
            (() ())

            // succ: forall(pred: self_type_constructor) -> self_type_constructor
            ((0) ())
        )
    )

    // Variant constructor index - THIS MUST BE A NUMBER LITERAL.
    0
)
```

### `Nat.succ`:

```zo
(
    vcon

    // Type
    (
        ind

        // Type
        Type0

        // Name
        "Nat"

        // Index types
        ()

        // Variants
        (
            // The DB index stack is
            // 0 => self_type_constructor: Type0

            // Variant syntax: (variant_constructor_param_types index_args)

            // zero: self_type_constructor
            (() ())

            // succ: forall(pred: self_type_constructor) -> self_type_constructor
            ((0) ())
        )
    )

    // Variant constructor index - THIS MUST BE A NUMBER LITERAL.
    1
)
```

As commented above,
the variant constructor indices use forward counting.
That is, the first variant constructor is `0`,
the second variant constructor is `1`, and so on.
Variant constructor indices should not be confused with
DeBruijn indices, which use backwards counting.

It is also important to note that
the variant constructor indices must be number literals.

## Variants

This section uses Zozen's `let` syntax.
Furthermore, some obvious code is abbreviated with `...`.

### `3`:

```zozen
let Nat = ...
let zero = (vcon Nat 0)
let succ = (vcon Nat 1)

return (succ (succ (succ zero)))
```

### `[3]` (i.e., singleton list containing `3`):

```zozen
let Nat = ...
let zero = ...
let succ = ...
let three = ...

let List = ...
let NatList = (List Nat)
let NatList_nil = (vcon NatList 0)
let NatList_cons = (vcon NatList 1)

return (NatList_cons three NatList_nil)
```

### `(Eq Nat 3 3)`:

```zozen
let Nat = ...
let zero = ...
let succ = ...
let three = ...

let Eq = ...
let EqNat3 = (Eq Nat three)
let refl_nat_3 = (vcon EqNat3 0)
let proof_that_3_equals_3 = refl_nat_3
```

## Matching

This section also uses the Zozen syntax.
Some obvious code is abbreviated with `...`.

### Is zero

```zozen
let Nat = ...
let zero = ...
let succ = ...
let three = ...
let Bool = ...
let true = ...
let false = ...

return (
    match

    // Matchee
    three

    // Return type
    Bool

    // Cases
    (
        // `Nat.zero` case

        // DB index stack is empty

        true

        // `Nat.succ` case

        // DB index stack is
        // 0 => pred: Nat

        false
    )
)
```

### Pred or zero

```zozen
let Nat = ...
let zero = ...
let succ = ...
let three = ...

return (
    match

    // Matchee
    three

    // Return type
    Nat

    // Cases
    (
        // `Nat.zero` case

        // DB index stack is empty

        zero

        // `Nat.succ` case

        // DB index stack is
        // 0 => pred: Nat

        0
    )
)
```

## Functions

This section also uses the Zozen syntax.
Some obvious code is abbreviated with `...`.

### `not`

```zozen
let Bool = ...
let true = ...
let false = ...

return
(
    fun

    // Decreasing param index.
    // THIS MUST BE A NUMBER LITERAL OR `nonrec`.
    // If the function is non-recursive,
    // you can write `nonrec`.
    nonrec

    // Param types
    (Bool)

    // Return type
    Bool

    // Body
        // DB index stack is
        // 0 => self_fun (inaccessible): forall(b': Bool) -> Bool
        // 1 => b: Bool
    (

        match

        // Matchee
        1

        // Return type
        Bool

        // Cases
        (
            // True case
            false

            // False case
            true
        )
    )
)
```

As commented above,
the decreasing param index must be a number literal
or the `nonrec` keyword.

### `is_even`

```zozen
let Nat = ...
let zero = ...
let succ = ...
let Bool = ...
let true = ...
let false = ...
let not = ...

return
(
    fun

    // Decreasing param index
    0

    // Param types
    (Nat)

    // Return type
    Bool

    // Body
        // DB index stack is
        // 0 => self_fun: forall(n': Nat) -> Nat
        // 1 => n: Nat
    (
        match

        // Matchee
        1 // n

        // Return type
        Bool

        // Cases
        (
            // Zero case
            true

            // Succ case
                // DB index stack is
                // 0 => npred: Nat
                // 1 => self_fun: forall(n': Nat) -> Nat
                // 2 => n: Nat
            (not (1 0))
        )
    )
)
```

## Function application

The syntax is `(callee arg0 arg1 ... argN)`.
There are no nullary functions.
Therefore, there is never a need to write `(callee)`.

## Foralls

This section also uses the Zozen syntax.
Some obvious code is abbreviated with `...`.

### `typeof List`

```zozen
(
    for

    // Param types
    (Type0)

    // Return type
    Type0
)
```

### `typeof Eq`:

```zo
(
    for

    // Param types
    (
        Type0 // T: Type0
        0 // x: T
    )

    // Return type
    (
        for

        // Param types
        (
            // DB index stack is
            // 0 => x: T
            // 1 => T: Type0

            1 // y: T
        )

        // Return type
        Type0
    )
)
```

## Universes

A universe is `Type0`, `Type1`, `Type2`, or `Type<n>` for any `<n>`.
