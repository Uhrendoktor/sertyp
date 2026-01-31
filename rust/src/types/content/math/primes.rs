use crate::{Content, Integer, TypedItem};

/// For more information visit the typst documentation: [math.primes](https://typst.app/docs/reference/math/primes/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Primes {
    pub count: TypedItem<Integer>,
}

crate::impl_all!(Content<'a>::MathPrimes, Primes {'a}, "math.primes");
