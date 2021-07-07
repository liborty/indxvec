pub mod indices;
pub mod merge;
use std::ops::{Deref,DerefMut};
use std::fmt::Write;

/// macro `here!()` gives `&str` with the current `file:line path::function` for error messages.
#[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f); 
        format!("\n{}:{} {}", file!(), line!(), &name[..name.len()-3])
    }}
}

/// Helper function `write vector`. Formats Vec<T> as a space separated list 
/// that can be Displayed without recourse to Debug.
/// This must be done in Rust item by item, hence the iteration.
/// You can remove the green colour incantations at the beginning
/// and at the end, if not wanted.
pub fn wv<T>(v: &[T]) -> String where T:Copy+std::fmt::Display {
    let s =
        v.iter().fold(String::from("\x1B[01;92m[ "),
        |mut s,&n| {write!(s,"{} ",n).ok(); s} )
        +"]\x1B[0m";
    s
}

/// Helper function to format in green a single item. 
pub fn wi<T>(item: &T) -> String where T:std::fmt::Display {
    let s = String::from("\x1B[01;92m");
    s + &item.to_string() + "\x1B[0m"
}

/// GreenIt (GI) struct facilitates printing (in green) any
/// singular type that has Display implemented.
#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub struct GI<T: std::fmt::Display>(pub T);
impl<T: std::fmt::Display> std::fmt::Display for GI<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\x1B[01;92m{}\x1B[0m", self.0.to_string())
    }
}

/// Wrapper struct for Generic Sets.
/// Constructor is not needed, plain brackets GS(&slice) work just as well
#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub struct GS<'a,T>(pub &'a[T]);
 
impl<'a,T> Deref for GS<'a,T> {
    type Target = &'a[T]; // Vec<T>;
    fn deref(& self) -> &Self::Target {
        &self.0
    }
}

impl<'a,T> DerefMut for GS<'a,T> { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Display implemented for struct GS.
impl<'a, T: std::fmt::Display> std::fmt::Display for GS<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::from("\x1B[01;92m[");
        let n = self.0.len();
        if n > 0 {
            s.push_str(&self.0[0].to_string()); // first item
            for i in 1..n {
                s.push_str(", ");
                s.push_str(&self.0[i].to_string());
            }
        }
        write!(f, "{}]\x1B[0m", s)
    }
}

/// Methods to manipulate indices of `Vec<usize>` type.
pub trait Indices { 
    /// Reverse an index slice by simple reverse iteration.
    fn revindex(self) -> Vec<usize>; 
    /// Invert an index.
    fn invindex(self) -> Vec<usize>;
    /// complement of the index - turns ranks from/to ascending/descending
    fn complindex(self) -> Vec<usize>;
    /// Collect values from `v` in the order of indices in self.
    fn unindex<T: Copy>(self, v:&[T], ascending:bool) -> Vec<T>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.  
    fn ucorrelation(self, v: &[usize]) -> f64; 
    /// Potentially useful clone-recast of &[usize] to Vec<f64> 
    fn indx_to_f64 (self) -> Vec<f64>;
}
