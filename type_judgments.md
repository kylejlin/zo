# Type Judgments

## Overview

- Notational conventions
- Params
- `ind` expressions
- Variant constructors
- Matching
- Functions
- Function application
- Foralls
- Universes

## Notational conventions

1. Define `(@cfor <param_types> <return_type>)` as
   `(cfor <param_types> <return_type>)` if `<param_types>` is nonempty,
   and `<return_type>` if `<param_types>` is empty.

   "cfor" stands for "collapsing for", since the `for` "collapses"
   when it has zero params.

2. Define `(@capp <callee> <args>)`
   as `(<callee> <args>)` if `<args>` is nonempty,
   and `<callee>` if `<args>` is empty.

   "capp" stands for "collapsing application",
   since the application "collapses" when it has zero args.

3. If you see Zozen-specific syntax
   (e.g., `let` definitions, or named variables like `Nat`),
   you can automatically
   assume we are using Zozen.
   We will not explicitly say "We are using Zozen syntax" every time.
   This will not cause any ambiguity because
   "vanilla" Zo does not support this syntax.

## Params

A param `0` has type `context[0]`.
A param `1` has type `context[1]`.
In general, a param `n` has type `context[n]`.

## `ind` expressions

### General rule

Suppose we have some `ind` expression.
By definition, it has the form:

```zolike
(
    ind

    <output_type> // This is a guaranteed to be a literal
                  // `Type<p>` for some `<p>`.

    <name>

    (
        <index_type0>
        <index_type1>
        ...
        <index_type_m>
    )

    (
        <variant0>
        <variant1>
        ...
        <variant_n>
    )
)
```

If this expression has any type at all,
then it has the type

`(@cfor (index_type0 index_type1 ... index_type_n) <output_type>)`.

In order to have a type,
the expression must meet the following conditions:

1. Every index type is well-typed. Formally:

   For every `i`, there exists some `q_i` such that
   `<index_type_i>` has a type of `Type<q_i>`.

2. The index types are consistent with `<output_type>`. Formally:

   `<output_type>` is greater than or equal to
   the maximum of the set `{ typeof(<index_type_i>) | i \in [0, m] }`.
   If there are no indices, then `<output_type>` may be any `Type<p>`.

3. For every variant:

   1. Every constructor param is well-typed.
   2. Every return type index arg is well-typed.
   3. The number of index args matches the number of indices.
   4. The constructor param types are consistent with `<output_type>`. Formally:

      By definition, the variant is of the form
      `((<param0> <param1> ... <param_q>) <_index_args>)`.

      `<output_type>` is greater than or equal to
      the maximum of the set `{ typeof(<param_i>) | i \in [0, q] }`.
      If this variant has no constructor params,
      this variant vacuously satisfies the consistency requirement.

   5. The constructor satisifes the strict positivity requirement.
      TODO: Define this.

### `Nat`

Consider the below type expression for the Peano nat:

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

It has the type `(@cfor () Type0)`, which simplifies to `Type0`.

### `Eq(Nat, zero)`

Consider the below type expression for `Eq(Nat, zero)`:

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

It has the type `(@cfor (Nat) Type0)`, which simplifies to `(for (Nat) Type0)`.

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

            // succ (succ's details are irrelevant for this example)
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
