use crate::Integer;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Primes {
    pub count: Integer
}

crate::impl_all_content!(Primes.Math, "math.primes");