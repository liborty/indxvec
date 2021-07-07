use crate::Indices;
use crate::here;

/// Reverse a generic slice by reverse iteration.
/// Creates a new Vec. Its naive use for descending sort etc. 
/// is to be avoided for efficiency reasons. 
pub fn revs<T>(s: &[T]) -> Vec<T> where T: Copy, 
    { s.iter().rev().map(|&x| x).collect::<Vec<T>>() }

/// Finds minimum, minimum's first index, maximum, maximum's first index 
pub fn minmax<T>(v:&[T])  -> (T, usize, T, usize) where T: PartialOrd+Copy {  
    let (mut min, mut max) = (v[0],v[0]); // initialise both to the first item 
    let (mut mini,mut maxi) = (0,0); // indices of min, max
    for i in 1..v.len() {
        let x = v[i];
        if x < min { min = x; mini = i } 
        else if x > max { max = x; maxi = i }
    };
    (min, mini, max, maxi)
}

/// Removes repetitions from an explicitly ordered set.
pub fn sansrepeat<T>(s:&[T]) -> Vec<T> where T: PartialOrd+Copy {
    let mut r:Vec<T> = Vec::new();       
    let n = s.len();
    if n == 0 { return r }
    let mut last:T = s[0];
    r.push(last);  
    for i in 1..n { 
        let si = s[i];
        if si == last { continue }
        else { last = si; r.push(si) }
    }
    r
}

/// Finds the first occurence of item `m` in slice `s` by full iteration.
/// Returns `Some(index)` to the slice or `None` (when it  has gone to the end).
/// Note that it uses only partial order and thus accepts any item that is neither 
/// greater nor smaller than `m` (equality by default). 
/// Suitable for small unordered sets. 
/// For longer lists or repeated membership tests, it is better to
/// index sort them and then use faster binary `memsearch` (see below).
pub fn member<T>(s:&[T], m:T) -> Option<usize> where T: PartialOrd+Copy {
    for (i,&x) in s.iter().enumerate() { 
        if x < m { continue }
        if x > m { continue }
        return Some(i) 
    };
    None
}

/// Binary search of an explicitly sorted list (in ascending order).
/// Returns `Some(index)` of any item that is 
/// neither smaller nor greater than val. 
/// When none are found, returns `None`.
/// Example use: membership of an ordered set. 
pub fn memsearch<T>(s:&[T], val: T)  -> Option<usize> where T: PartialOrd {     
    let n = s.len();
    if n == 0 { return None } // the slice s is empty
    if n == 1 {  // the slice contains a single item 
        if s[0] < val { return None }
        if s[0] > val { return None }
        return Some(0) } 
    let mut lo = 0_usize; // initial index of the low limit   
    if val < s[lo] { return None } // val is smaller than the smallest item in s 
    let mut hi = n-1; // index of the last item
    if s[hi] < val { return None }; // val exceeds the greatest item in s   
    loop {
        let gap = hi - lo;
        if gap <= 1 { return None } // termination, nothing left in the middle
        let mid = hi-gap/2; 
        // if mid's value is greater than val, reduce the high index to it
        if s[mid] > val { hi = mid; continue } 
        // if mid's value is smaller than val, raise the low index to it
        if s[mid] < val { lo = mid; continue } 
        return Some(mid) // otherwise found it!     
    }
}

/// Binary search of an indexed list (in ascending order).
/// Returns `Some(index)` of any item that is 
/// neither smaller nor greater than val. 
/// When none are found, returns `None`.
/// Example use: membership of an indexed ordered set. 
pub fn memsearch_indexed<T>(s:&[T], i:&[usize], val: T)  -> Option<usize> where T: PartialOrd {     
    let n = s.len();
    if n == 0 { return None } // the slice s is empty
    if n == 1 {  // the slice contains a single item 
        if s[0] < val { return None }
        if s[0] > val { return None }
        return Some(0) } 
    let mut lo = 0_usize; // initial index of the low limit   
    if val < s[i[lo]] { return None } // val is smaller than the smallest item in s 
    let mut hi = n-1; // index of the last item
    if s[i[hi]] < val { return None }; // val exceeds the greatest item in s   
    loop {
        let gap = hi - lo;
        if gap <= 1 { return None } // termination, nothing left in the middle
        let mid = hi-gap/2; 
        // if mid's value is greater than val, reduce the high index to it
        if s[i[mid]] > val { hi = mid; continue } 
        // if mid's value is smaller than val, raise the low index to it
        if s[i[mid]] < val { lo = mid; continue } 
        return Some(mid) // otherwise found it!     
    }
}

