# Indxvec

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github">](https://github.com/liborty/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/d/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/indxvec?logo=rust">](https://docs.rs/indxvec)
[![Actions Status](https://github.com/liborty/indxvec/workflows/compilation/badge.svg)](https://github.com/liborty/indxvec/actions)

Vecs indexing, ranking, sorting, merging, searching, reversing, intersecting, printing, etc.

## The following will import everything

```rust
use indxvec::{ MinMax, here, printing::*, Search, Indices, Vecops, Mutops, Printing };
```

## Description

This crate is lightweight and has no dependencies. The methods of all traits can be functionally chained to achieve numerous manipulations of Ranges, Vecs, and their indices in compact form.

The facilities provided are:

* ranking, sorting (merge sort and hash sort), merging, binary searching, indexing, selecting, partitioning
* many useful operations on generic vectors and their indices
* set operations
* serialising generic slices and slices of vectors to Strings: `to_plainstr()`
* printing generic slices and slices of vectors: `pvec()`
* writing generic slices and slices of vectors to files: `wvec(&mut f)`
* coloured pretty printing (ANSI terminal output, mainly for testing)
* macro `here!()` for more informative errors reporting

It is highly recommended to read and run [`tests/tests.rs`](https://github.com/liborty/indxvec/blob/main/tests/tests.rs) to learn from examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order. It is also necessary to run the timing benchmark `sorts()` on its own for meaningful results.

```bash
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Glossary

* **Sort Index** - is obtained by stable merge sort `sort_indexed`  or by `hashsort_indexed`. The original data is immutable (unchanged). The sort index produced is a list of subscripts to the data, such that the first subscript identifies the smallest item in the data, and so on (in ascending order). Suitable for bulky data that are not easily moved. It answers the question: 'what data item occupies a given sort position?'.

* **Reversing an index** - sort index can be reversed by generic reversal operation `revs()`, or `mutrevs()`. This has the effect of changing between ascending/descending sort orders without re-sorting or even reversing the (possibly bulky) actual data.

* **Rank Index** - corresponds to the given data order, listing the sort positions (ranks) for the data items, e.g.the third entry in the rank index gives the rank of the third data item. Some statistical measures require ranks of data. It answers the question: 'what is the sort position of a given data item?'.

* **Inverting an index** - sort index and rank index are mutually inverse. Thus they can be easily switched by `invindex()`. This is usually the easiest way to obtain a rank index. They will both be equal to `0..n` for data that is already in ascending order.

* **Complement of an index** - beware that the standard reversal will not convert directly between ascending and descending ranks. This purpose is served by `complindex()`. Alternatively, descending ranks can be reconstructed by applying `invindex()` to a descending sort index.

* **Unindexing** - given a sort index and some data, `unindex()` will pick the data in the new order defined by the sort index. It can be used to efficiently transform lots of data vectors into the same (fixed) order. For example: Suppose we have vectors: `keys` and `data_1,..data_n`, not explicitly joined together in some bulky Struct elements. The sort index obtained by: `let indx = keys.sort_indexed()` can then be efficiently applied to sort the data vectors individually, e.g. `indx.unindex(data_n,true)` (false to obtain a descending order at no extra cost).

## Trait `Search`

Contains general purpose binary search `binary_all`. As far as I know, this algorithm is new and unique. It is very  fast, especially over long ranges. It is also very general and capable of many varied applications.

The method is applied to a range of indices of any numeric type. Thus it can be used in functionally chained 'builder style APIs', to select only the subrange matching the target.

It takes a closure that captures the target. The closure fetches the sorted data item (from any source) for the index argument and compares it against the target. It returns `Ordering`, according to how it defines the logic of the match test. Descending order of data is automatically detected and the ordering is automatically swapped.

The search algorithm itself uses this probing to steer the search range towards the match (by reducing the range appropriately). When the target is not present, its sorted insert position is returned instead, as an empty range.

The first hit encountered will be anywhere within a range of matching partially equal items. The algorithm then conducts two more binary searches, in both directions away from the hit. These secondary searches are applied only within the most reduced half ranges obtained from the completed first search. First non-matching positions in both directions are found, giving the final result: the full matching range.

```rust
/// Search algoritms implemented on Range<T>
pub trait Search<T> {

/// Unchecked first hit or sort order, used by `binary-all`
fn binary_any(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> (T, Range<T>)
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>;

/// General Binary Search using a closure to sample its own data and target - gives full matching range, fast
fn binary_all(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> Range<T>
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>;
}
```

## Trait `Indices`

```rust
use indxvec::{Indices};
```

The methods of this trait are implemented for slices of subscripts, i.e. they take the type `&[usize]` as input (self) and produce new index `Vec<usize>`, new data vector `Vec<T>` or `Vec<f64>`, or other results, as appropriate. Please see the Glossary below for descriptions of the indices and operations on them.

```rust
/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices {
    /// Reverse an index slice by simple reverse iteration.
    fn revindex(self) -> Vec<usize>;
    /// Invert an index - turns a sort index into rank index and vice-versa
    fn invindex(self) -> Vec<usize>;
    /// Complement of an index - reverses the ranking order
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of index in self. Or opposite order.
    fn unindex<T: Clone>(self, v:&[T], ascending:bool) -> Vec<T>;
    /// Pearson's correlation coefficient of two slices, typically ranks.  
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64 (self) -> Vec<f64>;
}
```

## Trait Vecops

```rust
use indxvec::{Vecops};
```

The methods of this trait are applicable to all generic slices `&[T]` (the data). Thus they will work on all  Rust primitive numeric end types, such as f64. They can also work on slices holding any arbitrarily complex end type `T`, as long as the required traits, `PartialOrd` and/or `Clone`, are  implemented for `T`.

```rust
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
    fn minmax_slice(self,i:usize, n:usize) -> MinMax<T>
        where T: PartialOrd+Clone;
    /// MinMax of a subset of self in range i..n
    fn minmax_indexed(self, idx:&[usize], i:usize, n:usize) -> MinMax<T>
        where T: PartialOrd+Clone;
    /// Reversed copy of self
    fn revs(self) -> Vec<T> where T:Clone;
    /// Repeated items removed
    fn sansrepeat(self) -> Vec<T> where T: PartialEq+Clone;
    /// Some(subscript) of the first occurence of m, or None
    fn member(self, m:T, forward:bool) -> Option<usize>
        where T: PartialEq+Clone;
    /// Binary search of a slice in ascending or descending order.
    fn binsearch(self, val:&T) -> Range<usize> 
        where T: PartialOrd;
    /// Binary search of an index sorted slice in ascending or descending order. 
    /// Like binsearch but using indirection via idx.
    fn binsearch_indexed(self, idx:&[usize], val:&T) -> Range<usize> 
        where T: PartialOrd;
    /// Counts partially equal occurrences of val 
    /// by simple linear search of an unordered set
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
    /// Divides an unordered set into three by the pivot. 
    /// The results are subscripts to self.   
    fn partition_indexed(self, pivot: T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
        where T: PartialOrd+Clone;
    /// Merges (unites) two sorted sets, result is also sorted    
    fn merge(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Clone;
    /// Merges (unites) two sets, using their sort indices,
    /// giving also the resulting sort index
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize])
        -> (Vec<T>, Vec<usize>) where T: PartialOrd+Clone;
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
    fn isorttwo(self,  idx: &mut[usize], i0: usize, i1: usize) -> bool
        where T:PartialOrd;
    /// Utility, sorts any three items into ascending order
    fn isortthree(self, idx: &mut[usize], i0: usize, i1:usize, i2:usize)
        where T: PartialOrd; 
    /// Stable Hash sort
    fn hashsort_indexed(self) -> Vec<usize> 
        where T: PartialOrd+Clone,f64:From<T>;
    /// Utility used by hashsort_indexed
    fn hashsortslice(self, idx: &mut[usize], i: usize, n: usize, min:T, max:T) 
        where T: PartialOrd+Clone,f64:From<T>;
    /// Stable hash sort. Returns new sorted data vector (ascending or descending)
    fn sorth(self, ascending: bool) -> Vec<T> 
        where T: PartialOrd+Clone,f64:From<T>;
    /// Makes a sort index for self, using key generating closure `keyfn`
    fn keyindex(self, keyfn:fn(&T)->f64, ascending:bool) -> Vec<usize>;
}
```

## Trait Mutops

```rust
use indxvec::{Mutops};
```

This trait contains `muthashsort`, which overwrites `self` with sorted data. When we do not need to keep the original order, this is the most efficient way to sort.

**Nota bene:** `muthashsort` really wins on longer Vecs. For about one thousand items upwards, it is on average about 25%-30% faster than the default Rust (Quicksort) `sort_unstable`.

```rust
/// Mutable Operators on `&mut[T]`
pub trait Mutops<T> {
/// Sorts a mutable slice in place.
 fn mutsort(self) where T:PartialOrd;
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
    where T: PartialOrd+Clone, F64:From<T>;
/// Sorts n items from i in self. Used by muthashsort.
fn muthashsortslice(self, i:usize, n:usize, min:T, max:T) 
    where T: PartialOrd+Clone, F64:From<T>;
}
```

## Trait `Printing`

```rust
use indxvec::Printing;    // the trait methods
use indxvec::printing::*; // the colour constants
```

This trait provides utility methods to 'stringify' (serialise) generic slices and slices of `Vec`s. Also, methods for writing or printing them. Optionally, it enables printing them in bold ANSI terminal colours for adding emphasis. See `tests/tests.rs` for examples of usage.

```rust
pub trait Printing<T> {

    /// Methods to serialize and render the resulting string
    /// in bold ANSI terminal colours.
    fn rd(self) -> String where Self: Sized { 
        format!("{RD}{}{UN}",self.to_str()) }
    fn gr(self) -> String where Self: Sized { 
        format!("{GR}{}{UN}",self.to_str()) }
    fn yl(self) -> String where Self: Sized { 
        format!("{YL}{}{UN}",self.to_str()) }    
    fn bl(self) -> String where Self: Sized { 
        format!("{BL}{}{UN}",self.to_str()) }
    fn mg(self) -> String where Self: Sized { 
        format!("{MG}{}{UN}",self.to_str()) }
    fn cy(self) -> String where Self: Sized { 
        format!("{CY}{}{UN}",self.to_str()) }        

    /// Method to write vector(s) to file f (without brackets). 
    /// Passes up io errors
    fn wvec(self,f:&mut File) -> Result<(), io::Error> where Self: Sized { 
        Ok(write!(*f,"{} ", self.to_plainstr())?) 
    }

    /// Method to print vector(s) to stdout (without brackets).
    fn pvec(self) where Self: Sized { 
        print!("{} ", self.to_plainstr()) 
    }
    
    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Adds square brackets around Vecs (prettier lists).
    /// Implementation code is in `printing.rs`. 
    fn to_str(self) -> String;

    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Implementation code is in `printing.rs`.
    fn to_plainstr(self) -> String;
}
```

The methods of this trait are implemented for generic individual items `T`, for slices `&[T]` for slices of slices `&[&[T]]` and for slices of Vecs `&[Vec<T>]`. Note that these types are normally unprintable in Rust (do not have `Display` implemented).

The following methods: `.to_plainstr`, `.to_str()`, `.gr()`, `.rd()`, `.yl()` `.bl()`, `.mg()`, `.cy()` convert all these types to printable strings. The colouring methods just add the relevant colouring to the formatted output of `.to_str()`.

`fn wvec(self,f:&mut File) -> Result<(), io::Error> where Self: Sized;`  
writes plain space separated values (`.ssv`) to files, possibly raising io::Error(s).

`fn pvec(self) where Self: Sized;`  
prints to stdout.

For finer control of the colouring, import the colour constants from module `printing` and use them in any formatting strings manually. For example,
switching colours:

```rust  
use indxvec::printing::*; // ANSI colours constants
println!("{GR}green text, {RD}red warning, {BL}feeling blue{UN}");
```

Note that all of these methods and interpolations set their own new colour regardless of the previous settings. Interpolating `{UN}` resets the terminal to its default foreground rendering.
`UN` is automatically appended at the end of strings produced by the colouring methods `rd()..cy()`. Be careful to always close with one of these, or explicit `{UN}`, otherwise all the following output will continue with the last selected colour foreground rendering.

Example from `tests/tests.rs`:

```rust
println!("Memsearch for {BL}{midval}{UN}, found at: {}", vm
    .memsearch(midval)
    .map_or_else(||"None".rd(),|x| x.gr())
);
```

`memsearch` returns `Option(None)`, when `midval` is not found in `vm`. Here, `None` will be printed in red, while any found item will be printed in green. This is also an example of how to process `Option`s without the long-winded `match` statements.

## Struct and Utility Functions

```rust
use indxvec::{MinMax,here};
```

* `pub struct Minmax` holds minimum and maximum values of a `Vec` and their indices.
* `here!()` is a macro giving the filename, line number and function name of the place from where it was invoked. It can be interpolated into any error/tracing messages and reports.

## Release Notes (Latest First)

**Version 1.4.3** - Updated dev dependency `ran 1.0.4`. Added github action `cargo check`.

**Version 1.4.2** - Introduced automatic sort order detection in `binary_all`, thus allowing more code simplification in methods `binsearch` and `binsearch_indexed` that depend on it.

**Version 1.4.1** - Rewritten `binsearch` and `binsearch_indexed` from trait Vecops as encapsulations of the general purpose `binary_all` from trait Sort. Reduced the code size.

**Version 1.4.0** - Introduced new trait Search: `impl<T> Search<T> for Range<T>`. The search algorithms can now be applied in 'builder style chained API's', filtering the ranges.

**Version 1.3.11** - Added module `search.rs`. Improved general `binary_any` and `binary_all` search algorithms now within.
