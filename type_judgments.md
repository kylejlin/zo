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

1. Define `(@cfor <parenthesized_param_types> <return_type>)` as
   `(cfor <parenthesized_param_types> <return_type>)` if `<parenthesized_param_types>` is nonempty,
   and `<return_type>` if `<parenthesized_param_types>` is empty.

   "cfor" stands for "collapsing for", since the `for` "collapses"
   when it has zero params.

2. Define `(@capp <callee> <parenthesized_args>)`
   as `(<callee> <parenthesized_args>)` if `<parenthesized_args>` is nonempty,
   and `<callee>` if `<parenthesized_args>` is empty.

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

### General rule

Suppose we have some `vcon` expression.
By definition, it has the form

```zolike
(
    vcon
    <ind_expression>
    <variant_constructor_index>
)
```

In order to have a type,
this must meet the following conditions:

1. `<ind_expression>` is a well-typed `ind` expression.
2. The variant constructor index is valid. Formally:

   `<variant_constructor_index> \in [0, variant_count)`,
   where `variant_count` is the number
   of variants in `<ind_expression>`.

If the above conditions are met,
then this expression has the type

```zolike
(
    @cfor

    (
        (@shift -1 0 (@replace 0 <ind_expression> (param_type_0)))
        (@shift -1 1 (@replace 1 <ind_expression> (param_type_1)))
        (@shift -1 2 (@replace 2 <ind_expression> (param_type_2)))
        ...
        (@shift -1 m (@replace m <ind_expression> (param_type_m)))
    )

    (
        @capp

        <ind_expression>

        (
            (@shift -1 m+1 (@replace m+1 <ind_expression> (iarg0)))
            (@shift -1 m+1 (@replace m+1 <ind_expression> (iarg1)))
            (@shift -1 m+1 (@replace m+1 <ind_expression> (iarg2)))
            ...
            (@shift -1 m+1 (@replace m+1 <ind_expression> (iarg_n)))
        )
    )
)
```

where we define
`((param_type0 ... param_type_m) (iarg0 ... iarg_n))`
as the `<variant_constructor_index>`th variant of
`<ind_expression>`.

### `Nat.zero`

Consider the below expression for `zero`:

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

It has the type `(@cfor () (@capp Nat ()))`,
which simplifies to `Nat`.

### `Nat.succ`

Consider the below expression for `succ`:

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
    1
)
```

It has the type `(@cfor (Nat) (@capp Nat ()))`,
which simplifies to `(for (Nat) Nat)`.
