#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let FIELDS = (
  "linear": ("stops", "angle", "relative", "space"),
  "radial": ("stops", "center", "focal-center", "focal-radius", "radius", "relative", "space"),
  "conic": ("stops", "angle", "center", "relative", "space"),
);

#let serializer(g) = {
  utils.assert_type(g, gradient);
  
  import "function.typ" as func_;
  let kind = g.kind();
  let dict = (
    kind: func_.serializer(kind, ctx: "gradient"),
  );
  for field in FIELDS.at(repr(kind)) {
    dict.insert(
      field, 
      generic.serialize(eval(
        "g."+field+"()", 
        scope: (g: g)
      ))
    );
  }
  return generic.raw_serializer(dictionary)(dict);
};

#let deserializer(d) = {
  utils.assert_type(d, dictionary);

  import "function.typ" as func_;
  let kind = func_.deserializer(d.remove("kind"));

  let args = utils.str_dict();
  for (field, val) in d.pairs() {
      args.insert(field, generic.deserialize(val));
  };
  let stops = args.remove("stops");

  return kind(
      ..stops,
      ..args
  );
}

#let test() = {
  generic.test_repr(gradient.linear(
    ..color.map.viridis,
  ));
};