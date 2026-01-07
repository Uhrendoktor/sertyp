#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer = generic.raw_serializer(bool);

#let deserializer = generic.raw_serializer(bool);

#let test() = {
  generic.test(true);
  generic.test(false);
};