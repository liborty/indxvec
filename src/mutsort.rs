use crate::{here,Mutsort,Vecops};

impl<T> Mutsort<T> for &mut[T] {

    /// swap two slice items if they are out of ascending order
    fn compswap(self, i1: usize, i2: usize) where T: PartialOrd { 
        if self[i1] > self[i2] { self.swap(i1,i2) } 
    }
        
    /// N recursive hash sort.
    /// Sorts mutable first argument (slice) in place
    /// Requires [min,max], the data range, that must enclose all its values. 
    /// The range is often known in advance. If not, it can be obtained with `minmaxt`.
    fn hashsort(self, min:f64, max:f64) where T: PartialOrd+Copy, f64:From<T> {
        if min >= max { panic!("{} data range must be min < max",here!()); };
        let n = self.len();
        match n {
            0|1 => (),
            2 => self.compswap(0,1),
            3 => {
                self.compswap(0,1);
                self.compswap(1,2);
                self.compswap(0,1)
            },
            _ => { self.hashsortr(0,n,min,max) }
        }   
    }
    
    fn hashsortr(self, i:usize, n:usize, min:f64, max:f64) 
        where T: PartialOrd+Copy, f64:From<T> {
    
        if n == 0 { panic!("{} unexpected zero length",here!())};  
        // hash is a constant s.t. (x-min)*hash is in [0,n)
        // -1e-10 stops subscript quite reaching n and causing out of bounds error  
        let hash = (n as f64 - 1e-10 ) / (max-min); 
        let mut freqvec:Vec<Vec<T>> = vec![Vec::new();n];
        // group current index items into buckets by their associated self[] values
        for &xi in self.iter().skip(i).take(n) { 
            freqvec[(hash*(f64::from(xi)-min)).floor() as usize].push(xi);
        };
        // flatten the buckets into the original index list 
        let mut isub = i; 
        for v in freqvec.iter() { 
            let vlen = v.len();     
            // print!("{} ",vlen);
            match vlen {
            0 => continue, // empty bucket
            1 => { self[isub] = v[0]; isub += 1; }, // copy the item to the main index
            2 => { 
                if v[1] < v[0] { self[isub] = v[1]; self[isub+1] = v[0];}
                else { self[isub] = v[0]; self[isub+1] = v[1]; };  
                isub += 2; 
            },
            3 => {
                self[isub] = v[0]; self[isub+1] = v[1]; self[isub+2] = v[2];   
                self.compswap(isub,isub+1);
                self.compswap(isub+1,isub+2);
                self.compswap(isub,isub+1);
                isub += 3;
            },
            x if x == n => { 
                // this bucket alone is populated, 
                // items in it are most likely all equal
                // we need not copy v back as no sorting took place
                let mx = self.minmax_slice(  isub, vlen);
                if mx.minindex < mx.maxindex {  // not all the same
                    let mut hold = self[i]; // swap minindex to the front
                    self[i] = self[mx.minindex]; 
                    self[mx.minindex] = hold;
                    hold = self[i+n-1]; // swap maxindex to the end
                    self[i+n-1] = self[mx.maxindex]; 
                    self[mx.maxindex] = hold;
                    // recurse to sort the rest, within the new reduced range
                    self.hashsortr(i+1,n-2,f64::from(mx.min),f64::from(mx.max)); 
                };
                return; // all items were equal, or are now sorted
            },
            _ => { 
                // first fill the index with the grouped items from v
                let isubprev = isub;
                for &item in v { self[isub] = item; isub += 1; }; 
                let mx = self.minmax_slice(isubprev, vlen);
                if mx.minindex < mx.maxindex { // else are all equal 
                    let mut hold = self[isubprev]; // swap minindex to the front
                    self[isubprev] = self[mx.minindex]; 
                    self[mx.minindex] = hold;
                    hold = self[isub-1]; // swap maxindex to the end
                    self[isub-1] = self[mx.maxindex]; 
                    self[mx.maxindex] = hold;
                    // recurse to sort the rest
                    self.hashsortr(isubprev+1,vlen-2,f64::from(mx.min),f64::from(mx.max)); 
                    }; // the items in this bucket were equal or are now sorted but there are more buckets
                } 
            } // end of match 
        } // end of for
    } 
    
    }
    