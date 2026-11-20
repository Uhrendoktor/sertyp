# Sertyp

**Type-preserving serialization between Typst and Rust.**

Sertyp lets a Typst document pass values to a Rust/WASM plugin and get real
Typst values back. Colors stay colors, lengths stay lengths, and content stays
content — rather than being flattened into strings that the Rust side has to
parse by hand.

## How it works

Sertyp has two parts that use the same intermediate format:

- **Typst package:** serializes Typst values into a structured representation
  and, when needed, encodes that representation as CBOR.
- **Rust crate:** decodes the CBOR into typed Rust values such as `Color`,
  `Length`, `Content`, or the general `Item` enum. It can encode the result
  again for Typst to read.

The `sertyp.call` helper handles the round trip: encode the arguments, call
the WASM function, and deserialize its result.

## A small example

A Rust plugin function can use Sertyp's types directly:

```rust
use wasm_minimal_protocol::*;
use sertyp::{typst_func, Integer, String};

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

#[typst_func]
pub fn fibonacci<'a>(n: Integer) -> Result<Integer, String<'a>> {
    let n: i32 = n.try_into().map_err(|_| "Invalid integer range")?;

    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }

    Ok(b.into())
}
```

Call it from Typst with the package:

```typst
#import "@preview/sertyp:0.1.5" as sertyp

#let plugin = plugin("path/to/plugin.wasm")
#assert(sertyp.call(plugin.fibonacci, 10) == 89)
```

Add the Rust crate to your plugin's `Cargo.toml`:

```toml
[dependencies]
sertyp = "0.1.5"
wasm-minimal-protocol = "0.2"
```

The default Rust features include typed content support. See the
[Rust README](rust/README.md) for the available types and feature details.

## Used in practice

[Kalt](https://typst.app/universe/package/kalt) is a good example of what
Sertyp is for. Kalt evaluates nested Typst equations in Rust — including
expressions with complex numbers, matrices, and vectors — while keeping the
input and result in Typst's own value model.

Its Typst-side `comp` function is essentially a small wrapper around
`sertyp.call`: it sends the equation to the WASM backend and receives the
evaluated result as a Typst value. See the
[Kalt source](https://github.com/Uhrendoktor/kalt).

## What can it serialize?

Sertyp supports most commonly used Typst values, including:

- **Basic values:** booleans, integers, floats, decimals, strings, bytes,
  `none`, and `auto`
- **Collections:** arrays, dictionaries, and arguments
- **Numbers and units:** lengths, angles, ratios, fractions, durations, and
  relative lengths
- **Content:** text, equations, symbols, labels, and other supported content
- **Visual values:** colors, strokes, gradients, alignments, directions, and
  tilings
- **Other types:** functions, modules, selectors, types, styles, dates, and
  versions

The [Typst README](typst/README.md) has examples and the full type list.

## Why use Sertyp?

Typst's built-in `repr()` is useful for inspection, but it is not a reliable
data format for moving values between runtimes. Encoding a value as plain CBOR
also does not, by itself, preserve the meaning of Typst-specific types.

Sertyp keeps the structure and type information together, so Rust code can work
with values instead of parsing their textual representation. The
`call` helper also removes most of the repetitive CBOR and WASM boundary
handling.

## Documentation

- [Typst package: usage, API, and examples](typst/README.md)
- [Rust backend: types and plugin development](rust/README.md)

## Limitations and safety

- Deserialization uses Typst's `eval()` for some values. Only deserialize
  data you trust.
- Runtime-dependent values such as `context` cannot be fully serialized.
- Selector support is partial.
- Inline functions can be serialized, but their captured closure state is not
  preserved.

Issues and pull requests are welcome. Sertyp is released under the MIT license.
