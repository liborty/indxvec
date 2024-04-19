#![warn(missing_docs)]
//! Vecs searching, indexing, ranking, sorting, merging, reversing, intersecting, printing, etc.

/// Implementation of trait Indices for `&[usize]`
pub mod indices;
/// Implementation of trait Mutops for `&mut[T]`
pub mod mutops;
/// Utilities for serializing, writing and printing (optionally in colours) generic vectors.
pub mod printing;
/// Implementation of trait Search for Range<T>
pub mod search;
/// Implementation of trait Vecops for `&[T]`
pub mod vecops;

use core::{
    cmp::{Ordering, Ordering::*, Reverse},
    ops::Range
};
use printing::*;
use std::{collections::BinaryHeap, fs::File, io, io::Write};

/// Macro `here!("message")` gives `&str` with the `file:line path::function-name` of where it was invoked,
/// followed by the passed "message" - useful for informative errors
#[macro_export]
macro_rules! here {
    ($msg:expr) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!(
            "\n{}:{} {} - {}",
            file!(),
            line!(),
            &name[..name.len() - 3],
            $msg
        )
    }};
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
    T: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "min: {GR}{}{UN}, minindex: {YL}{}{UN}, max: {GR}{}{UN}, maxindex: {YL}{}{UN}",
            self.min, self.minindex, self.max, self.maxindex
        )
    }
}

/// function to sort f64s
pub fn qsortf64(v: &mut [f64]) {
    v.sort_unstable_by(|a, b| a.total_cmp(b))
}

/// Trait to serialize tuples `&(T,T)` and `&(T,T,T)` and
/// slices `&[T]`, `&[&[T]]`, `&[Vec<T>]`.
/// Suitable for printing or writing to files pairs, triplets,
/// all kinds of Vecs and slices and irregularly shaped 2D matrices.  
/// All are converted into Strings and optionally decorated and coloured.
/// Included are methods and constants to render the resulting String
/// in six primary bold ANSI terminal colours.
pub trait Printing<T>
where
    Self: Sized,
{
    /// Printable in red
    fn rd(self) -> String {
        format!("{RD}{}{UN}", self.to_str())
    }
    /// Printable in green
    fn gr(self) -> String {
        format!("{GR}{}{UN}", self.to_str())
    }
    /// Printable in blue    
    fn bl(self) -> String {
        format!("{BL}{}{UN}", self.to_str())
    }
    /// Printable in yellow
    fn yl(self) -> String {
        format!("{YL}{}{UN}", self.to_str())
    }
    /// Printable in magenta
    fn mg(self) -> String {
        format!("{MG}{}{UN}", self.to_str())
    }
    /// Printable in cyan
    fn cy(self) -> String {
        format!("{CY}{}{UN}", self.to_str())
    }

    /// Method to write vector(s) to file f (space separated, without brackets).
    /// Passes up io errors
    fn wvec(self, f: &mut File) -> Result<(), io::Error> {
        Ok(write!(*f, "{} ", self.to_plainstr())?)
    }

    /// Method to print vector(s) to stdout (space separated,without brackets).
    fn pvec(self) {
        print!("{} ", self.to_plainstr())
    }

    /// Method to serialize.
    /// Decorates Vecs with square brackets and tuples with round ones.
    /// Implementation code is in `printing.rs`.
    fn to_str(self) -> String;

    /// Method to serialize in minimal form (space separated, no brackets)
    /// Implementation code is in `printing.rs`.
    fn to_plainstr(self) -> String;
}

/// Binary search algoritms implemented on RangeInclusive<T>.
/// Using a closure `cmpr` to sample and compare data to captured target.
pub trait Search<T> {
    /// Unchecked first Ok(hit) or Err(insert order for a missing item).
    fn binary_by(self, cmpr: impl FnMut(T) -> Ordering) -> Result<T, T>;
    /// Unchecked first hit or insert order, and the final search range.
    fn binary_any(&self, cmpr: impl FnMut(T) -> Ordering) -> (T, Range<T>);
    /// General Binary Search, returns the range of all matching items
    fn binary_all(&self, cmpr: impl FnMut(T) -> Ordering) -> Range<T>;
}

/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices {
    /// Indices::newindex(n) creates a new index without rePartialOrdering
    fn newindex(n: usize) -> Vec<usize> {
        Vec::from_iter(0..n)
    }
    /// Invert an index - turns a sort order into rank order and vice-versa
    fn invindex(self) -> Vec<usize>;
    /// complement of an index - reverses the ranking order
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of indices in self.
    fn unindex<T>(self, v: &[T], ascending: bool) -> Vec<T>
    where
        T: Clone;
    /// Correlation coefficient of two &[usize] slices.
    /// Pearsons on raw data, Spearman's when applied to ranks.
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64>;
}

