# Indxvec [![crates.io](https://img.shields.io/crates/v/indxvec?logo=rust)](https://crates.io/crates/indxvec) [![crates.io](https://img.shields.io/crates/d/indxvec?logo=rust)](https://crates.io/crates/indxvec) [![GitHub last commit](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)](https://github.com/liborty/indxvec) [![Actions Status](https://github.com/liborty/indxvec/workflows/test/badge.svg)](https://github.com/liborty/indxvec/actions)

Author: Libor Spacek

## Usage

Written using 100% safe Rust.

The following will import everything:

```rust
use indxvec::{ here, qsortf64(), MinMax, Search, Indices, Vecops, Mutops, Printing, printing::* };
```

## Description

Vectors sorting, searching, indexing, ranking, merging, reversing, intersecting, printing, ..

`Indxvec` is lightweight and has no dependencies. The methods of all traits can be functionally chained to achieve numerous manipulations of `Ranges`, `Vec`s, and their indices, in compact form.

The facilities provided are:

* general binary search
* ranking, sorting (merge sort and hash sort), merging, indexing, selecting, partitioning
* many useful operations on generic vectors and their indices
* set operations
* serialising generic slices and slices of vectors to Strings: `to_plainstr()`
* printing and writing generic slices and slices of vectors: `pvec()`, `wvec(&mut f)`
* coloured pretty printing (ANSI terminal output, mainly for testing)
* macro `here!()` for more informative errors reporting