/// Binary search of an explicitly sorted list (in ascending order).
/// Returns the index of the first item that is greater than val. 
/// When none are greater, returns s.len() (invalid index but logical).
/// Example use: looking up cummulative probability density functions. 
pub fn binsearch<T>(s:&[T], val: T)  -> usize where T: PartialOrd {     
    let n = s.len();
    if n < 2 { panic!("{} vec of data is too short!",here!()) }     
    if s[0] > val { return 0_usize }; // the first item already exceeds val
    let mut hi = n-1; // valid index of the last item
    if s[hi] <= val { return n }; // no items exceed val
    let mut lo = 0_usize; // initial index of the low limit     
    loop {
        let gap = hi - lo;
        if gap <= 1 { return hi }
        let tryi = lo+gap/2; 
        // if tryi's value is above val, reduce the high index to it
        if s[tryi] > val { hi = tryi; continue }            
        // else tryi's value is not greater than val, raise the low index to it
        // jumps also over any repeated equal values. 
        lo = tryi
    }  
}

/// Unites two ascending explicitly sorted generic vectors,
/// by classical selection and copying of their head items into the result.
/// This is the union of two ordered sets.
pub fn unite<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 { // v1 is now processed
            for i in i2..l2 { resvec.push(v2[i]) } // copy out the rest of v2
            break // and terminate
        }
        if i2 == l2 { // v2 is now processed
            for i in i1..l1 { resvec.push(v1[i])} // copy out the rest of v1
            break // and terminate
        }
        if v1[i1] < v2[i2] { resvec.push(v1[i1]); i1 += 1; continue };
        if v1[i1] > v2[i2] { resvec.push(v2[i2]); i2 += 1; continue }; 
        // here they are equal, so consume one, skip the other
        resvec.push(v1[i1]); i1 += 1; i2 += 1
    }
    resvec
}

/// Unites two ascending index-sorted generic vectors.
/// This is the union of two index ordered sets.
/// Returns a single explicitly ordered set.
pub fn unite_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 { // v1 is now processed
            for i in i2..l2 { resvec.push(v2[ix2[i]]) } // copy out the rest of v2
            break // and terminate
        }
        if i2 == l2 { // v2 is now processed
            for i in i1..l1 { resvec.push(v1[ix1[i]]) } // copy out the rest of v1
            break // and terminate
        }
        if v1[ix1[i1]] < v2[ix2[i2]] { resvec.push(v1[ix1[i1]]); i1 += 1; continue };
        if v1[ix1[i1]] > v2[ix2[i2]] { resvec.push(v2[ix2[i2]]); i2 += 1; continue }; 
        // here they are equal, so consume the first, skip both
        resvec.push(v1[ix1[i1]]); i1 += 1; i2 += 1
    }
    resvec
}

/// Intersects two ascending explicitly sorted generic vectors.
pub fn intersect<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 {  break } // v1 is now empty 
        if i2 == l2 {  break } // v2 is now empty 
        if v1[i1] < v2[i2] { i1 += 1; continue };
        if v1[i1] > v2[i2] { i2 += 1; continue }; 
        // here they are equal, so consume one, skip both
        resvec.push(v1[i1]); i1 += 1; i2 += 1
    }
    resvec
}

/// Intersects two ascending index-sorted generic vectors. 
/// Returns a single explicitly ordered set.
pub fn intersect_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 { break } // v1 is now processed, terminate 
        if i2 == l2 { break }  // v2 is now processed, terminate
        if v1[ix1[i1]] < v2[ix2[i2]] { i1 += 1; continue }; // skip v1 value
        if v1[ix1[i1]] > v2[ix2[i2]] { i2 += 1; continue }; // skip v2 value
        // here they are equal, so consume the first
        resvec.push(v1[ix1[i1]]); i1 += 1; i2 += 1
    }
    resvec
}

