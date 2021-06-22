mod indices;
pub mod merge;

/// macro here!() gives &str with the current file:line path::function for error messages
#[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // For function name only:
        // let fnct = match &name[..name.len()-3].rfind(':') {
        //    Some(pos) => &name[pos + 1..name.len() - 3],
        //    None => &name[..name.len()-3],
        // };
        format!("\n{}:{} {}", file!(), line!(), &name[..name.len()-3])
    }}
}

/// GreenVec (GV) wrapper struct facilitates printing (in green) vector
/// of any end type that has Display implemented.
pub struct GV<'a, T: std::fmt::Display>(pub &'a[T]);
impl<'a, T: std::fmt::Display> std::fmt::Display for GV<'a,T> {
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

/*
#[derive(Eq, Debug, Clone, PartialEq)]
struct GenVec<T>(Vec<T>);
impl<'a,T> GenVec<T> {
    pub fn new() -> GenVec<T> { GenVec(Vec::new()) }
    pub fn from_vec(x: Vec<T>) -> GenVec<T> { GenVec(x) }
}
impl<T> Deref for GenVec<T> {
    type Target = Vec<T>; // Vec<T>;
    fn deref(& self) -> &Self::Target {
        &self.0
    }
}
impl<'a,T> DerefMut for GenVec<T> { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: std::fmt::Display> std::fmt::Display for GenVec<T> {
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
*/


/// Methods to manipulate indices of Vec<usize> type
pub trait Indices {
    /// Reverse index
    fn invindex(self) -> Vec<usize>;
    /// Collects f64 values from `v` as per indices in self.
    fn unindex<T: Copy>(self, ascending:bool, v:&[T]) -> Vec<T>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.  
    fn ucorrelation(self, v: &[usize]) -> f64;  
}
