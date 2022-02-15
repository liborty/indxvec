pub mod indices;  // implementation for trait Indices
pub mod printing; // implementations for trait Printing<T>
pub mod merge;    // set manipulating functions

/// When printed, turns the terminal foreground rendering to bold green
pub const GR: &str = "\x1B[01;32m";
/// When printed, turns the terminal foreground rendering to bold red
pub const RD: &str = "\x1B[01;31m";
/// Returns the terminal rendering to default
pub const UN: &str = "\x1B[0m";

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
            "min: {}, minindex {}, max: {}, maxindex: {}",
            self.min.gr(),
            self.minindex.gr(),
            self.max.gr(),
            self.maxindex.gr()
        )
    }
}

/// Helper function to copy and cast entire &[T] to Vec<f64>.
/// Like the standard .to_vec() method but also recasts to f64 end type
pub fn tof64<T>(s: &[T]) -> Vec<f64> where T: Copy, f64: From<T>, {
    s.iter().map(|&x| f64::from(x)).collect()
}

/// Trait to serialize slices of generic items &[T] (vectors)
/// and slices of Vecs of generic items &[Vec<T>] (matrices).
/// All are converted into printable strings.
pub trait Printing<T> {
    /// Method to serialize and render the resulting string in bold green.
    /// This is the default implementation applicable to all types that
    /// trait `Printing` is implemented for
    fn gr(self) -> String  where  Self: Sized,    {
        format!("{GR}{}{UN}", self.to_str())
    }
    /// Method to serialize and render the resulting string in bold red.
    fn red(self) -> String  where  Self: Sized,    {
        format!("{RD}{}{UN}", self.to_str())
    }
    /// Method to serialize generic items, slices, and slices of Vecs.
    /// Can be also implemented on any other types.
    fn to_str(self) -> String;
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
    fn unindex<T: Copy>(self, v: &[T], ascending: bool) -> Vec<T>;
    /// Collects values from v, as f64s, in the order given by self index.
    fn unindexf64<T: Copy>(self, v: &[T], ascending: bool) -> Vec<f64> where f64: From<T>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.
    fn ucorrelation(self, v: &[usize]) -> f64;
    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64>;
}
