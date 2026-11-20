#import "generic.typ" as generic;
#import "../utils.typ" as utils;

#let serializer(c, request) = {
  utils.assert_type(c, color)

  import "array.typ" as array_
  import "function.typ" as func_
  return request(
    (
      (c.components(), array_),
      (c.space(), func_),
    ),
    none,
    ((components, space), _n) => (
      components: components,
      space: space,
    ),
  )
}

#let deserializer(a, ctx, request) = {
  utils.assert_type(a, dictionary)

  import "array.typ" as array_
  import "function.typ" as func_
  return request(
    (
      (a.components, array_),
      (a.space, func_),
    ),
    none,
    ((components, space), _n) => {
      return space(..components)
    },
  )
}

#let test(cycle) = {
  let null = cycle(rgb(1, 2, 3))
  let null = cycle(rgb(50%, 25%, 75%))
  let null = cycle(color.hsv(120deg, 50%, 4))
  let null = cycle(color.hsl(240deg, 30%, 60%))
};
