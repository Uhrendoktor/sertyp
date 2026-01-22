use crate::{Integer, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Primes {
    pub count: TypedItem<Integer>
}

crate::impl_all_content!(Primes.Math, "math.primes");