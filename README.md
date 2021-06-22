# IndxVec

![Crates.io](https://img.shields.io/crates/v/indxvec?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section:

```rust
indxvec = "^0.1" 
```

and import into your source file(s) any structs, functions and/or traits that you want:

```rust
use indxvec::{here,GV,merge::*,Indices};
```

## Introduction

Indxvec is a spin-off from `rstats`, as a self-contained unit, both in subject matter and in not having any dependencies. Only `tests.rs` needs  `anyhow`. 

The tools included are: efficient ranking, sorting, merging, searching and indices manipulations. They are  applicable to generic vectors: Vec\<T\>. They will work on any
arbitrarily complex end type T, as long as you implement their required traits, mostly just PartialOrd and/or Copy for T.

## Testing and Examples

Consult `tests/tests.rs`. To run the tests, use single thread. It will be slower but will produce the results in the right order:

```rust
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Macro, struct and functions

The main content is in the module `src/merge.rs`. See the documentation.

## Trait Index

The functions of this trait are implemented for vectors of subscripts, i.e. `&[usize]`.

* `invindex` - method for inverting an index, e.g. given a sort index, returns ranks and vice versa.

* `unindex` - collects values from a vector in the order given by an index. This will, for example, sort a vector into sort order when supplied with a sort index.

* `ucorrelation` - Pearson's correlation coefficient of two indices, typically ranks. This is the same as Spearman's correlation of the original data.
