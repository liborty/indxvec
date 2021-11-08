# IndxVec

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github">](https://github.com/liborty/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indxvec.svg?style=for-the-badge&color=green&logo=rust" height="20">](https://crates.io/crates/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/d/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-indxvec-green?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/indxvec/)

## Usage

Import into your source file(s) macro `here`, trait `Indices` and functions  as needed. The trait `Indices` is implemented on type `&[usize]`, i.e.  subscripts to slices/vectors. The functions are in module `merge.rs`. They usually take generic slices `&[T]` as arguments and produce new index vectors and/or other results. The following `use` statement imports everything:

`use indxvec::{MinMax,here,wv,wi,Indices,merge::*};`

It is highly recommended that you read and run tests/tests.rs to learn from examples of usage. To run the tests, use a single thread. It may be slower but it will write the results in the right order:

`cargo test --release -- --test-threads=1 --nocapture --color always`

## Description

`Indxvec` is a self-contained crate in terms of the subject matter. It does not have any dependencies. Some of its primitive elements are  used by higher level crates  `sets` and `rstats`.

The tools included are: efficient ranking, sorting, merging, searching, set operations and indices manipulations. They are  applicable to generic slices `&[T]`. Thus they will work on Rust  primitive types but also on slices holding any arbitrarily complex end type `T` of your own, as long as you implement for it the required traits, mostly just `PartialOrd` and/or `Copy`.

## Functions

are in the module `src/merge.rs`. They mostly take some generic slice(s) `&[T]` and produce the indices into them of type `Vec<usize>`, onto which the methods of the `Indices` trait can be conveniently chained. See the documentation.

## Trait Index

The methods of this trait are implemented for slices of subscripts, i.e. they take the type `&[usize]` as input (self) and produce new index `Vec<usize>`, new data vector `Vec<T>`, or other results as appropriate.

## Release Notes (Latest First)
**Version 0.2.11** - added some badges and improved `readme`.

**Version 0.2.9** - added struct MinMax for returning values from function 'minmax' and displaying them. Removed function `wt` used previously for displaying them as tuples.

**Version 0.2.6** - added `unindexf64` for convenience. Same as `unindex` but the output is always converted to `Vec<f64>`.

**Version 0.2.5** - added `memsearchdesc_indexed` = binary search of a descending indexed set.

**Version 0.2.4** - added helper function `wt` = write tuple. Added `memsearchdesc` = binary search of a descending set.

**Version 0.2.3** - general tidying up and readme update.

**Version 0.2.2** - prettification of tests: replaced GV and GI with functions `wv` and `wi` respectively. Added `revindex` to `Indices` trait, so that it can be functionally chained with its other methods.

**Version 0.2.1** - moved GI from `rstats` to here. Fixed `minmax`.

**Version 0.2.0** - added set operations: `sansrepeat, member, memsearch, memsearch_indexed, unite, unite_indexed, intersect, intersect_indexed, diff, diff_indexed`.  They are also used, with type/struct  wrappers, by crate `sets`.
