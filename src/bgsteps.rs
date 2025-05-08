use crate::RNG_BITWIDTH;
use std::collections::HashMap;

use ark_ff::Field;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::field::Fq;
pub fn baby_steps_giant_steps(g: Fq, a: Fq) -> Option<u64> {
    let baby_steps: HashMap<Fq, u64> = (0..(1 << (RNG_BITWIDTH / 2)))
        .into_par_iter()
        .map(|step: u64| (g.pow([step]), step))
        .collect();
    let alpha_m = g
        .pow([1 << (RNG_BITWIDTH / 2)])
        .inverse()
        .expect("Failed to compute g^(-1<<RNG_BITWIDTH/2)");
    println!("Successfully computed all baby steps.");
    let mut gamma = a.clone();
    for i in 0..(1 << (RNG_BITWIDTH / 2)) as u64 {
        if let Some(&exp) = baby_steps.get(&gamma) {
            return Some((1 << (RNG_BITWIDTH / 2)) * i + exp);
        } else {
            gamma = alpha_m * gamma;
        }
    }
    let res = (0..(1 << (RNG_BITWIDTH / 2)) as u64)
        .into_par_iter()
        .filter_map(|gstep| {
            let gamma = alpha_m.pow([gstep]);
            if let Some(&exp) = baby_steps.get(&(&a * &gamma)) {
                Some((1 << (RNG_BITWIDTH / 2)) * gstep + exp)
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();
    res.first().copied()
}
