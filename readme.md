# **Tysauni - Type Safe Unions**

Tysauni is a simple tool for making type safe unions using rusts Enum's.

## **Limitations**

these are only for now if i find better ways to do this i will...

The register macro only works on enums that only contain varients with a single unnamed feild that contains a struct type with the record macro.

the record enum only works on structs.

The reasons for these limitations is that under the hood i cant find the ident

## **Example**

Simply create a Enum that has all your desired union types wrapped in a tuple enum like this:

_! make sure that the enum vars have the same name as the type !_

```rust
use tysauni::{register, record};

#[record]
struct TypeA {
	value: i32
}
#[record]
struct TypeB {
	value: String
}

#[register]
enum SomeRegister {
	TypeA(TypeA),
	TypeB(TypeB),
}

let some_value: SomeRegister = SomeRegister::TypeA(TypeA{value: 10})
```

_! It is advised to use Traits for interacting with the underlying data inside the register as this ensures that each type inside the register has the same forms of interaction. !_

Then to get the value of the object

```rust
let known_value = some_value.resolve();
```

the resolve method will extract the inner value.

to wrap a value in its correct enum use:

```rust
let value_type_a: TypeA = TypeA::new();
let wrapped_value: SomeRegister = SomeRegister<TypeA>::(value_type_a);
```

under the hood this works by adding in a method for the record structs

If you are confused what the usecase of this would be imagine a vector that can hold many different types but they all share a trait that interacts the same with them.

I made this tool to allow me to create a raylib render pool but really any place where you cant define a type with generics or traits this _should_ work.
