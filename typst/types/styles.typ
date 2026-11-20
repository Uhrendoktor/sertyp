#import "../utils.typ" as utils;
#import "generic.typ" as generic;

#let serializer(s, request) = {
  utils.assert(str(type(s)), "styles")

  return request(((s.child, "generic"),), none, ((child,), _n) => child)
}

#let deserializer(s, ctx, request) = {
  utils.assert_type(s, str)
  return eval(s)
}

#let test(cycle) = {
  // TODO: Implement test
}
