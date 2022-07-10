use crate::{here,MinMax,F64,inf64,Indices,Vecops};
use std::iter::FromIterator;

impl<T> Vecops<T> for &[T] {

/// Maximum value T of slice &[T]
fn maxt(self) -> T where T: PartialOrd+Copy {
    let mut max = &self[0];
    self.iter().skip(1).for_each(|s| {
        if s > max { max = s }
    });
    *max
}

/// Minimum value T of slice &[T]
fn mint(self) -> T where T: PartialOrd+Copy {
    let mut min = &self[0];
    self.iter().skip(1).for_each(|s| {
        if s < min { min = s }
    });
    *min
}

/// Minimum and maximum (T,T) of a slice &[T]
fn minmaxt(self) -> (T, T) where T: PartialOrd+Copy {
    let mut x1 = self[0];
    let mut x2 = x1;
    self.iter().skip(1).for_each(|&s| {
        if s < x1 { x1 = s } 
        else if s > x2 { x2 = s };
    });
    (x1, x2)
}

/// Minimum, minimum's first index, maximum, maximum's first index
fn minmax(self) -> MinMax<T> where T: PartialOrd+Copy {
    let mut min = self[0];
    let mut max = min; // initialise both to the first item
    let (mut minindex, mut maxindex) = (0, 0); // indices of min, max
    self.iter().enumerate().skip(1).for_each(|(i, &x)| {
        if x < min { min = x; minindex = i; } 
        else if x > max { max = x; maxindex = i }
    });
    MinMax {
        min,
        minindex,
        max,
        maxindex,
    }
}

/// Finds min and max of a subset of self, defined by its subslice between i,i+n.
/// Returns min of self, its index, max of self, its index.
fn minmax_slice(self, i:usize, n:usize) -> MinMax<T> where T: PartialOrd+Copy {
    let mut min = self[i];
    let mut max = min;
    let mut minindex = i; // indices of min, max 
    let mut maxindex = minindex;
    for (j,&x) in self.iter().enumerate().skip(i+1).take(n-1) {
        if x < min { min = x; minindex = j; } 
        else if x > max { max = x; maxindex = j; };
    };
    MinMax { min, minindex, max, maxindex }
}

/// Using only a subset of self, defined by its idx subslice between i,i+n.
/// Returns min of self, its index's index, max of self, its index's index.
fn minmax_indexed(self, idx:&[usize], i:usize, n:usize) -> MinMax<T>
    where T: PartialOrd+Copy {
    let mut min = self[idx[i]];
    let mut max = min;
    let mut minix = 0; // indices of indices of min, max 
    let mut maxix = minix;
    for (ii,&ix) in idx.iter().enumerate().skip(i+1).take(n-1) {
        if self[ix] < min { min = self[ix]; minix = ii; } 
        else if self[ix] > max { max = self[ix]; maxix = ii; };
    };
    MinMax { min, minindex:minix, max, maxindex:maxix }
}

/// Reverse a generic slice by reverse iteration.
/// Creates a new Vec. Its naive use for descending sort etc.
/// is to be avoided for efficiency reasons.
fn revs(self) -> Vec<T> where T: Copy {
    self.iter().rev().copied().collect::<Vec<T>>()
}

/// Removes repetitions from an explicitly ordered set.
fn sansrepeat(self) -> Vec<T> where T: PartialEq+Copy { 
    if self.len() < 2 { return self.to_vec(); };
    let mut r: Vec<T> = Vec::new();
    let mut last: T = self[0];
    r.push(last);
    self.iter().skip(1).for_each(|&si| {
        if si != last { last = si; r.push(si) }
    });
    r
}

/// Finds the first occurence of item `m` in self by iteration.
/// Returns `Some(index)` to the slice or `None` (when it  has gone to the end).
/// Note that it uses only partial order and thus accepts any item that is neither
/// greater nor smaller than `m` (equality by default).
/// Suitable for small unordered sets.
/// For longer lists or repeated membership tests, it is better to
/// index sort them and then use faster binary `memsearch` (see below).
fn member(self, m: T) -> Option<usize> where T: PartialEq+Copy {
    for (i, &x) in self.iter().enumerate() {
        if x == m { return Some(i); };
    }
    None
}

/// Binary search of an explicitly sorted list (in ascending order).
/// Returns `Some(index)` of any item that is equal to val.
/// When none are found, returns `None`.
/// Example use: membership of an ascending ordered set.
fn memsearch(self, val: T) -> Option<usize> where T: PartialOrd {
    let n = self.len();
    if n == 0 { return None; } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if self[0] == val { return Some(0); }
        else { return None; }
    }
    let mut lo = 0_usize; // initial index of the low limit
    if val < self[lo] {
        return None;
    } // val is smaller than the smallest item in self
    let mut hi = n - 1; // index of the last item
    if self[hi] < val {
        return None;
    }; // val exceeds the greatest item in self
    loop {
        let gap = hi - lo;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = hi - gap / 2;
        // if mid's value is greater than val, reduce the high index to it
        if self[mid] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, raise the low index to it
        if self[mid] < val {
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
/// Example use: membership of a descending ordered set.
fn memsearchdesc(self, val: T) -> Option<usize> where T:PartialOrd {
    let n = self.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if self[0] < val {
            return None;
        }
        if self[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = n - 1; // initial index of the low limit
    if val < self[lo] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = 0_usize; // index of the last item
    if val > self[hi] {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = lo - hi;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = lo - gap / 2;
        // if mid's value is greater than val, increase the high index to it
        if self[mid] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, lower the low index to it
        if self[mid] < val {
            lo = mid;
            continue;
        }
        return Some(mid); // otherwise found it!
    }
}

/// Binary search of an indexed list (in ascending order).
/// Just like `memsearch` but uses sort index instead of explicitly sorted list. 
/// Returns `Some(index)` into the sort order, of any item that is
/// neither smaller nor greater than val. 
/// Its position in the original unsorted data is: i[index].
/// When none are found, returns `None`.
/// Example use: membership of an indexed ordered set.
fn memsearch_indexed(self, i: &[usize], val: T) -> Option<usize> 
    where T: PartialOrd {
    let n = self.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if self[0] < val {
            return None;
        }
        if self[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = 0_usize; // initial index of the low limit
    if val < self[i[lo]] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = n - 1; // index of the last item
    if self[i[hi]] < val {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = hi - lo;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = hi - gap / 2;
        // if mid's value is greater than val, reduce the high index to it
        if self[i[mid]] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, raise the low index to it
        if self[i[mid]] < val {
            lo = mid;
            continue;
        }
        return Some(mid); // otherwise found it!
    }
}

/// Binary search of an indexed list (in descending order).
/// Just like `memsearchdesc` but uses descending sort index instead of explicitly sorted list. 
/// Returns `Some(index)` (in desc. order) of any item that is neither smaller nor greater than val.
/// Its position in the original unsorted data is: i[index].
/// To find the member position in the original unsorted data, simply use i[index].
/// When none are found, returns `None`.
fn memsearchdesc_indexed(self, i: &[usize], val: T) -> Option<usize> where T: PartialOrd {
    let n = self.len();
    if n == 0 {
        return None;
    } // the slice s is empty
    if n == 1 {
        // the slice contains a single item
        if self[0] < val {
            return None;
        }
        if self[0] > val {
            return None;
        }
        return Some(0);
    }
    let mut lo = n - 1; // initial index of the low limit
    if val < self[i[lo]] {
        return None;
    } // val is smaller than the smallest item in s
    let mut hi = 0_usize; // index of the last item
    if self[i[hi]] < val {
        return None;
    }; // val exceeds the greatest item in s
    loop {
        let gap = lo - hi;
        if gap <= 1 {
            return None;
        } // termination, nothing left in the middle
        let mid = lo - gap / 2;
        // if mid's value is greater than val, reduce the high index to it
        if self[i[mid]] > val {
            hi = mid;
            continue;
        }
        // if mid's value is smaller than val, raise the low index to it
        if self[i[mid]] < val {
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
/// However, there can be more than one such items, or none.
/// Example use: rapidly looking up particular values of monotonic 
/// (e.g. cummulative probability density) functions.
fn binsearch(self, val: T) -> usize where T: PartialOrd {
    let n = self.len();
    if n == 0 {
        panic!("{} empty vec of data!", here!())
    };
    let mut hi = n - 1; // valid index of the last item
    if self[0] > val {
        return 0_usize;
    }; // the first item already exceeds val
    if self[hi] <= val {
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
        if self[mid] > val {
            hi = mid;
            continue;
        };
        // else raise the low index to mid; jumps also over any multiple equal values.
        lo = mid;
    }
}

/// Binary search of an explicitly sorted list in descending order.
/// Returns an index of the first item that is smaller than val in descending order.
/// When none are smaller, returns s.len() (invalid index but logical).
/// The complement index (the result subtracted from s.len()), gives
/// the first item in ascending order that is not smaller than val.
/// Note that both complements of binsearch and binsearchdesc,
/// in their respective opposite orderings, refer to the same preceding item
/// iff there exists precisely one item equal to val.
/// However, there can be more than one such items or none.
/// Example use: looking up cummulative probability density functions.
fn binsearchdesc(self, val: T) -> usize where T: PartialOrd {
    let n = self.len();
    if n == 0 {
        panic!("{} empty vec of data!", here!())
    };
    let mut hi = n - 1; // valid index of the last item
    if self[0] < val {
        return 0_usize;
    }; // the first item is already less than val
    if self[hi] >= val {
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
        if self[mid] < val {
            hi = mid;
            continue;
        };
        // else raise the low index to mid; jumps also over any multiple equal values.
        lo = mid;
    }
}

/// Binary search of an index sorted list in ascending order.
/// Returns a sort index of the first item that is greater than val.
/// When none are greater, returns s.len() (invalid index but logical).
/// Its position in the original unsorted data is: i[index].
/// Its value in the original unsorted data is: self[i[index]].
/// The complement index (the result subtracted from s.len()), gives
/// the first item in descending order that is not greater than val.
/// Note that both complements of binsearch and binsearchdesc,
/// in their respective opposite orderings, refer to the same preceding item
/// iff there exists precisely one item equal to val.
/// However, there can be more than one such items or none.
/// Example use: looking up cummulative probability density functions.
fn binsearch_indexed(self, i:&[usize], val: T) -> usize where T: PartialOrd {
    let n = self.len();
    if n == 0 {
        panic!("{} empty vec of data!", here!())
    };
    let mut hi = n - 1; // valid index of the last item
    if val < self[i[0]] {
        return 0_usize;
    }; // the first item already exceeds val
    if val >= self[i[hi]] {
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
        if val < self[i[mid]] {
            hi = mid;
            continue;
        };
        // else raise the low index to mid; jumps also over any multiple equal values.
        lo = mid;
    }
}

/// Binary search of an index sorted list in descending order.
/// Returns an index of the first item that is smaller than val (in descending order). 
/// When none are smaller, returns s.len() (invalid index but logical).
/// To find its position in the original unsorted data, use i[index].
/// To find its value in the original unsorted data, use self[i[index]].
/// The complement index (the result subtracted from s.len()), gives
/// the first item in ascending order that is not smaller than val.
/// Note that both complements of binsearch and binsearchdesc,
/// in their respective opposite orderings, refer to the same preceding item
/// iff there exists precisely one item equal to val.
/// However, there can be more than one such items or none.
/// Example use: looking up cummulative probability density functions.
fn binsearchdesc_indexed(self, i:&[usize], val: T) -> usize where T: PartialOrd {
    let n = self.len();
    if n == 0 {
        panic!("{} empty vec of data!", here!())
    };
    let mut hi = n - 1; // valid index of the last item
    if val > self[i[0]]  {
        return 0_usize;
    }; // the first item is already less than val
    if val <= self[i[hi]] {
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
        if self[i[mid]] < val {
            hi = mid;
            continue;
        };
        // else raise the low index to mid; jumps also over any multiple equal values.
        lo = mid;
    }
}

/// Counts occurrences of val by simple linear search of any unordered set
fn occurs(self, val:T) -> usize where T: PartialOrd {
    let mut count:usize = 0;
    for s in self {
        if val < *s { continue;};
        if val > *s { continue;};
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
/// use indxvec::Vecops;
/// let s = [1.,2.,3.14159,3.14159,4.,5.,6.];
/// let sindx = s.mergesort_indexed(); // only one sorting
/// let sasc = sindx.unindex(&s,true);   // explicit ascending
/// let sdesc = sindx.unindex(&s,false); // explicit descending
/// assert_eq!(sasc.occurs_multiple(&sdesc,3.14159),2);
/// ```
fn occurs_multiple(self, sdesc: &[T], val: T) -> usize where T: PartialOrd+Copy {
    let ascsub = self.binsearch(val);
    if ascsub == 0 { return 0; }; // val not found
    let descsub = sdesc.binsearchdesc(val);
    if descsub == 0 {
        eprintln!("{} The two sorts are not of the same list?", here!());
    };
    ascsub + descsub - self.len()
}

/// Unites (joins) two unsorted sets. For union of sorted sets, use `merge`
fn unite_unsorted(self, v: &[T]) -> Vec<T> where T: Clone {
    [self, v].concat()
}

/// Unites two ascending index-sorted generic vectors.
/// This is the union of two index sorted sets.
/// Returns a single explicitly ordered set.
fn unite_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where T: PartialOrd+Copy {
    let l1 = self.len();
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
                resvec.push(self[ix1[i]])
            } // copy out the rest of v1
            break; // and terminate
        }
        if self[ix1[i1]] < v2[ix2[i2]] {
            resvec.push(self[ix1[i1]]);
            i1 += 1;
            continue;
        };
        if self[ix1[i1]] > v2[ix2[i2]] {
            resvec.push(v2[ix2[i2]]);
            i2 += 1;
            continue;
        };
        // here they are equal, so consume the first, skip both
        resvec.push(self[ix1[i1]]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Intersects two ascending explicitly sorted generic vectors.
fn intersect(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy {
    let l1 = self.len();
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
        if self[i1] < v2[i2] {
            i1 += 1;
            continue;
        };
        if self[i1] > v2[i2] {
            i2 += 1;
            continue;
        };
        // here they are equal, so consume one, skip both
        resvec.push(self[i1]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Intersects two ascending index-sorted generic vectors.
/// Returns a single explicitly ordered set.
fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where T: PartialOrd+Copy {
    let l1 = self.len();
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
        if self[ix1[i1]] < v2[ix2[i2]] {
            i1 += 1;
            continue;
        }; // skip v1 value
        if self[ix1[i1]] > v2[ix2[i2]] {
            i2 += 1;
            continue;
        }; // skip v2 value
           // here they are equal, so consume the first
        resvec.push(self[ix1[i1]]);
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Sets difference: deleting elements of the second from the first.
/// Two ascending explicitly sorted generic vectors.
fn diff(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy {
    let l1 = self.len();
    let l2 = v2.len();
    let mut resvec: Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {
            break;
        } // v1 is now empty
        if i2 == l2 {
            self.iter().skip(i1).for_each(|&v| resvec.push(v)); // copy out the rest of v1
            break; // and terminate
        }
        if self[i1] < v2[i2] {
            resvec.push(self[i1]);
            i1 += 1;
            continue;
        }; // this v1 survived
        if self[i1] > v2[i2] {
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
fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where T: PartialOrd+Copy {
    let l1 = self.len();
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
                resvec.push(self[ix1[i]])
            } // copy out the rest of v1
            break; // and terminate
        }
        if self[ix1[i1]] < v2[ix2[i2]] {
            resvec.push(self[ix1[i1]]);
            i1 += 1;
            continue;
        }; // this v1 survived
        if self[ix1[i1]] > v2[ix2[i2]] {
            i2 += 1;
            continue;
        }; // this v2 is unused
           // here they are equal, so subtract them out, i.e. skip both
        i1 += 1;
        i2 += 1
    }
    resvec
}

/// Partition with respect to a pivot into three sets
fn partition(self, pivot:T) -> (Vec<T>, Vec<T>, Vec<T>)
    where T: PartialOrd+Copy {
    let n = self.len();
    let mut negset: Vec<T> = Vec::with_capacity(n);
    let mut eqset: Vec<T> = Vec::with_capacity(n);
    let mut posset: Vec<T> = Vec::with_capacity(n);
    for &item in self {
        if item < pivot { negset.push(item) }
        else if item > pivot  { posset.push(item) }
        else  { eqset.push(item) };  
    }; 
    (negset, eqset, posset)
}

/// Partition by pivot gives three sets of indices.
fn partition_indexed(self, pivot: T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
    where T: PartialOrd+Copy {
    let n = self.len();
    let mut negset: Vec<usize> = Vec::with_capacity(n);
    let mut eqset: Vec<usize> = Vec::with_capacity(n);
    let mut posset: Vec<usize> = Vec::with_capacity(n);
    for (i, &vi) in self.iter().enumerate() {
        if vi < pivot { negset.push(i) }
        else if vi > pivot  { posset.push(i) }
        else  { eqset.push(i) };  
    }; 
    (negset, eqset, posset)
}

/// Merges two explicitly ascending sorted generic vectors,
/// by classical selection and copying of their head items into the result.
/// Consider using merge_indexed instead, especially for non-primitive end types T.
fn merge(self, v2: &[T]) -> Vec<T> where T: PartialOrd+Copy {
    let l1 = self.len();
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
            self.iter().skip(i1).for_each(|&v| resvec.push(v)); // copy out the rest of v1
            break; // and terminate
        }
        if self[i1] < v2[i2] {
            resvec.push(self[i1]);
            i1 += 1;
            continue;
        };
        if self[i1] > v2[i2] {
            resvec.push(v2[i2]);
            i2 += 1;
            continue;
        };
        // here they are equal, so consume both
        resvec.push(self[i1]);
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
fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
    where T: PartialOrd+Copy {
    let res = [self, v2].concat(); // no individual shuffling, just one concatenation
    let l = idx1.len();
    // shift up all items in idx2 by length of indx1, so that they will
    // refer correctly to the second part of the concatenated vector
    let idx2shifted: Vec<usize> = idx2.iter().map(|x| l + x).collect();
    // now merge the indices
    let residx = res.merge_indices(idx1, &idx2shifted);
    (res, residx)
}

/// Merges the sort indices of two concatenated vectors.
/// Data in s is not changed at all, only consulted for the comparisons.
/// This function is used by  `mergesort` and `merge_indexed`.
fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
    where T: PartialOrd+Copy {
    let l1 = idx1.len();
    let l2 = idx2.len();
    let mut residx: Vec<usize> = Vec::with_capacity(l1 + l2);
    let mut i1 = 0;
    let mut i2 = 0;
    let mut head1 = self[idx1[i1]];
    let mut head2 = self[idx2[i2]];
    loop {
        if head1 < head2 {
            residx.push(idx1[i1]);
            i1 += 1;
            if i1 == l1 {
                // idx1 is now fully processed
                idx2.iter().skip(i2).for_each(|&v| residx.push(v)); // copy out the rest of idx2
                break; // and terminate
            }
            head1 = self[idx1[i1]]; // else move to the next idx1 value
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
            head2 = self[idx2[i2]]; // else move to the next idx2 value
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
        head1 = self[idx1[i1]];
        residx.push(idx2[i2]);
        i2 += 1;
        if i2 == l2 {
            // idx2 is now processed
            idx1.iter().skip(i1).for_each(|&v| residx.push(v)); // copy out the rest of idx1
            break; // and terminate
        }
        head2 = self[idx2[i2]];
    }
    residx
}

/// Doubly recursive non-destructive merge sort.
/// The data is not moved or mutated.
/// Efficiency is comparable to quicksort but more stable
/// Returns a vector of indices to s from i to i+n,
/// such that the indexed values are in ascending sort order (a sort index).
/// Only the index values are being moved.
fn mergesortslice(self, i: usize, n: usize) -> Vec<usize>
    where T: PartialOrd+Copy {
    if n == 1 {
        let res = vec![i];
        return res;
    }; // recursion termination
    if n == 2 {
        // also terminate with two sorted items (for efficiency)
        if self[i + 1] < self[i] {
            return vec![i + 1, i];
        } else {
            return vec![i, i + 1];
        }
    }
    let n1 = n / 2; // the first part (the parts do not have to be the same)
    let n2 = n - n1; // the remaining second part
    let sv1 = self.mergesortslice(i, n1); // recursively sort the first half
    let sv2 = self.mergesortslice(i + n1, n2); // recursively sort the second half
    // Now merge the two sorted indices into one and return it
    self.merge_indices(&sv1, &sv2)
}

/// The main mergesort
/// Wraps mergesortslice, to obtain the whole sort index
fn mergesort_indexed(self) -> Vec<usize> where T:PartialOrd+Copy {
    self.mergesortslice(0, self.len())
}

/// Immutable merge sort. Returns new sorted data vector (ascending or descending).
/// Wraps mergesortslice. 
/// Mergesortslice and mergesort_indexed produce only an ascending index.
/// Sortm will produce descending data order with ascending == false.
fn sortm(self, ascending: bool) -> Vec<T> where T: PartialOrd+Copy {
    self
        .mergesortslice(0, self.len())
        .unindex(self, ascending)
}

/// Fast ranking of many T items, with only `n*(log(n)+1)` complexity.
/// Ranking is done by inverting the sort index.
/// Sort index is in sorted order, giving data positions.
/// Ranking is in data order, giving sorted order positions.
/// Thus sort index and ranks are in an inverse relationship.
/// They are easily converted by `.invindex()` (for: invert index).
fn rank(self, ascending: bool) -> Vec<usize> where T: PartialOrd+Copy {
    let n = self.len();
    let sortindex = self.mergesortslice(0, n);
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

/// swap any two index items, if their data items (self) are not in ascending order
fn isorttwo(self,  idx: &mut[usize], i0: usize, i1: usize) -> bool where T:PartialOrd { 
    if self[idx[i0]] > self[idx[i1]] { idx.swap(i0,i1); true }
    else { false }
}

/// sort three index items if their self items are out of ascending order
fn isortthree(self, idx: &mut[usize], i0: usize, i1:usize, i2:usize) where T: PartialOrd { 
        self.isorttwo(idx,i0,i1);
        if self.isorttwo(idx,i1,i2) 
            { self.isorttwo(idx,i0,i1); };   
    }

/// N recursive non-destructive hash sort.
/// Input data are read only. Output is sort index.
/// Requires min,max, the data range, that must enclose all its values. 
/// The range is often known. If not, it can be obtained with `minmaxt()`.
fn hashsort_indexed(self) -> Vec<usize> 
    where T: PartialOrd+Copy, F64:From<T> { 
    let (min,max) = self.minmaxt(); 
    let n = self.len();
    // create a mutable index for the result
    let mut idx = Vec::from_iter(0..n); 
    self.hashsortslice(&mut idx,0,n,min,max); // sorts idx
    idx 
}   

fn hashsortslice(self, idx: &mut[usize], i: usize, n: usize, min:T, max:T) 
    where T: PartialOrd+Copy, F64:From<T> { 
    // Recursion termination conditions
    match n {
        0 => { return; }, // nothing to do
        1 => { idx[i] = i; return; }, // enter one item, no sorting
        2 => { self.isorttwo(idx, i, i+1); return; },
        3 => { self.isortthree(idx, i,i+1,i+2); return; },
        _ => () // carry on below
    };      
    let fmin = inf64(min);
    let fmax = inf64(max); 
    // hash is a constant s.t. (x-min)*hash is in [0,n] 
    let hash = (n as f64) / (fmax-fmin);  
    let mut buckets:Vec<Vec<usize>> = vec![Vec::new();n];
    // group current index items into buckets by their associated self[] values
    for &xi in idx.iter().skip(i).take(n) {  
        let mut hashsub = (hash*(inf64(self[xi])-fmin)).floor() as usize; 
        if hashsub == n { hashsub -=1 }; // reduce subscripts to [0,n-1] 
        buckets[hashsub].push(xi);
    }
    // sort the buckets into the index list 
    let mut isub = i; 
    for bucket in buckets.iter() { 
        let blen = bucket.len(); 
        // println!("hashsortslice bucket start: {} items: {}",isub,blen);   
        match blen {
        0 => continue, // empty bucket
        1 => { idx[isub] = bucket[0]; isub += 1; }, // copy the item to the main index
        2 => { 
            idx[isub] = bucket[0]; idx[isub+1] = bucket[1];
            self.isorttwo(idx, isub, isub+1);
            isub += 2; 
        },
        3 => {
            idx[isub] = bucket[0]; idx[isub+1] = bucket[1]; idx[isub+2] = bucket[2];   
            self.isortthree(idx,isub,isub+1,isub+2); 
            isub += 3;
        },
        x if x == n => { 
            // this bucket alone is populated, 
            // items in it are most likely all equal
            let mx = self.minmax_indexed(idx, isub, blen);
            if mx.min < mx.max { // recurse with the new range 
                self.isorttwo(idx,isub,mx.minindex); // swap minindex to the front 
                self.isorttwo(idx,mx.maxindex,isub+n-1); // swap maxindex to the end 
                // recurse to sort the rest
                self.hashsortslice(idx,i+1,blen-2,mx.min,mx.max); 
            };
            return; // all items were equal, or are now sorted
        },
        _ => { 
            // copy to the index the grouped unsorted items from bucket
            let isubprev = isub;
            for &item in bucket { idx[isub] = item; isub += 1; }; 
            let mx = self.minmax_indexed( idx, isubprev, blen);
            if mx.min < mx.max { // else are all equal
                self.isorttwo(idx,isubprev,mx.minindex); // swap minindex to the front 
                self.isorttwo(idx,mx.maxindex,isub-1); // swap maxindex to the end  
                // recurse to sort the rest
                self.hashsortslice(idx,isubprev+1,blen-2,mx.min,mx.max); 
                };
            } 
        }
    }
}

/// Immutable hash sort. Returns new sorted data vector (ascending or descending).
/// Wraps mergesortslice. 
/// Mergesortslice and mergesort_indexed produce only an ascending index.
/// Sortm will produce descending data order with ascending == false.
fn sorth(self, ascending: bool) -> Vec<T> 
    where T: PartialOrd+Copy,F64:From<T> {
    self
        .hashsort_indexed()
        .unindex(self, ascending)
}

}
