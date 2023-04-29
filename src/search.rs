use crate::Search;
use core::{
    cmp::{Ordering, Ordering::*, PartialOrd},
    ops::{Add, Div, Range, RangeInclusive, Sub},
};

impl<T,U> Search<T,U> for RangeInclusive<T>
where
    T: PartialOrd + Copy + From<u8> + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
    U: Copy + Fn(T) -> Ordering
{
    /// Binary search for an index of any item matching the target within the open interval
    /// `(*self.start(),*self.end())`.
    /// Closure `cmpr` probes some ordered data and compares it with the captured target.
    /// This code is thus agnostic about the type of the target (and the data).
    /// Descending order data can be handled by reversing the order of the comparison operands.
    /// Returns the index of the first hit that is PartiallyEqual to the target
    /// and its last search envelope `lo..hi`.  
    /// When the target is not found, then `(ip, lo..ip)` is returned, where ip is the target's insert position.
    /// The (indexing) range values can be of any generic type T, satisfying the listed trait bounds.
    /// Typically usize for searching in-memory, u128 for searching disks or internet,
    /// or f64 for solving nonlinear equations.
    fn binary_any(&self, cmpr: U) -> (T, Range<T>) {
        let mut lo = *self.start(); // initial low index
        let mut hi = *self.end();   // initial high index
        loop {
            let mid = lo + (hi - lo) / 2.into(); // binary chop with truncation
            if mid > lo {
                // still some interval left
                match cmpr(mid) {
                    Less => lo = mid,
                    Greater => hi = mid,
                    Equal => {
                        // the first match hit
                        return (mid, lo..hi);
                    }
                }
            } else {
                // interval is exhausted without a match, hi is the insert position
                return (hi, lo..hi);
            };
        }
    }

    /// General Binary Search for finding all the matches.
    /// Searches within the specified RangeInclusive<T> index.
    /// The (indexing) range values can be of any generic type T (satisfying the listed bounds):
    /// usize for indexing in-memory, u128 for searching whole disks or internet,
    /// f64 for solving equations which might not converge using other methods.
    /// Comparator closure `cmpr` is comparing data against a target captured from its environment.
    /// Using closures enables custom comparisons of user's own data types.
    /// This code is also agnostic about the type of the target (and of the data).
    /// When the target is in order before self.start, empty `self.start..self.start` range is returned.
    /// When the target is in order after self.end, `self.end..self.end` is returned.
    /// When target is not found, then `ip..ip` is returned, where ip is its insert position.
    /// Otherwise the range of all consecutive values PartiallyEqual to the target is returned.
    fn binary_all(&self, cmpr: U) -> Range<T> {
        let matches_end = |probe| -> Ordering {
            let ord = cmpr(probe);
            if ord == Equal { Less } else {ord}
        };
        let matches_start = |probe| -> Ordering {
            let ord = cmpr(probe);
            if ord == Equal { Greater } else {ord}
        };
        let lo = *self.start(); // initial low index
        let ihi = *self.end();  // initial high index
        let hi = ihi + 1.into();
        if self.is_empty() {
            return lo..hi;
        };

        // Checking end cases
        match cmpr(lo) {
            Greater => {
                return lo..lo;
            } // item is before the range
            Equal => {
                if cmpr(ihi) == Equal {
                    // all in range match
                    return lo..hi;
                };
                let (lor, _) = self.binary_any(matches_end);
                return lo..lor + 1.into();
            }
            _ => (),
        };
        match cmpr(ihi) {
            Less => {
                return hi..hi;
            } // item is after the range
            Equal => {
                let (lor, _) = self.binary_any(matches_start);
                return lor..hi;
            }
            _ => (),
        };
        // lo and hi will now never be equal to target
        // Binary search for any match, with the given closure
        let (hit, lastrange) = self.binary_any(cmpr);
        // Not found, return empty range with sort position
        if hit == lastrange.end {
            return hit..hit;
        };
        // Search down in the last interval for the start of the matching range
        let (lowend, _) = (lastrange.start..=hit)
            .binary_any(matches_start);
        // Search up in the last interval for the end of the matching range
        let (highend, _) = (hit..=lastrange.end)
            .binary_any(matches_end);
        lowend..highend
    }
}
