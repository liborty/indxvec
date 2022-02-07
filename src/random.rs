//use std::fmt::Write;
use crate::here;

/// This is needed for converting random numbers in u64 to f64
const MANTISSA_MAX: f64 = (1u64 << f64::MANTISSA_DIGITS) as f64; // is 2^53
const MANTISSA_MASK: u64 = (1u64 << f64::MANTISSA_DIGITS) - 1;   // is 53 low bits set

/// Generates f64 random number in the standardised range [0,1).
/// The range can be linearly transformed to any [min,max]. For example:
/// `(6.*ranf64(seed)).floor() as u8 + 1_u8` generates random numbers in
/// the range [0,5],then adds 1 (min), giving the classical dice roller in [1,6].
/// We have not provided a function for this because you have to specify
/// your own target type to convert to.
///
/// Very fast and simple, using shifts and XOR.
/// Takes a mutable seed which is changed and cycled.
/// Based on: George Marsaglia, Xorshift RNGs, Journal of Statistical Software 08(i14), Jan 2003.
/// For cryptography, use real random source.
#[inline]
pub fn ranf64(rseed: &mut u64) -> f64 {
    let mut seed = *rseed; // load rseed value into a local register
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    *rseed = seed;  // update 'global' rseed for next time use
    // drop 11 digits from u64 to fit into [0.,1.) interval
    (seed & MANTISSA_MASK) as f64 / MANTISSA_MAX
}

/// Generates a vector of random numbers in the interval [0_f64,1_f64).
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

/// Generates a vector of random numbers in the interval [0_u8,255_u8].
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

/// Generates n vectors of size d, filled with random numbers in the interval [0_f64,1_f64).
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

/// Generates n vectors of size d, filled with random numbers in the interval [0_u8,255_u8].
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
