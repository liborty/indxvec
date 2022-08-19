#![warn(missing_docs)]
//! Vecs indexing, ranking, sorting, merging, searching, reversing, 
//! intersecting, printing, etc.

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
use core::{ops::Range};

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
            self.min,
            self.minindex,
            self.max,
            self.maxindex
        )
    }
}

/// General Binary Search
/// Search only within the specified range, which is always ascending. 
/// Probe obtains the data value T at key:usize, by any means, 
/// from any kind of sorted data or monotonic function.
/// The sort order can be either ascending or descending (increasing/decreasing).
/// PartialOrd has to be implemented separately for custom types T.
/// When item is in order before range.start, Err(range.start) is returned.
/// When item is in order after range.end, Err(range.end) is returned.
/// Otherwise binary_find returns Range of all the consecutive values PartiallyEqual to the sought item:&T.
/// When item was not found, then the returned_range will be empty and 
/// returned_range.start (and end) will give the sort position where the item can be inserted.
pub fn binary_find<T,F>(range:Range<usize>,probe: F, item:&T )  
    -> Result<Range<usize>,usize> where T:PartialOrd, F:Fn(usize)->T { 

    // binary search lands possibly anywhere within several matching items
    // closure `last` finds the end of their range   
    let last = |idx:usize| -> usize { 
        let mut lastidx = idx+1;
        for i in idx+1..range.end { // move end up
            if item == &probe(i) { lastidx += 1; } else { break; }; 
        }
        lastidx
    };
    // closure `first` finds the start of the range of the matching items  
    let first = |idx:usize| -> usize {
        let mut firstidx = idx;
        for i in (range.start..idx).rev() { // move start down
            if item == &probe(i) { firstidx -= 1; } else { break; }; 
        }
        firstidx
    }; 

    // Checking for errors, special cases and order
    if range.is_empty() { return Err(range.end) }; 
    let firstval = probe(range.start); 
    let lastval = probe(range.end-1);
    // search range data is all equal to item 
    if firstval == lastval { return Ok(range); }; 
    // when data is in descending order, reverse all comparisons
    let ordered = if firstval < lastval { |a:&T,b:&T| a < b } 
    else { |a:&T,b:&T| b < a }; // comparisons closure defined by the sort order
    if ordered(item,&firstval) { return Err(range.start); } // item is before the range.start
    else if ordered(&lastval,item) { return Err(range.end); } // item is beyond the range.end 
    if item == &firstval { // item is equal to the first data item
        return Ok(range.start..last(range.start));
    };
    if item == &lastval { // item is equal to the last data item in range
        return Ok(first(range.end-1)..range.end);
    };

    // Clean binary search
    let mut hi = range.end - 1; // initial high index
    let mut lo = range.start; // initial low index
    loop {
        let mid = lo + (hi-lo) / 2; // binary chop here with truncation
        if mid > lo { // still some range left
            let midval = probe(mid);
            if ordered(&midval,item) { lo = mid; continue; };
            if ordered(item,&midval) { hi = mid; continue; }; 
            // neither greater nor smaller, hence we found match(es) 
            return Ok(first(mid)..last(mid));            
        }
        else { return Ok(hi..hi) }; // interval is exhausted, val not found
    }
}


/// Trait to serialize slices of generic items `&[T]` (vectors)
/// and slices of Vecs of generic items `&[Vec<T>]` (matrices).
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

    /// Method to write vector(s) to file f (space separated, without brackets). 
    /// Passes up io errors
    fn wvec(self,f:&mut File) -> Result<(), io::Error> { 
        Ok(write!(*f,"{} ", self.to_plainstr())?) 
    }

    /// Method to print vector(s) to stdout (space separated,without brackets).
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
    fn unindex<T>(self, v: &[T], ascending: bool) -> Vec<T> where T:Clone;
    /// Correlation coefficient of two &[usize] slices. 
    /// Pearsons on raw data, Spearman's when applied to ranks.
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64>;
}

/// Methods to manipulate generic Vecs and slices of type `&[T]`
pub trait Vecops<T> {  

