# Zo

Zo is an experimental programming language.
It is a work in progress, and not suitable for use.

## Brainstorm

### Unanswered questions

- Does it ever make sense to call `self_fun` in a parameterized type?
  - Should we disallow it?
- Match equality generation rules.
  - Exploit constructor injectivity?
- Should match expressions have an explicit output type annotation?
- How to tag types to prevent unwanted duck typing?
- Should string literals be first class, or a derived feature?
- Records, modules, packages

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

#### TODO (Solved): Redesign this

What's to stop people from writing something like `(@type (@for @universe $0))`?
I think I need to add restrictions.

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

            // Therefore...

            ($2 $1 $3) // Return `add(a_pred, b)` where `add` is the outer function.
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

### Inductive type expressions revisited

Let's try redesigning inductive type expressions.
First, let's change `@type` to `@ind`, to free up the keyword `@type` for the type of types.

Here is the classic Peano-style Nat:

```zo
(@ind $0 (@for $0 $1))
```

Next, let's allow types to take indices.

Here's the new way of writing the `Nat` type.

```zo
(
    @ind
    // Indices:
    () // No indices

    // The DeBruijn stack is now
    // $0 => self_type_constructor (Nat)

    // Variant types:

    // zero: Nat
    $0

    // succ: forall(pred: Nat) => Nat
    (
        @for
        (
            $0 // pred: Nat
        )
        $1 // Nat
    )
)
```

The DeBruin index stack is _progressively_ extended `[index_1, index_2, ... index_n, self_type_constructor]`.
Again, note that the stack is progressively extended.
See the Functions section for details.

For each variant type, there must exist some `X` such that the variant type equals `X`
or the variant type equals `(@for (...) X)` where `X` is either:

- The self type constructor
- The self type constructor applied to one or more index arguments.

### Parameterized inductive types

Here is `List` parameterized over some type `T`:

```zo
(
    @fun

    // Param types
    (
        @type // T: Type
    )

    // Return type
    @type

    // Body
    (
        @ind

        // Indices:
        ()

        // The DeBruijn stack is now
        // $0 => List(T) - i.e., self type
        // $1 => List - i.e., self fun // <-- TODO: Is this safe to use in a total language? Then again, we're not aiming for totality anyways.
        // $2 => T

        // nil: List(T)
        $0

        // cons: forall(car: T, cdr: List(T)) => List(T)
        (
            @for
            (
                $2 // car: T
                $0 // cdr: List(T)
            )
            // The DeBruijn stack is now
            // $0 => cdr
            // $1 => car
            // $2 => List(T) - i.e., self type
            // $3 => List - i.e., self fun

            $2 // List(T)
        )
    )
)
```

We could also define it with type indices instead of type params.

**TODO:** Do we need type indices? What if you just call the "self fun" with a new arg?
But that just might lead to infinite recursion at typechecking time.
The only way it would _not_ lead to infinite recursion if the argument eventually decreases
(not necessarily monotonically, though that might make things easier to check).

Ah wait, we definitely need indices if we want types like `Equal`.
If we used params, you could construct `Equal(0, 1)` (and every other possible bogus contradiction).
