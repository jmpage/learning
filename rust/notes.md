# Rust Notes

## General Syntax

Statements do not return a value, end in a `;`. Expressions do return a value
and omit a trailing semicolon.

Variables and functions:

``` rust
let x: u32 = 1; // type is optional

fn function_name(var_name: type) -> ret_type {
  // etc
}
```

## Testing

Conventions:
- unit tests live in src/ in same files as code that they're testing
- integration tests live in tests/ directory
- rust will not treat tests/common/mod.rs as a test file. Follow this convention
  for test setup code
- binary crates cannot be tested with integration tests because src/main.rs is
  not exposed publicly. Instead, create a binary + library crate with a thin
  main.rs wrapping the library

## Of Note

### Ownership

Rust uses the concept of ownership to establish memory safety without relying on
garbage collection. When an owner goes out of scope, the memory that it owns is
freed automatically.

#### Rules

In rust, a value must follow these rules:

1. It has a variable which is it's owner
2. It only has one owner at a time
3. It is dropped when its owner goes out of scope

#### Moves

Ownership may be transferred between variables through assignment or by passing
the value to a function. When this happens, the first owner becomes invalid: its
value will not be freed when it goes out of scope and the variable can no longer
be referenced without raising an error.

#### References

A value may be "borrowed" from another variable with by referencing it with `&`:

``` rust
print_it(&foo);

fn print_it(value: &String) {
    println!("{}", value);
}
```

Referenced values are immutable by default.

##### Mutability

References may be made mutable via `&mut`:

``` rust
bang(&mut foo);

fn bang(value: &mut String) {
    value.push_str("!");
}
```

Caveats:

1. There may only be one mutable reference in a given scope per value being
   referenced.
2. An immutable and mutable reference to the same data cannot coexist in the
   same scope. The last reference will invalidate the previous ones.

### Types

#### Structs

A struct may be mutable if the variable is mutable. It's all or nothing: structs
may not have mutable and immutable components.

``` rust
struct Foo {
  bar: String,
  baz: u32
}

// An immutable struct
let bar = String::from("test");
let foo = Foo {
  bar,
  baz: 0
};

println!("bar: {}", foo.bar);

// A mutable struct
let mut faz = Foo {
  baz: 0,
  ..foo // Update syntax: copies remaining valus from foo
};

faz.baz = 1;

println!("baz: {}", faz.baz);
```

##### Tuple Structs

``` rust
struct Point(i32, i32, i32);
struct Vector(Point, Point);
```

##### Unit Structs

Have no fields and behave similarly to unit type `()`

##### Ownership

Structs may store references to data owned by something else. However,
lifetimes must be used in order to accomplish this.

#### Enums

``` rust
// example from rust book ch 06
enum IpAddrKind {
  V4,
  V6
}

let four = IpAddrKind::V4;

// example with tuple structs as enum variants
enum IpAddrKind2 {
  V4(u8, u8, u8, u8),
  V6(String),
}
```