    /// Helper function to copy and cast entire &[T] to `Vec<f64>`. 
    fn tof64(self) -> Vec<f64> where T: Clone, f64: From<T>;
    /// Maximum value in self
    fn maxt(self) -> T where T: PartialOrd+Clone;
    /// Minimum value in self
    fn mint(self) -> T where T: PartialOrd+Clone;
    /// Minimum and maximum values in self
    fn minmaxt(self) -> (T, T) where T: PartialOrd+Clone;
    /// Returns MinMax{min, minindex, max, maxindex}
    fn minmax(self) -> MinMax<T> where T: PartialOrd+Clone;
    /// MinMax of n items starting at subscript i
    fn minmax_slice(self,i:usize, n:usize) -> MinMax<T> where T: PartialOrd+Clone;
    /// MinMax of a subset of self, defined by its idx subslice between i,i+n.
    fn minmax_indexed(self, idx:&[usize], i:usize, n:usize) -> MinMax<T>
        where T: PartialOrd+Clone;
    /// Reversed copy of self
    fn revs(self) -> Vec<T> where T:Clone;
    /// Repeated items removed
    fn sansrepeat(self) -> Vec<T> where T: PartialEq+Clone;
    /// Some(subscript) of the first occurence of m, or None
    fn member(self, m:T, forward:bool) -> Option<usize> where T: PartialEq+Clone;
    /// Binary search of a slice in ascending or descending order.
    fn binsearch(self, val:&T) -> Range<usize> where T: PartialOrd;
    /// Binary search of an index sorted slice in ascending or descending order. 
    /// Like binsearch but using indirection via idx.
    fn binsearch_indexed(self, idx:&[usize], val:&T) -> Range<usize> where T: PartialOrd;
    /// Counts partially equal occurrences of val by simple linear search of an unordered set
    fn occurs(self, val:T) -> usize where T: PartialOrd;
    /// Unites (concatenates) two unsorted slices. For union of sorted slices, use `merge`
    fn unite_unsorted(self, v: &[T]) -> Vec<T> where T: Clone;
    /// Unites two ascending index-sorted slices.
    fn unite_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Clone; 
    /// Intersects two ascending explicitly sorted generic vectors.
    fn intersect(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Clone;
    /// Intersects two ascending index sorted vectors.
    fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Clone;
    /// Removes items of sorted v2 from sorted self.
    fn diff(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Clone;
    /// Removes items of v2 from self using their sort indices.
    fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Clone;
    /// Divides an unordered set into three: items smaller than pivot, equal, and greater
    fn partition(self, pivot:T) -> (Vec<T>, Vec<T>, Vec<T>)
        where T: PartialOrd+Clone;
    /// Divides an unordered set into three by the pivot. The results are subscripts to self   
    fn partition_indexed(self, pivot: T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
        where T: PartialOrd+Clone;
    /// Merges (unites) two sorted sets, result is also sorted    
    fn merge(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Clone;
    /// Merges (unites) two sets, using their sort indices, giving also the resulting sort index
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
        where T: PartialOrd+Clone;
    /// Used by `merge_indexed`
    fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
        where T: PartialOrd+Clone;
    /// Stable Merge sort main method, giving sort index
    fn mergesort_indexed(self) -> Vec<usize> where T:PartialOrd+Clone;
    /// Utility used by mergesort_indexed
    fn mergesortslice(self, i: usize, n: usize) -> Vec<usize>
        where T: PartialOrd+Clone;
    /// Stable Merge sort, explicitly sorted result obtained via mergesort_indexed 
    fn sortm(self, ascending: bool) -> Vec<T> where T: PartialOrd+Clone;
    /// Rank index obtained via mergesort_indexed
    fn rank(self, ascending: bool) -> Vec<usize> where T: PartialOrd+Clone;
    /// Utility, swaps any two items into ascending order
    fn isorttwo(self,  idx: &mut[usize], i0: usize, i1: usize) -> bool where T:PartialOrd;
    /// Utility, sorts any three items into ascending order
    fn isortthree(self, idx: &mut[usize], i0: usize, i1:usize, i2:usize) where T: PartialOrd; 
    /// Stable hash sort giving sort index
    fn hashsort_indexed(self) -> Vec<usize> 
        where T: PartialOrd+Clone,f64:From<T>;
    /// Utility used by hashsort_indexed
    fn hashsortslice(self, idx: &mut[usize], i: usize, n: usize, min:T, max:T) 
        where T: PartialOrd+Clone,f64:From<T>;
    /// Stable hash sort. Returns new sorted data vector (ascending or descending)
    fn sorth(self, ascending: bool) -> Vec<T> 
        where T: PartialOrd+Clone,f64:From<T>;
    /// Makes a sort index for self, using key generating closure `keyfn`
    fn keyindex(self, keyfn:fn(&T) -> f64, ascending:bool) -> Vec<usize>;
}

/// Mutable Operators on `&mut[T]`
pub trait Mutops<T> {
/// Sorts a mutable slice in place.
 fn mutquicksort(self) where T: PartialOrd;
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
    where T: PartialOrd+Clone, f64:From<T>;
/// Sorts n items from i in self. Used by muthashsort.
fn muthashsortslice(self, i:usize, n:usize, min:T, max:T) 
    where T: PartialOrd+Clone, f64:From<T>;
}