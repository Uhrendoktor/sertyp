# Sertyp

Type-preserving serialization for Typst. Serialize Typst values to CBOR and work with them in Rust WASM plugins using typed data structures.

## What is this?

Sertyp bridges Typst and Rust for WASM plugin development. Instead of passing untyped bytes or parsing strings, you get:

- **Typst package**: Serialize any Typst value (colors, lengths, content, etc.) to CBOR
- **Rust library**: Deserialize CBOR into typed structs with utility methods
- **Full roundtrips**: No information loss — serialize and deserialize without degradation

Example: Pass `rgb(255, 128, 0)` to a plugin, work with it as `Color { space: RGB, components: [255, 128, 0] }` in Rust, and return typed results back to Typst.

## Quick Start

### In Typst

```typst
#import "@preview/sertyp:0.1.1";
#let fibonacci(n) = {
    let plugin = plugin("<...>.wasm");
    sertyp.call(plugin.fibonacci, n);
}

#assert(fibonacci(10) == 89)
```

### In Rust (plugin)

```rust
use wasm_minimal_protocol::*;
use sertyp::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

#[typst_func]
pub fn fibonacci<'a>(
    n: Integer,
) -> sertyp::Result<'a, Integer> {
    let n: i32 = match n.try_into() {
        Ok(n) => n,
        Err(_) => return Err("Invalid integer range".into()).into()
    };
    
    let (mut v0, mut v1) = (0, 1);
    for _ in 0..n {
        (v0, v1) = (v1, v0 + v1);
    }

    Ok(v1.into()).into()
}
```

## Supported Types

- **Primitives**: bool, int, float, decimal, string, bytes, none, auto
- **Collections**: array, dictionary
- **Visual**: color (RGB, CMYK, OkLab, HSL, etc.), stroke, gradient, alignment
- **Numeric**: length, angle, ratio, fraction, duration, relative
- **Content**: content, label, symbol, regex
- **Advanced**: function, module, type, arguments, styles, datetime, version

See [typst/README.md](typst/README.md) for the full list and examples.

## Why Sertyp?

Typst's built-in WASM communication uses untyped bytes. You either:
- Parse `repr()` strings (fragile, loses structure)
- Use `cbor.encode()` directly (loses type information)
- Manually build serialization for each type

Sertyp provides:
- **Type preservation**: Lengths stay lengths, colors stay colors
- **Structured access**: Extract color components, decompose lengths, parse function names
- **Zero boilerplate**: Automatic serialization for all supported types

## Documentation and further examples

- **[Typst package](typst/README.md)**: API reference, examples
- **[Rust library](rust/README.md)**: Data structures, common patterns, plugin development

## Contributing

Issues and PRs welcome

## Limitations

- **Security**: Deserialization uses `eval()` — only deserialize trusted data
- **Dynamic types**: `context` and runtime-dependent elements cannot be fully serialized
- **Closures**: Inline functions lose their captured state

See individual READMEs for details.
