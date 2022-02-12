// use std::cmp::Ordering;
use crate::Indices;
use crate::{here, MinMax};
// use std::fmt::Display;
use std::iter::FromIterator;
// use std::ops::Sub;

/// Maximum value T of slice &[T]
pub fn maxt<T>(v: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut max = &v[0];
    v.iter().skip(1).for_each(|s| {
        if s > max {
            max = s
        }
    });
    *max
}

/// Minimum value T of slice &[T]
pub fn mint<T>(v: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut min = &v[0];
    v.iter().skip(1).for_each(|s| {
        if s < min {
            min = s
        }
    });
    *min
}

/// Minimum and maximum (T,T) of a slice &[T]
pub fn minmaxt<T>(v: &[T]) -> (T, T)
where
    T: PartialOrd + Copy,
{
    let mut x1 = v[0];
    let mut x2 = x1;
    v.iter().skip(1).for_each(|&s| {
        if s < x1 {
            x1 = s
        } else if s > x2 {
            x2 = s
        };
    });
    (x1, x2)
}

/// Minimum, minimum's first index, maximum, maximum's first index
pub fn minmax<T>(v: &[T]) -> MinMax<T>
where
    T: PartialOrd + Copy,
{
    let (mut min, mut max) = (v[0], v[0]); // initialise both to the first item
    let (mut minindex, mut maxindex) = (0, 0); // indices of min, max
    v.iter().enumerate().skip(1).for_each(|(i, &x)| {
        if x < min {
            min = x;
            minindex = i
        } else if x > max {
            max = x;
            maxindex = i
        }
    });
    MinMax {
        min,
        minindex,
        max,
        maxindex,
    }
}

/// Reverse a generic slice by reverse iteration.
/// Creates a new Vec. Its naive use for descending sort etc.
/// is to be avoided for efficiency reasons.
pub fn revs<T>(s: &[T]) -> Vec<T>
where
    T: Copy,
{
    s.iter().rev().copied().collect::<Vec<T>>()
}

/// Removes repetitions from an explicitly ordered set.
pub fn sansrepeat<T>(s: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if s.len() < 2 {
        return s.to_vec();
    };
    let mut r: Vec<T> = Vec::new();
    let mut last: T = s[0];
    r.push(last);
    s.iter().skip(1).for_each(|&si| {
        if si != last {
            last = si;
            r.push(si)
        }
    });
    r
}

/// Finds the first occurence of item `m` in slice `s` by full iteration.
/// Returns `Some(index)` to the slice or `None` (when it  has gone to the end).
/// Note that it uses only partial order and thus accepts any item that is neither
/// greater nor smaller than `m` (equality by default).
/// Suitable for small unordered sets.
/// For longer lists or repeated membership tests, it is better to
/// index sort them and then use faster binary `memsearch` (see below).
pub fn member<T>(s: &[T], m: T) -> Option<usize>
where
    T: PartialOrd + Copy,
{
    for (i, &x) in s.iter().enumerate() {
        if x < m {
            continue;
        }
        if x > m {
            continue;
        }
        return Some(i);
    }
    None
}

