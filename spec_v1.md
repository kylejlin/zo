# Zo Informal Syntax Spec

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

    // Decreasing index. THIS MUST BE A NUMBER LITERAL.
    // This is equal to the arity
    // if the fun is non-recursive.
    // In this case, the fun is indeed non-recursive,
    // so we set this to the arity (2).
    2

    // Param types
    (Type0 0)

    // Return type
    Type1

    // Body
    (
        ind

        type1

        "Eq"

        // Index types
        (
            // The DB index stack is
            // 0 => self_fun: forall(T': Type0, x': T') -> Type1
            //      Note this is inaccessible since the fun is
            //      declared as non-recursive.
            // 1 => x: T
            // 2 => T: Type0

            2 // y: T
        )

        // Variant constructors
        (
            // The DB index stack is
            // 0 => self_type_constructor: forall(y': T) -> Type1
            //
            // Note that indices are not added to the DB stack in this case.
            // You have to manually add them.
            //
            // 1 => self_fun (inaccessible): forall(T': Type0, x': T') -> Type1
            // 2 => x: T
            // 3 => T: Type0

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

    // Decreasing arg index (in this case, non-recursive)
    1

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

I could write the full code out, but then things would get long.
So, for this section, I will use Zozen's `let` syntax.

### `3`:

```zozen
let Nat =
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

            // zero: self_type_constructor
            (() ())

            // succ: forall(pred: self_type_constructor) -> self_type_constructor
            ((0) ())
        )
    )

let zero = (vcon Nat 0)
let succ = (vcon Nat 1)

return (succ (succ (succ zero)))
```

### `[3]` (i.e., singleton list containing `3`):

```zozen
// START Copy previous code

let Nat =
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

            // zero: self_type_constructor
            (() ())

            // succ: forall(pred: self_type_constructor) -> self_type_constructor
            ((0) ())
        )
    )

let zero = (vcon Nat 0)
let succ = (vcon Nat 1)

// END Copy previous code

let three = (succ (succ (succ zero)))

let List =
    (
        fun

        // Decreasing arg index (in this case, non-recursive)
        1

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

let NatList = (List Nat)
let NatList_nil = (vcon NatList 0)
let NatList_cons = (vcon NatList 1)

return (NatList_cons three NatList_nil)
```

### `(Eq Nat 3 3)`:

```zozen
// START Copy previous code

let Nat =
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

            // zero: self_type_constructor
            (() ())

            // succ: forall(pred: self_type_constructor) -> self_type_constructor
            ((0) ())
        )
    )

let zero = (vcon Nat 0)
let succ = (vcon Nat 1)

// END Copy previous code

let three = (succ (succ (succ zero)))

let Eq =
    (
        fun

        // Decreasing index.
        // This is equal to the arity
        // if the fun is non-recursive.
        // In this case, the fun is indeed non-recursive,
        // so we set this to the arity (2).
        2

        // Param types
        (Type0 0)

        // Return type
        Type1

        // Body
        (
            ind

            type1

            "Eq"

            // Index types
            (
                // The DB index stack is
                // 0 => self_fun: forall(T': Type0, x': T') -> Type1
                //      Note this is inaccessible since the fun is
                //      declared as non-recursive.
                // 1 => x: T
                // 2 => T: Type0

                2 // y: T
            )

            // Variant constructors
            (
                // The DB index stack is
                // 0 => self_type_constructor: forall(y': T) -> Type1
                //
                // Note that indices are not added to the DB stack in this case.
                // You have to manually add them.
                //
                // 1 => self_fun (inaccessible): forall(T': Type0, x': T') -> Type1
                // 2 => x: T
                // 3 => T: Type0

                // refl: self_type_constructor(x)
                // In other words
                // refl: Eq(T, x)[x]
                (() (2))
            )
        )
    )

let EqNat3 = (Eq Nat three)
let refl_nat_3 = (vcon EqNat3 0)
let proof_that_3_equals_3 = refl_nat_3
```
