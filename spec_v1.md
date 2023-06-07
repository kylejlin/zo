# Zo Informal Spec

## Type expressions

Peano Nat:

```zo
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
```

Equal:

```
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

        // Variants
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

Boolean:

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

List:

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
