#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer(a, request) = {
  utils.assert_type(a, array)
  return request(a, a, (items, _a) => {
    items
  })
};

#let deserializer(a, ctx, request) = {
  utils.assert_type(a, array)

  return request(a.map(v => (v, generic)), none, (a, _n) => {
    a
  })
}

#let test(cycle) = {
  let arr = ()
  cycle(arr)
  for v in (1, 2.5, "test", rgb("#ff0000")) {
    arr = arr + (v,)
    cycle(arr)
  }

  // Exercise serialization of deeply nested arrays without using the
  // roundtrip plugin, so this specifically covers the iterative serializer.
  let nested = 0
  for _ in range(1024) {
    nested = (nested,)
  }
  let serialized = generic.serializer(nested)
  let depth = 0
  while type(serialized) == array {
    utils.assert(serialized.len(), 1)
    serialized = serialized.at(0)
    depth += 1
  }
  utils.assert(depth, 1024)
  utils.assert(serialized, 0)
};
