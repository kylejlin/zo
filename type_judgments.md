# Type Judgments

## Contents

- Disclaimer
- Notational conventions
- Params
- `ind` expressions
- `vcon` expressions
- `match` expressions
- `fun` expressions
- Function application
- `for` expressions
- `Type<n>` expressions

## Disclaimer

This spec is written for my personal use.
It contains many notational inconsistencies.
If information is obvious to me, I usually elide it,
even if it's important.
So read it at your own risk.

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

4. We abbreviate "De Bruijn" index as "DB index".

5. We use two contexts: a _type context_ and an _equality context_.
   We abbreviate these as "tcontext" and "econtext", respectively.

   The tcontext is a stack of expressions,
   corresponding to the types of the DB indices.

   The econtext is a stack of pairs of expressions.
   Each pair's elements are judgmentally equal.
   For example, if `econtext = [(0, 1), (6, 4)]`,
   then `0` and `1` are judgmentally equal,
   and `6` and `4` are judgmentally equal.

   You must take care to shift the econtext when
   you modify the tcontext.

6. `(@shift <shift_amount> <cutoff> <expression>)` shifts DB indices.

7. `(@replace <replacee_db_index> <replacement> <target>)`
   replaces occurences of `<replacee_db_index>`
   in `<target>` with `<replacement>`.

## Params

A param `0` has type `tcontext[0]`.
A param `1` has type `tcontext[1]`.
In general, a param `n` has type `tcontext[n]`.

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

If the above conditions are met,
then this expression has the type:

`(@cfor (index_type0 index_type1 ... index_type_n) <output_type>)`.

### `Nat`

Consider the below type expression for the Peano nat:

```zo
(
    ind

    Type0

    "Nat"

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

    Type0

    "Eq"

    // Index types
    (
        Nat
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

## `vcon` expressions

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
this expression must meet the following conditions:

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

### `Eq(Nat, zero).refl`

```zozen
let Nat = ...
let zero = ...
let Eq_Nat_zero =
    (
        ind

        Type0

        "Eq"

        // Index types
        (
            Nat
        )

        // Variant constructors
        (
            // refl: self_type_constructor(zero)
            // In other words
            // refl: Eq(Nat, zero)[zero]
            (() (zero))
        )
    )

return
(
    vcon

    Eq_Nat_zero

    0
)
```

It has the type
`(@cfor () (@capp Eq_Nat_zero zero))`,
which simplifies to
`(Eq_Nat_zero zero)`.

### `Le(zero).step`

```zozen
let Nat = ...
let zero = ...
let succ = ...
let Le_zero = (
    ind

    Type0

    "Le"

    // Index types
    (Nat)

    // Variants
    (
        // DB index stack is
        // 0 => self_type_constructor: forall(rhs: Nat) -> Type0

        // refl
        (() (zero))

        // step
        (
            // Variant constructor param types
            (
                Nat // rhs_pred: Nat

                    // DB index stack is
                    // 0 => rhs_pred: Nat
                    // 1 => self_type_constructor: forall(rhs: Nat) -> Type0

                (1 0) // lhs_le_rhs_pred: Le(lhs)[rhs_pred]
            )

            // Index args

                // DB index stack is
                // 0 => lhs_le_rhs_pred: Le(lhs)[rhs_pred]
                // 1 => rhs_pred: Nat
                // 2 => self_type_constructor: forall(rhs: Nat) -> Type0

            ((succ 1))
        )
    )
)

