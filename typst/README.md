# Sertyp

**Send Typst values to Rust/WASM plugins and get them back as Typst values.**

Sertyp serializes Typst values into a structured, type-preserving
representation. It is useful when a plugin needs more than a string: a color
should arrive as a color, a length as a length, and math or markup content as
content that Typst can still display and use.

## Install

```typst
#import "@preview/sertyp:0.1.5" as sertyp
```

## A first round trip

Serialize a value, then reconstruct it:

```typst
#import "@preview/sertyp:0.1.5" as sertyp

#let value = [
  Total displaced soil by glacial flow:
  $ 7.32 beta + sum_(i=0)^nabla (Q_i (a_i - epsilon)) / 2 $
  #metadata(title: "Glacial Flow Calculation")
]

#let encoded = sertyp.serialize(value)
#let restored = sertyp.deserialize(encoded)

#assert(repr(restored) == repr(value))
```

The serialized form keeps the information needed to rebuild the original
Typst value. Primitive values remain simple; structured values use nested
arrays, dictionaries, and type tags.

## Passing values through a WASM plugin

For plugin calls, Sertyp can encode the intermediate representation as CBOR.
On the Rust side, the
[Rust crate](https://github.com/Uhrendoktor/sertyp/blob/0.1.5/rust/README.md)
turns that data into typed values such as `Item`, `Color`, `Length`, and
`Content`.

The `call` helper takes care of the round trip:

```typst
#import "@preview/sertyp:0.1.5" as sertyp

#let plugin = plugin("path/to/plugin.wasm")
#let result = sertyp.call(plugin.evaluate, $ 1 + 2 $)
```

It serializes each positional argument, calls the plugin, and deserializes the
CBOR result. The plugin can work with Sertyp's Rust types instead of parsing
Typst values from strings.

## Used by Kalt

[Kalt](https://typst.app/universe/package/kalt) is a practical example of this
workflow. It evaluates nested Typst equations in a Rust/WASM backend and
returns the result as a Typst value. Its Typst-side `comp` function uses
`sertyp.call` to pass the equation to Rust and bring the result back.

Kalt supports expressions involving complex numbers, matrices, vectors, and
mathematical functions. See the
[Kalt repository](https://github.com/Uhrendoktor/kalt).

## API

### `serialize(value) -> any`

Converts a Typst value into Sertyp's intermediate representation.

```typst
#let data = sertyp.serialize(rgb(255, 0, 0))
```

### `deserialize(value) -> any`

Reconstructs a Typst value from its intermediate representation.

```typst
#let color = sertyp.deserialize(data)
```

### `serialize-cbor(value) -> bytes`

Encodes the intermediate representation as CBOR, ready to cross a WASM
boundary.

### `deserialize-cbor(bytes) -> any`

Decodes CBOR and reconstructs the corresponding Typst value.

### `call(function, ..args) -> any`

Serializes positional arguments, calls the supplied plugin function with their
CBOR representation, and deserializes the returned CBOR.

### `call-debug(function, ..args) -> any`

Like `call`, but returns the intermediate representation of the plugin result.
This is handy when inspecting what crossed the boundary.

## Supported types

- **Basic:** `bool`, `int`, `float`, `decimal`, `string`, `bytes`,
  `none`, `auto`
- **Collections:** `array`, `dictionary`, `arguments`
- **Numbers and units:** `length`, `angle`, `ratio`, `fraction`,
  `duration`, `relative`
- **Text and content:** `content`, `label`, `regex`, `symbol`,
  `version`, `datetime`
- **Visual:** `color`, `stroke`, `gradient`, `alignment`, `direction`,
  `tiling`
- **Advanced:** `function`, `module`, `selector`, `type`, `styles`
- **Errors:** Sertyp panic values

## Limitations and safety

- Deserialization uses Typst's `eval()` for some values. **Only deserialize
  data you trust.**
- Runtime-dependent values such as `context` cannot be fully serialized.
- Selector serialization is partial.
- Inline functions can be serialized, but their captured closure state is not
  preserved.

## Rust plugin development

The [Rust README](https://github.com/Uhrendoktor/sertyp/blob/0.1.5/rust/README.md)
covers the Rust types, `#[typst_func]`, input and return values, and plugin
development.
