#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer(a, request) = {
  utils.assert_type(a, arguments)

  return request(
    (
      a.pos(),
      a.named(),
    ),
    none,
    ((pos, named), _n) => (
      pos: pos,
      named: named,
    ),
  )
}

#let deserializer(a, ctx, request) = {
  utils.assert_type(a, dictionary)

  import "dictionary.typ" as dict_
  import "array.typ" as array_
  return request(
    (
      (a.at("pos"), array_),
      (a.at("named"), dict_),
    ),
    none,
    ((pos, named), _n) => {
      return arguments(
        ..pos,
        ..named,
      )
    },
  )
}

#let test(cycle) = {
  import "dictionary.typ" as dict_
  import "array.typ" as array_

  let args = arguments(3, 4, foo: "bar", baz: 42, qux: (1, 2))
  cycle(args)
};
