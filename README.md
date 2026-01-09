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
#import "@preview/sertyp:0.1.1"

// Serialize data for a plugin
#let data = rgb(255, 128, 0)
#let bytes = sertyp.serialize_cbor(data)

// Call plugin
#let result_bytes = plugin("my_plugin.wasm").process(bytes)

// Deserialize result
#let result = sertyp.deserialize_cbor(result_bytes)
```

### In Rust (plugin)

```rust
use sertyp::{deserialize_cbor, serialize_cbor, Item, Color};

#[wasm_bindgen]
pub fn process(data: &[u8]) -> Vec<u8> {
    let item = deserialize_cbor(data).unwrap();
    
    match item {
        Item::Color(color) => {
            // Work with typed color
            println!("Color space: {:?}", color.space);
            // color.components is an Array
        }
        _ => {}
    }
    
    serialize_cbor(result).unwrap()
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

### Problem

Typst's built-in WASM communication uses untyped bytes. You either:
- Parse `repr()` strings (fragile, loses structure)
- Use `cbor.encode()` directly (loses type information)
- Manually build serialization for each type

### Solution

Sertyp provides:
- **Type preservation**: Lengths stay lengths, colors stay colors
- **Structured access**: Extract color components, decompose lengths, parse function names
- **Zero boilerplate**: Automatic serialization for all supported types

## Documentation

- **[Typst package](typst/README.md)**: API reference, examples, type preservation
- **[Rust library](rust/README.md)**: Data structures, common patterns, plugin development

## Examples

### Pass complex content to a plugin

```typst
#import "@preview/sertyp:0.1.1"

#let content = [
    $ sum_(i=0)^n i = (n(n+1))/2 $
    #table(columns: 2, [A], [B], [1], [2])
]

#let processed = sertyp.deserialize_cbor(
    plugin("processor.wasm").transform(sertyp.serialize_cbor(content))
)
```

### Work with typed data in Rust

```rust
use sertyp::{Length, Angle, Function};

// Extract length components
let length: Length = /* ... */;
println!("Absolute: {}{}", length.abs, length.unit);
println!("Em-relative: {}", length.em);

// Decompose angles
let angle: Angle = /* ... */;
println!("{}°", angle.value); // angle.unit is Degrees or Radians

// Parse function names
let func: Function = /* ... */;
if let Some(name) = func.full_name() {
    println!("Function: {}", name); // "math.floor", "color.rgb", etc.
}
```

## Development

### Building

```bash
# Build Rust library
cd rust
cargo build --release

# Build example plugin (WASM)
cd ../test_plugin
cargo build --release --target wasm32-unknown-unknown

# Test roundtrip serialization
typst compile test.typ --root ..
```

### Testing

The test plugin validates roundtrip serialization for all types:

```bash
cd test_plugin
cargo build --release --target wasm32-unknown-unknown
typst compile test.typ --root ..
```

## License

MIT License - see [LICENSE](LICENSE)

## Contributing

Issues and PRs welcome at https://github.com/Uhrendoktor/sertyp

## Limitations

- **Security**: Deserialization uses `eval()` — only deserialize trusted data
- **Dynamic types**: `context` and runtime-dependent elements cannot be fully serialized
- **Closures**: Inline functions lose their captured state

See individual READMEs for details.
