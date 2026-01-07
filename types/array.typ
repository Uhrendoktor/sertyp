#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer(a) = {
  utils.assert_type(a, array);
  return generic.raw_serializer(array)(a.map((item) => {
    generic.serialize(item)
  }));
};

#let deserializer(a) = {
  utils.assert_type(a, array);

  return a.map((item) => {
      return generic.deserialize(item);
  });
}

#let test() = {
  let arr = ();
  generic.test(arr);
  for v in (1, 2.5, "test", rgb("#ff0000")) {
    arr = arr + (v,);
    generic.test(arr);
  }
};