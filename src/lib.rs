mod indices;
mod merge;

/// simple error handling
use anyhow::{Result,bail}; 
use rstats::{here,GI,GV};

/// GreenVec (GV) struct facilitates printing (in green) vector
/// of any end type that has Display implemented.
/*pub struct GV<'a, T: std::fmt::Display>(pub &'a[T]);
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
*/
/// Median and quartiles
#[derive(Default)]
pub struct Med {
    pub lquartile: f64,
    pub median: f64,
    pub uquartile: f64,
}
impl std::fmt::Display for Med {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "median:\n\tLower Q: {}\n\tMedian:  {}\n\tUpper Q: {}",
            GI(self.lquartile),
            GI(self.median),
            GI(self.uquartile)
        )
    }
}

/// Mean and standard deviation (or std ratio for geometric mean).
#[derive(Default)]
pub struct MStats {
    pub mean: f64,
    pub std: f64,
}
impl std::fmt::Display for MStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mean±std: {}±{}", GI(self.mean), GI(self.std))
    }
}

/// Vector algebra on one or two vectors.
pub trait Merge {

    /// Binary search for insert index I in sorted vector 
    fn binsearch(self, v: f64) -> usize;
    /// Merges two ascending sorted vectors
    fn merge(self, v: &[f64]) -> Vec<f64>;
    /// Merges two sort indices, returns simply concatenated Vec<f64> and new sort index into it
    fn merge_immutable(self, idx1: &[usize], v2: &[f64], idx2: &[usize]) -> ( Vec<f64>,Vec<usize> );
    /// merge indices of two already concatenated sorted vectors
    fn merge_indices(self, idx1:&[usize], idx2:&[usize]) -> Vec<usize>;
    /// Sorted vector, is wrapper for mergesort below
    fn sortm(self, ascending:bool) -> Vec<f64>;

    /// Ranking with only n*log(n) complexity, using 'mergesort'
    fn rank(self, ascending:bool) -> Vec<usize>;
    /// Immutable merge sort, makes a sort index
    fn mergesort(self, i:usize, n:usize) -> Vec<usize>;
}

/// Methods to manipulate indices of Vec<usize> type
pub trait Indices {
    /// Reverse index
    fn invindex(self) -> Vec<usize>;
    /// Collects f64 values from `v` as per indices in self.
    fn unindex(self, ascending:bool, v:&[f64]) -> Vec<f64>;
    /// Collects u8 from `v` as per indices in self.
    fn unindexu8(self, ascending:bool, v:&[u8]) -> Vec<u8>;
    /// Pearson's correlation coefficient of two slices, typically the ranks.  
    fn ucorrelation(self, v: &[usize]) -> f64;  
}
