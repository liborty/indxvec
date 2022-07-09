use crate::{Mutops,Vecops};

impl<T> Mutops<T> for &mut[T] {

    /// Destructive reversal by swapping
    fn mutrevs(self) {
        let n = self.len();
        for i in 0..n/2 { self.swap(i,n-i-1) }
    }

   /// sort two slice items if they are out of ascending order
    fn mutsorttwo(self, i0: usize, i1:usize) -> bool where T: PartialOrd { 
        if self[i0] > self[i1] { self.swap(i0,i1); true }
        else { false }
    }

    /// sort three slice items if they are out of ascending order
    fn mutsortthree(self, i0: usize, i1:usize, i2:usize) where T: PartialOrd { 
        self.mutsorttwo(i0,i1);
        if self.mutsorttwo(i1,i2) { self.mutsorttwo(i0,i1); };    
    }
        
    /// N recursive hash sort.
    /// Sorts mutable first argument in place
    fn muthashsort(self) where T: PartialOrd+Copy, f64:From<T> {
        let (min,max) = self.minmaxt();
        self.muthashsortslice(0,self.len(),min,max);
    }

    /// Does the work for `muthashsort` 
    /// Requires [min,max], the data range, that must enclose all its values. 
    /// If the range is known in advance, use this in preference to `muthashsort`
    /// to save finding it
    fn muthashsortslice(self, i:usize, n:usize, min:T, max:T) 
        where T: PartialOrd+Copy, f64:From<T> { 
        // Recursion termination conditions
        match n {
            0|1 => { return; }, // no sorting needed
            2 => { self.mutsorttwo(i,i+1); return; },
            3 => { self.mutsortthree(i,i+1,i+2); return; },
            _ => ()
            };
        // convert limits to f64 for accurate hash calculations            
        let fmax = f64::from(max);
        let fmin = f64::from(min);
        // hash is a precomputed factor, s.t. ((x-min)*hash).floor() subscripts will be in [0,n]
        // this is then reduced to [0,n-1] 
        let hash = n as f64 / (fmax-fmin); 
        let mut buckets:Vec<Vec<T>> = vec![Vec::new();n];

        // group data items into buckets, subscripted by the data hash values
        for &xi in self.iter().skip(i).take(n) {
            let mut hashsub = (hash*(f64::from(xi)-fmin)).floor() as usize; 
            if hashsub == n { hashsub -= 1; }; 
            buckets[hashsub].push(xi);  
        };

        // isub to point at the current place in the self data
        let mut isub = i;
        // sort and copy the buckets into the self  
        for bucket in buckets.iter() { 
            let blen = bucket.len(); // size of the current bucket    
            // up to three items in a bucket can be sorted immediately
            // println!("muthashsortslice bucket start: {} items: {}",isub,blen);  
            match blen {
                0 => continue, // empty bucket
                1 => { self[isub] = bucket[0]; isub += 1; }, // copy the item to the sorted nut self
                2 => { 
                    self[isub] = bucket[0]; self[isub+1] = bucket[1];
                    self.mutsorttwo(isub,isub+1);            
                    isub += 2; 
                },
                3 => {
                    self[isub] = bucket[0]; self[isub+1] = bucket[1]; self[isub+2] = bucket[2];   
                    self.mutsortthree(isub,isub+1,isub+2); 
                    isub += 3;
                },
                x if x == n => { 
                    // this bucket alone is populated, 
                    // items in it are most likely all equal
                    // we need not copy bucket into self, as no grouping took place
                    let mx = self.minmax_slice(isub, blen);
                    if mx.min < mx.max {  // not all the same
                        self.mutsorttwo(isub,mx.minindex); // swap min to the front
                        self.mutsorttwo(mx.maxindex,isub+n-1); // and swap max to the end
                        // recurse to sort the rest, within the new reduced range
                        self.muthashsortslice(isub+1,blen-2,mx.min,mx.max); 
                    }; 
                    return; // all items in this single bucket were equal, or are now sorted
                },
                _ => { 
                    // copy to self the grouped but as yet unsorted items from bucket
                    let isubprev = isub;
                    for &item in bucket { self[isub] = item; isub += 1; }; 
                    // isub-1 now points at the last copied item
                    let mx = self.minmax_slice(isubprev, blen);
                    if mx.min < mx.max { 
                        self.mutsorttwo(isubprev,mx.minindex); // swap min to the front
                        self.mutsorttwo(mx.maxindex,isub-1); // and swap max to the end
                        // recurse to sort the rest in between, with reduced data range
                        self.muthashsortslice(isubprev+1,blen-2,mx.min,mx.max); 
                    }; 
                } // items in this bucket were equal or are now sorted
            } // end of match (this bucket) but there may be more
        } // end of for (all buckets)
    } 
}
    