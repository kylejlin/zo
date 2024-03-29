use super::*;

#[test]
fn showcase() {
    let src = r#"
ind Nat
    case zero
    case succ(pred: Nat)
    return Set0

ind Eq0[_: Nat]
    case refl: [zero]
    return Prop0

let one = succ(zero)

ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Set0

// Using aind:
let Eq2 = aind(T: Set0, left: T) Eq2[_: T]
    case refl: [left]
    return Set0
let refl2 = vcon0(T: Set0, left: T) Eq2[_: T]
    case refl: [left]
    return Set0

// Using aind and parameterizing with afun:
let Eq3 = afun(T: Set0, left: T): Set0
    aind Eq2[_: T]
        case refl: [left]
        return Set0
let refl3 = afun(T: Set0, left: T): Eq2(T, left)(left)
    vcon0 Eq2[_: T]
        case refl: [left]
        return Set0

// Flat-equality
fun Fleq(T: Set0, left: T, right: T): Set0
    Eq(T, left)(right)

// Inline the `Eq(T, left)` part:
fun Fleq2(T: Set0, left: T, right: T): Set0
    ind Eq[_: T]
        case refl: [left]
        return Set0
    Eq(right)

ind(T: Set0) List
    case nil
    case cons(car: T, cdr: List)
    return Set0

fun add(-a: Nat, b: Nat): Nat
    match a
    case zero:
        b
    case succ(pred):
        succ(add(pred, b))
    return1 Nat

// Using non-def fun:
let add2 = afun add2(-a: Nat, b: Nat): Nat
    match a
    case zero:
        b
    case succ(pred):
        succ(add2(pred, b))
    return1 Nat

let two = add(one, one)

ind Bool
    case true
    case false
    return Set0

fun not(b: Bool): Bool
    match b
    case true:
        false
    case false:
        true
    return1 Bool

fun not_involutive(b: Bool): Fleq(Bool, b, not(not(b)))
    match b
    case true:
        refl(Bool)(true)
    case false:
        refl(Bool)(false)
    use b1 return1 Fleq(Bool, b, not(not(b1)))

fun filter(T: Set0, -list: List(T), pred: For(_: T) -> Bool): List(T)
    match list
    case nil:
        nil(T)
    case cons(car, cdr):
        match pred(car)
        case true:
            cons(T)(car, filter(cdr, pred))
        case false:
            filter(cdr, pred)
        return1 List(T)
    return1 List(T)

fun eq_commutative(T: Set0, a: T, b: T, eq: Fleq(T, a, b)): Fleq(T, b, a)
    match eq
    case refl:
        refl(T)(a)
    use [c] return Fleq(T, c, a)
    
succ(succ(zero))
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}
