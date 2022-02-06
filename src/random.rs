//use std::fmt::Write;
use crate::here;

/// This is needed for converting random numbers in u64 to f64
pub const MANTISSA_MAX: u64 = (2 ^ f64::MANTISSA_DIGITS) as u64; // is 2^53

/// Generates f64 random numbers in the standardised range [0,1].
/// Very fast and simple, using XOR.
/// For cryptography, use randomness source from the device instead.
/// Takes a mutable seed which is changed and cycled.
/// Based on: George Marsaglia, Xorshift RNGs, Journal of Statistical Software 08(i14), Jan 2003.
pub fn ranf64(rseed: &mut u64) -> f64 {
    let mut locs = *rseed;
    locs ^= locs << 13;
    locs ^= locs >> 7;
    locs ^= locs << 17;
    *rseed = locs; // update the outside seed
    // have to drop 11 most significant digits from random u64 to fit into f64.
    // Normal cast to 53 bit mantissa would drop 11 least significant
    // (most varying) digits, which is not what we want here.
    (locs % MANTISSA_MAX) as f64 / MANTISSA_MAX as f64
}

/// Generates vector of random numbers in the interval [0_f64;1_f64].
/// Seed keeps updating, so we can reuse the same variable.
pub fn ranvf64(size: usize, seed: &mut u64) -> Vec<f64> {
    if size == 0 {
        panic!("{} zero size", here!())
    };
    let mut resvec = Vec::with_capacity(size);
    for _i in 0..size {
        resvec.push(ranf64(seed));
    }
    resvec
}

/// Generates vector of random numbers in interval [0_u8;255_u8].
/// Seed keeps updating, so we can reuse the same variable.
pub fn ranvu8(size: usize, seed: &mut u64) -> Vec<u8> {
    if size == 0 {
        panic!("{} zero size", here!())
    };
    let mut resvec = Vec::with_capacity(size);
    for _i in 0..size {
        resvec.push((256. * ranf64(seed)).floor() as u8)
    }
    resvec
}

/// Generates n vectors of dimension d, filled with random numbers in interval [0_f64;1_f64].
pub fn ranvvf64(d: usize, n: usize, seed: &mut u64) -> Vec<Vec<f64>> {
    if n * d < 1 {
        panic!("{} non positive dimensions", here!())
    }
    let mut v: Vec<Vec<f64>> = Vec::with_capacity(n);
    // each row gets a new seed
    for _i in 0..n {
        v.push(ranvf64(d, seed))
    }
    v
}

/// Generates n vectors of dimension d, filled with random numbers in interval [0_u8;255_u8].
pub fn ranvvu8(d: usize, n: usize, seed: &mut u64) -> Vec<Vec<u8>> {
    if n * d < 1 {
        panic!("{}\n\tnon positive dimensions", here!())
    }
    let mut v: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _i in 0..n {
        v.push(ranvu8(d, seed))
    }
    v
}
