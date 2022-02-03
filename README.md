# Indxvec

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github">](https://github.com/liborty/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/d/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/indxvec?logo=rust">](https://docs.rs/indxvec)

## Description

`Indxvec` is a self-contained crate: it has no dependencies. It is used by higher level crates  `sets` and `rstats`.

The tools included are: efficient ranking, sorting, merging, searching, set operations and indices manipulations. They are  applicable to generic slices `&[T]`. Thus they will work on Rust primitive end types, such as f64. They can also work on slices holding any arbitrarily complex end type `T`, as long as the required traits, mostly just `PartialOrd` and/or `Copy`, are  implemented for T.

## Usage

### Import into your source file(s) from the top `crate` level these auxiliary utilities, as needed:

`use indxvec::{MinMax,here,tof64};`

* struct `MinMax` - is a wrapper for the minimum value, minimum's index, maximum value, maximum's index, of a vector or a slice
* macro `here` - is for more informative error reports
* function `tof64` - copies and recasts vectors or slices of generic end type T to end type f64, when such conversion is possible.

### Import trait `Indices`

`use indxvec::Indices;`

Trait `Indices` is implemented on type `&[usize]`, i.e. slices of subscripts to slices and vectors.

### Import trait `Printing`

`use indxvec::Printing`;

This trait provides utility methods to stringify (serialise for printing) generic slices and slices of slices. Optionally enables printing in bold green for emphasis (see `tests/tests.rs`).

### Import functions from module `merge.rs`

`use indxvec::{merge::*};`

> These functions usually take some generic slice(s) of data `&[T]` as arguments and produce indices into them of type `Vec<usize>` (index vectors). The methods of `Indices` trait can be conveniently chained onto them.

The following `use` statement imports everything:

`use indxvec::{MinMax,here,wv,wi,printvv,Indices,merge::*};`

## Testing

It is highly recommended to read and run `tests/tests.rs` to learn from examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

`cargo test --release -- --test-threads=1 --nocapture --color always`

## Trait Indices

The methods of this trait are implemented for slices of subscripts, i.e. they take the type `&[usize]` as input (self) and produce new index `Vec<usize>`, new data vector `Vec<T>`, or other results as appropriate:

```rust
/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices { 
    /// Reverse an index slice by simple reverse iteration.
    fn revindex(self) -> Vec<usize>; 
    /// Invert an index.
    fn invindex(self) -> Vec<usize>;
    /// complement of an index - turns ranks from/to 
    /// ascending/descending
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of index in self.
    fn unindex<T: Copy>(self, v:&[T], ascending:bool) -> Vec<T>;
    /// Collects values from v, as f64s, 
    /// in the order given by self index.    
    fn unindexf64<T: Copy>(self, v:&[T], ascending: bool) -> 
        Vec<f64> where f64:From<T>;
    /// Pearson's correlation coefficient of two slices, 
    /// typically the ranks.  
    fn ucorrelation(self, v: &[usize]) -> f64; 
    /// Potentially useful clone-recast of &[usize] to Vec<f64> 
    fn indx_to_f64 (self) -> Vec<f64>;
}
```
## Trait Printing

This trait is implemented for generic individual items T, for slices &[T] and for slices of slices &[&[T]]:

```rust 
/// Method `to_str()` to serialize generic items, slices and slices of slices.
/// Method `gr()` to serialize and make the resulting string 
/// come out in bold green when printed.
pub trait Printing<T> {
    fn gr(self) -> String where Self:Sized {
        format!("{GR}{}{UNGR}",self.to_str())
    }  
    fn to_str(self) -> String; 
}
```

## Functions Signatures

These functions can be found in module `src/merge.rs`:

```rust
/// Maximum value T of slice &[T]
pub fn maxt<T>(v:&[T]) -> T where T:PartialOrd+Copy

/// Minimum value T of slice &[T]
pub fn mint<T>(v:&[T]) -> T where T:PartialOrd+Copy 

/// Minimum and maximum (T,T) of a slice &[T]
pub fn minmaxt<T>(v:&[T]) -> (T,T) where T:PartialOrd+Copy

/// Minimum, minimum's first index, maximum, maximum's first index 
pub fn minmax<T>(v:&[T])  -> MinMax<T> where T: PartialOrd+Copy

// Reverse a generic slice by reverse iteration.
pub fn revs<T>(s: &[T]) -> Vec<T> where T: Copy

/// Removes repetitions from an explicitly ordered set.
pub fn sansrepeat<T>(s:&[T]) -> Vec<T> where T: PartialOrd+Copy

/// Finds the first occurrence of item `m` in slice `s` by full iteration.
pub fn member<T>(s:&[T], m:T) -> Option<usize> where T: PartialOrd+Copy 

/// Binary search of an explicitly sorted list (in ascending order).
pub fn memsearch<T>(s:&[T], val: T)  -> Option<usize> where T: PartialOrd

/// Binary search of an explicitly sorted list (in descending order).
pub fn memsearchdesc<T>(s:&[T], val: T)  -> Option<usize> where T: PartialOrd

/// Binary search of an indexed list (in ascending order).
pub fn memsearch_indexed<T>(s:&[T], i:&[usize], val: T)  -> Option<usize> where T: PartialOrd 

/// Binary search of an explicitly sorted list in ascending order.
pub fn binsearch<T>(s:&[T], val:T)  -> usize where T: PartialOrd

/// Binary search of an explicitly sorted list in descending order.
pub fn binsearchdesc<T>(s:&[T], val:T) -> usize where T: PartialOrd 

/// Counts occurrences of val using ascending and descending sorts of some set
pub fn occurs<T>(sasc:&[T],sdesc:&[T],val:T) -> usize where T: PartialOrd+Copy+Display

/// Unites two ascending explicitly sorted generic slices
pub fn unite<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy

/// Unites two ascending index-sorted generic vectors.
pub fn unite_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T> where T: PartialOrd+Copy

/// Intersects two ascending explicitly sorted generic vectors.
pub fn intersect<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy

/// Intersects two ascending index-sorted generic vectors. 
pub fn intersect_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T> where T: PartialOrd+Copy

/// Sets difference: deleting elements of the second from the first.
pub fn diff<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy

/// Sets difference: deleting elements of the second from the first.
pub fn diff_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T> where T: PartialOrd+Copy

/// Merges two ascending sorted generic vectors.
pub fn merge<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy

/// Merges two ascending sort indices.
pub fn merge_indexed<T>(v1:&[T], idx1: &[usize], v2: &[T], idx2: &[usize]) -> ( Vec<T>,Vec<usize> ) where T: PartialOrd+Copy

/// Merges the sort indices of two concatenated vectors.
fn merge_indices<T>(s: &[T], idx1:&[usize], idx2:&[usize]) -> Vec<usize>
    where T: PartialOrd+Copy

/// Doubly recursive non-destructive merge sort.
pub fn mergesort<T>(s:&[T], i:usize, n:usize) -> Vec<usize> 
    where T: PartialOrd+Copy

/// A wrapper for mergesort, to obtain the sort index
pub fn sortidx<T>(s:&[T]) -> Vec<usize> where T:PartialOrd+Copy

/// Immutable sort. Returns new sorted vector (ascending or descending)
pub fn sortm<T>(s:&[T], ascending:bool) -> Vec<T> where T: PartialOrd+Copy

/// Fast ranking of many T items, with only `n*(log(n)+1)` complexity
pub fn rank<T>(s:&[T], ascending:bool) -> Vec<usize> where T:PartialOrd+Copy 
```

## Release Notes (Latest First)

**Version 1.0.3** - Added utilities functions `maxt, mint, minmaxt`. Rationalised the  functions for printing generic slices and slices of slices. They are now in trait `Printing`, so they are turned into chainable methods. There is now just `.to_str()` and `.gr()`. The latter also serialises the slices to strings but additionally makes them bold green.

**Version 1.0.2** - Added function `occurs` that efficiently counts occurrences of specified items in a set with repetitions.

**Version 1.0.1** - Some code style tidying up. Added function `binsearchdesc` for completeness and symmetry with `binsearch`.

**Version 1.0.0** - `indxvec` has been stable for some time now, so it gets promoted to v1.0.0. There are some improvements to `README.md` to mark the occasion.

**Version 0.2.12** - added utility function `printvv` to prettyprint vectors of vectors.

**Version 0.2.11** - added some badges and improved `readme`.

**Version 0.2.9** - added struct MinMax for returning values from function 'minmax' and displaying them. Removed function `wt` used previously for displaying them as tuples.

**Version 0.2.6** - added `unindexf64` for convenience. Same as `unindex` but the output is always converted to `Vec<f64>`.

**Version 0.2.5** - added `memsearchdesc_indexed` = binary search of a descending indexed set.

**Version 0.2.4** - added helper function `wt` = write tuple. Added `memsearchdesc` = binary search of a descending set.

**Version 0.2.3** - general tidying up and readme update.

**Version 0.2.2** - replaced GV and GI with functions `wv` and `wi` respectively. Added `revindex` to `Indices` trait, so that it can be functionally chained with its other methods.

**Version 0.2.1** - moved GI from `rstats` to here. Fixed `minmax`.

**Version 0.2.0** - added set operations: `sansrepeat, member, memsearch, memsearch_indexed, unite, unite_indexed, intersect, intersect_indexed, diff, diff_indexed`.  They are also used, with type/struct  wrappers, by crate `sets`.
