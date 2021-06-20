//use crate::Merge;
use crate::{Indices,Merge};
use rstats::{here};

impl Merge for &[f64] {

    /// Returns index to the first item that is strictly greater than v, 
    /// using binary search of an ascending sorted list.
    /// When none are greater, returns self.len(). 
    /// User must check for this index overflow: if the returned index == 0, then v was below the list,
    /// else use index-1 as a valid index to the last item that is less than or equal to v.
    /// This then is the right index to use for looking up cummulative probability density functions. 
    fn binsearch(self, v: f64) -> usize {
        let n = self.len();
        if n < 2 { panic!("{} list is too short!",here!()) }
        if v < self[0] { return 0_usize }; // v is smaller than the first item
        let mut hi = n-1; // valid index of the last item
        if v > self[hi] { return n }; // indicates that v is greater than the last item
        let mut lo = 0_usize; // initial index of the low limit 
    
        loop {
            let gap = hi - lo;
            if gap <= 1 { return hi }
            let tryi = lo+gap/2; 
            // if tryi index's value is above v, reduce the high index
            if self[tryi] > v { hi = tryi; continue }            
            // else indexed value is not greater than v, raise the low index;
            // jumps also repeating equal values. 
            lo = tryi
        }  
    }

    /// Merges two ascending sorted vectors &[f64]
    fn merge(self, v: &[f64]) -> Vec<f64> {
        let mut resvec:Vec<f64> = Vec::new();
        let l1 = self.len();
        let l2 = v.len();
        let mut i1 = 0;
        let mut i2 = 0;
        loop {
            if i1 == l1 { // self is now processed
                for i in i2..l2 { resvec.push(v[i]) } // copy out the rest of v
                break // and terminate
            }
            if i2 == l2 { // v is now processed
                for i in i1..l1 { resvec.push(self[i])} // copy out the rest of self
                break // and terminate
            }
            if self[i1] < v[i2] { resvec.push(self[i1]); i1 += 1; continue };
            if self[i1] > v[i2] { resvec.push(v[i2]); i2 += 1; continue }; 
            // here they are equal, so consume both
            resvec.push(self[i1]); i1 += 1;
            resvec.push(v[i2]); i2 += 1
        }
        resvec
    }
 
    /// Merges two ascending sorted vectors' indices, returns concatenated Vec<f64> and new index into it.
    /// Mostly just a wrapper for merge_indices()
    fn merge_immutable(self, idx1: &[usize], v2: &[f64], idx2: &[usize]) -> ( Vec<f64>,Vec<usize> ) {
        let resvec = [self,v2].concat(); // no sorting, just concatenation 
        let l = idx1.len();
        let idx2shifted:Vec<usize> = idx2.iter().map(|x| l+x ).collect(); // shift up the second index
        let residx = resvec.merge_indices(idx1,&idx2shifted);   
        ( resvec, residx )
    }

    /// Merges indices of two already concatenated sorted vectors: 
    /// self is untouched, only sort indices are merged.
    /// Used by `mergesort` and `merge_immutable`. 
    fn merge_indices(self, idx1:&[usize], idx2:&[usize]) -> Vec<usize> {
        let l1 = idx1.len();
        let l2 = idx2.len();
        let mut residx:Vec<usize> = Vec::new(); 
        let mut i1 = 0;  let mut i2 = 0;
        let mut head1 = self[idx1[i1]]; let mut head2 = self[idx2[i2]];
        loop {
            if head1 < head2 { 
                residx.push(idx1[i1]);
                i1 += 1;  
                if i1 == l1 { // idx1 is now fully processed
                    for i in i2..l2 { residx.push(idx2[i]) } // copy out the rest of idx2
                    break // and terminate
                }
                head1 = self[idx1[i1]]; // else move to the next value
                continue
            }
            if head1 > head2 { 
                residx.push(idx2[i2]); 
                i2 += 1; 
                if i2 == l2 { // idx2 is now processed
                    for i in i1..l1 { residx.push(idx1[i]) } // copy out the rest of idx1
                    break // and terminate
                }                    
                head2 = self[idx2[i2]]; 
                continue
            } 
            // here the heads are equal, so consume both
            residx.push(idx1[i1]); 
            i1 += 1; 
            if i1 == l1 { // idx1 is now fully processed
                for i in i2..l2 { residx.push(idx2[i]) } // copy out the rest of idx2
                break // and terminate
            }
            head1 = self[idx1[i1]];
            residx.push(idx2[i2]); 
            i2 += 1; 
            if i2 == l2 { // idx2 is now processed
                for i in i1..l1 { residx.push(idx1[i]) } // copy out the rest of idx1
                break // and terminate
            }                    
            head2 = self[idx2[i2]];            
       }
        residx
    }
    
    /// Immutable sort. Returns new sorted vector, just like 'sortf' above
    /// but using our indexing 'mergesort' below.
    /// Simply passes the boolean flag 'ascending' onto 'unindex'.
    fn sortm(self, ascending:bool) -> Vec<f64> {
        self.mergesort(0,self.len()).unindex(ascending,&self)
    }

    /// Ranking of self by inverting the (merge) sort index.  
    /// Sort index is in sorted order, giving indices to the original data positions.
    /// Ranking is in  original data order, giving their positions in the sorted order (sort index).
    /// Thus they are in an inverse relationship, easily converted by `.invindex()`
    /// Fast ranking of many f64 items, ranking `self` with only n*(log(n)+1) complexity.
    fn rank(self, ascending:bool) -> Vec<usize> {
        let n = self.len();
        let sortindex = self.mergesort(0,n);
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

    /// Doubly recursive non-destructive merge sort. The data is read-only, it is not moved or mutated. 
    /// Returns vector of indices to self from i to i+n, such that the indexed values are in sort order.  
    /// Thus we are moving only the index (key) values instead of the actual values. 
    fn mergesort(self, i:usize, n:usize) -> Vec<usize> {

        if n == 1 { let res = vec![i]; return res };  // recursion termination
        if n == 2 {  // also terminate with two sorted items (for efficiency)          
            if self[i+1] < self[i] { return vec![i+1,i] } else { return vec![i,i+1] }
        }       
        let n1 = n / 2;  // the first half
        let n2 = n - n1; // the remaining second half
        let sv1 = self.mergesort(i, n1); // recursively sort the first half
        let sv2 = self.mergesort(i+n1, n2); // recursively sort the second half 
        // Now we will merge the two sorted indices into one      
        self.merge_indices(&sv1,&sv2)
    }
}
