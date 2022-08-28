use crate::Search;

use core::{
    cmp::{Ordering, Ordering::*},
    ops::{Add, Div, Range, Sub},
};

impl<T> Search<T> for Range<T> {
    /// Binary search for an index of any item matching the target.
    /// Searches within the specified Range<T>, which is always ascending.
    /// The range should have been checked for trivial cases already, i.e.
    /// it must satisfy: data[range.start()] < target < data[range.end()-1].
    /// The (indexing) range values can be of any generic type T satisfying the listed trait bounds.
    /// Typically usize for searching efficiently in-memory, u128 for searching whole disks or internet,
    /// or f64 for solving equations.
    /// Comparator closure `cmpr` is comparing against search target captured from its environment.
    /// The sort order is reflected by `cmpr` and can be either ascending or descending (increasing/decreasing).
    /// Returns the index of the first hit that is PartiallyEqual to target and
    /// its closest enclosing interval (the last low bound .. the last high bound).
    /// When the target is not found, then hit == high bound = the insert position.
    fn binary_any(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> (T, Range<T>)
    where
        T: PartialOrd + Copy + From<u8> 
            +Add<Output = T> + Sub<Output = T> + Div<Output = T>
    {
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

    /// General Binary Search with very fast method for finding all the matches.
    /// Search within the specified Range<T> index, which is always ascending.
    /// The (indexing) range values can be of any generic type T satisfying the listed bounds.
    /// Typically usize for indexing efficiently in-memory, u128 for searching whole disks or internet.
    /// Comparator closure `cmpr` is comparing against a target captured from its environment.
    /// The sort order, reflected by `cmpr`, can be either ascending or descending (increasing/decreasing).
    /// It is automatically detected.
    /// When the target is in order before self.start, empty self self.start..self.start range is returned.
    /// When the target is in order after self.end, self.end..self.end is returned.
    /// When target is not found, then an empty range with 
    /// its start (and end) equal to the sort position is returned.
    /// Otherwise returns the range of all the consecutive values PartiallyEqual to the target.

    fn binary_all(&self, cmpr: &mut impl FnMut(&T) -> Ordering) -> Range<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>,
    {
        let hi = self.end; // initial high index
        let lo = self.start; // initial low index
        if self.is_empty() {
            return lo..hi;
        };
        let descending = ( cmpr(&lo) == Greater ) && ( cmpr(&(hi - T::from(1))) == Less );
        let mut comp = |x| { let c = cmpr(&x); if descending { c.reverse() } else { c } };
 

        fn upend(o: Ordering) -> Ordering {
            if o == Equal {
                Less
            } else {
                o
            }
        }
        fn downend(o: Ordering) -> Ordering {
            if o == Equal {
                Greater
            } else {
                o
            }
        }

        // Checking end cases
        match comp(lo) {
            Greater => {
                return lo..lo;
            } // item is before the self
            Equal => {
                if comp(hi) == Equal {
                    // all in range match
                    return lo..hi + T::from(1);
                };
                let (lor, _) = self.binary_any(&mut |&probe| upend(comp(probe)));
                return lo..lor + T::from(1);
            }
            _ => (),
        };

        match comp(hi - T::from(1)) {
            // must not check beyond the range
            Less => {
                return self.end..self.end;
            } // item is after the self
            Equal => {
                let (lor, _) = self.binary_any(&mut |&probe| downend(comp(probe)));
                return lor + T::from(1)..hi;
            }
            _ => (),
        };
        // Now lo and hi will never be equal to target
        // Binary search for first match
        let (hit, rrange) = self.binary_any(&mut |&probe| comp(probe));
        // If no hit, return empty range with sort position
        if hit == rrange.end { return hit..hit }; 
        // Binary search in the narrowest interval for the start of the matching range
        let (lowend, _) = (rrange.start..hit).binary_any(&mut |&probe| downend(comp(probe)));
        // Binary search in the narrowest interval for the end of the matching range
        let (highend, _) = (hit..rrange.end).binary_any(&mut |&probe| upend(comp(probe)));
        lowend..highend
    }
}
