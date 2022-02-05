pub mod indices;
pub mod merge;
use std::fmt::Write;

/// When printed, turns the terminal foreground rendering to bold green
pub const GR: &str = "\x1B[01;32m";
/// Returns the terminal rendering to default
pub const UN: &str = "\x1B[0m";
/// This is needed for converting random numbers in u64 to f64
pub const MANTISSA_MAX: u64 = (2 ^ f64::MANTISSA_DIGITS) as u64; // is 2^53

/// Macro `here!()` gives `&str` with the `file:line path::function-name` of where it was called from.
/// This string will be rendered in bold red on (linux) terminals, so as to easily find the real error!
#[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!(
            "\n\x1B[01;31m{}:{} {}\x1B[0m",
            file!(),
            line!(),
            &name[..name.len() - 3]
        )
    }};
}

/// Minimum value, its index, Maximum value, its index
#[derive(Default)]
pub struct MinMax<T> {
    pub min: T,
    pub minindex: usize,
    pub max: T,
    pub maxindex: usize,
}
impl<T> std::fmt::Display for MinMax<T>
where
    T: Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "min: {}, minindex {}, max: {}, maxindex: {}",
            self.min.gr(),
            self.minindex.gr(),
            self.max.gr(),
            self.maxindex.gr()
        )
    }
}

/// Helper function to copy and cast entire &[T] to Vec<f64>.
/// Like the standard .to_vec() method but also casts to f64 end type
pub fn tof64<T>(s: &[T]) -> Vec<f64>
where
    T: Copy,
    f64: From<T>,
{
    s.iter().map(|&x| f64::from(x)).collect()
}

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

/// Method `to_str()` to serialize generic items, slices, and slices of slices.
/// Method `gr()` to serialize and make the resulting string come out in bold green when printed
pub trait Printing<T> {
    /// Method `gr()` to serialize and make the resulting string
    /// bold green when printed.
    /// This is the default implementation applicable to all types that
    /// trait `Printing` is implemented for
    fn gr(self) -> String
    where
        Self: Sized,
    {
        format!("{GR}{}{UN}", self.to_str())
    }

    /// Method to serialize generic items, slices, and slices of Vecs.
    fn to_str(self) -> String;
}

impl<T> Printing<T> for T
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.to_string()
    }
}

impl<T> Printing<T> for &[T]
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.iter().fold(String::from("["), |mut s, item| {
            write!(s, " {}", item).ok();
            s
        }) + " ]"
    }
}

impl<T> Printing<T> for &[&[T]]
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.iter().fold(String::from("["), |mut s, &item| {
            writeln!(s, " {}", item.to_str()).ok();
            s
        }) + "]"
    }
}

impl<T> Printing<T> for &[Vec<T>]
where
    T: std::fmt::Display,
{
    fn to_str(self) -> String {
        self.iter().fold(String::from("[\n"), |mut s, item| {
            writeln!(s, " {}", item.to_str()).ok();
            s
        }) + "]"
    }
}

/// This just prints the items one by one instead of serializing
pub fn printvv<T>(s: &[Vec<T>])
where
    T: Copy + std::fmt::Display,
{
    println!("{GR}[");
    for v in s {
        println!(" {}", v.to_str())
    }
    println!("]{UN}");
}

/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices {
    /// Reverse an index slice by simple reverse iteration.
    fn revindex(self) -> Vec<usize>;
    /// Invert an index.
    fn invindex(self) -> Vec<usize>;
    /// complement of the index - turns ranks from/to ascending/descending
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of indices in self.
    fn unindex<T: Copy>(self, v: &[T], ascending: bool) -> Vec<T>;
    /// Collects values from v, as f64s, in the order given by self index.
    fn unindexf64<T: Copy>(self, v: &[T], ascending: bool) -> Vec<f64>
    where
        f64: From<T>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64>;
}
