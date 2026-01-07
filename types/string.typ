#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer = generic.raw_serializer(str);

#let deserializer = generic.raw_deserializer(str);

#let test() = {
  for v in ("", "Hello, World!", "1234567890", "Special chars: !@#$%^&*()", "\n\r\t\b\x\\\²\"") {
    generic.test(v);
  }
};