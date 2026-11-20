#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer(m, request) = {
  utils.assert_type(m, module)

  let fn = ()
  for (key, member) in dictionary(m).pairs() {
    fn.push(repr(member))
  }
  let name = repr(m).match(regex("\<module (.*?)\>")).captures.at(0)

  import "array.typ" as array_
  return request(((fn, array_),), name, ((member,), name) => (
    name: name,
    member: member,
  ))
}

#let deserializer(a, ctx, request) = {
  utils.assert_type(a, dictionary)

  return eval("{import \"" + a.at("name") + ".typ\" as __mod_; __mod_}")
}

#let test(cycle) = {
  import "module.typ" as mod_

  utils.assert(
    generic.serializer(mod_),
    (
      type: "module",
      value: (
        name: "module",
        member: (
          "<module generic>",
          "<module utils>",
          "serializer",
          "deserializer",
          "test",
        ),
      ),
    ),
  )
  let null = cycle(mod_)
  let null = cycle(array_)
}
