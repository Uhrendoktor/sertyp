#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer = generic.repr_serializer(type(none));

#let deserializer(s) = {
  utils.assert_type(s, str);
  return none;
};

#let test() = {
  generic.test(none);
}