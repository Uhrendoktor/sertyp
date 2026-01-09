#import "../typst/lib.typ" as sertyp;
#import "../typst/types/generic.typ" as generic;

#let test() = {
  let test_plugin = plugin("./target/wasm32-unknown-unknown/release/test_plugin.wasm");

  let c = sertyp.deserialize-cbor(test_plugin.fibonacci(sertyp.serialize-cbor(10)));

  let cycle(data) = {
    let cycled = sertyp.deserialize-cbor(test_plugin.cycle(sertyp.serialize-cbor(data)));

    if type(data) in (dictionary, arguments) {
      sertyp.utils.assert(data, cycled);
    } else {
      sertyp.utils.assert(repr(data), repr(cycled));
    }
    sertyp.utils.assert(type(data), type(cycled));
    cycled;
  }

  for t in (
    str,
    int,
    float,
    bytes,
    bool,
    label,
    type,
    decimal,
    array,
    dictionary,
    content,
    function,
    arguments,
    "styles",
    length,
    relative,
    angle,
    fraction,
    ratio,
    color,
    gradient,
    tiling,
    symbol,
    version,
    datetime,
    duration,
    module,
    regex,
    alignment,
    direction,
    stroke,
  ) {
    let mod = generic.type_mod(t)
    let null = mod.test(cycle);
  }

  cycle([
    Total displaced soil by glacial flow:
    $ 7.32 beta + sum_(i=0)^nabla (Q_i (a_i - epsilon)) / 2 $
    #metadata("Glacial Flow Calculation")
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
  ]);
}

#test()