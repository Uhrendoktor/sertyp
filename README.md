# Sertyp

Serialization and deserialization of most typst types into an intermediate representation.
Contrary to the string produced by `repr` or `cbor(cbor.encode(..))` this representation stores more type information an is less ambiguous.
Moreover the deserialization produces the actual **displayable** value instead of a representation of it.

The intermediate representation can be used in combination with `cbor` to communicate non ambiguos type information accros the WASM boundary.

## Examples

### WASM
```typst
#import "@preview/sertyp:0.1.1";
#let plugin = plugin("...");

#let I = $mat(1,0;0,1)$
#let It = deserialize_cbor(plugin.transpose(serialize_cbor(I)));
#assert(repr(It) == repr($mat(0,1;1,0)$));
```

### General
```typst
#import "@preview/sertyp:0.1.1";
#let content = [
    Total displaced soil by glacial flow:
    $ 7.32 beta + sum_(i=0)^nabla (Q_i (a_i - epsilon)) / 2 $
    #metadata(title: "Glacial Flow Calculation")
    #table(
        columns: (1fr, auto),
        inset: 10pt,
        align: horizon,
        table.header(
          [*Volume*], [*Parameters*],
        ),
        $ pi h (D^2 - d^2) / 4 $,
        [
          $h$: height \
          $D$: outer radius \
          $d$: inner radius
        ],
        $ sqrt(2) / 12 a^3 $,
        [$a$: edge length]
    )
]
#let serialized = sertyp.serialize(content);
#let deserialized = sertyp.deserialize(serialized);
#assert(repr(deserialized) == repr(content));
```

## Types

Includes support for:
- **Primitives**: `boolean`, `integer`, `float`, `decimal`, `string`, `bytes`, `none`, `auto`
- **Collections**: `array`, `dictionary`
- **Numeric**: `length`, `angle`, `ratio`, `fraction`, `duration`, `relative`
- **Text & Data**: `content`, `label`, `regex`, `symbol`, `version`, `datetime`
- **Visual**: `color`, `stroke`, `gradient`, `alignment`, `direction`, `tiling`
- **Advanced**: `function`, `generic`, `module`, `selector`, `type`


Does not currently include support for:
- **TODO**: `selector` (a custom parser is required for proper serialization of `repr(selector)`)
- **Exceptions**: `context`, ... Most dynamic elements cannot be serialized properly. 