return
(
    vcon

    Le_zero

    1
)
```

It has the type

```zozen
(
    @cfor

    (
        Nat
        (Le_zero 0)
    )

    (
        @capp

        Le_zero

        (
            (succ 1)
        )
    )
)
```

which simplifies to

```zozen
(
    for
    (Nat (Le_zero 0))
    (Le_zero (succ 1))
)
```

## `match` expressions

### General rule

Suppose we have a `match` expression.
By definition, it must have the form

```zolike
(
    match

    <matchee>

    <return_type>

    (
        <return_val0>
        <return_val1>
        ...
        <return_val_n>
    )
)
```

In order to have a type,
this expression must meet the following conditions:

1. `<matchee>` is well-typed, and its type is an `ind` expression or `ind` app.
2. `<return_type>` is well-typed, and its type is a `Type` expression.
3. The number of `<return_val>`s equals the number of variants in the
   `<matchee>`.
4. For every `<return_val_i>`:
   1. `<return_val_i>` has some type `return_type_i`
      under the extended tcontext and econtext.
   2. `return_type_i` is compatible with `<return_type>`
      under the extended tcontext and econtext.

### Extending the tcontext and econtext

Suppose we have

```zolike
(
    match

    <matchee>

    <return_type>

    (
        <return_val0>
        <return_val1>
        ...
        <return_val_m>
    )
)
```

where `<matchee>` has type `(@capp <ind_type> (<matchee_index0> ... <matchee_index_n>))`.

Then for any `i \in [0, m]`:

First, we define the following:

1. Let the `i`th variant constructor of `<ind_type>` have index args
   `(vcon_index0 ... vcon_index_n)`.
2. Let the `i`th variant constructor of `<ind_type>` have param types
   `(vcon_param_type0 ... vcon_param_type_p)`.

Then:

1. Add `vcon_param_type0`, ..., `vcon_param_type_p` to the tcontext.
2. Upshift the econtext by `p+1`.
3. Add `(vcon_index0 |-> (@shift p+1 0 matchee_index0))`,
   `(vcon_index1 |-> (@shift p+1 0 matchee_index1))`,
   ... `(vcon_index_n |-> (@shift p+1 0 matchee_index_n))`
   to the econtext.
4. Add `((@shift p+1 0 <matchee>) |-> (@capp (vcon <ind_type> i) (0 1 2 ... p)))` to the econtext.

### TODO Examples

## `fun` expressions

### General rule

Suppose we have an expression of the form

```zolike
(
    fun

    <decreasing_param_index>

    (<param_type0> ... <param_type_m>)

    <return_type>

    <body>
)
```

In order to have a type,
the expression must meet the following requirements:

1. `<decreasing_param_index>` is `nonrec` or some `n \in [0, m]`.
2. The `n`th param is decreasing in every recursive call in `<body>`.
   We describe this in more detail later.
3. For every `i`, `<param_type_i>` has a type of `Type<q_i>`.
4. `<return_type>` has a type of `Type<q_return>`.
5. `<body>` has a type of `<return_type>`.

If the following conditions are met,
this expression has the type

```zolike
(
    for

    (<param_type0> ... <param_type_m>)

    <return_type>
)
```

### Decreasing param check

Every recursive call must pass a _syntactic substruct_ of the `n`th param
as the recursive call's `n`th arg.
We define _syntactic substruct_ (abbreviated as "substruct") as follows:

1. Base case: For any `match` expression
   where the matchee is the `n`th param,
   all match case params are substructs.
2. For any `match` expression
   where the matchee is a substruct,
   all match case params are substructs.
3. For any `match` expression where all
   the return values are substructs,
   the `match` expression itself is a substruct.

## Function applications

Suppose we have an expression of the form:

```zolike
(<callee> <arg0> ... <arg_m>)
```

In order to have a type,
the expression must meet the following requirements:

1. `<callee>` has the type `(for (param_type0 ... param_type_m) return_type)`.
   Notice that the number of params must match the number of args (i.e., `m`).
2. For each `i`, `<arg_i>` has the type

   ```zolike
   (@suball i-1 param_type_i)
   ```

   where we define `(@suball <j> <expression>)` as

   ```zolike
   (
       @replace
       <j>
       <arg_j>
       ...
       (
           @replace
           2
           <arg2>
           (
               @replace
               1
               <arg1>
               (@replace 0 <arg0> <expression>)
           )
       )
   )
   ```

If the following conditions are met,
the expression has the type:

```zolike
(@suball m <return_type>)
```

## `for` expressions

Suppose we have an expression of the form:

```zolike
(
    for

    (<param_type0> ... <param_type_m>)

    <return_type>
)
```

In order to have a type,
the expression must meet the following requirements:

1. For every `i`, `<param_type_i>` has the type `Type<q_i>`.
2. `<return_type>` has the type `Type<q_return>`.

If the following conditions are met,
the expression has the type

```zolike
Type<q_max>
```

...where `q_max` equals the maximum of the set
`{ q0, q1, ... q_m, q_return }`.

## `Type<n>` expressions

`Type<n>` has the type `Type<n+1>`.