/// Sets difference: deleting elements of the second from the first.
/// Two ascending explicitly sorted generic vectors.
pub fn diff<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 { break } // v1 is now empty 
        if i2 == l2 { 
            for i in i1..l1 { resvec.push(v1[i]) } // copy out the rest of v1
            break // and terminate
        }
        if v1[i1] < v2[i2] { resvec.push(v1[i1]); i1 += 1; continue }; // this v1 survived
        if v1[i1] > v2[i2] { i2 += 1; continue }; // this v2 is unused
        // here they are equal, so subtract them out, i.e. skip both
        i1 += 1; i2 += 1
    }
    resvec
}

/// Sets difference: deleting elements of the second from the first.
/// Two ascending index sorted generic vectors.
pub fn diff_indexed<T>(v1: &[T], ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    loop {
        if i1 == l1 { break } // v1 is now empty 
        if i2 == l2 { 
            for i in i1..l1 { resvec.push(v1[ix1[i]]) } // copy out the rest of v1
            break // and terminate
        }
        if v1[ix1[i1]] < v2[ix2[i2]] { resvec.push(v1[ix1[i1]]); i1 += 1; continue }; // this v1 survived
        if v1[ix1[i1]] > v2[ix2[i2]] { i2 += 1; continue }; // this v2 is unused
        // here they are equal, so subtract them out, i.e. skip both
        i1 += 1; i2 += 1
    }
    resvec
}

/// Merges two ascending sorted generic vectors,
/// by classical selection and copying of their head items into the result.
/// Consider using merge_indexed instead, especially for non-primitive end types T. 
pub fn merge<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy, {  
    let l1 = v1.len();
    let l2 = v2.len();
    let mut resvec:Vec<T> = Vec::with_capacity(l1+l2);
    let mut i1 = 0;
    let mut i2 = 0;
    loop {
        if i1 == l1 { // v1 is now processed
            for i in i2..l2 { resvec.push(v2[i]) } // copy out the rest of v2
            break // and terminate
        }
        if i2 == l2 { // v2 is now processed
            for i in i1..l1 { resvec.push(v1[i])} // copy out the rest of v1
            break // and terminate
        }
        if v1[i1] < v2[i2] { resvec.push(v1[i1]); i1 += 1; continue };
        if v1[i1] > v2[i2] { resvec.push(v2[i2]); i2 += 1; continue }; 
        // here they are equal, so consume both
        resvec.push(v1[i1]); i1 += 1;
        resvec.push(v2[i2]); i2 += 1
    }
    resvec
}

/// Merges two ascending sort indices.
/// Data is not shuffled at all, v2 is just concatenated onto v1
/// in one go and both remain in their original order. 
/// Returns the concatenated vector and a new valid sort index into it.
pub fn merge_indexed<T>(v1:&[T], idx1: &[usize], v2: &[T], idx2: &[usize]) -> ( Vec<T>,Vec<usize> ) 
    where T: PartialOrd+Copy, {    
    let res = [v1,v2].concat(); // no individual shuffling, just one concatenation     
    let l = idx1.len();
    // shift up all items in idx2 by length of indx1, so that they will 
    // refer correctly to the second part of the concatenated vector
    let idx2shifted:Vec<usize> = idx2.iter().map(|x| l+x ).collect();
    // now merge the indices      
    let residx = merge_indices(&res,idx1,&idx2shifted);   
    ( res, residx )
}

