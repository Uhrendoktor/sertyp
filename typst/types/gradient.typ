#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#import "array.typ" as array_;
#import "function.typ" as func_;
#import "angle.typ" as angle_;
#import "ratio.typ" as ratio_;

#let FIELDS = (
  "linear": (
    "stops": generic,
    "space": func_,
    "relative": generic,
    "angle": angle_,
  ),
  "radial": (
    "stops": generic,
    "space": func_,
    "relative": generic,
    "center": array_,
    "radius": ratio_,
    "focal-center": generic,
    "focal-radius": ratio_,
  ),
  "conic": (
    "stops": generic,
    "angle": angle_,
    "center": array_,
    "relative": generic,
    "space": func_,
  ),
);

#let serializer(g, request) = {
  utils.assert_type(g, gradient)

  let kind = g.kind()
  let fields = FIELDS.at(repr(kind))
  import "function.typ" as func_
  let deps = ((kind, func_),)
  for (field, ty) in fields.pairs() {
    let serializer = if ty == generic {
      generic.serialize_step
    } else {
      ty
    }
    deps.push((
      eval(
        "g." + field + "()",
        scope: (g: g),
      ),
      serializer,
    ))
  }

  return request(deps, fields.keys(), (values, field_names) => {
    let dict = utils.str_dict()
    dict.insert("kind", values.at(0))
    for (i, field) in field_names.enumerate() {
      dict.insert(field, values.at(i + 1))
    }
    dict
  })
}

#let deserializer(d, ctx, request) = {
  utils.assert_type(d, dictionary)
  import "function.typ" as func_

  request(((d.remove("kind"), func_),), d, ((kind,), d) => {
    let deps = ()
    for (field, val) in d.pairs() {
      let ty = FIELDS.at(repr(kind)).at(field)
      deps.push((val, ty))
    }
    request(deps, d, (deps, d) => {
      let args = utils.str_dict()
      for (i, field) in d.keys().enumerate() {
        args.insert(field, deps.at(i))
      }
      let stops = args.remove("stops")

      return kind(
        ..stops,
        ..args,
      )
    })
  })
}


#let test(cycle) = {
  let null = cycle(gradient.linear(
    ..color.map.viridis,
  ))
};
