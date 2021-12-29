# IndxVec

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github">](https://github.com/liborty/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/d/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/indxvec?logo=rust">](https://docs.rs/indxvec)

## Description

`Indxvec` is a self-contained crate: it has no dependencies. It is used by higher level crates  `sets` and `rstats`.

The tools included are: efficient ranking, sorting, merging, searching, set operations and indices manipulations. They are  applicable to generic slices `&[T]`. Thus they will work on Rust primitive end types, such as f64. They can also work on slices holding any arbitrarily complex end type `T`, as long as the required traits, mostly just `PartialOrd` and/or `Copy`, are  implemented for T.

## Usage

Import into your source file(s) from the top `crate` level:
* struct `MinMax`: a convenience wrapper for returning the minimum and the maximum values of a vector or slice and their indices
* macro `here`: for more informative error reports
* functions `wi,wv` to pretty print in green a single generic item and a generic vector, respectively.
* function `printvv` to pretty-print a generic vector of vectors.

Import trait `Indices`

> Trait `Indices` is implemented on type `&[usize]`, i.e. slices of subscripts to slices and vectors (more details below).

Import functions from module `merge.rs` 

> These functions usually take generic slices of data `&[T]` as arguments and produce new index vectors and/or other results (more details below). 

The following `use` statement imports everything from `indxvec`:

`use indxvec::{MinMax,here,wv,wi,printvv,Indices,merge::*};`

It is highly recommended to read and run `tests/tests.rs` to learn from examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

`cargo test --release -- --test-threads=1 --nocapture --color always`

## Functions

are in module `src/merge.rs`. They mostly take some generic slice(s) `&[T]` and produce the indices into them of type `Vec<usize>`, onto which the methods of the `Indices` trait can be conveniently chained. See the documentation.

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

## Release Notes (Latest First)

**Version 1.0.0** - `indxvec` has been stable for some time now, so it gets promoted to v1.0.0. There are some improvements to `README.md` to mark the occasion. 

**Version 0.2.12** - added utility function `printvv` to prettyprint vectors of vectors.

**Version 0.2.11** - added some badges and improved `readme`.

**Version 0.2.9** - added struct MinMax for returning values from function 'minmax' and displaying them. Removed function `wt` used previously for displaying them as tuples.

**Version 0.2.6** - added `unindexf64` for convenience. Same as `unindex` but the output is always converted to `Vec<f64>`.

**Version 0.2.5** - added `memsearchdesc_indexed` = binary search of a descending indexed set.

**Version 0.2.4** - added helper function `wt` = write tuple. Added `memsearchdesc` = binary search of a descending set.

**Version 0.2.3** - general tidying up and readme update.

**Version 0.2.2** - prettification of tests: replaced GV and GI with functions `wv` and `wi` respectively. Added `revindex` to `Indices` trait, so that it can be functionally chained with its other methods.

**Version 0.2.1** - moved GI from `rstats` to here. Fixed `minmax`.

**Version 0.2.0** - added set operations: `sansrepeat, member, memsearch, memsearch_indexed, unite, unite_indexed, intersect, intersect_indexed, diff, diff_indexed`.  They are also used, with type/struct  wrappers, by crate `sets`.