It is highly recommended to read and run [`tests/tests.rs`](https://github.com/liborty/indxvec/blob/main/tests/tests.rs) to learn from examples of usage therein. Use a single thread to run them to keep the output in the right order. It is necessary to run the timing benchmark `sorts()` on its own for meaningful results.

```bash
cargo test --release -- --test-threads=1 --nocapture
cargo test sorts --release -- --nocapture
```

Or just clicking the above `test` badge leads to the logs of the automated test run.

## Glossary

* **Sort Index** - is obtained by stable merge sort `sort_indexed`  or by `hashsort_indexed`. The original data is immutable (unchanged). The sort index produced is a list of subscripts to the data, such that the first subscript identifies the smallest item in the data, and so on (in ascending order). Suitable for bulky data that are not easily moved. It answers the question: 'what data item occupies a given sort position?'.

* **K-Sort Index** - allows more efficient sort implementation when only the first k items of the Sort Index are needed.

* **Reversing an index** - sort index can be reversed by generic reversal operation `revs()`, or `mutrevs()`. This has the effect of changing between ascending/descending sort orders without re-sorting or even reversing the (possibly bulky) actual data.

* **Rank Index** - corresponds to the given data order, listing the sort positions (ranks) for the data items, e.g.the third entry in the rank index gives the rank of the third data item. Some statistical measures require ranks of data. It answers the question: 'what are the sort positions of the  data items?'.

* **Inverting an index** - sort index and rank index are mutually inverse. Thus they can be easily switched by `invindex()`. This is usually the easiest way to obtain rank index. They will both be equal to `0..n` for data that is already in ascending order.

* **Complement of an index** - beware that the standard reversal will not convert directly between ascending and descending ranks. This purpose is served by `complindex()`. Alternatively, descending ranks can be reconstructed by applying `invindex()` to a descending sort index.

* **Unindexing** - given an explicit sort index and some data, `unindex()` will pick the data in the new order defined by the sort index. It can be used to efficiently transform lots of data vectors into the same (fixed) order. For example: Suppose we have vectors: `keys` and `data_1,..data_n`, not explicitly joined together in some common data structure. The sort index obtained by e.g.: `let index = keys.hashsort_indexed();` can then be efficiently applied to sort the data vectors individually: `index.unindex(data_n,true)` (false to obtain a descending order at no extra cost).

## Trait Search

Is implemented for `RangeInclusive<T>`, specifying the range of search. Its binary search methods are not restricted to explicit data of any particular type. Probing of data is done by the comparator closure `cmpr`, which captures some data item from somewhere and a target and defines their comparison. Data subscripts are not limited to `usize`. The comparator specified in the call can be easily logically reversed, e.g. `|data_item,target| target.cmp(data_item)`. These methods will then work on data in implicit descending order.

```rust
/// Binary search algoritms implemented on RangeInclusive<T>.
/// Using a closure `cmpr` to sample and compare data to captured target.
pub trait Search<T> {
    /// Unchecked  Ok(first hit) or Err(insert order of a missing item).
    fn binary_by(self, cmpr: impl FnMut(T) -> Ordering) -> Result <T,T>;
    /// Unchecked first hit or insert order, and the final search range.
    fn binary_any(&self, cmpr: impl FnMut(T) -> Ordering) -> (T, Range<T>);
    /// General Binary Search, returns the range of all matching items
    fn binary_all(&self, cmpr: impl FnMut(T)-> Ordering) -> Range<T>;
}
```

**`binary_by`**

Binary search within an inclusive range. When the target is missing, its insert position is returned as `Err<T>`.  
Same as `std::slice::binary_search_by()` but is more general.

**`binary_any`**

finds and returns the first hit and its last enclosing range. The returned range is used by `binary_all` to constrain its search for all matches. Also, `binary_any` can be used on its own when any matching item will do. For example, to iteratively solve non-linear equations, using range values of `f64` type (see [`tests/tests.rs`](https://github.com/liborty/indxvec/blob/main/tests/tests.rs)).

**`binary_all`**

Binary search that finds all the matches. This implementation is uniquely general. It is also very fast, especially over long ranges.

Searches within the given `RangeInclusive<T>` (self). It can be used in functionally chained 'builder style APIs', that select the subrange closer bracketing the target.

The range values can be of any generic type T (satisfying the listed bounds), e.g. usize for indexing in-memory, u128 for searching whole disks or internet,
f64 for solving equations...

Comparator closure `cmpr` is comparing data against a target captured from its environment.
Using closures enables custom comparisons of user's own data types. Also, this code is agnostic about the type of the target (and of the data)!

When the target is in order before self.start, empty `self.start..self.start` range is returned.  
When the target is in order after self.end, `self.end..self.end` is returned.  
When the target is not found, then `ip..ip` is returned, where `ip` is its insert position.

Otherwise the range of all consecutive values `PartiallyEqual` to the target is returned.

The first hit encountered will be anywhere within some unknown number of matching items. The algorithm then conducts two more binary searches in both directions away from the first hit. These secondary searches are applied only within the last (narrowest) range found during the main search. First non-matching items in both directions are found, giving the full enclosed matching range.

## Trait `Indices`

```rust
use indxvec::{Indices};
```

The methods of this trait are implemented for slices of subscripts, i.e. they take the type `&[usize]` as input (self) and produce new index `Vec<usize>`, new data vector `Vec<T>` or `Vec<f64>`, or other results, as appropriate. Please see the Glossary for descriptions of the indices and the operations on them.

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
use indxvec::Vecops;
```

The methods of this trait are applicable to all generic slices `&[T]` (the data). Thus they will work on all Rust primitive numeric end types, such as f64. They can also work on slices holding any arbitrarily complex end type `T`, as long as the often required traits, `Ord` and/or `Clone`, are  implemented for `T`. The methods are too numerous to list here, please see their declarations in `lib.rs` and their source in `vecops.rs`.

## Trait Mutops

```rust
use indxvec::Mutops;
```

This trait contains mutable reverse and mutable sort methods. They all overwrite `self` with their outputs. When we do not need to preserve the original order, this is often the most efficient way to sort. Non-destructive versions are implemented in trait `Vecops`.

### mutisort

It is often useful to avoid trait constrains on the end-type being sorted, such as `Ord` or  `Partial_Ord`. Such constraints are 'sticky' and have to be then applied everywhere.
Our new `mutisort` (insert log sort) sidesteps these problems by taking a custom closure comparator. Its complexity is the best achievable for comparator sorts. It is almost as fast as the std provided sort, which eventually beats it only because it can take advantage of unstable Rust mem moves. Tested on floats, `mutisort` is actually faster up to the data length of about 4500. Additionally, `mutisort` allows sorting just within a specified range (sub-slice).

The comparator closure argument can be easily reversed to carry out descending sort.

Its non destructive versions are `Vecops::isort_indexed`, which returns an explicit sort index and `Vcops::isort_refs()` which returns references `Vec<&T>` in the sort order and is a bit faster. Neither of these two copies the potentially bulky end-types (the data items).

```rust
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
    /// mutable reversal of &mut[T]
    fn mutrevs(self);
    /// swaps two indexed items into ascending order
    fn mutsorttwo(self, i0: usize, i1: usize) -> bool
    where
        T: PartialOrd;
    /// mutably sorts three indexed items into ascending order
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
```

## Trait `Printing`

```rust
use indxvec::Printing;    // the trait methods
use indxvec::printing::*; // the ANSI colour constants
```

See `tests/tests.rs` for examples of usage.

Suitable for printing or writing to files up to 4-tuples of differing type items, all kinds of Vecs and slices and irregularly shaped 2D matrices.

Serializes tuples: `&(T,U)`, `&(T,U,V)`, `&(T,U,V,W)`  
and slices: `&[T]`, `&[&[T]]`, `&[Vec<T>]`.

Additionally, `wvec` writes contents of self as plain space separated values (`.ssv`) to File, possibly raising io::Error(s):

```rust
fn wvec(self,f:&mut File) -> Result<(), io::Error> where Self: Sized;
```

Similarly, `pvec` prints to `stdout`:

```rust
fn pvec(self) where Self: Sized;
```

All above listed types are converted to Strings and optionally decorated and coloured. Included are methods and constants to render the resulting String in six primary bold ANSI terminal colours.

Note that all these types are unprintable in standard Rust (they do not have `Display` implemented). Which is a big stumbling block for beginners. The methods of this trait convert all these types to printable (writeable) strings.

The colouring methods add the relevant colouring to the string output. This makes testing output much prettier and avoids reliance on Debug mode in production code. For finer control of the colouring, import the colour constants from  `printing::*` and use them in formatting strings manually. For example, switching colours:

```rust  
use indxvec::printing::*; // ANSI colours constants
println!("{GR}green text, {RD}red warning, {BL}feeling blue{UN}");
```

Note that all of these colouring interpolations set their own new colour regardless of the previous settings. Interpolating `{UN}` resets the terminal to its default foreground rendering.
`UN` is automatically appended at the end of strings produced by the colouring methods `rd()..cy()`. Be careful to always close with one of these, or explicit `{UN}`. Otherwise all the following output will continue with the last selected colour foreground rendering!

Example from `tests/tests.rs`:

```rust
println!("Memsearch for {BL}{midval}{UN}, found at: {}", 
    vm.memsearch(midval)
    .map_or_else(||"None".rd(),|x| x.gr())
);
```

`memsearch` returns `Option(None)`, when `midval` is not found in `vm`. Here, `None` will be printed in red, while any found item will be printed in green. Since x has been converted to `String` by `.gr()`, both closures return the same types, as required by `map_or_else`.

## Struct and Utility Functions

```rust
use indxvec::{MinMax,here};
```

* `pub struct Minmax` holds minimum and maximum values of a `Vec` and their indices.
* `here!()` is a macro giving the filename, line number and function name of the place from where it was invoked. It can be interpolated into any error/tracing messages and reports.
* `qsortf64()` applies `sort_unstable_by()` to a mutable slice of f64s safely, using `total_cmp()`.

## Release Notes (Latest First)

**Version 1.9.0** Fn closure argument in trait Search changed to FnMut on user request. Added method `partbinary` to trait Mutops

**Version 1.8.9** Added associated function `part` to trait `Mutops` (call it as: `<&mut [T]>::part(s, &rng, c)`).  
Added method `ref_vec` and associated function `deref_vec` to trait `Vecops`.

**Version 1.8.8** Upgraded to `ran 2.0`.

**Version 1.8.7** Improved `isort_refs()` and `isort_indexed`.

**Version 1.8.6** Added `isort_refs()` suitable for bulky end-types. Added `best_k`, possibly the fastest way to extract and sort k greatest or smallest items (by custom comparator).

**Version 1.8.5** Added new algorithm 'insert log sort': `mutisort()` and `isort_indexed()` to `Mutops` and `Vecops` traits respectively. Also to `tests.rs`.

**Version 1.8.4** Added `binary_by()` to trait `Search`. It behaves like  `std::slice::binary_search_by()` but is more general, not expecting explicit data of any particular type. Nor is the indexing limited to `usize`.

**Version 1.8.3** Added `&str` argument to macro `here(msg:&str)` to incorporate payload error messages. Changed `ierror` to `idx_error`. It now returns `Result` (Err variant), that can be more conveniently processed upstream with just the `?` operator.  It is not really used in the code yet, so this improvement should be backwards compatible.
Example: `return idx_error("size",here!("my specific further message"))?` will do all the necessary IdxError reporting for the `Size` variant, plus output the custom message with file, line location and method name.

**Version 1.8.2** Some minor tidying up and additions to tests. Upped dependencies.

**Version 1.8.1** Added function `qsortf64()` which sorts safely f64s.

**Version 1.8.0** Changed trait of closure arguments from `&mut FnMut(&T)` to `Fn(T)`, which is adequate and simpler.
