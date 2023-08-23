# Indxvec [![crates.io](https://img.shields.io/crates/v/indxvec?logo=rust)](https://crates.io/crates/indxvec) [![GitHub last commit](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)](https://github.com/liborty/indxvec) [![Actions Status](https://github.com/liborty/indxvec/workflows/test/badge.svg)](https://github.com/liborty/indxvec/actions)

Author: Libor Spacek

## Usage

Written using 100% safe Rust.

The following will import everything:

```rust
use indxvec::{ here, MinMax, Search, Indices, Vecops, Mutops, Printing, printing::* };
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

* **Rank Index** - corresponds to the given data order, listing the sort positions (ranks) for the data items, e.g.the third entry in the rank index gives the rank of the third data item. Some statistical measures require ranks of data. It answers the question: 'what is the sort position of a given data item?'.

* **Inverting an index** - sort index and rank index are mutually inverse. Thus they can be easily switched by `invindex()`. This is usually the easiest way to obtain a rank index. They will both be equal to `0..n` for data that is already in ascending order.

* **Complement of an index** - beware that the standard reversal will not convert directly between ascending and descending ranks. This purpose is served by `complindex()`. Alternatively, descending ranks can be reconstructed by applying `invindex()` to a descending sort index.

* **Unindexing** - given a sort index and some data, `unindex()` will pick the data in the new order defined by the sort index. It can be used to efficiently transform lots of data vectors into the same (fixed) order. For example: Suppose we have vectors: `keys` and `data_1,..data_n`, not explicitly joined together in some bulky structure. The sort index obtained by:  
`let indx = keys.sort_indexed();`  
can then be efficiently applied to sort the data vectors individually, e.g. `indx.unindex(data_n,true)` (false to obtain a descending order at no extra cost).

## Trait Search

**`binary_all`**

Binary Search for finding all the matches. This implementation is uniquely general. It is also very fast, especially over long ranges.

Searches within the given `RangeInclusive<T>` (self). It can be used in functionally chained 'builder style APIs', that select the subrange closer bracketing the target.
The range values can be of any generic type T (satisfying the listed bounds), e.g.
usize for indexing in-memory, u128 for searching whole disks or internet,
f64 for solving equations which might not converge using other methods...

Comparator closure `cmpr` is comparing data against a target captured from its environment.
Using closures enables custom comparisons of user's own data types. Also, this code is agnostic about the type of the target (and of the data)!

When the target is in order before self.start, empty `self.start..self.start` range is returned.  
When the target is in order after self.end, `self.end..self.end` is returned.  
When the target is not found, then `ip..ip` is returned, where `ip` is its insert position.

Otherwise the range of all consecutive values `PartiallyEqual` to the target is returned.

The first hit encountered will be anywhere within some unknown number of matching items. The algorithm then conducts two more binary searches in both directions away from the first hit. These secondary searches are applied only within the last (narrowest) range found during the main search. First non-matching items in both directions are found, giving the full enclosed matching range.

**`binary_any`**

finds and returns only the first hit and its last enclosing range. It is used by `binary_all` for its three searches. It can also be used on its own when just one found item will do. For example, to solve non-linear equations, using range values of `f64` type.

```rust
/// Binary search algoritms implemented on RangeInclusive<T>.
/// Using a closure `cmpr` to sample and compare data to a captured target.
pub trait Search<T, U> {
    /// Unchecked first hit or insert order, and the final search range.
    fn binary_any(&self, cmpr: U) -> (T, Range<T>);
    /// General Binary Search, returns the range of all matching items
    fn binary_all(&self, cmpr: U) -> Range<T>;}
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
use indxvec::Vecops;
```

The methods of this trait are applicable to all generic slices `&[T]` (the data). Thus they will work on all Rust primitive numeric end types, such as f64. They can also work on slices holding any arbitrarily complex end type `T`, as long as the required traits, `PartialOrd` and/or `Clone`, are  implemented for `T`. The methods are too numerous to list here, please see their declarations in `lib.rs` and their source in `vecops.rs`.

## Trait Mutops

```rust
use indxvec::Mutops;
```

This trait contains `muthashsort`, which overwrites `self` with sorted data. When we do not need to keep the original order, this is the most efficient way to sort. A non-destructive version `sorth` is implemented in trait `Vecops`.

**Nota bene:** `muthashsort` really wins on longer Vecs. For about one thousand items upwards, it is on average about 25%-30% faster than the default Rust (Quicksort) `sort_unstable`.

```rust
/// Mutable Operators on `&mut[T]`
pub trait Mutops<T> {
    /// Sorts a mutable slice in place.
    fn mutquicksort(self)
    where
        T: PartialOrd;
    /// mutable reversal, general utility
    fn mutrevs(self);
    /// mutably swaps two indexed items into ascending order
    fn mutsorttwo(self, i0: usize, i1: usize) -> bool
    where
        T: PartialOrd;
    /// mutably sorts three indexed items into ascending order
    fn mutsortthree(self, i0: usize, i1: usize, i2: usize)
    where
        T: PartialOrd;
    /// Possibly the fastest sort for long lists. Wraps  `muthashsortslice`.
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

## Release Notes (Latest First)

**Version 1.8.0** Changed trait of closure arguments from `&mut FnMut(&T)` to `Fn(T)`, which is adequate and simpler.

**Version 1.7.1** Minor test/bug fixes and tidying up.

**Version 1.7.0** More simplification. Removed trait BinarySearch, which was just a couple of wrappers for Search methods.

**Version 1.6.0** Simplified the binary search code.

**Version 1.5.1** Made `biggest_k` and `smallest_k` even more efficient. Upped `ran` dependency to 1.1.

**Version 1.5.0** Bumped up version because of some minor breaking changes.

**Version 1.4.16** Added: `biggest_k` to complement `smallest_k`. Returns `BinaryHeap<Reverse<&T>>` of k biggest items.

**Version 1.4.15** Tuples with items of different types now also print.

**Version 1.4.14** Pruning: removed `max_1_min_k` and `max_2_min_k`, specific to medians, to `medians` crate code.

**Version 1.4.13** Added to trait Printing the capability to print pairs `&(T,T)` and triples `&(T,T,T)`, to avoid reliance on Debug mode in common situations (passing simple uniform tuple results).

**Version 1.4.11** - Added to `Vecops` `smallest_k` method, similar to `smallest_k_heap`, except it avoids unnecessary copying (is suitable for complex types T). It returns just the final Vec of k smallest items. Also added `max_1_min_k` and `max_2_min_k`, to be used in crate `medians`. The point of these methods is that they find these values in the most efficient manner, using BinaryHeap. Added here because there may be also other uses for them. Typically picking a group to qualify to 'the final' and some overall winners.

**Version 1.4.10** - Added method  
`smallest_k_heap(self, k: usize) -> BinaryHeap<T>`  
to Vecops. It efficiently returns max heap of k smallest items.

**Version 1.4.9** - Breaking change of hash sort methods. They now require a closure `quantify` for converting any user type T to f64 (it defines how to build an `f64` sort key from any type). This makes prerequisite for `sorth` explicit and gives more power to the user. It is no longer necessary to implement `From` trait for every such user type and its methods of quantification, of which there could be many. It is not reasonable to expect the users to have to do that. This new capability is demonstrated at the beginning of test `text()` (fast sorting of words by their length with a simple closure).

**Version 1.4.8** - Added trait `Binarysearch` with two convenient and safer wrapper methods for the previously introduced methods in `Search`. Now using `RangeInclusive<T>` for safe input range.

**Version 1.4.7** - General tidying up, mostly just of the documentation.

**Version 1.4.6** - Added function `search_all` which is a kind of easier wrapper for `binary_all`, without the need to specify the sort order.

**Version 1.4.5** - Improved `binary_all` usage. Added `solve` to trait `Search` for solving equations (with guaranteed convergence, unlike secant methods). Added demonstration to `tests.rs`.

**Version 1.4.4** - No change to functionality. Added fully automated github action testing, outputs can be found by clicking the test badge at the top of this document.

**Version 1.4.3** - Updated dev dependency `ran`. Added github action.

**Version 1.4.2** - Introduced automatic sort order detection in `binary_all`, thus allowing more code simplification in methods `binsearch` and `binsearch_indexed` that depend on it.

**Version 1.4.1** - Rewritten `binsearch` and `binsearch_indexed` from trait Vecops as encapsulations of the general purpose `binary_all` from trait Sort. Reduced the code size.

**Version 1.4.0** - Introduced new trait Search: `impl<T> Search<T> for Range<T>`. The search algorithms can now be applied in 'builder style chained API's', filtering the ranges.
