#![warn(missing_docs)]
//! Statistics, Vector Algebra, 
//! Characterising Multidimensional Data, Machine Learning,
//! Data Analysis

/// Implementation of trait Indices for `&[usize]`
pub mod indices; 
/// Utilities for serializing, writing and printing (optionally in colours)
/// generic vectors.
pub mod printing;
/// Implementation of trait Vecops for `&[T]` 
pub mod vecops;
/// Implementation of trait Mutops for `&mut[T]`
pub mod mutops;


use std::io;
use std::io::Write;
use std::fs::File;
use printing::*;

/// Macro `here!()` gives `&str` with the `file:line path::function-name` of where it was called from.
#[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!(
            "\n{}:{} {}",file!(),line!(), &name[..name.len() - 3]
        )
    }};
}

/// Helper function to copy and cast entire &[T] to `Vec<f64>`.
/// Like the standard `.to_vec()` method but also recasts to f64 end type
pub fn tof64<T>(s: &[T]) -> Vec<f64> where T: Copy, f64: From<T>, {
    s.iter().map(|&x| f64::from(x)).collect()
}

/// struct for minimum value, its index, maximum value, its index
#[derive(Default)]
pub struct MinMax<T> {
    /// Minimum value
    pub min: T,
    /// Subscript (index) of the minimum
    pub minindex: usize,
    /// Maximum value
    pub max: T,
    /// Subscript (index) of the maximum
    pub maxindex: usize,
}

/// Display implementation for MinMax struct
impl<T> std::fmt::Display for MinMax<T>
where
    T: Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "min: {GR}{}{UN}, minindex: {YL}{}{UN}, max: {GR}{}{UN}, maxindex: {YL}{}{UN}",
            self.min,
            self.minindex,
            self.max,
            self.maxindex
        )
    }
}

/// Trait to serialize slices of generic items &[T] (vectors)
/// and slices of Vecs of generic items &[Vec<T>] (matrices).
/// All are converted into printable strings and optionally coloured.
/// Also, methods to serialize and render the resulting string
/// in bold ANSI terminal colours.
pub trait Printing<T> where Self: Sized {

    /// Printable in red
    fn rd(self) -> String { format!("{RD}{}{UN}",self.to_str()) }
    /// Printable in green
    fn gr(self) -> String { format!("{GR}{}{UN}",self.to_str()) }
    /// Printable in blue    
    fn bl(self) -> String { format!("{BL}{}{UN}",self.to_str()) }
    /// Printable in yellow
    fn yl(self) -> String { format!("{YL}{}{UN}",self.to_str()) }
    /// Printable in magenta
    fn mg(self) -> String { format!("{MG}{}{UN}",self.to_str()) }
    /// Printable in cyan
    fn cy(self) -> String { format!("{CY}{}{UN}",self.to_str()) }        

    /// Method to write vector(s) to file f (without brackets). 
    /// Passes up io errors
    fn wvec(self,f:&mut File) -> Result<(), io::Error> { 
        Ok(write!(*f,"{} ", self.to_plainstr())?) 
    }

    /// Method to print vector(s) to stdout (without brackets).
    fn pvec(self)  { print!("{} ", self.to_plainstr()) }
    
    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Adds square brackets around Vecs (prettier lists).
    /// Implementation code is in `printing.rs`. 
    fn to_str(self) -> String;

    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Implementation code is in `printing.rs`.
    fn to_plainstr(self) -> String;

}

/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices {

    /// Create a trivial index that embodies the current order
    fn newindex(n:usize) -> Vec<usize> { Vec::from_iter(0..n) }
    /// Invert an index - turns a sort order into rank order and vice-versa
    fn invindex(self) -> Vec<usize>;
    /// complement of an index - reverses the ranking order
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of indices in self.
    fn unindex<T>(self, v: &[T], ascending: bool) -> Vec<T>
        where T:Copy;
    /// Collects values from v, as f64s, in the order given by self index.
    fn unindexf64<T>(self, v: &[T], ascending: bool) -> Vec<f64> where T:Copy,f64: From<T>;
    /// Correlation coefficient of two &[usize] slices. Pearsons on raw data, Spearman's when applied to ranks.
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64>;
}

