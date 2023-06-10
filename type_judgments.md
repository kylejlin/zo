# Type Judgments

## Overview

- Params
- Type expressions
- Variant constructors
- Variants
- Matching
- Functions
- Function application
- Foralls
- Universes

## Params

A param `0` has type `context[0]`.
A param `1` has type `context[1]`.
In general, a param `n` has type `context[n]`.

## Type expressions

### Zero indices

A type expression of the form `(ind <type_n> ...)` has type `<type_n>`.
For example, the below Peano nat has the type `Type0`.

```zo
(
    ind

    Type0 // <-- This determines the type expression's type.

    "Nat"

    // Notice there are no indices.
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

### One or more indices

A type expression of the form

```zolike
(ind <type_n> _any_name (index_type0 index_type1 ... index_type_n) ...)
```

has the type

```zolike
(for (index_type0 index_type1 ... index_type_n) <type_n>)
```

For example, consider the below type expression for `Eq(Nat, zero)`.
We use Zozen notation for brevity.

```zozen
let Nat = ...
let zero = ...

return
(
    ind

    Type0 // <-- This is the type_n

    "Eq"

    // Index types
    (
        Nat // <-- This is index_type0
    )

    // Variant constructors
    (
        // refl: self_type_constructor(zero)
        // In other words
        // refl: Eq(Nat, zero)[zero]
        (() (zero))
    )
)
```

The type (written in Zozen notation) is `(for (Nat) Type0)`.

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
            // zero: self_type_constructor
            (() ())

            // succ (its details are irrelevant for this example)
            ((0) ())
        )
    )

    // Variant constructor index
    0
)
```

1. First, since the variant constructor index is `0`,
   we find the zeroth constructor (namely, `(() ())`).
2. Then, we take the constructor params (the left `()`),
   and create a `(for <params> <return_type>)`, with `<params>`
   replaced by the constructor params.
   This gives us `(for () <return_type>)`.
   We will fill in the `<return_type>` placeholder in a later step.
3. Then, we take the TODO
