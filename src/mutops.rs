use crate::{Mutops, Vecops};
use core::cmp::{Ordering, Ordering::*};
use core::ops::Range;

impl<T> Mutops<T> for &mut [T] {
    /// Partitions `s: &mut [u64]` within range `rng`, using bitmask.  
    /// Returns the boundary of the rearranged partitions gtstart, where  
    /// `rng.start..gtstart` (may be empty) contains items with no bits matching the bitmask,  
    /// `gtstart..rng.end` (may be empty) contains items with some matching bits.
    fn part_binary(self, rng: &Range<usize>, bitmask: u64) -> usize
    where
        T: Copy, u64: From<T>
    {
        let mut gtstart = rng.start;
        for &lt in self.iter().take(rng.end).skip(rng.start) {
            if (<T as std::convert::Into<u64>>::into(lt) & bitmask) == 0 {
                gtstart += 1;
            } else {
                break;
            };
        }
        for i in gtstart + 1..rng.end {
            if (<T as std::convert::Into<u64>>::into(self[i]) & bitmask) == 0 {
                self.swap(gtstart, i);
                gtstart += 1;
            };
        }
        gtstart
    }
    /// Destructive reversal by swapping
    fn mutrevs(self) {
        let n = self.len();
        for i in 0..n / 2 {
            self.swap(i, n - i - 1)
        }
    }

    /// sort two slice items if they are out of ascending order
    fn mutsorttwo(self, i0: usize, i1: usize) -> bool
    where
        T: PartialOrd,
    {
        if self[i0] > self[i1] {
            self.swap(i0, i1);
            true
        } else {
            false
        }
    }

    /// sort three slice items if they are out of ascending order
    fn mutsortthree(self, i0: usize, i1: usize, i2: usize)
    where
        T: PartialOrd,
    {
        self.mutsorttwo(i0, i1);
        if self.mutsorttwo(i1, i2) {
            self.mutsorttwo(i0, i1);
        };
    }

    /// Requires [min,max], the data range, that must enclose all its values.
    /// If the range is known in advance, use this in preference to `muthashsort`
    /// to save finding it
    fn muthashsortslice(
        self,
        i: usize,
        n: usize,
        fmin: f64,
        fmax: f64,
        quantify: impl Copy + Fn(&T) -> f64,
    ) where
        T: PartialOrd + Clone,
    {
        // convert limits to f64 for accurate hash calculations
        // hash is a precomputed factor, s.t. ((x-min)*hash).floor() subscripts will be in [0,n]
        // this is then reduced to [0,n-1]
        let hash = n as f64 / (fmax - fmin);
        let mut buckets: Vec<Vec<T>> = vec![Vec::new(); n];

        // group data items into buckets, subscripted by the data hash values
        for xi in self.iter().skip(i).take(n) {
            let mut hashsub = (hash * (quantify(xi) - fmin)).floor() as usize;
            if hashsub == n {
                hashsub -= 1;
            };
            buckets[hashsub].push(xi.clone());
        }

        // isub to point at the current place in the self data
        let mut isub = i;
        // sort and copy the buckets into the self
        for bucket in buckets.iter_mut() {
            let blen = bucket.len(); // size of the current bucket
                                     // up to three items in a bucket can be sorted immediately
                                     // println!("muthashsortslice bucket start: {} items: {}",isub,blen);
            match blen {
                0 => continue, // empty bucket
                // copy the item to the sorted mut self
                1 => {
                    self[isub] = bucket[0].clone();
                    isub += 1;
                }
                2 => {
                    self[isub] = bucket[0].clone();
                    self[isub + 1] = bucket[1].clone();
                    self.mutsorttwo(isub, isub + 1);
                    isub += 2;
                }
                3 => {
                    self[isub] = bucket[0].clone();
                    self[isub + 1] = bucket[1].clone();
                    self[isub + 2] = bucket[2].clone();
                    self.mutsortthree(isub, isub + 1, isub + 2);
                    isub += 3;
                }
                x if x < 120 => {
                    // small buckets sorted by quicksort
                    bucket.sort_unstable_by(|a, b| quantify(a).total_cmp(&quantify(b)));
                    for item in bucket {
                        self[isub] = item.clone();
                        isub += 1;
                    } // copy sorted bucket to self
                }
                x if x == n => {
                    // this bucket alone is populated,
                    // items in it are most likely all equal
                    // we need not copy bucket into self, as no grouping to buckets took place
                    let mx = self.minmax_slice(isub, blen);
                    if mx.min < mx.max {
                        // not all the same
                        self.mutsorttwo(isub, mx.minindex); // swap min to the front
                        self.mutsorttwo(mx.maxindex, isub + n - 1); // and swap max to the end
                                                                    // recurse to sort the rest, within the new reduced range
                        self.muthashsortslice(
                            isub + 1,
                            blen - 2,
                            quantify(&mx.min),
                            quantify(&mx.max),
                            quantify,
                        );
                    };
                    return; // all items in this single bucket were equal, or are now sorted
                }
                _ => {
                    // copy to self the grouped but as yet unsorted items from bucket
                    let isubprev = isub;
                    for item in bucket {
                        self[isub] = item.clone();
                        isub += 1;
                    }
                    // isub-1 now points at the last copied item
                    let mx = self.minmax_slice(isubprev, blen);
                    if mx.min < mx.max {
                        self.mutsorttwo(isubprev, mx.minindex); // swap min to the front
                        self.mutsorttwo(mx.maxindex, isub - 1); // and swap max to the end
                                                                // recurse to sort the rest in between, with reduced data range
                        self.muthashsortslice(
                            isubprev + 1,
                            blen - 2,
                            quantify(&mx.min),
                            quantify(&mx.max),
                            quantify,
                        );
                    };
                } // items in this bucket were equal or are now sorted
            } // end of match (this bucket) but there may be more
        } // end of for (all buckets)
    }

    /// N recursive hash sort.
    /// Sorts mutable first argument in place
    /// Takes closure `quantify` for converting user type T to f64
    fn muthashsort(self, quantify: impl Copy + Fn(&T) -> f64)
    where
        T: PartialOrd + Clone,
    {
        let n = self.len();
        if n < 120 {
            // use default Rust sort for short Vecs
            self.sort_unstable_by(|a, b| quantify(a).total_cmp(&quantify(b)));
            return;
        };
        let (min, max) = self.minmaxt();
        self.muthashsortslice(0, n, quantify(&min), quantify(&max), quantify);
    }

    /// Mutable insert logsort. Pass in reversed comparator `c` for descending sort
    fn mutisort<F>(self, rng: Range<usize>, c: F)
    where
        T: Copy,
        F: Fn(&T, &T) -> Ordering,
    {
        if self.len() < 2 {
            return;
        };
        if c(&self[rng.start + 1], &self[rng.start]) == Less {
            self.swap(rng.start, rng.start + 1);
        };
        for i in rng.start + 2..rng.end {
            if c(&self[i], &self[i - 1]) == Less {
                let target = self[i];
                let insert = match self[rng.start..i - 1].binary_search_by(|j| c(j, &target)) {
                    Ok(ins) => ins + 1,
                    Err(ins) => ins, // *ins when using Search::binary_by()
                };
                self.copy_within(insert..i, insert + 1);
                self[insert] = target;
            };
        }
    }
}
