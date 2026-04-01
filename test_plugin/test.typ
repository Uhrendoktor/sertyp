#import "../typst/lib.typ" as sertyp;
#import "../typst/types/generic.typ" as generic;

#let test_plugin = plugin("./target/wasm32-unknown-unknown/release/test_plugin.wasm")

#let test() = {
  let cycle(data) = {
    let cycled = sertyp.call(test_plugin.cycle, data)

    if type(data) in (content, dictionary, arguments) {
      sertyp.utils.assert(data, cycled)
    } else {
      sertyp.utils.assert(repr(data), repr(cycled))
    }
    sertyp.utils.assert(type(data), type(cycled))
    cycled
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
    "panic",
  ) {
    let mod = generic.type_mod(t)
    let null = mod.test(cycle)
  }

  cycle([
    Total displaced soil by glacial flow:
    $ 7.32 beta + sum_(i=0)^nabla (Q_i (a_i - epsilon)) / 2 $
    #metadata("Glacial Flow Calculation")
    #table(
      columns: (1fr, auto),
      inset: 10pt,
      align: horizon,
      table.header([*Volume*], [*Parameters*]),
      $ pi h (D^2 - d^2) / 4 $,
      [
        $h$: height \
        $D$: outer radius \
        $d$: inner radius
      ],

      $ sqrt(2) / 12 a^3 $, [$a$: edge length],
    )
  ])

  // cause nested/cascaded errors
  sertyp.call(
    test_plugin.not_expecting_error,
    sertyp.call(test_plugin.not_expecting_error, sertyp.error-box(
      "Test Error",
      "This is an intended test error message.",
    )),
  )
}

#test()


#let b = sertyp.serialize(place(box()))

#let annotate(..args) = {
  box(place(..args))
  h(0pt, weak: true)
}

#let a = [A placed #annotate(square(), dy: 2pt)
  square in my text.]

#let a = sertyp.call-debug(
  test_plugin.test_sequence2,
  [a,,,,,,,,,,asdasdasd,],
)
#sertyp.call(test_plugin.test_sequence2, [[] [aasdad,,,,,,asdasdasd,]])