/// Binary search of an explicitly sorted list (in ascending order).
/// Returns `Some(index)` of any item that is
/// neither smaller nor greater than val.
/// When none are found, returns `None`.
/// Example use: membership of an ascending ordered set.
pub fn memsearch<T>(s: &[T], val: T) -> Option<usize>
where
    T: PartialOrd,
{
    let n = s.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if s[0] < val {
            return None;
        }
        if s[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = 0_usize; // initial index of the low limit
    if val < s[lo] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = n - 1; // index of the last item
    if s[hi] < val {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = hi - lo;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = hi - gap / 2;
        // if mid's value is greater than val, reduce the high index to it
        if s[mid] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, raise the low index to it
        if s[mid] < val {
            lo = mid;
            continue;
        }
        return Some(mid); // otherwise found it!
    }
}

/// Binary search of an explicitly sorted list (in descending order).
/// Returns `Some(index)` of any item that is
/// neither smaller nor greater than val.
/// When none are found, returns `None`.
/// Example use: membership of an descending ordered set.
pub fn memsearchdesc<T>(s: &[T], val: T) -> Option<usize>
where
    T: PartialOrd,
{
    let n = s.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if s[0] < val {
            return None;
        }
        if s[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = n - 1; // initial index of the low limit
    if val < s[lo] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = 0_usize; // index of the last item
    if val > s[hi] {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = lo - hi;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = lo - gap / 2;
        // if mid's value is greater than val, increase the high index to it
        if s[mid] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, lower the low index to it
        if s[mid] < val {
            lo = mid;
            continue;
        }
        return Some(mid); // otherwise found it!
    }
}

/// Binary search of an indexed list (in ascending order).
/// Returns `Some(index)` of any item that is
/// neither smaller nor greater than val.
/// When none are found, returns `None`.
/// Example use: membership of an indexed ordered set.
pub fn memsearch_indexed<T>(s: &[T], i: &[usize], val: T) -> Option<usize>
where
    T: PartialOrd,
{
    let n = s.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if s[0] < val {
            return None;
        }
        if s[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = 0_usize; // initial index of the low limit
    if val < s[i[lo]] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = n - 1; // index of the last item
    if s[i[hi]] < val {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = hi - lo;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = hi - gap / 2;
        // if mid's value is greater than val, reduce the high index to it
        if s[i[mid]] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, raise the low index to it
        if s[i[mid]] < val {
            lo = mid;
            continue;
        }
        return Some(mid); // otherwise found it!
    }
}

/// Binary search of an indexed list (in descending order).
/// Returns `Some(index)` of any item that is
/// neither smaller nor greater than val.
/// When none are found, returns `None`.
/// Example use: membership of an indexed descending set.
pub fn memsearchdesc_indexed<T>(s: &[T], i: &[usize], val: T) -> Option<usize>
where
    T: PartialOrd,
{
    let n = s.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if s[0] < val {
            return None;
        }
        if s[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = n - 1; // initial index of the low limit
    if val < s[i[lo]] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = 0_usize; // index of the last item
    if s[i[hi]] < val {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = lo - hi;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = lo - gap / 2;
        // if mid's value is greater than val, reduce the high index to it
        if s[i[mid]] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, raise the low index to it
        if s[i[mid]] < val {
            lo = mid;
            continue;
        }
        return Some(mid); // otherwise found it!
    }
}

/// Binary search of an explicitly sorted list in ascending order.
/// Returns an index of the first item that is greater than val.
/// When none are greater, returns s.len() (invalid index but logical).
/// The complement index (the result subtracted from s.len()), gives
/// the first item in descending order that is not greater than val.
/// Note that both complements of binsearch and binsearchdesc,
/// in their respective opposite orderings, refer to the same preceding item
/// iff there exists precisely one item equal to val.
/// However, there can be more than one such items or none.
/// Example use: looking up cummulative probability density functions.
pub fn binsearch<T>(s: &[T], val: T) -> usize
where
    T: PartialOrd,
{
    let n = s.len();
    if n == 0 {
        panic!("{} empty vec of data!", here!())
    };
    let mut hi = n - 1; // valid index of the last item
    if s[0] > val {
        return 0_usize;
    }; // the first item already exceeds val
    if s[hi] <= val {
        return n;
    }; // no items exceed val
    let mut lo = 0_usize; // initial index of the low limit
    loop {
        let gap = hi - lo;
        if gap <= 1 {
            return hi;
        };
        let mid = lo + gap / 2;
        // mid item is greater than val, reduce the high index to it
        if s[mid] > val {
            hi = mid;
            continue;
        };
        // else raise the low index to mid; jumps also over any multiple equal values.
        lo = mid;
    }
}
/// Binary search of an explicitly sorted list in descending order.
/// Returns an index of the first item that is smaller than val.
/// When none are smaller, returns s.len() (invalid index but logical).
/// The complement index (the result subtracted from s.len()), gives
/// the first item in ascending order that is not smaller than val.
/// Note that both complements of binsearch and binsearchdesc,
/// in their respective opposite orderings, refer to the same preceding item
/// iff there exists precisely one item equal to val.
/// However, there can be more than one such items or none.
/// Example use: looking up cummulative probability density functions.
pub fn binsearchdesc<T>(s: &[T], val: T) -> usize
where
    T: PartialOrd,
{
    let n = s.len();
    if n == 0 {
        panic!("{} empty vec of data!", here!())
    };
    let mut hi = n - 1; // valid index of the last item
    if s[0] < val {
        return 0_usize;
    }; // the first item is already less than val
    if s[hi] >= val {
        return n;
    }; // no item is less than val
    let mut lo = 0_usize; // initial index of the low limit
    loop {
        let gap = hi - lo;
        if gap <= 1 {
            return hi;
        };
        let mid = lo + gap / 2;
        //mid item is less than val, reduce the high index to it
        if s[mid] < val {
            hi = mid;
            continue;
        };
        // else raise the low index to mid; jumps also over any multiple equal values.
        lo = mid;
    }
}

/// Counts occurrences of val by simple linear search of any unordered set
pub fn occurs<T>(set: &[T], val:T) -> usize where T: PartialOrd+Copy {
    let mut count:usize = 0;
    for &s in set {
        if val < s { continue;};
        if val > s { continue;};
        count += 1;
    };
    count
}

/// Counts occurrences of val, using previously obtained
/// ascending explicit sort `sasc` and descending sort `sdesc`.
/// The two sorts must be of the same original set!
/// This is to facilitate counting of many
/// different values without having to repeat the sorting.
/// This function is efficient at counting
/// numerous repetitions in large sets (e.g. probabilities in stats).
/// Binary search from both ends is deployed: O(2log(n)).
/// # Example:
/// ```
/// use crate::indxvec::Indices;
/// use indxvec::merge::{sortidx,occurs_multiple};
/// let s = [3.141,3.14159,3.14159,3.142];
/// let sindx = sortidx(&s); // only one sorting
/// let sasc = sindx.unindex(&s,true);   // explicit ascending
/// let sdesc = sindx.unindex(&s,false); // explicit descending
/// assert_eq!(occurs_multiple(&sasc,&sdesc,3.14159),2);
/// ```
pub fn occurs_multiple<T>(sasc: &[T], sdesc: &[T], val: T) -> usize
where
    T: PartialOrd + Copy,
{
    let ascindex = binsearch(sasc, val);
    if ascindex == 0 {
        return 0;
    }; // val not found
    let descindex = binsearchdesc(sdesc, val);
    if descindex == 0 {
        eprintln!("{} The two sorts are not of the same list?", here!());
    };
    ascindex + descindex - sasc.len()
}

/// Unites two ascending explicitly sorted generic vectors,
/// by classical selection and copying of their head items into the result.
/// This is the union of two ordered sets.
pub fn unite<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            // v1 is now processed
            v2.iter().skip(i2).for_each(|&v| resvec.push(v)); // copy out the rest of v2
            break; // and terminate
        }
        if i2 == l2 {
            // v2 is now processed
            v1.iter().skip(i1).for_each(|&v| resvec.push(v)); // copy out the rest of v1
            break; // and terminate
        }
        if v1[i1] < v2[i2] {
            resvec.push(v1[i1]);
            i1 += 1;
            continue;
        };
        if v1[i1] > v2[i2] {
            resvec.push(v2[i2]);
            i2 += 1;
            continue;
        };
        // here they are equal, so consume one, skip the other
        resvec.push(v1[i1]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Unites two ascending index-sorted generic vectors.
/// This is the union of two index ordered sets.
/// Returns a single explicitly ordered set.
pub fn unite_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            // v1 is now processed
            for i in i2..l2 {
                resvec.push(v2[ix2[i]])
            } // copy out the rest of v2
            break; // and terminate
        }
        if i2 == l2 {
            // v2 is now processed
            for i in i1..l1 {
                resvec.push(v1[ix1[i]])
            } // copy out the rest of v1
            break; // and terminate
        }
        if v1[ix1[i1]] < v2[ix2[i2]] {
            resvec.push(v1[ix1[i1]]);
            i1 += 1;
            continue;
        };
        if v1[ix1[i1]] > v2[ix2[i2]] {
            resvec.push(v2[ix2[i2]]);
            i2 += 1;
            continue;
        };
        // here they are equal, so consume the first, skip both
        resvec.push(v1[ix1[i1]]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Intersects two ascending explicitly sorted generic vectors.
pub fn intersect<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            break;
        } // v1 is now empty
        if i2 == l2 {
            break;
        } // v2 is now empty
        if v1[i1] < v2[i2] {
            i1 += 1;
            continue;
        };
        if v1[i1] > v2[i2] {
            i2 += 1;
            continue;
        };
        // here they are equal, so consume one, skip both
        resvec.push(v1[i1]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Intersects two ascending index-sorted generic vectors.
/// Returns a single explicitly ordered set.
pub fn intersect_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            break;
        } // v1 is now processed, terminate
        if i2 == l2 {
            break;
        } // v2 is now processed, terminate
        if v1[ix1[i1]] < v2[ix2[i2]] {
            i1 += 1;
            continue;
        }; // skip v1 value
        if v1[ix1[i1]] > v2[ix2[i2]] {
            i2 += 1;
            continue;
        }; // skip v2 value
           // here they are equal, so consume the first
        resvec.push(v1[ix1[i1]]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Sets difference: deleting elements of the second from the first.
/// Two ascending explicitly sorted generic vectors.
pub fn diff<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            break;
        } // v1 is now empty
        if i2 == l2 {
            v1.iter().skip(i1).for_each(|&v| resvec.push(v)); // copy out the rest of v1
            break; // and terminate
        }
        if v1[i1] < v2[i2] {
            resvec.push(v1[i1]);
            i1 += 1;
            continue;
        }; // this v1 survived
        if v1[i1] > v2[i2] {
            i2 += 1;
            continue;
        }; // this v2 is unused
           // here they are equal, so subtract them out, i.e. skip both
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Sets difference: deleting elements of the second from the first.
/// Two ascending index sorted generic vectors.
pub fn diff_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            break;
        } // v1 is now empty
        if i2 == l2 {
            for i in i1..l1 {
                resvec.push(v1[ix1[i]])
            } // copy out the rest of v1
            break; // and terminate
        }
        if v1[ix1[i1]] < v2[ix2[i2]] {
            resvec.push(v1[ix1[i1]]);
            i1 += 1;
            continue;
        }; // this v1 survived
        if v1[ix1[i1]] > v2[ix2[i2]] {
            i2 += 1;
            continue;
        }; // this v2 is unused
           // here they are equal, so subtract them out, i.e. skip both
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Partition by pivot gives two sets of indices.
/// Items that are equal to pivot are ignored.
pub fn partition_indexed<T>(v: &[T], pivot: T) -> (Vec<usize>, Vec<usize>)
where
    T: PartialOrd + Copy,
{
    let n = v.len();
    let mut negset: Vec<usize> = Vec::with_capacity(n);
    let mut posset: Vec<usize> = Vec::with_capacity(n);
    for (i, &vi) in v.iter().enumerate() {
        if vi > pivot {
            posset.push(i);
        } else {
            negset.push(i);
        };
    }
    (negset, posset)
}

/// Merges two ascending sorted generic vectors,
/// by classical selection and copying of their head items into the result.
/// Consider using merge_indexed instead, especially for non-primitive end types T.
pub fn merge<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::with_capacity(l1 + l2);
    let mut i1 = 0;
    let mut i2 = 0;
    loop {
        if i1 == l1 {
            // v1 is now processed
            v2.iter().skip(i2).for_each(|&v| resvec.push(v)); // copy out the rest of v2
            break; // and terminate
        }
        if i2 == l2 {
            // v2 is now processed
            v1.iter().skip(i1).for_each(|&v| resvec.push(v)); // copy out the rest of v1
            break; // and terminate
        }
        if v1[i1] < v2[i2] {
            resvec.push(v1[i1]);
            i1 += 1;
            continue;
        };
        if v1[i1] > v2[i2] {
            resvec.push(v2[i2]);
            i2 += 1;
            continue;
        };
        // here they are equal, so consume both
        resvec.push(v1[i1]);
        i1 += 1;
        resvec.push(v2[i2]);
        i2 += 1
    }
    resvec
}

/// Merges two ascending sort indices.
/// Data is not shuffled at all, v2 is just concatenated onto v1
/// in one go and both remain in their original order.
/// Returns the concatenated vector and a new valid sort index into it.
pub fn merge_indexed<T>(v1: &[T], idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
where
    T: PartialOrd + Copy,
{
    let res = [v1, v2].concat(); // no individual shuffling, just one concatenation
    let l = idx1.len();
    // shift up all items in idx2 by length of indx1, so that they will
    // refer correctly to the second part of the concatenated vector
    let idx2shifted: Vec<usize> = idx2.iter().map(|x| l + x).collect();
    // now merge the indices
    let residx = merge_indices(&res, idx1, &idx2shifted);
    (res, residx)
}

/// Merges the sort indices of two concatenated vectors.
/// Data in s is not changed at all, only consulted for the comparisons.
/// This function is used by  `mergesort` and `merge_indexed`.
fn merge_indices<T>(s: &[T], idx1: &[usize], idx2: &[usize]) -> Vec<usize>
where
    T: PartialOrd + Copy,
{
    let l1 = idx1.len();
    let l2 = idx2.len();
    let mut residx: Vec<usize> = Vec::with_capacity(l1 + l2);
    let mut i1 = 0;
    let mut i2 = 0;
    let mut head1 = s[idx1[i1]];
    let mut head2 = s[idx2[i2]];
    loop {
        if head1 < head2 {
            residx.push(idx1[i1]);
            i1 += 1;
            if i1 == l1 {
                // idx1 is now fully processed
                idx2.iter().skip(i2).for_each(|&v| residx.push(v)); // copy out the rest of idx2
                break; // and terminate
            }
            head1 = s[idx1[i1]]; // else move to the next idx1 value
            continue;
        }
        if head1 > head2 {
            residx.push(idx2[i2]);
            i2 += 1;
            if i2 == l2 {
                // idx2 is now processed
                idx1.iter().skip(i1).for_each(|&v| residx.push(v)); // copy out the rest of idx1
                break; // and terminate
            }
            head2 = s[idx2[i2]]; // else move to the next idx2 value
            continue;
        }
        // here the heads are equal, so consume both
        residx.push(idx1[i1]);
        i1 += 1;
        if i1 == l1 {
            // idx1 is now fully processed
            idx2.iter().skip(i2).for_each(|&v| residx.push(v)); // copy out the rest of idx2
            break; // and terminate
        }
        head1 = s[idx1[i1]];
        residx.push(idx2[i2]);
        i2 += 1;
        if i2 == l2 {
            // idx2 is now processed
            idx1.iter().skip(i1).for_each(|&v| residx.push(v)); // copy out the rest of idx1
            break; // and terminate
        }
        head2 = s[idx2[i2]];
    }
    residx
}

/// Doubly recursive non-destructive merge sort.
/// The data is not moved or mutated.
/// Efficiency is comparable to quicksort but more stable
/// Returns a vector of indices to s from i to i+n,
/// such that the indexed values are in ascending sort order (a sort index).
/// Only the index values are being moved.
pub fn mergesort<T>(s: &[T], i: usize, n: usize) -> Vec<usize>
where
    T: PartialOrd + Copy,
{
    if n == 1 {
        let res = vec![i];
        return res;
    }; // recursion termination
    if n == 2 {
        // also terminate with two sorted items (for efficiency)
        if s[i + 1] < s[i] {
            return vec![i + 1, i];
        } else {
            return vec![i, i + 1];
        }
    }
    let n1 = n / 2; // the first part (the parts do not have to be the same)
    let n2 = n - n1; // the remaining second part
    let sv1 = mergesort(s, i, n1); // recursively sort the first half
    let sv2 = mergesort(s, i + n1, n2); // recursively sort the second half
                                        // Now merge the two sorted indices into one and return it
    merge_indices(s, &sv1, &sv2)
}

/// A wrapper for mergesort, to obtain the sort index
/// of the (whole) input vector. Simpler than sortm.
pub fn sortidx<T>(s: &[T]) -> Vec<usize>
where
    T: PartialOrd + Copy,
{
    mergesort(s, 0, s.len())
}

/// Immutable sort. Returns new sorted vector (ascending or descending).
/// Is a wrapper for mergesort. Passes the boolean flag 'ascending' onto 'unindex'.
/// Mergesort by itself always produces only an ascending index.
pub fn sortm<T>(s: &[T], ascending: bool) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    mergesort(s, 0, s.len()).unindex(s, ascending)
}

/// Fast ranking of many T items, with only `n*(log(n)+1)` complexity.
/// Ranking is done by inverting the sort index.
/// Sort index is in sorted order, giving data positions.
/// Ranking is in data order, giving sorted order positions.
/// Thus sort index and ranks are in an inverse relationship.
/// They are easily converted by `.invindex()` (for: invert index).
pub fn rank<T>(s: &[T], ascending: bool) -> Vec<usize>
where
    T: PartialOrd + Copy,
{
    let n = s.len();
    let sortindex = mergesort(s, 0, n);
    let mut rankvec: Vec<usize> = vec![0; n];
    if ascending {
        for (i, &sortpos) in sortindex.iter().enumerate() {
            rankvec[sortpos] = i
        }
    } else {
        // rank in the order of descending values
        for (i, &sortpos) in sortindex.iter().enumerate() {
            rankvec[sortpos] = n - i - 1
        }
    }
    rankvec
}

/// swap any two index items, if they are out of ascending order
fn testswap<T>(s: &[T],  idx: &mut[usize], i1: usize, i2: usize) 
where T: PartialOrd + Copy { 
    if s[idx[i1]] > s[idx[i2]] { 
        let hold = idx[i1]; 
        idx[i1] = idx[i2]; 
        idx[i2] = hold 
    };
}

/// N recursive non-destructive hash sort.
/// Input data are read only. Output is sort index.
/// Requires min,max, an estimated range of the data, that must enclose all
/// its values. This is often known in advance or can be obtained with `minmaxt`.
pub fn hashsort<T>(s: &[T], min:f64, max:f64) -> Vec<usize>
where T: PartialOrd + Copy, f64:From<T> {
    if min >= max { panic!("{} data range must be min < max",here!()); };
    let n = s.len();
    match n {
        0 => panic!("{} zero size vector",here!()),
        1 => return vec![0],
        2 => if s[0] > s[1] { return vec![1,0] } else { return vec![0,1] },
        3 => {
            let mut idx = Vec::from_iter(0..3); // mutable index
            testswap(s,&mut idx,0,1);
            testswap(s,&mut idx,1,2);
            testswap(s,&mut idx,0,1);
            idx 
        },
        _ => {
            let mut idx = Vec::from_iter(0..n); // initial mutable index
            hashsortrec(s,&mut idx,0,n,min,max); // sorts idx
            idx
        }
    }   
}

fn hashsortrec<T>(s: &[T], idx: &mut[usize], i: usize, n: usize, min:f64, max:f64) 
where T: PartialOrd+Copy, f64:From<T>
{    
    if n == 0 { panic!("{} unexpected zero length",here!())}; 
    if max == min { return };
    // hash is a constant s.t. (x-min)*hash is in [0,n]  
    let hash = (n as f64) / (max-min); 
    // we allocate one extra bucket so that the max values do not cause overflow with subscript n
    let mut freqvec:Vec<Vec<usize>> = vec![Vec::new();n+1];
    // group current index items into buckets by their associated s[] values
    for &xi in idx.iter().skip(i).take(n) { 
        let sub:usize = (hash*(f64::from(s[xi])-min)).floor() as usize;
        freqvec[sub].push(xi) }

    // flatten the buckets into the original index list 
    let mut isub = i; 
    for (vsub,v) in freqvec.iter().enumerate() { 
        let vlen = v.len();
        // print!("{} ",vlen);
        match vlen {
        0 => continue, // empty bucket
        1 => { idx[isub] = v[0]; isub += 1; }, // copy the item to the main index
        2 => { 
            idx[isub] = v[0]; idx[isub+1] = v[1];
            testswap(s,idx,isub,isub+1); 
            isub += 2 
        },
        3 => {
            idx[isub] = v[0]; idx[isub+1] = v[1]; idx[isub+2] = v[2];   
            testswap(s,idx,isub,isub+1);
            testswap(s,idx,isub+1,isub+2);
            testswap(s,idx,isub,isub+1);
            isub += 3
        },
        _ => {
            // first fill the index with the grouped items from v
            for &item in v { idx[isub] = item; isub += 1; }; 
            let vsubf = vsub as f64;  
            hashsortrec(s,idx,isub-vlen,vlen,vsubf/hash,(1.0+vsubf)/hash);
        }
        } 
    }
}
