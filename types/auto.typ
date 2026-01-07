#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer = generic.repr_serializer(type(auto));

#let deserializer(s) = {
  utils.assert_type(s, str);
  return auto;
};

#let test() = {
  generic.test(auto);
}