#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer(v, request) = {
  utils.assert_type(v, version)

  import "integer.typ" as int_
  return generic.raw_serializer(dictionary)((
    major: int_.serializer(v.major, none),
    minor: int_.serializer(v.minor, none),
    patch: int_.serializer(v.patch, none),
  ), none)
};

#let deserializer(d, ctx, request) = {
  utils.assert_type(d, dictionary)

  import "integer.typ" as int_
  return request(
    (
      (d.at("major"), int_),
      (d.at("minor"), int_),
      (d.at("patch"), int_),
    ),
    none,
    ((major, minor, patch), _n) => {
      return version(major, minor, patch)
    },
  )
};

#let test(cycle) = {
  import "dictionary.typ" as dict_

  for v in (
    version(1, 0, 0),
    version(2, 5, 3),
    version(0, 1, 10),
  ) {
    let null = cycle(v)
  }
}
