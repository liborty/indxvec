pub mod indices;  // implementation for trait Indices
pub mod printing; // implementations for trait Printing<T>
pub mod vecops;
pub mod merge;    // set manipulating functions

use std::io;
use std::io::Write;
use std::fs::File;
use printing::*;

/// Macro `here!()` gives `&str` with the `file:line path::function-name` of where it was called from.
/// This string will be rendered in bold red on (linux) terminals, so as to easily find the
/// first real error in voluminous confusing traces of avalanching Rust errors.
#[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!(
            "\n\x1B[01;31m{}:{} {}\x1B[0m",
            file!(),
            line!(),
            &name[..name.len() - 3]
        )
    }};
}

/// Helper function to copy and cast entire &[T] to Vec<f64>.
/// Like the standard .to_vec() method but also recasts to f64 end type
pub fn tof64<T>(s: &[T]) -> Vec<f64> where T: Copy, f64: From<T>, {
    s.iter().map(|&x| f64::from(x)).collect()
}

/// struct for minimum value, its index, maximum value, its index
#[derive(Default)]
pub struct MinMax<T> {
    pub min: T,
    pub minindex: usize,
    pub max: T,
    pub maxindex: usize,
}
/// Display implementation for MinMax struct
impl<T> std::fmt::Display for MinMax<T>
where
    T: Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "min: {GR}{}{UN}, minindex: {YL}{}{UN}, max: {GR}{}{UN}, maxindex: {YL}{}{UN}",
            self.min,
            self.minindex,
            self.max,
            self.maxindex
        )
    }
}

/// Trait to serialize slices of generic items &[T] (vectors)
/// and slices of Vecs of generic items &[Vec<T>] (matrices).
/// All are converted into printable strings and optionally coloured.
pub trait Printing<T> where Self: Sized {

    /// Methods to serialize and render the resulting string
    /// in bold ANSI terminal colours.
    fn rd(self) -> String { format!("{RD}{}{UN}",self.to_str()) }
    fn gr(self) -> String { format!("{GR}{}{UN}",self.to_str()) }
    fn yl(self) -> String { format!("{YL}{}{UN}",self.to_str()) }    
    fn bl(self) -> String { format!("{BL}{}{UN}",self.to_str()) }
    fn mg(self) -> String { format!("{MG}{}{UN}",self.to_str()) }
    fn cy(self) -> String { format!("{CY}{}{UN}",self.to_str()) }        

    /// Method to write vector(s) to file f (without brackets). 
    /// Passes up io errors
    fn wvec(self,f:&mut File) -> Result<(), io::Error> { 
        Ok(write!(*f,"{} ", self.to_plainstr())?) 
    }

    /// Method to print vector(s) to stdout (without brackets).
    fn pvec(self)  { print!("{} ", self.to_plainstr()) }
    
    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Adds square brackets around Vecs (prettier lists).
    /// Implementation code is in `printing.rs`. 
    fn to_str(self) -> String;

    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Implementation code is in `printing.rs`.
    fn to_plainstr(self) -> String;

}

/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices {
    /// Reverse an index slice by simple reverse iteration.
    fn revindex(self) -> Vec<usize>;
    /// Invert an index - turns a sort order into rank order and vice-versa
    fn invindex(self) -> Vec<usize>;
    /// complement of an index - reverses the ranking order
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of indices in self.
    fn unindex<T>(self, v: &[T], ascending: bool) -> Vec<T>
        where T:Copy;
    /// Collects values from v, as f64s, in the order given by self index.
    fn unindexf64<T>(self, v: &[T], ascending: bool) -> Vec<f64> where T:Copy,f64: From<T>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64>;
}

/// Methods to manipulate Vecs
pub trait Vecops<T> {
    fn newindex(n:usize) -> Vec<usize> { Vec::from_iter(0..n) }

    fn maxt(self) -> T where T: PartialOrd+Copy;
    fn mint(self) -> T where T: PartialOrd+Copy;
    fn minmaxt(self) -> (T, T) where T: PartialOrd+Copy;
    fn minmax(self) -> MinMax<T> where T: PartialOrd+Copy;
    fn minmax_slice(self,i:usize, n:usize) -> MinMax<T> where T: PartialOrd+Copy;
    fn minmax_indexed(self, idx:&[usize], i:usize, n:usize) -> MinMax<T>
        where T: PartialOrd+Copy;
    fn revs(self) -> Vec<T> where T: Copy;
    fn sansrepeat(self) -> Vec<T> where T: PartialEq+Copy;
    fn member(self, m: T) -> Option<usize> where T: PartialEq+Copy;
    fn memsearch(self, val: T) -> Option<usize> where T: PartialOrd;
    fn memsearchdesc(self, val: T) -> Option<usize> where T:PartialOrd;
    fn memsearch_indexed(self, i: &[usize], val: T) -> Option<usize> where T:PartialOrd;
    fn memsearchdesc_indexed(self, i: &[usize], val: T) -> Option<usize> where T: PartialOrd;
    fn binsearch(self, val: T) -> usize where T: PartialOrd;
    fn binsearchdesc(self, val: T) -> usize where T: PartialOrd;
    fn occurs(self, val:T) -> usize where T: PartialOrd;
    fn occurs_multiple(self, sdesc: &[T], val: T) -> usize where T: PartialOrd+Copy;
    fn unite(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    fn unite_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    fn intersect(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    fn diff(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
        where T: PartialOrd+Copy;
    fn partition(self, pivot:T) -> (Vec<T>, Vec<T>, Vec<T>)
        where T: PartialOrd+Copy;
    fn partition_indexed(self, pivot: T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
        where T: PartialOrd+Copy;
    fn merge(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy;
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
        where T: PartialOrd+Copy;
    fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
        where T: PartialOrd+Copy;
    fn mergesort(self, i: usize, n: usize) -> Vec<usize>
        where T: PartialOrd+Copy;
    fn sortidx(self) -> Vec<usize> where T:PartialOrd+Copy; 
    fn sortm(self, ascending: bool) -> Vec<T> where T: PartialOrd+Copy;
    fn rank(self, ascending: bool) -> Vec<usize>
        where T: PartialOrd+Copy;
    fn testswap(self,  idx: &mut[usize], i1: usize, i2: usize)
        where T:PartialOrd;
    fn hashsort_indexed(self, min:f64, max:f64) -> Vec<usize> 
        where T: PartialOrd+Copy, f64:From<T>;
    fn hashsortrec(self, idx: &mut[usize], i: usize, n: usize, min:f64, max:f64) 
        where T: PartialOrd+Copy, f64:From<T>;

}
pub trait Mutsort<T> {
fn compswap(self, i1: usize, i2: usize) where T: PartialOrd;
fn hashsort(self, min:f64, max:f64) where T: PartialOrd+Copy, f64:From<T>;
fn hashsortr(self, i:usize, n:usize, min:f64, max:f64) 
    where T: PartialOrd+Copy, f64:From<T>;
}