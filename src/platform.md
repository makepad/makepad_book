# Makepad Platform

## Live System

The *Live System* enables updates of Makepad applications during runtime. Full live coding can’t be achieved since Rust is a statically compiled language. Therefore Makepad follows a hybrid approach:

| Layout and style | Application logic |
| --- | --- |
| LIVE DSL (Makepad’s Domain-specific-language for layout and style) | Rust code |
| Live updatable / live coding | Not live updatable / no live coding |
| Interpreted at runtime | Not interpreted at runtime |

*Live DSL* code blocks are embedded in Rust source files via procedural macros.

Example: Button widget with a live codeable color
```rust
live_design! {
    Button = {{Button}} {
        color: #f00
    }
}

#[derive(Live)]
pub struct Button {
   #[live] color: DVec4,
}
```

This `Button` struct defines a button widget with a single field, `color`, which specifies its color. (Additional fields and methods that a real button would require were omitted for the sake of exposition.)

The `#[derive(Live)]` attribute automatically derives the `Live` trait for `Button`, providing all the necessary machinery for `Button` to interact with the live system. In particular, it provides a method that returns type information about `Button`, such as its names and field types.

The `#[live]` attribute informs the `#[derive(Live)]` attribute which fields of `Button` participate in live coding. Conversely, the `#[rust]` attribute would mark fields that should not participate.

The `live_design` macro defines a `live_design` function that registers a block of Live DSL code with the live system. The code within the macro is serialized to a string, which is later parsed by the live system.

Fields of newly created Rust structs are initialized to their default values. Makepad's live system then *applies* the Live DSL definition of the struct, overwriting all fields for which the Live DSL has definitions.

![schematic system](images/Schematic_System.png)

> 💡 No other object can inherit from the same struct, as there can only be one definition of it.

In this case, `Button: {{Button}}` defines a Live DSL object called `Button`. This object "inherits" from a Rust struct also called `Button`. The Live system then applies the struct's Live DSL definition and updates the default `color` value to the one specified in the `Button` Live DSL object.

The Live system enables live coding by providing the machinery for an IDE to:

- connect to a running application
- detect whether the code being edited is part of a Live DSL block
- send the new Live DSL code to the running application as a string
- reapply the definitions in the new Live DSL code to their corresponding structs

The Live DSL resembles serialization formats like JSON. It can be used to initialize and update existing objects, as well as to create new ones. Additionally, it has several features that JSON lacks, making it more suitable for things like overridable styling. One of these features is inheritance, which will be discussed next.

### Inheritance

**Example: Widget with two buttons**

```rust
live_design! {
    TwoButtons = {{TwoButtons}} {
        button_0: {},
        button_1: {
            color: #0f0,
        }
    }
}

#[derive(Live)]
pub struct TwoButtons {
   #[live] button_0: Button,
   #[live] button_1: Button,
}
```

The Live DSL object `TwoButtons`  provides the Live DSL definition for the Rust struct `TwoButtons`.

The previous example ignored that the Live system *expands* the Live DSL definition to its corresponding struct before applying it. Expansion is a form of prototypical inheritance that works as follows:

1. The Live system encounters a Live DSL object that inherits from a Rust struct.
2. It iterates over all of the struct’s fields that are marked with `#[live]`, using the type information provided by the `Live` trait.
3. The system sets all types that have a registered Live DSL definition to their respective Live DSL value.

The recursive overwriting rules are best illustrated by an example:

Object `A`

```rust
A = {{A}}{
    a0: { b0: 1.0, b1: 2.0 },
    a1: { b0: 3.0 },
}
```

Object `B`

```rust
B = <A>{
    a0: { b0: 5.0 },
    a1: { b1: 6.0 },
}
```

Object resulting from overwriting `A` with `B`

```rust
{
    a0: { b0: 5.0, b1: 2.0 },
    a1: { b0: 3.0, b1: 6.0 },
}
```

Object resulting from overwriting `A` with `B`

**Overwriting rules**

- When a field appears in `A`, but not in `B`,
it also appears in the result. (Diagram **Field 1**)
- When a field appears in `B`, but not in `A`,
it also appears in the result. (Diagram **Field 2**)
- When a field appears in both `A` and both `B`,
the version in `B` wins. (Diagram **Field 3B**)

![inheritance.png](images/inheritance_s.png)

> 💡 Expansion is a recursive process. Thus, when the Live system copies a Live DSL definition to set as the value of a field, the Live DSL definition will have already been recursively expanded.

Simplified before / after comparison of the `TwoButtons` Live DSL definition:

Before expansion

```rust
TwoButtons = {{TwoButtons}} {
    button_0: {},
    button_1: {
        color: #0f0,
    }
}

```

After expansion

```rust
TwoButtons = {{TwoButtons}} {
    button_0: {{Button}} {
        color: #f00,
    },
    button_1: {{Button}} {
        color: #0f0,
    }
}
```

- `button_0`
Only the Live DSL definition for `Button` was copied over, so that `button_0` has the same color as the original `Button`.
- `button_1` 
The value of `color` was overridden by copying over `#0f0` from the DSL object.
If the Live DSL definition of `Button` would have had any other fields, they would have appeared unchanged.
