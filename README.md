# Zo

Zo is an experimental programming language.
It is a work in progress, and not suitable for use.

## Syntax

### Inductive type expressions

Here is the classic Peano-style Nat:

```zo
(@type $0 (@for $0 $1))
```

Translating to pseudocode:

```
(@type N => N (@for _: N => N))
```

The general `type` syntax is

```
(@type VARIANT1 VARIANT2 ...)
```

A `type` expression expands the DeBruijn index stack by `[self_type]`.

### Constructing members of inductive types (aka "introduction")

Zero:

```zo
(
    @new
    (@type $0 (@for $0 $1))
    :0
)
```

One:

```zo
(
    @new
    (@type $0 (@for $0 $1))
    (:1 :0)
)
```

Two:

```zo
(
    @new
    (@type $0 (@for $0 $1))
    (:1 (:1 :0))
)
```

**TODO: Redesign this**

What's to stop people from writing something like `(@type (@for @universe $0))`?
I think I need to add restrictions.

### Matching against members of inductive types (aka "elimination")

```zo
(
    @match
    // The matchee is zero.
    (
        @new
        (@type $0 (@for $0 $1))
        :0
    )
    // If the matchee is zero, evaluate to one.
    (
        @new
        (@type $0 (@for $0 $1))
        (:1 :0)
    )
    // If the matchee is not zero, evaluate to zero.
    (
        @fun
        (
            (@type $0 (@for $0 $1)) // _a: Nat
        )
        // Ignore the parameter (`_a`) and unconditionally return zero.
        (
            @new
            (@type $0 (@for $0 $1))
            :0
        )
    )
)
```

### Functions

```
(
    @fun

    // Params (both of type `Nat`)
    (
        (@type $0 (@for $0 $1)) // a: Nat
        (@type $0 (@for $0 $1)) // b: Nat
    )

    // Return type
    (@type $0 (@for $0 $1)) // Nat

    // Body:
    (
        // The DeBruijn indices are:
        // $0 => function
        // $1 => b
        // $2 => a

        @match
        $2 // `a`

        // If a is zero:
        $1 // Return `b`

        // If a is succ(a_pred):
        (
            @fun
            (
                (@type $0 (@for $0 $1)) // a_pred: Nat
            )

            // The DeBruijn indices are:
            // $0 => inner function
            // $1 => a_pred
            // $2 => outer function
            // $3 => b
            // $4 => a

            //
            // Therefore...

            ($2 $1 $3) // Return `(add a_pred b)` where `add` is the outer function.
        )
    )
)
```

A fun expression _progressively_ extends the DeBruijn index stack by `[param_1, param_2, ... param_n, self_fun]`.
The extension is progressive, meaning:

- For `param_1`'s type def, the stack is extended by `[]` (i.e., it's not extended at all)
- For `param_2`'s type def, the stack is extended by `param_1`
- For `param_3`'s type def, the stack is extended by `param_2`
- ...so on for the rest of the params.
- For the return type, the stack is extended by `[param_1, param_2, ... param_n]`.
- For the body, the stack is extended by `[param_1, param_2, ... param_n, self_fun]`.