/// Methods to manipulate generic Vecs and slices of type `&[T]`
pub trait Vecops<T> {
    /// Maximum value in self
    fn maxt(self) -> T where T: PartialOrd+Copy;
    /// Minimum value in self
    fn mint(self) -> T where T: PartialOrd+Copy;
    /// Minimum and maximum values in self
    fn minmaxt(self) -> (T, T) where T: PartialOrd+Copy;
    /// Returns MinMax{min, minindex, max, maxindex}
    fn minmax(self) -> MinMax<T> where T: PartialOrd+Copy;
    /// MinMax of n items starting at subscript i
    fn minmax_slice(self,i:usize, n:usize) -> MinMax<T> where T: PartialOrd+Copy;
    /// MinMax of a subset of self, defined by its idx subslice between i,i+n.
    fn minmax_indexed(self, idx:&[usize], i:usize, n:usize) -> MinMax<T>
        where T: PartialOrd+Copy;
    /// Reversed copy of self
    fn revs(self) -> Vec<T> where T: Copy;
    /// Repeated items removed
    fn sansrepeat(self) -> Vec<T> where T: PartialEq+Copy;
    /// Some(subscript) of the first occurence of m, or None
    fn member(self, m: T) -> Option<usize> where T: PartialEq+Copy;
    /// Binary search for the subscript of the first occurence of val
    fn memsearch(self, val: T) -> Option<usize> where T: PartialOrd;
    /// Binary search for the subscript of the last occurence of val
    fn memsearchdesc(self, val: T) -> Option<usize> where T:PartialOrd;
    /// Binary search for val via ascending sort index i, 
    /// returns subscript of val
    fn memsearch_indexed(self, i: &[usize], val: T) -> Option<usize> where T:PartialOrd;
    /// Backwards binary search for val via descending sort index i,
    /// returns subscript of val 
    fn memsearchdesc_indexed(self, i: &[usize], val: T) -> Option<usize> where T: PartialOrd;
    /// Binary search of an explicitly sorted list in ascending order.
    /// Returns subscript of the first item that is greater than val.
    /// When none are greater, returns s.len()
    fn binsearch(self, val: T) -> usize where T: PartialOrd;
    /// Binary search of an explicitly sorted list in descending order.
    /// Returns subscript of the first item that is smaller than val.
    /// When none are smaller, returns s.len() 
    fn binsearchdesc(self, val: T) -> usize where T: PartialOrd;
    /// Binary search of an index sorted list in ascending order.
    /// Returns subscript of the first item that is greater than val.
    fn binsearch_indexed(self, i:&[usize], val: T) -> usize where T: PartialOrd;
    /// Binary search of an index sorted list in descending order.
    /// Returns subscript of the first item that is smaller than val (in descending order). 
    fn binsearchdesc_indexed(self, i:&[usize], val: T) -> usize where T: PartialOrd;
    /// Counts occurrences of val by simple linear search of an unordered set
    fn occurs(self, val:T) -> usize where T: PartialOrd;
    /// Efficiently counts number of occurences from ascending and descending sorts
    fn occurs_multiple(self, sdesc: &[T], val: T) -> usize where T: PartialOrd+Copy;
    /// Unites (concatenates) two unsorted slices. For union of sorted slices, use `merge`
    fn unite_unsorted(self, v: &[T]) -> Vec<T> where T: Clone;
    /// Unites two ascending index-sorted slices.
    fn unite_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy; 
    /// Intersects two ascending explicitly sorted generic vectors.
    fn intersect(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    /// Intersects two ascending index sorted vectors.
    fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    /// Removes items of sorted v2 from sorted self.
    fn diff(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    /// Removes items of v2 from self using their sort indices.
    fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    /// Divides an unordered set into three: items smaller than pivot, equal, and greater
    fn partition(self, pivot:T) -> (Vec<T>, Vec<T>, Vec<T>)
        where T: PartialOrd+Copy;
    /// Divides an unordered set into three by the pivot. The results are subscripts to self   
    fn partition_indexed(self, pivot: T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
        where T: PartialOrd+Copy;
    /// Merges (unites) two sorted sets, result is also sorted    
    fn merge(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    /// Merges (unites) two sets, using their sort indices, giving also the resulting sort index
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
        where T: PartialOrd+Copy;
    /// Used by `merge_indexed`
    fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
        where T: PartialOrd+Copy;
    /// Stable Merge sort main method, giving sort index
    fn mergesort_indexed(self) -> Vec<usize> where T:PartialOrd+Copy;
    /// Utility used by mergesort_indexed
    fn mergesortslice(self, i: usize, n: usize) -> Vec<usize>
        where T: PartialOrd+Copy;
    /// Stable Merge sort, explicitly sorted result obtained via mergesort_indexed 
    fn sortm(self, ascending: bool) -> Vec<T> where T: PartialOrd+Copy;
    /// Rank index obtained via mergesort_indexed
    fn rank(self, ascending: bool) -> Vec<usize> where T: PartialOrd+Copy;
    /// Utility, swaps any two items into ascending order
    fn isorttwo(self,  idx: &mut[usize], i0: usize, i1: usize) -> bool where T:PartialOrd;
    /// Utility, sorts any three items into ascending order
    fn isortthree(self, idx: &mut[usize], i0: usize, i1:usize, i2:usize) where T: PartialOrd; 
    /// Stable Hash sort
    fn hashsort_indexed(self) -> Vec<usize> 
        where T: PartialOrd+Copy,F64:From<T>;
    /// Utility used by hashsort_indexed
    fn hashsortslice(self, idx: &mut[usize], i: usize, n: usize, min:T, max:T) 
        where T: PartialOrd+Copy,F64:From<T>;
    /// Immutable hash sort. Returns new sorted data vector (ascending or descending)
    fn sorth(self, ascending: bool) -> Vec<T> 
        where T: PartialOrd+Copy,F64:From<T>;
}


/// Wrapper type for custom conversions to f64
pub struct F64(pub f64);

/// Example custom conversion &str -> f64.
/// Quantifies alphabetic order using the first seven bytes
/// that will (almost) fit into an f64. 
/// Good enough to effectively sort most words.
/// Enables hashsort of &str end types.
/// Merge sort does no calculations, just binary set divisions,
/// so PartialOrd is good enough for it. 
impl From<&str> for F64 {
    fn from(s:&str) -> F64 {
        if s.is_empty() { return F64(0_f64) };
        let bytes = s.as_bytes();
        let mut res = bytes[0] as f64;
        for i in 1..7 {
            res *= 256.;
            if i < bytes.len() { res += bytes[i] as f64; }
        };
        F64(res)    
    }
}

impl From<usize> for F64 {
    fn from(s:usize) -> F64 { F64(s as f64) } 
}

impl From<u8> for F64 {
    fn from(s:u8) -> F64 { F64(s as f64)}  
}

impl From<u16> for F64 {
    fn from(s:u16) -> F64 { F64(s as f64) } 
}

impl From<u32> for F64 {
    fn from(s:u32) -> F64 { F64(s as f64) } 
}

impl From<u64> for F64 {
    fn from(s:u64) -> F64 { F64(s as f64) } 
}

impl From<i8> for F64 {
    fn from(s:i8) -> F64 { F64(s as f64)}  
}

impl From<i16> for F64 {
    fn from(s:i16) -> F64 { F64(s as f64) } 
}

impl From<i32> for F64 {
    fn from(s:i32) -> F64 { F64(s as f64) } 
}

impl From<i64> for F64 {
    fn from(s:i64) -> F64 { F64(s as f64) } 
}

impl From<f32> for F64 {
    fn from(s:f32) -> F64 { F64(s as f64) } 
}

impl From<f64> for F64 {
    fn from(s:f64) -> F64 { F64(s) } 
}

/// Applies conversions, including custom, from T to F64 
/// and unwraps to f64
pub fn inf64<T>(arg:T) -> f64 where F64:From<T> {
    let F64(res) = F64::from(arg);
    res
}

/// Mutable Operators on `&mut[T]`
pub trait Mutops<T> {
/// mutable reversal, general utility
fn mutrevs(self);
/// utility that mutably swaps two indexed items into ascending order
fn mutsorttwo(self, i0:usize, i1:usize) -> bool
    where T: PartialOrd;
/// utility that mutably bubble sorts three indexed items into ascending order
fn mutsortthree(self, i0:usize, i1:usize, i2:usize)
    where T: PartialOrd;
/// Possibly the fastest sort for long lists. Wrapper for `muthashsortslice`.
fn muthashsort(self)
    where T: PartialOrd+Copy, F64:From<T>;
/// Sorts n items from i in self. Used by muthashsort.
fn muthashsortslice(self, i:usize, n:usize, min:T, max:T) 
    where T: PartialOrd+Copy, F64:From<T>;
}