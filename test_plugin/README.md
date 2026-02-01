# Test Plugin

A test plugin that acts as sertyp test-suite. It validates roundtrip
serialization for all supported types through a WASM plugin.

## Building

Build the plugin in release mode:

```bash
cargo build --release
```

The compiled WASM module will be at
`target/wasm32-unknown-unknown/release/test_plugin.wasm`.

## Running Tests

After building, run the Typst test file from the project root:

```bash
cd ..
typst compile test_plugin/test.typ --root .
```

This will:

- Load the compiled WASM plugin
- Test serialization and deserialization roundtrips for all supported types
- Verify type preservation across the Typst ↔ Rust boundary

## Test Coverage

All supported types are checked with multiple argument permutations each.

See [test.typ](test.typ) for specific test cases and assertions.
