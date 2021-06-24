use crate::{Indices};
use crate::here;

/// Reverse a generic slice by reverse iteration.
/// Is immutable, creates a new Vec.
/// Its naive use for descending sort etc. is avoided for
/// efficiency reasons. Included here just for convenience.
pub fn revs<T>(s: &[T]) -> Vec<T> where T: Copy, 
    { s.iter().rev().map(|&x| x).collect::<Vec<T>>() }

/// Binary search of a sorted list (in ascending order).
/// Returns the index of the first item that is greater than val. 
/// When none are greater, returns (invalid but logical index value) s.len().
/// Example use: looking up cummulative probability density functions. 
pub fn binsearch<T>(s:&[T], val: T)  -> usize where T: PartialOrd, {     
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
        // if tryi index's value is above val, reduce the high index
        if s[tryi] > val { hi = tryi; continue }            
        // else indexed value is not greater than v, raise the low index;
        // jumps also repeating equal values. 
        lo = tryi
    }  
}

/// Merges two ascending sorted generic vectors,
/// by classical selection and copying of their head items into the result.
/// Consider using merge_indexed instead, especially for non-primitive end types T. 
pub fn merge<T>(v1: &[T], v2: &[T]) -> Vec<T> where T: PartialOrd+Copy, {  
    let mut resvec:Vec<T> = Vec::new();
    let l1 = v1.len();
    let l2 = v2.len();
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
    where T: PartialOrd+Copy,    
{     
    let res = [v1,v2].concat(); // no individual shuffling, just bulk concatenation     
    let l = idx1.len();
    // shift up all items in idx2 by length of indx1, so that they will 
    // refer correctly to the second part of the concatenated vector
    let idx2shifted:Vec<usize> = idx2.iter().map(|x| l+x ).collect();
    // now perform the merge of the indices only     
    let residx = merge_indices(&res,idx1,&idx2shifted);   
    ( res, residx )
}

/// Merges the sort indices of two simply concatenated vectors.
/// Data in s is not changed at all, only consulted for the comparisons. 
/// Used by  `mergesort` and `merge_indexed`. 
fn merge_indices<T>(s: &[T], idx1:&[usize], idx2:&[usize]) -> Vec<usize>
    where T: PartialOrd+Copy, {
    let l1 = idx1.len();
    let l2 = idx2.len();
    let mut residx:Vec<usize> = Vec::new(); 
    let mut i1 = 0;  let mut i2 = 0;
        let mut head1 = s[idx1[i1]]; let mut head2 = s[idx2[i2]];
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
/// The data is read-only, it is not moved or mutated.
/// Efficiency is comparable to quicksort. 
/// Returns a vector of indices to s from i to i+n,
/// such that the indexed values are in sort order (sort index).  
/// Thus only the index values are being moved. 
fn mergesort<T>(s:&[T], i:usize, n:usize) -> Vec<usize> 
    where T: PartialOrd+Copy {
    if n == 1 { let res = vec![i]; return res };  // recursion termination
    if n == 2 {  // also terminate with two sorted items (for efficiency)          
        if s[i+1] < s[i] { return vec![i+1,i] } else { return vec![i,i+1] }
    }       
    let n1 = n / 2;  // the first part (the parts do not have to be the same) 
    let n2 = n - n1; // the remaining second part
    let sv1 = mergesort(s, i, n1); // recursively sort the first half
    let sv2 = mergesort(s, i+n1, n2); // recursively sort the second half 
    // Now merge the two sorted indices into one      
    merge_indices(s,&sv1,&sv2)
}

/// A simple wrapper for mergesort, when we want just the sort index
/// of the entire input vector. Simpler than sortm.
pub fn sortidx<T>(s:&[T]) -> Vec<usize> where T:PartialOrd+Copy {
    mergesort(&s,0,s.len())
}

/// Immutable sort. Returns new sorted vector (ascending or descending)
/// Simple wrapper for mergesort. Passes the boolean flag 'ascending' onto 'unindex'.
pub fn sortm<T>(s:&[T], ascending:bool) -> Vec<T> where T: PartialOrd+Copy {
    mergesort(s,0,s.len()).unindex(s,ascending)
}   

/// Ranking by inverting the sort index.  
/// Sort index is in sorted order, giving indices to the original data positions.
/// Ranking is in  original data order, giving positions in the sorted order (sort index).
/// Thus they are in an inverse relationship, easily converted by `.invindex()`
/// Fast ranking of many T items, with only `n*(log(n)+1)` complexity.
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

