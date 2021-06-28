#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use indxvec::{GS,merge::*,Indices};

#[test]
fn indxvec() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,15.]; 
   println!("{:?}",*GS(&v)); //  derefs the struct GS back to slice of concrete type
   let (min,minix,max,maxi) = minmax(&v);
   println!("Min {}, minidx {}, max {}, maxidx {}",min,minix,max,maxi);
   println!("Ranks to f64:\n{:?}",&rank(&v,true).indx_to_f64());    
   println!("Sorted:       {}",GS(&sortm(&v,true))); // sorted data but index lost
   println!("Sorted:       {}",GS(&sortidx(&v).unindex(&v,true))); // same as sortm
   println!("Sorted:       {}",GS(&rank(&v,false).invindex().unindex(&v,false)));   
   println!("Ranks:        {}",GS(&rank(&v,true))); // how to get ranks
   println!("Ranks rev:    {}",GS(&revs(&rank(&v,true)))); // reverse funtion reverses any vector
   println!("Ranks desc:   {}",GS(&rank(&v,false))); // descending ranks, not the same as ranks reversed!!   
   println!("Sort index:   {}",GS(&sortidx(&v))); // sortindex, can be unindexed at anytime
   println!("Ranks to idx: {}",GS(&rank(&v,true).invindex()));  // ascending sort index from ranks
   println!("Sort ix desc: {}",GS(&rank(&v,false).invindex())); // descending sort index from ranks
   println!("Idx to ranks: {}",GS(&rank(&v,false).invindex().invindex())); // even inverses = original
   println!("Sorted rev:   {}",GS(&sortm(&v,false))); // descending sort, index lost
   println!("Sorted rev:   {}",GS(&revs(&sortm(&v,true)))); // the above simply reversed
   println!("Sorted rev:   {}",GS(&sortidx(&v).unindex(&v,false))); // more efficient reversal  
   println!("Sorted rev:   {}",GS(&rank(&v,false).invindex().unindex(&v,true))); // odd falses = reversed
   println!("Spearman corr against itself: {}",rank(&v,true).ucorrelation(&rank(&v,true))); //  1 for any Vec
   println!("Spearman corr against reversed: {}",rank(&v,true).ucorrelation(&rank(&v,false))); // -1 for any Vec
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v)); // merge two vecs using their sort indices
   println!("Twice sorted, Concatenated and Merged:\n{}",GS(&vi.unindex(&vm,true))); 
   println!("Searched for {}, found at: {}\n",14.0,binsearch(&vi.unindex(&vm,true),14.0)); // binary search  
   ()
}
