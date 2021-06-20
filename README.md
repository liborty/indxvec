# IndxVec

![Crates.io](https://img.shields.io/crates/v/indxvec?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section:

```rust
indxvec = "^0.1" 
```

and import into your source file(s) any structs, functions and/or traits that you want:

```rust
use indxvec::{....};
```

## Introduction

Indxvec was spun off `rstats`, as it holds sufficient interest in its own right. The general tools included here are efficient ranking, sorting, merging, searching and indices manipulations.

### Implementation

Indxvec is a lean minimalistic library that only depends on *anyhow* (for its simple error handling).

### Documentation

Follow the documentation link. Then select a trait of interest to see the skeletal comments on the prototype function declarations in lib.rs. To see more detailed comments, plus some examples from the implementation files, scroll to the bottom of the trait and unclick [+] to the left of the `implementations` of the trait.

To see tests, consult `tests.rs`. To run the tests, use single thread. It will be slower but will produce the results in the right order:

```rust
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Macro, structs and functions

## Traits

### Index

The functions of this trait are implemented for vectors of subscripts, i.e. `&[usize]`.

* `ucorrelation`(self, v: &[usize]) -> f64; Pearson's correlation coefficient of two slices, typically containing the ranks.  
* `invindex`(self) -> Vec\<usize\>; method for inverting an index, e.g. given a sort index, returns ranks and vice versa.
* `unindex`(self, v:&[f64]) -> Vec\<f64\>; collects values from v in the order given by self index.

## Recent Releases

* **Version 0.1.0** Initial release.
