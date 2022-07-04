# Indxvec

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github">](https://github.com/liborty/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/v/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="crates.io" src="https://img.shields.io/crates/d/indxvec?logo=rust">](https://crates.io/crates/indxvec)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/indxvec?logo=rust">](https://docs.rs/indxvec)

### The following will import everything

```rust
use indxvec::{MinMax,here,tof64,printing::*,Indices,Vecops,Mutsort,Printing};
```

## Description

This crate is lightweight and has no dependencies. The methods of all four traits can be functionally chained together to achieve numerous manipulations of vectors and their indices in compact form.

The facilities provided are:

* ranking, sorting (merge sort and hash sort), merging, searching, indexing, selecting, partitioning
* many useful operations on generic vectors and their indices
* set operations
* serialising generic slices and slices of vectors to Strings: `to_plainstr()`
* printing generic slices and slices of vectors: `pvec()`
* writing generic slices and slices of vectors to files: `wvec(&mut f)`
* coloured pretty printing (ANSI terminal output, mainly for testing)
* macro `here!()` for more informative errors reporting

It is highly recommended to read and run `tests/tests.rs` to learn from examples of usage. Use a single thread to run them. It may be a bit slower but it will write the results in the right order:

```bash
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Struct and utility functions

```rust
use indxvec::{MinMax,here,tof64};
```

* Struct `Minmax` is holds minimum and maximum values of a `Vec` and their indices.  
* `here!()` is a macro giving the filename, line number and function name of the place from where it was invoked. It can be interpolated into any error/tracing messages and reports.  
* `pub fn tof64<T>(s: &[T]) -> Vec<f64>...` utility that converts generic (numeric) Vecs to `Vec<f64>`.

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
    fn unindex<T: Copy>(self, v:&[T], ascending:bool) -> Vec<T>;
    /// Collects values from v, as f64s, in the order given by self index.    
    fn unindexf64<T: Copy>(self, v:&[T], ascending: bool) -> Vec<f64> 
        where f64:From<T>;
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

The methods of this trait are applicable to generic slices `&[T]` (the data). Thus they will work on all  Rust primitive numeric end types, such as f64. They can also work on slices holding any arbitrarily complex end type `T`, as long as the required traits, `PartialOrd` and/or `Copy`, are  implemented for `T`.

```rust
/// Methods to manipulate Vecs
pub trait Vecops<T> {
    /// Maximum value in self
    fn maxt(self) -> T where T: PartialOrd+Copy;
    /// Minimum value in self
    fn mint(self) -> T where T: PartialOrd+Copy;
    /// Minimum and maximum values in self
    fn minmaxt(self) -> (T, T) where T: PartialOrd+Copy;
    /// Returns MinMax{min, minindex, max, maxindex}
    fn minmax(self) -> MinMax<T> where T: PartialOrd+Copy;
    /// MinMax of n items starting at subscript i
    fn minmax_slice(self,i:usize, n:usize) -> MinMax<T> where T: PartialOrd+Copy;
    /// MinMax of a subset of self, defined by its idx subslice between i,i+n.
    fn minmax_indexed(self, idx:&[usize], i:usize, n:usize) -> MinMax<T>
        where T: PartialOrd+Copy;
    /// Reversed copy of self
    fn revs(self) -> Vec<T> where T: Copy;
    /// Repeated items removed
    fn sansrepeat(self) -> Vec<T> where T: PartialEq+Copy;
    /// Some(subscript) of the first occurence of m, or None
    fn member(self, m: T) -> Option<usize> where T: PartialEq+Copy;
    /// Binary search for the subscript of the first occurence of val
    fn memsearch(self, val: T) -> Option<usize> where T: PartialOrd;
    /// Binary search for the subscript of the last occurence of val
    fn memsearchdesc(self, val: T) -> Option<usize> where T:PartialOrd;
    /// Binary search for val via ascending sort index i
    fn memsearch_indexed(self, i: &[usize], val: T) -> Option<usize>
        where T:PartialOrd;
    /// Backwards binary search for val via descending sort index i
    fn memsearchdesc_indexed(self, i: &[usize], val: T) -> Option<usize>
        where T: PartialOrd;
    /// Binary search of an explicitly sorted list in ascending order.
    /// Returns an index of the first item that is greater than val.
    /// When none are greater, returns s.len()
    fn binsearch(self, val: T) -> usize where T: PartialOrd;
    /// Binary search of an explicitly sorted list in descending order.
    /// Returns an index of the first item that is smaller than val.
    /// When none are smaller, returns s.len() 
    fn binsearchdesc(self, val: T) -> usize where T: PartialOrd;
    /// Counts occurrences of val by simple linear search of an unordered set
    fn occurs(self, val:T) -> usize where T: PartialOrd;
    /// Efficiently counts number of occurences from ascending and descending sorts
    fn occurs_multiple(self, sdesc: &[T], val: T) -> usize
        where T: PartialOrd+Copy;
    /// Unites (concatenates) two unsorted sets. For union of sorted sets, use `merge`
    fn unite_unsorted(self, v: &[T]) -> Vec<T> where T: Clone;
    /// Intersects two ascending explicitly sorted generic vectors.
    fn intersect(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    /// Intersects two ascending index sorted vectors.
    fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    /// Removes items of sorted v2 from sorted self.
    fn diff(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    /// Removes items of v2 from self using their sort indices.
    fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    /// Divides an unordered set into three: 
    /// items smaller than pivot, equal, and greater
    fn partition(self, pivot:T) -> (Vec<T>, Vec<T>, Vec<T>)
        where T: PartialOrd+Copy;
    /// Divides an unordered set into three by the pivot.
    /// The results are subscripts to self   
    fn partition_indexed(self, pivot: T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
        where T: PartialOrd+Copy;
    /// Merges (unites) two sorted sets, result is also sorted    
    fn merge(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    /// Merges (unites) two sets, using their sort indices,
    /// giving also the resulting sort index
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) ->
        (Vec<T>, Vec<usize>) where T: PartialOrd+Copy;
    /// Used by `merge_indexed`
    fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
        where T: PartialOrd+Copy;
    /// Utility used by sortidx
    fn mergesort(self, i: usize, n: usize) -> Vec<usize>
        where T: PartialOrd+Copy;
    /// Stable Merge sort main method, giving sort index
    fn sortidx(self) -> Vec<usize> where T:PartialOrd+Copy;
    /// Stable Merge sort, explicitly sorted result obtained via sortidx 
    fn sortm(self, ascending: bool) -> Vec<T> where T: PartialOrd+Copy;
    /// Rank index obtained via sortidx
    fn rank(self, ascending: bool) -> Vec<usize> where T: PartialOrd+Copy;
    /// Utility, swaps any two items into ascending order
    fn isorttwo(self,  idx: &mut[usize], i0: usize, i1: usize) -> bool
        where T:PartialOrd;
    /// Utility, sorts any three items into ascending order
    fn isortthree(self, idx: &mut[usize], i0: usize, i1:usize, i2:usize)
        where T: PartialOrd; 
    /// Stable Hash sort
    fn hashsort_indexed(self, min:f64, max:f64) -> Vec<usize> 
        where T: PartialOrd+Copy, f64:From<T>;
    /// Utility used by hashsort_indexed
    fn hashsortslice(self, idx: &mut[usize], i: usize, n: usize, min:f64, max:f64) 
        where T: PartialOrd+Copy, f64:From<T>;
}
```

## Trait Mutsort

```rust
use indxvec::{Mutsort};
```

This trait contains `muthashsort`, which overwrites `self` with sorted data. When we do not need to keep the original order, this is the most efficient way to sort.

**Nota bene:** `muthashsort` really wins on longer Vecs. For about one thousand items upwards, it is on average about 25%-30% faster than the default Rust (Quicksort) `sort_unstable`.

```rust
pub trait Mutsort<T> {
/// utility to mutably swap two indexed items into ascending order
fn mutsorttwo(self, i0:usize, i1:usize) where T: PartialOrd;
/// utility to mutably bubble sort three indexed items into ascending order
fn mutsortthree(self, i0:usize, i1:usize, i2:usize) where T: PartialOrd;
/// Possibly the fastest sort for long lists. Wraps `muthashsortslice`.
fn muthashsort(self, min:f64, max:f64) where T: PartialOrd+Copy, f64:From<T>;
/// Sorts n items from i in self. Used by muthashsort.
fn muthashsortslice(self, i:usize, n:usize, min:f64, max:f64) 
    where T: PartialOrd+Copy, f64:From<T>;
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

The methods of this trait are implemented for generic individual items `T`, for slices `&[T]` for slices of slices `&[&[T]]` and for slices of vecs `&[Vec<T>]`. Note that these types are normally unprintable in Rust (do not have `Display` implemented).

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

`memsearch` returns `Option(None)`, when `midval` is not found in `vm`. Here, `None` will be printed in red, while any found item will be in green. This is also an example of how to process `Option`s without long-winded `match` statements.

## Glossary

* **Sort Index** - is obtained by stable merge sort `sort_indexed`  or by `hashsort_indexed`. The original data is immutable (unchanged). The sort index produced is a list of subscripts to the data, such that the first subscript identifies the smallest item in the data, and so on (in ascending order). Suitable for bulky data that are not easily moved. It answers the question: what data item occupies a given sort position?

* **Reversing an index** - Sort Index can be reversed, by standard reversal operation `revindex()`. This has the effect of changing between ascending/descending sort orders without re-sorting or reversing the (possibly bulky) actual data.

* **Rank Index** - corresponds to the given data order, listing the sort positions (ranks) for the data items, e.g.the third entry in the rank index gives the rank of the third data item. Some statistical measures require ranks of data. It answers the question: what is the sort position of a given data item?

* **Inverting an index** - Sort Index and Rank Index are mutually inverse. Thus they can be easily switched by `invindex()`. This is usually the best way to obtain a Rank Index. They will both be equal to `0..n` for data that is already in order.

* **Complement of an index** - beware that the standard reversal will not convert directly between ascending and descending ranks.  For this purpose, it is necessary to use `complindex()`. Alternatively, to apply `invindex()` to a descending sort index.

* **Unindexing** - given a sort index and some data, `unindex()` will reorder the data into the new order defined by the sort index. It can be used to efficiently transform lots of data vectors into the same (fixed) order. For example: Suppose we have vectors: `keys` and `data_1..data_n`, not explicitly joined together in some bulky Struct elements. The sort index obtained by `keys.sort_indexed()` can then be efficiently applied to sort all the data vectors.

## Release Notes (Latest First)

**Version 1.2.4** - Clarified some comments and `indxvec` test in `tests/tests.rs`.

**Version 1.2.3** - Added `binsearch_indexed` and `binsearchdesc_indexed` and their tests,  for symmetry with `memsearch` versions which only search for members, whereas `binsearch` finds order positions for non-members, too.

**Version 1.2.2** - Minor test clarification. Expanded the glossary.

**Version 1.2.1** - Removed the functions module `merge.rs`, it has been replaced by traits `Vecops` and `Mutsort`. Improved hashsorts. Added some more comments. Added short glossary.

**Version 1.2.0** - Changed functions in module `merge.rs` to trait methods in two new traits: `Vecops` and `Mutsort`. Applying trait methods is more idiomatic and easier to read when chained. Narrowed down some trait constraints. Kept the old functions for now for backwards compatibility but they will be removed in the next version to save space.

**Version 1.1.9** - Added method `to_plainstr()` to `Printing` trait to ease writing plain format to files.

**Version 1.1.8** - Added method `pvec(self)` to `Printing` trait. It prints `Vec`s to `stdout`. Completed all six ANSI terminal primary bold colours. Moved their constants to module `printing.rs`. Renamed `red()` to `rd()` for consistent two letter names. Updated and reorganised readme.

**Version 1.1.7** - Added method `wvec(self,&mut f)` to Printing. It writes vectors to file f and passes up errors. Added colour `bl()`. Added printing test. Prettier readme.md.

**Version 1.1.6** - Added simple `partition` into three sets (lt,eq,gt).

**Version 1.1.5** - Updated dev dependency to ran = "^0.3". Changed `partition_indexed` to include equal set. Tweaked printing layout.

**Version 1.1.4** - Minor change: `hashsort` min,max arguments type changed from T to  f64. This is more convenient for a priori known data range limits. Also to be the same as for `hashsort_indexed`. Added `newindex` and `minmax_slice` functions. Updated readme file.

**Version 1.1.3** - `hashsort` renamed to `hashsort_indexed`, in keeping with the naming convention here. New plain `hashsort` added: it sorts `&mut[T]` in place, just like does the default Rust sort. Suitable for long explicit sorts.

**Version 1.1.2** - Added `.red()` method to `Printing`. Some tidying up of `tests.rs` and the docs. `hashsort` improved.

**Version 1.1.0** - Added superfast n-recursive `hashsort`. Suitable for multithreading (to do).

**Version 1.0.9** - Minor changes to testing.rs to better test `ran`.

**Version 1.0.8** - Dependencies reorganization to minimise the footprint. The random numbers generation has now been moved to its own new crate `ran` and added here just as a development dependency where it rightfully belongs.

**Version 1.0.7** - Renamed function `occurs` to `occurs_multiple` and added a simple linear count of item occurrences: `occurs`.

**Version 1.0.6** - Some cosmetic changes to the code, readme and tests, no change of functionality.

**Version 1.0.5** - Added `partition_indexed` for partitioning into two sets of indices about a pivot. Moved all random number generating functions into new module `random.rs` (import changed to: `random::*`). Moved the implementations of Printing trait to new module `printing.rs` (this has no effect on users).

**Version 1.0.4** - here!() now highlights the (first) error in bold red. Added fast random number generation functions `ranf64, ranv64, ranvu8, ranvvf64, rannvvu8`.

**Version 1.0.3** - Added utilities functions `maxt, mint, minmaxt`. Rationalised the functions for printing generic slices and slices of vectors. They are now turned into two chainable methods in trait `Printing`: `.to_str()` and `.gr()`. The latter also serialises slices to strings but additionally makes them bold green.

**Version 1.0.2** - Added function `occurs` that efficiently counts occurrences of specified items in a set with repetitions.

**Version 1.0.1** - Some code style tidying up. Added function `binsearchdesc` for completeness and symmetry with `binsearch`.

**Version 1.0.0** - `indxvec` has been stable for some time now, so it gets promoted to v1.0.0. There are some improvements to `README.md` to mark the occasion.