/// Merges the sort indices of two concatenated vectors.
/// Data in s is not changed at all, only consulted for the comparisons. 
/// This function is used by  `mergesort` and `merge_indexed`. 
fn merge_indices<T>(s: &[T], idx1:&[usize], idx2:&[usize]) -> Vec<usize>
    where T: PartialOrd+Copy, {
    let l1 = idx1.len();
    let l2 = idx2.len();
    let mut residx:Vec<usize> = Vec::with_capacity(l1+l2); 
    let mut i1 = 0;  
    let mut i2 = 0;
    let mut head1 = s[idx1[i1]]; 
    let mut head2 = s[idx2[i2]];
    loop {
        if head1 < head2 { 
            residx.push(idx1[i1]);
            i1 += 1;  
            if i1 == l1 { // idx1 is now fully processed
                for i in i2..l2 { residx.push(idx2[i]) } // copy out the rest of idx2
                break // and terminate
            }
            head1 = s[idx1[i1]]; // else move to the next idx1 value
            continue
        }
        if head1 > head2 { 
            residx.push(idx2[i2]); 
            i2 += 1; 
            if i2 == l2 { // idx2 is now processed
                for i in i1..l1 { residx.push(idx1[i]) } // copy out the rest of idx1
                break // and terminate
            }                    
            head2 = s[idx2[i2]]; // else move to the next idx2 value
            continue
        } 
        // here the heads are equal, so consume both
        residx.push(idx1[i1]); 
        i1 += 1; 
        if i1 == l1 { // idx1 is now fully processed
            for i in i2..l2 { residx.push(idx2[i]) } // copy out the rest of idx2
            break // and terminate
        }
        head1 = s[idx1[i1]];
        residx.push(idx2[i2]); 
        i2 += 1; 
        if i2 == l2 { // idx2 is now processed
            for i in i1..l1 { residx.push(idx1[i]) } // copy out the rest of idx1
            break // and terminate
        }                    
        head2 = s[idx2[i2]];            
    }
    residx
}

/// Doubly recursive non-destructive merge sort.  
/// The data is not moved or mutated. 
/// Efficiency is comparable to quicksort. 
/// Returns a vector of indices to s from i to i+n,
/// such that the indexed values are in ascending sort order (a sort index).  
/// Only the index values are being moved. 
pub fn mergesort<T>(s:&[T], i:usize, n:usize) -> Vec<usize> 
    where T: PartialOrd+Copy {
    if n == 1 { let res = vec![i]; return res };  // recursion termination
    if n == 2 {  // also terminate with two sorted items (for efficiency)          
        if s[i+1] < s[i] { return vec![i+1,i] } else { return vec![i,i+1] }
    }       
    let n1 = n / 2;  // the first part (the parts do not have to be the same) 
    let n2 = n - n1; // the remaining second part
    let sv1 = mergesort(s, i, n1); // recursively sort the first half
    let sv2 = mergesort(s, i+n1, n2); // recursively sort the second half 
    // Now merge the two sorted indices into one and return it     
    merge_indices(s,&sv1,&sv2)
}

/// A wrapper for mergesort, to obtain the sort index
/// of the (whole) input vector. Simpler than sortm.
pub fn sortidx<T>(s:&[T]) -> Vec<usize> where T:PartialOrd+Copy {
    mergesort(&s,0,s.len())
}

/// Immutable sort. Returns new sorted vector (ascending or descending). 
/// Is a wrapper for mergesort. Passes the boolean flag 'ascending' onto 'unindex'.
/// Mergesort by itself always produces only an ascending index.
pub fn sortm<T>(s:&[T], ascending:bool) -> Vec<T> where T: PartialOrd+Copy {
    mergesort(s,0,s.len()).unindex(s,ascending)
}   

/// Fast ranking of many T items, with only `n*(log(n)+1)` complexity. 
/// Ranking is done by inverting the sort index.  
/// Sort index is in sorted order, giving data positions. 
/// Ranking is in data order, giving sorted order positions. 
/// Thus sort index and ranks are in an inverse relationship. 
/// They are easily converted by `.invindex()` (for: invert index).
pub fn rank<T>(s:&[T], ascending:bool) -> Vec<usize> where T:PartialOrd+Copy {
    let n = s.len();
    let sortindex = mergesort(s,0,n);
    let mut rankvec:Vec<usize> = vec![0;n];
    if ascending { 
        for (i,&sortpos) in sortindex.iter().enumerate() {
            rankvec[sortpos] = i
        } 
    } else { // rank in the order of descending values
        for (i,&sortpos) in sortindex.iter().enumerate() {
            rankvec[sortpos] = n-i-1 
        }
    }
    rankvec 
}
