# Sertyp - Rust Backend

Rust data structures for Typst values with serialization and deserialization from and into the sertyp CBOR format. Allows for straighforward communication between typst and WASM plugins.

## Overview

This library provides serialization and deserialization logic for the sertyp CBOR format into a handy rust data-structure and utility functions.

## Supported Types

**Primitives**: `bool`, `int`, `float`, `string`, `bytes`, `none`, `auto`

**Common**: `array`, `dict`, `function`, `type`, `decimal`

**Typst-specific**: `alignment`, `angle`, `color`, `length`, `relative`, `ratio`, `fraction`, `duration`, `datetime`, `symbol`, `label`, `regex`, `stroke`, `gradient`, `tiling`, `direction`, `version`, `module`, `styles`, `content`

## Writing a sertyp powered Rust WASM plugin

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

```typst
#import "@preview/sertyp:0.1.1";
#let fibonacci(n) = {
    let plugin = plugin("./target/wasm32-unknown-unknown/release/test_plugin.wasm");
    sertyp.call(test_plugin.cycle, data);
}

#assert(fibonacci(10) == 89)
```

## Development

### Running Tests

The test suite in `../test_plugin/` validates roundtrip serialization for all types:

```bash
cd ../test_plugin
cargo build --release
# Then run via Typst
typst compile ../test_plugin/test.typ --root ..
```

### Adding New Types

To add a new type:

1. Create `src/types/<newtype>.rs` with serialization logic
2. Add variant to `Item` enum in `src/types/mod.rs`
3. Update `ItemTagged_` enum for CBOR encoding
4. Implement conversion in `From<Item_>` and `From<Item>` blocks
5. Add corresponding Typst module in `../typst/types/<newtype>.typ`
