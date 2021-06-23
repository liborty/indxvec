# IndxVec

![Crates.io](https://img.shields.io/crates/v/indxvec?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section:

```rust
indxvec = "^0.1" 
```

Import into your source file(s) macro `here`, struct `GS`, functions and trait as you need. There is just one trait `Indices` implemented on indices of type &[usize]. There is a bunch of functions in module `merge` which usually take generic vector(s) as arguments and may produce some indices.

```rust
use indxvec::{here,GS,merge::*,Indices};
```

See tests/tests.rs for examples of usage. To run the tests, use single thread. It may be slower but will produce the results in the right order:

```rust
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Description

Indxvec is a spin-off from `rstats`. It is a self-contained unit, both in terms of the subject matter and also in not having any dependencies at all.

The tools included are: efficient ranking, sorting, merging, searching and indices manipulations. They are  applicable to generic vectors `Vec<T>` (or generic slices `&[T]`), thus they will work on primitive types but also on any arbitrarily complex end type T, as long as you implement their required traits, mostly just PartialOrd and/or Copy for T.

## Functions

are in the module `src/merge.rs`. They mostly take some generic data and produce the indices onto which the methods of the following trait can be conveniently chained. See the documentation.

## Trait Index

The functions of this trait are implemented for vectors of subscripts, i.e. `&[usize]`.

* `invindex` - method for inverting an index, e.g. given a sort index, returns ranks and vice versa.

* `unindex` - collects values from a vector in the order given by an index. This will, for example, sort a vector into sort order when supplied with a sort index.

* `ucorrelation` - Pearson's correlation coefficient of two indices, typically ranks. This is the same as Spearman's correlation of the original data.

## Release Notes (Latest First)

**Version 0.1.3** - added wrapper struct GS (generic slice), though it is not really needed. However, it does pretty-print generic vectors.
