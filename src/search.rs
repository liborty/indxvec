use core::{
    cmp::{Ordering, Ordering::*},
    ops::{Add, Div, Range, Sub},
};

/// Binary search for index of any item matching the target.
/// Searches within the specified Range<T> index, which is always ascending.
/// The range should have been checked for trivial cases already, i.e.
/// it must satisfy the requirement that data[range.start()] < target < data[range.end()-1].
/// The (indexing) range values can be of any generic type T satisfying the listed trait bounds.
/// Typically usize for searching efficiently in-memory, u128 for searching whole disks or internet,
/// or f64 for solving equations.
/// Comparator closure `cmpr` is comparing against search target captured from its environment.
/// The sort order is reflected by `cmpr` and can be either ascending or descending (increasing/decreasing).
/// Returns the index of the first hit that is PartiallyEqual to target and
/// its closest enclosing interval (the last low bound .. the last high bound).
/// When the target is not found, then the mid value is its sort order position ( and is equal to the low bound ).
pub fn binary_any<T>(range: &Range<T>, cmpr: &mut impl FnMut(&T) -> Ordering) -> (T, Range<T>)
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>,
{
    // Binary search: lo and hi are never equal to target
    let mut hi = range.end; // initial high index
    let mut lo = range.start; // initial low index
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
            // interval exhausted, no match, mid is sort position
            return (mid, lo..hi);
        };
    }
}

/// General Binary Search with very fast method for finding all the matches.
/// Search within the specified Range<T> index, which is always ascending.
/// The (indexing) range values can be of any generic type T satisfying the listed bounds.
/// Typically usize for searching efficiently in-memory, u128 for searching whole disks or internet.
/// Comparator closure `cmpr` is comparing against a target captured from its environment.
/// The sort order, reflected by `cmpr`, can be either ascending or descending (increasing/decreasing).
/// When the target is in order before self.start, empty self self.start..self.start range is returned.
/// When the target is in order after self.end, self.end+1..self.end+1 is returned.
/// Otherwise returns Range of all the consecutive values PartiallyEqual to the target.
/// When target is not found, then the returned range will be empty and
/// its start (and end) will be the sort order position.
pub fn binary_all<T>(range: &Range<T>, cmpr: &mut impl FnMut(&T) -> Ordering) -> Range<T>
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>,
    // f64:From<T>, usize:From<T>, u128:From<T>
{
    let hi = range.end; // initial high index
    let lo = range.start; // initial low index
    if range.is_empty() {
        return lo..hi;
    };

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
    match cmpr(&lo) {
        Greater => {
            return lo..lo;
        } // item is before the self
        Equal => {
            if cmpr(&hi) == Equal {
                // all in range match
                return lo..hi + T::from(1);
            };
            let (lor, _) = binary_any(range, &mut |&probe| upend(cmpr(&probe)));
            return lo..lor + T::from(1);
        }
        _ => (),
    };

    match cmpr(&(hi - T::from(1))) {
        // must not check beyond the range
        Less => {
            return range.end..range.end;
        } // item is after the self
        Equal => {
            let (lor, _) = binary_any(range, &mut |&probe| downend(cmpr(&probe)));
            return lor + T::from(1)..hi;
        }
        _ => (),
    };
    // Now lo and hi will never be equal to target
    // Binary search for first match
    let (mid, rrange) = binary_any(range, cmpr);
    // Binary search in the narrowest interval for the start of the matching range
    let (lowend, _) = binary_any(&(rrange.start..mid), &mut |&probe| downend(cmpr(&probe)));
    // Binary search in the narrowest interval for the end of the matching range
    let (highend, _) = binary_any(&(mid..rrange.end), &mut |&probe| upend(cmpr(&probe)));
    lowend + T::from(1)..highend + T::from(1)
}
