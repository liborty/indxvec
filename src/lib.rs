pub mod indices;
pub mod merge;
use std::fmt::Write;
//use std::fmt::Pointer;

pub const GR:&str = "\x1B[01;92m";
pub const UNGR:&str = "\x1B[0m";
pub const GRBRACKET:&str = "\x1B[01;92m[";
pub const BRACKETUNGR:&str = "]\x1B[0m";

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

/// Minimum value, its index, Maximum value, its index
#[derive(Default)]
pub struct MinMax<T> {
    pub min: T,
    pub minindex: usize,
    pub max: T,
    pub maxindex: usize
}
impl <T>std::fmt::Display for MinMax<T> where T:Copy+std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"min: {}, minindex {}, max: {}, maxindex: {}", 
        self.min.gr(),self.minindex.gr(),self.max.gr(),self.maxindex.gr() )
    }
}

/// Helper function to copy and cast entire &[T] to Vec<f64>.
/// Like the standard .to_vec() method but also casts to f64 end type
pub fn tof64<T>(s: &[T]) -> Vec<f64> where T: Copy, f64: From<T> {
    s.iter().map(| &x | f64::from(x)).collect()
}

/// Method `to_str()` to serialize generic items, slices, and slices of slices.
/// Method `gr()` to serialize and make the resulting string come out in bold green when printed
pub trait Printing<T> {

    /// Method `gr()` to serialize and make the resulting string 
    /// come out in bold green when printed.
    /// This is a default implementation applicable to all types that
    /// trait `Printing` is implemented for
    fn gr(self) -> String where Self:Sized {
        format!("{GR}{}{UNGR}",self.to_str())
    }

    /// Method to serialize generic items, slices, and slices of slices.  
    fn to_str(self) -> String; 
}

impl<T> Printing<T> for T where T:std::fmt::Display {
    fn to_str(self) -> String { self.to_string() } 
}

impl<T> Printing<T> for &[T] where T:std::fmt::Display {
    fn to_str(self) -> String {
        self.iter().fold(
            String::from("["),
            |mut s,item| { write!(s," {}",item).ok(); s } )
        +" ]"    
    } 
}

impl<T> Printing<T> for &[&[T]] where T:std::fmt::Display {
    fn to_str(self) -> String {
        self.iter().fold(
            String::from("["),
            |mut s,item| { writeln!(s," {}",item.to_str()).ok(); s } )
        +"]"    
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
    /// Collects values from v, as f64s, in the order given by self index.    
    fn unindexf64<T: Copy>(self, v:&[T], ascending: bool) -> Vec<f64> where f64:From<T>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.  
    fn ucorrelation(self, v: &[usize]) -> f64; 
    /// Potentially useful clone-recast of &[usize] to Vec<f64> 
    fn indx_to_f64 (self) -> Vec<f64>;
}
