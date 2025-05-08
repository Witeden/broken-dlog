use crate::field::Fq;
use ark_ff::{BigInt, PrimeField};
use num::BigUint;
pub fn str_to_field_elem(s: &str) -> Fq {
    let n = BigUint::parse_bytes(s.as_bytes(), 10).expect("Invalid number string");
    let bytes = n.to_bytes_le(); // Little-endian byte order

    let mut u64s = [0u64; 32];
    for (i, chunk) in bytes.chunks(8).enumerate() {
        let mut buf = [0u8; 8];
        buf[..chunk.len()].copy_from_slice(chunk);
        u64s[i] = u64::from_le_bytes(buf);
    }

    Fq::from_bigint(BigInt::new(u64s)).expect("Could not convert string to field element.")
}
