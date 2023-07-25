use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const POW: u64 = 31;
const MOD: u64 = 1e9 as u64 + 7;

pub fn calculate_poly_hash_for_collection<TElement>(slice: &[TElement]) -> u64
where
    TElement: Hash,
{
    let mut hash = 1;
    let mut pow = 1;

    for element in slice {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);

        hash = (hash + (1 + hasher.finish()).wrapping_mul(pow)) % MOD;
        pow = (pow.wrapping_mul(POW)) % MOD;
    }

    hash
}
