# Sertyp - Rust Backend

Rust data structures for Typst values with serialization and deserialization
from and into the sertyp CBOR format. Allows for straighforward communication
between typst and WASM plugins.

## Overview

This library provides serialization and deserialization logic for the **sertyp**
CBOR format into a handy rust data-structure and utility functions.

## Supported Types

**Primitives**: `bool`, `int`, `float`, `string`, `bytes`, `none`, `auto`

**Common**: `array`, `dict`, `function`, `type`, `decimal`

**Typst-specific**: `alignment`, `angle`, `color`, `length`, `relative`,
`ratio`, `fraction`, `duration`, `datetime`, `symbol`, `label`, `regex`,
`stroke`, `gradient`, `tiling`, `direction`, `version`, `module`, `styles`,
`content`

**Content** By enabling the `content` feature, many content types are usable as
well `math.mat`, `math.vec`, `math.accent`, `math.attach`, `metadata`, `text`,
`equation`, and many more.

## Example

### Rust

```rust
use wasm_minimal_protocol::*;
use sertyp::{typst_func, Integer, String};

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

// Result errors are automatically converted to typst panics.
#[typst_func]
pub fn fibonacci<'a>(n: Integer) -> Result<Integer, String<'a>> {
    let n: i32 = n.try_into().map_err(|_| "Invalid integer range")?;

    let (mut v0, mut v1) = (0, 1);
    for _ in 0..n {
        (v0, v1) = (v1, v0 + v1);
    }

    Ok(v1.into())
}
```

_Typst usage below_

Each function decorated with `#[typst_func]` can use the following types

### Input types

It must specify a single argument that must implement `TryFrom<Item<'_>>`.

This behavior is by default supported for:

- All variants defined in `Item<'_>`.
- `TypedContent`: If the `content` feature is enabled, the input parameter may
  be a typed content.
  ```typst
  #[typst_func]
  pub fn example<'a>(
      arg: TypedContent<Matrix<'_>>
  ) -> ... { ... }
  ```
- `TypedArray`: For arrays of specific types
  ```typst
  #[typst_func]
  pub fn example<'a>(
      dirs: TypedArray<Direction>
  ) -> ... { ... }
  ```
- `Pair`: For a tuple of two elements with same type (usefull when transmitting
  Coordinates etc.).
  ```typst
  #[typst_func]
  pub fn example<'a>(
      dirs: Pair<Float>
  ) -> ... { ... }
  ```
- `Or`: When the type may be one of two options. For `auto` and `none` values,
  the shorthands `AutoOr<T>` and `NoneOr<T>` exist as well.
  ```typst
  #[typst_func]
  pub fn example<'a>(
      dirs: Or<Float, TypedArray<Integer>>
  ) -> ... { ... }
  ```

### Output types

It must specify a return type that must implement `Into<Item<'_>>`.

This behavior is by default supported for:

- All types mentioned in the input types section
- `Result` (`std::result::Result<T, sertyp::String<'a>>`). Returning an error
  will automatically be cast into a typst runtime panic.
  ```typst
  #[typst_func]
  pub fn example<'a>(...) -> Result<Integer, String<'a>> { ... }
  ```

### Typst

Plugin functions can easily be imported and called:

```typst
#let plugin = plugin("<...>.wasm");
#let result = sertyp.call(plugin.fibonacci, 10);
#assert result == 89;
```

With a bit more effort this call can be wrapped into a handy typst function.
Writing those wrapper functions is highly recommended, as it makes the
interaction with your plugin much more intuitive.

```typst
#import "@preview/sertyp:0.1.4";

#let fibonacci(n) = {
    let plugin = plugin("<...>.wasm");
    sertyp.call(plugin.fibonacci, n);
}

#assert(fibonacci(10) == 89)
```
