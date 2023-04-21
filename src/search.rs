use crate::{here, Binarysearch, Search};
use core::{
    cmp::{Ordering, PartialOrd, Ordering::*},
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
        assert!(!self.is_empty(),"{} empty range given!", here!()); 
        if sample(self.start()) < sample(self.end()) {
            self.binary_any(&mut |probe| 
                sample(probe).partial_cmp(&target)
                .expect("partial_cmp failed"))
        } else {
            self.binary_any(&mut |probe| 
                target.partial_cmp(&sample(probe))
                .expect("partial_cmp failed"))
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
            self.binary_all(&mut |probe| 
                sample(probe).partial_cmp(&target)
                .expect("partial_cmp failed"))
        } else {
            self.binary_all(&mut |probe| 
                target.partial_cmp(&sample(probe))
                .expect("partial_cmp failed"))
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
{
    /// Binary search for an index of any item matching the target.  
    /// Searches specified RangeInclusive<T>.  
    /// Comparator closure `cmpr` is comparing some data against a target captured in it.
    /// The sort order of the data can be either ascending or descending but it must be reflected by `cmpr`.
    /// Returns the index of the first hit that is PartiallyEqual to the target and
    /// its last enclosing search interval lo..=hi.  
    /// When the target is not found, then (ip, lo..=ip) is returned,
    /// where ip is the target's insert position and lo is the last lower bound.
    /// The (indexing) range values can be of any generic type T, satisfying the listed trait bounds.
    /// Typically usize for searching efficiently in-memory, u128 for searching whole disks or internet,
    /// or f64 for solving nonlinear equations.
    fn binary_any(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> (T, Range<T>) {
        // Binary search: lo and hi should never be equal to target here
        let mut hi = *self.end(); // initial high index
        let mut lo = *self.start(); // initial low index
        loop {
            let mid = lo + (hi - lo) / 2.into(); // binary chop here with truncation
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
    fn binary_all(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> Range<T> {
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
        let lo = self.start(); // initial low index
        let ihi = self.end(); // initial high index
        let hi = *ihi + one;
        if self.is_empty() {
            return *lo..hi;
        };

        // Checking end cases
        match cmpr(lo) {
            Greater => {
                return *lo..*lo;
            } // item is before the range
            Equal => {
                if cmpr(ihi) == Equal {
                    // all in range match
                    return *lo..hi;
                };
                let (lor, _) = self.binary_any(&mut |probe| upend(cmpr(probe)));
                return *lo..lor + one;
            }
            _ => (),
        };
        match cmpr(ihi) {
            Less => {
                return hi..hi;
            } // item is after the range
            Equal => {
                let (lor, _) = self.binary_any(&mut |probe| downend(cmpr(probe)));
                return lor..hi;
            }
            _ => (),
        };
        // Now lo and hi will never be equal to target
        // Binary search for first match
        let (hit, lastrange) = self.binary_any(&mut |probe| cmpr(probe));
        // No hit, return empty range with sort position
        if hit == lastrange.end {
            return hit..hit;
        };
        // Binary search in the narrowest interval for the start of the matching range
        let (lowend, _) = (lastrange.start..=hit).binary_any(&mut |probe| downend(cmpr(probe)));
        // Binary search in the narrowest interval for the end of the matching range
        let (highend, _) = (hit..=lastrange.end - one).binary_any(&mut |probe| upend(cmpr(probe)));
        lowend..highend
    }
}
