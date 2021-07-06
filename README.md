# IndxVec  ![Crates.io](https://img.shields.io/crates/v/indxvec?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section:

```rust
indxvec = "^0.2" 
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

Indxvec is a spin-off from `rstats`. It is a self-contained unit, both in terms of the subject matter and also in not having any dependencies.

The tools included are: efficient ranking, sorting, merging, searching, set operations and indices manipulations. They are  applicable to generic slices `&[T]`. Thus they will work on primitive types but also on any arbitrarily complex end type `T`, as long as you implement their required traits, mostly just PartialOrd and/or Copy for `T`.

## Functions

are in the module `src/merge.rs`. They mostly take some generic slice(s) `&[T]` and produce the indices into them of type `Vec<usize>`, onto which the methods of the following trait can be conveniently chained. See the documentation.

## Trait Index

The methods of this trait are implemented for slices of subscripts, i.e. they take the type `&[usize]` as input (self).

## Release Notes (Latest First)

**Version 0.2.1** - moved GI from `rstats` to here.

**Version 0.2.0** - added set operations: `sansrepeat, member, memsearch, memsearch_indexed, unite, unite_indexed, intersect, intersect_indexed, diff, diff_indexed`.  They are also used, with type/struct  wrappers, by crate `sets`.

**Version 0.1.9** - added method `complindex` to trait `Indices`.

**Version 0.1.8** - added function `minmax` to module `merge`.

**Version 0.1.7** - added convenience conversion method `indx_to_f64`.

**Version 0.1.6** - improved comments. Used Vec::with_capacity for new vectors of known lengths. Maybe a bit faster but no change in functionality.

**Version 0.1.5** - fixed an inconsistency in `binsearch` result.

**Version 0.1.4** - swapped arguments of `unindex` for compatibility. Added more comments.

**Version 0.1.3** - added wrapper struct GS (generic slice), though it is not really needed. However, it does pretty-print generic vectors.