/// Methods to manipulate generic Vecs and slices of type `&[T]`
pub trait Vecops<'a, T> {
    /// Builds Vec<T> from refs in Vec<&T> (inverse of ref_vec())
    fn deref_vec(v: &[&T], rng: Range<usize>) -> Vec<T>
    where
        T: Clone,
    {
        v.iter()
            .take(rng.end)
            .skip(rng.start)
            .map(|&x| x.clone())
            .collect()
    }
    /// Constructs ref wrapped `Vec<&T>` from `&[T] in rng`
    fn ref_vec(self, rng: Range<usize>) -> Vec<&'a T>;
    /// Helper function to copy and cast entire &[T] to `Vec<f64>`.
    fn tof64(self) -> Vec<f64>
    where
        T: Clone,
        f64: From<T>;
    /// Maximum value in self
    fn maxt(self) -> T
    where
        T: PartialOrd + Clone;
    /// Minimum value in self
    fn mint(self) -> T
    where
        T: PartialOrd + Clone;
    /// Minimum and maximum values in self
    fn minmaxt(self) -> (T, T)
    where
        T: PartialOrd + Clone;
    /// Returns MinMax{min, minindex, max, maxindex}
    fn minmax(self) -> MinMax<T>
    where
        T: PartialOrd + Clone;
    /// MinMax of n items starting at subscript i
    fn minmax_slice(self, i: usize, n: usize) -> MinMax<T>
    where
        T: PartialOrd + Clone;
    /// MinMax of a subset of self, defined by its idx subslice between i,i+n.
    fn minmax_indexed(self, idx: &[usize], i: usize, n: usize) -> MinMax<T>
    where
        T: PartialOrd + Clone;
    /// Reversed copy of self
    fn revs(self) -> Vec<T>
    where
        T: Clone;
    /// Repeated items removed
    fn sansrepeat(self) -> Vec<T>
    where
        T: PartialEq + Clone;
    /// Some(subscript) of the first occurence of m, or None
    fn member(self, m: T, forward: bool) -> Option<usize>
    where
        T: PartialEq + Clone;
    /// Counts partially equal occurrences of val by simple linear search of an unPartialOrdered set
    fn occurs(self, val: T) -> usize
    where
        T: PartialOrd;
    /// Unites (concatenates) two unsorted slices. For union of sorted slices, use `merge`
    fn unite_unsorted(self, v: &[T]) -> Vec<T>
    where
        T: Clone;
    /// Unites two ascending index-sorted slices.
    fn unite_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Intersects two ascending explicitly sorted generic vectors.
    fn intersect(self, v2: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Intersects two ascending index sorted vectors.
    fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Removes items of sorted v2 from sorted self.
    fn diff(self, v2: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Removes items of v2 from self using their sort indices.
    fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Divides an unordered set into three: items smaller than pivot, equal, and greater
    fn partition(self, pivot: &T) -> (Vec<T>, Vec<T>, Vec<T>)
    where
        T: PartialOrd + Clone;
    /// Divides an unordered set into three by the pivot. The results are subscripts to self   
    fn partition_indexed(self, pivot: &T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
    where
        T: PartialOrd + Clone;
    /// Binary Search. Automatic descending PartialOrder detection.
    fn binsearch(self, target: &T) -> Range<usize>
    where
        T: PartialOrd + Copy;
    /// Binary Search via index. Automatic descending PartialOrder detection
    fn binsearch_indexed(self, idx: &[usize], target: &T) -> Range<usize>
    where
        T: PartialOrd + Copy;
    /// Merges (unites) two sorted sets, result is also sorted    
    fn merge(self, v2: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Merges (unites) two sets, using their sort indices, giving also the resulting sort index
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
    where
        T: PartialOrd + Clone;
    /// Used by `merge_indexed`
    fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
    where
        T: PartialOrd + Clone;
    /// Stable Merge sort main method, giving sort index
    fn mergesort_indexed(self) -> Vec<usize>
    where
        T: PartialOrd + Clone;
    /// Utility used by mergesort_indexed
    fn mergesortslice(self, i: usize, n: usize) -> Vec<usize>
    where
        T: PartialOrd + Clone;
    /// Stable Merge sort, explicitly sorted result obtained via mergesort_indexed
    fn sortm(self, ascending: bool) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Rank index obtained via mergesort_indexed
    fn rank(self, ascending: bool) -> Vec<usize>
    where
        T: PartialOrd + Clone;
    /// Utility, swaps any two items into ascending order
    fn isorttwo(self, idx: &mut [usize], i0: usize, i1: usize)
    where
        T: PartialOrd;
    /// Utility, sorts any three items into ascending order
    fn isortthree(self, idx: &mut [usize], i0: usize, i1: usize, i2: usize)
    where
        T: PartialOrd;
    /// Stable hash sort giving sort index
    fn hashsort_indexed(self, quantify: impl Copy + Fn(&T) -> f64) -> Vec<usize>
    where
        T: PartialOrd + Clone;
    /// Utility used by hashsort_indexed
    fn hashsortslice(
        self,
        idx: &mut [usize],
        i: usize,
        n: usize,
        min: f64,
        max: f64,
        quantify: impl Copy + Fn(&T) -> f64,
    ) where
        T: PartialOrd + Clone;
    /// Stable hash sort. Returns new sorted data vector (ascending or descending)
    fn sorth(self, quantify: impl Copy + Fn(&T) -> f64, ascending: bool) -> Vec<T>
    where
        T: PartialOrd + Clone;
    /// Heap of k smallest items in no particular order, except the first one is maximum.
    /// Best for finding just the one k-ranked item
    fn smallest_k(&self, k: usize) -> BinaryHeap<&T>
    where
        T: Ord;
    /// Heap of k biggest items in no particular order, except the first one is minimum
    /// Best for finding just the one k complement ranked item (k-th from the end).
    fn biggest_k(&self, k: usize) -> BinaryHeap<Reverse<&T>>
    where
        T: Ord;
    /// Sort index by insert logsort. Preserves data.  

    /// Insert logsort, returns sort index. Reverse `c` for descending order
    fn isort_indexed<F>(self, rng: Range<usize>, c: F) -> Vec<usize>
    where
        F: Fn(&T, &T) -> Ordering;
    /// Insert logsort of refs (within range). Suitable for bulky end-types.
    /// Faster than `isort_indexed`, as it does not construct an explicit index.
    fn isort_refs<F>(self, rng: Range<usize>, c: F) -> Vec<&'a T>
    where
        F: Fn(&T, &T) -> Ordering;
    /// First k sorted items from rng (ascending or descending, depending on `c`)
    /// Faster than `smallest/biggest_k, followed by `.to_sorted_vec()`.
    fn best_k<F>(self, k: usize, rng: Range<usize>, c: F) -> Vec<&'a T>
    where
        F: Fn(&T, &T) -> Ordering;
}

/// Mutable Operators on `&mut[T]`
pub trait Mutops<T> {
    /// Associated method `part` partitions `s: &mut [&T]` within range `rng`, using comparator `c`.  
    /// Suitable pivot should be selected and placed in `s[rng.start]`.  
    /// Returns the boundaries of the rearranged partitions, (eqstart,gtstart), where  
    /// `rng.start..eqstart` (may be empty) contains references to items lesser than the pivot,  
    /// `gtstart-eqstart` is the number (>= 1) of items equal to the pivot (contains undefined references)  
    /// `gtstart..rng.end` (may be empty) contains references to items greater than the pivot.
    fn part(
        s: &mut [&T],
        rng: &Range<usize>,
        c: &mut impl FnMut(&T, &T) -> Ordering,
    ) -> (usize, usize) {
        // get pivot from the first location
        let pivot = s[rng.start];
        let mut eqstart = rng.start;
        let mut gtstart = eqstart + 1;
        for t in rng.start + 1..rng.end {
            match c(s[t], pivot) {
                Less => {
                    s[eqstart] = s[t];
                    eqstart += 1;
                    s[t] = s[gtstart];
                    gtstart += 1;
                }
                Equal => {
                    s[t] = s[gtstart];
                    gtstart += 1;
                }
                Greater => (),
            }
        }
        (eqstart, gtstart)
    }

    /// partitions by bitmask
    fn part_binary(self, rng: &Range<usize>, bitmask: u64) -> usize
    where
        T: Copy, u64: From<T>;
    /// mutable reversal, general utility
    fn mutrevs(self);
    /// utility that mutably swaps two indexed items into ascending order
    fn mutsorttwo(self, i0: usize, i1: usize) -> bool
    where
        T: PartialOrd;
    /// utility that mutably bubble sorts three indexed items into ascending order
    fn mutsortthree(self, i0: usize, i1: usize, i2: usize)
    where
        T: PartialOrd;
    /// Possibly the fastest sort for long lists. Wrapper for `muthashsortslice`.
    fn muthashsort(self, quantify: impl Copy + Fn(&T) -> f64)
    where
        T: PartialOrd + Clone;
    /// Sorts n items from i in self. Used by muthashsort.
    fn muthashsortslice(
        self,
        i: usize,
        n: usize,
        min: f64,
        max: f64,
        quantify: impl Copy + Fn(&T) -> f64,
    ) where
        T: PartialOrd + Clone;
    /// Mutable insert logsort. Pass in reversed comparator `c` for descending sort
    fn mutisort<F>(self, rng: Range<usize>, c: F)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering;
}
