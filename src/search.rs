use crate::{compare, here, Binarysearch, Search};
use core::{
    cmp::{Ordering, Ordering::*},
    ops::{Add, Div, Mul, Range, RangeInclusive, Sub},
};

impl<T, U> Binarysearch<T, U> for RangeInclusive<T>
where
    T: PartialOrd
        + Copy
        + From<u8>
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>,
    U: PartialOrd,
{
    /// Binary search for an index of any item matching the target.  
    /// Searches the specified RangeInclusive<T>.  
    /// Closure `sample` returns data items of generic type U from any source.
    /// The sort order of the data can be either ascending or descending, it is automatically detected.
    /// Returns the index of the first hit that is PartiallyEqual to the target and
    /// its last enclosing interval lo..hi.  
    /// When the target is not found, then (ip, lo..ip) is returned,
    /// where ip is the target's insert position (index) and lo is the last lower bound.
    /// The (indexing) range values can be of any generic type T satisfying the listed trait bounds.
    /// Typically usize for searching efficiently in-memory, u128 for searching whole disks or the internet,
    /// or f64 for solving nonlinear equations.
    fn find_any(self, sample: &mut impl FnMut(&T) -> U, target: U) -> (T, Range<T>) {
        if self.is_empty() {
            panic!("{} empty range given!", here!());
        }
        if sample(self.start()) < sample(self.end()) {
            self.binary_any(&mut |probe| {
                let fnval = sample(probe);
                compare(&target, &fnval)
            })
        } else {
            self.binary_any(&mut |probe| {
                let fnval = sample(probe);
                compare(&fnval, &target)
            })
        }
    }

    /// General Binary Search. Fast nethod for finding all the matches of target (the last argument).  
    /// Search within the specified `Range<T>` where `range.start <= range.end`.  
    /// `Range<T>` indexing values can be of any generic type satisfying the listed bounds.
    /// Typically `usize` for indexing efficiently in-memory, `u128` for searching whole disks or internet, etc.
    /// Closure `sample` fetches individual items from the (sorted) data source.  
    /// The sort order of the data can be either ascending or descending.
    /// It is automatically detected.
    /// When the target is in sort order before self.start, empty self.start..self.start range is returned.
    /// When the target is in sort order after self.end, self.end..self.end is returned.
    /// When target is not found, then ip..ip is returned, where ip is its insert position.
    /// Otherwise returns the range of all consecutive values PartiallyEqual to the target.
    fn find_all(self, sample: &mut impl FnMut(&T) -> U, target: U) -> Range<T> {
        if sample(self.start()) <= sample(self.end()) {
            self.binary_all(&mut |&probe| compare(&target, &sample(&probe)), true)
        } else {
            self.binary_all(&mut |&probe| compare(&target, &sample(&probe)), false)
        }
    }
}

impl<T> Search<T> for RangeInclusive<T>
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
    /// Searches specified RangeInclusive<T>.  
    /// Comparator closure `cmpr` is comparing against the target specified (captured) in it.
    /// The sort order of the data can be either ascending or descending but it must be reflected by `cmpr`.
    /// Returns the index of the first hit that is PartiallyEqual to target and
    /// its last enclosing interval lo..=hi.  
    /// When the target is not found, then (ip, lo..=ip) is returned,
    /// where ip is the target's insert position and lo is the last lower bound.
    /// The (indexing) range values can be of any generic type T satisfying the listed trait bounds.
    /// Typically usize for searching efficiently in-memory, u128 for searching whole disks or internet,
    /// or f64 for solving nonlinear equations.
    fn binary_any(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> (T, Range<T>) {
        // Binary search: lo and hi should never be equal to target here
        let mut hi = *self.end(); // initial high index
        let mut lo = *self.start(); // initial low index
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
        let one = T::from(1);
        let lo = *self.start(); // initial low index
        let hi = *self.end(); // initial high index
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
        match comp(hi) {
            Less => {
                return hi + one..hi + one;
            } // item is after the range
            Equal => {
                let (lor, _) = self.binary_any(&mut |&probe| downend(comp(probe)));
                return lor + one..hi + one;
            }
            _ => (),
        };
        // Now lo and hi will never be equal to target
        // Binary search for first match
        let (hit, lastrange) = self.binary_any(&mut |&probe| comp(probe));
        // No hit, return empty range with sort position
        if hit == lastrange.end {
            return hit..hit;
        };
        // Binary search in the narrowest interval for the start of the matching range
        let (lowend, _) = (lastrange.start..=hit).binary_any(&mut |&probe| downend(comp(probe)));
        // Binary search in the narrowest interval for the end of the matching range
        let (highend, _) = (hit..=lastrange.end - one).binary_any(&mut |&probe| upend(comp(probe)));
        lowend..highend
    }
}
