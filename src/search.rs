use crate::{compare, Search};
use core::{
    cmp::{Ordering, Ordering::*},
    ops::{Add, Div, Range, Sub},
};
use std::ops::Mul;

impl<T> Search<T> for Range<T>
where
    T: PartialOrd
        + Copy
        + From<u8>
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>,
{
    /// Binary search for an index of any item matching the target.
    /// Searches within the specified Range<T>, which is always ascending.
    /// The range should have been checked for trivial cases already, i.e.
    /// it must satisfy: data[range.start()] < target < data[range.end()-1].
    /// The (indexing) range values can be of any generic type T satisfying the listed trait bounds.
    /// Typically usize for searching efficiently in-memory, u128 for searching whole disks or internet,
    /// or f64 for solving equations.
    /// Comparator closure `cmpr` is comparing against search target captured from its environment.
    /// The sort order must be reflected by `cmpr` and can be either ascending or descending (increasing/decreasing).
    /// Returns the index of the first hit that is PartiallyEqual to target and
    /// its closest enclosing interval (the last low bound .. the last high bound).
    /// When the target is not found, then the returned insert position == high bound.
    fn binary_any(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> (T, Range<T>) {
        // Binary search: lo and hi are never equal to target
        let mut hi = self.end; // initial high index
        let mut lo = self.start; // initial low index
        loop {
            let mid = lo + (hi - lo) / T::from(2); // binary chop here with truncation
            if mid > lo {
                // mid == lo means interval exhausted (lo rather than hi because of truncation)
                // still some interval left
                match cmpr(&mid) {
                    Less => lo = mid,
                    Greater => hi = mid,
                    Equal => {
                        // the first match hit
                        return (mid, lo..hi);
                    }
                }
            } else {
                // interval exhausted, no match, hi is insert position
                return (hi, lo..hi);
            };
        }
    }

    /// General Binary Search for finding all the matches.
    /// Search within the specified Range<T> index, which is always ascending.
    /// The (indexing) range values can be of any generic type T satisfying the listed bounds.
    /// Typically usize for indexing efficiently in-memory, u128 for searching whole disks or internet,
    /// f64 for solving equations which might not converge using secant and other methods.
    /// Comparator closure `cmpr` is comparing against a target captured from its environment.
    /// The sort order of the data can be either ascending or descending (increasing/decreasing).
    /// The order must be specified by the `ascending` argument.
    /// When the target is in order before self.start, empty self self.start..self.start range is returned.
    /// When the target is in order after self.end, self.end..self.end is returned.
    /// When target is not found, then ip..ip is returned, where ip is its insert position.
    /// Otherwise the range of all consecutive values PartiallyEqual to the target is returned.
    fn binary_all(&self, cmpr: &mut impl FnMut(&T) -> Ordering, ascending: bool) -> Range<T> {
        fn upend(ord: Ordering) -> Ordering {
            if ord == Equal {
                Less
            } else {
                ord
            }
        }
        fn downend(ord: Ordering) -> Ordering {
            if ord == Equal {
                Greater
            } else {
                ord
            }
        }
        let lo = self.start; // initial low index
        let hi = self.end; // initial high index
        let one = T::from(1);
        if self.is_empty() {
            return lo..hi;
        };
        let mut comp = |x| {
            let c = cmpr(&x);
            if ascending {
                c
            } else {
                c.reverse()
            }
        };
        // Checking end cases
        match comp(lo) {
            Greater => {
                return lo..lo;
            } // item is before the range
            Equal => {
                if comp(hi) == Equal {
                    // all in range match
                    return lo..hi + one;
                };
                let (lor, _) = self.binary_any(&mut |&probe| upend(comp(probe)));
                return lo..lor + one;
            }
            _ => (),
        };
        match comp(hi - one) {
            // must not check beyond the range
            Less => {
                return hi..hi;
            } // item is after the range
            Equal => {
                let (lor, _) = self.binary_any(&mut |&probe| downend(comp(probe)));
                return lor + one..hi;
            }
            _ => (),
        };
        // Now lo and hi will never be equal to target
        // Binary search for first match
        let (hit, rrange) = self.binary_any(&mut |&probe| comp(probe));
        // If no hit, return empty range with sort position
        if hit == rrange.end {
            return hit..hit;
        };
        // Binary search in the narrowest interval for the start of the matching range
        let (lowend, _) = (rrange.start..hit).binary_any(&mut |&probe| downend(comp(probe)));
        // Binary search in the narrowest interval for the end of the matching range
        let (highend, _) = (hit..rrange.end).binary_any(&mut |&probe| upend(comp(probe)));
        lowend..highend
    }

    /// Nonlinear equation solver using binary search  
    /// Finds a root in the input range, such that function(root) == 0.  
    /// Function is supplied as a simple closure and can be increasing or decreasing.
    fn solve(self, function: impl Fn(&T) -> T) -> (T, Range<T>) {
        let zero = T::from(0);
        if function(&self.start) < function(&self.end) {
            self.binary_any(&mut |probe| {
                let fnval = function(probe);
                compare(&zero, &fnval)
            })
        } else {
            self.binary_any(&mut |probe| {
                let fnval = function(probe);
                compare(&fnval, &zero)
            })
        }
    }
}
