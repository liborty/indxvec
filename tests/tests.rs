#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use indxvec::{GS,merge::*,Indices};

#[test]
fn indxvec() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,16.]; 
   println!("{:?}",*GS(&v)); //  derefs the struct GS back to slice of concrete type
   let (min,minix,max,maxi) = minmax(&v);
   println!("Min {}, minidx {}, max {}, maxidx {}",min,minix,max,maxi);
   println!("Ranks to f64:\n{:?}",&rank(&v,true).indx_to_f64());    
   println!("Sorted:       {}",GS(&sortm(&v,true))); // sorted data but index lost
   println!("Sorted:       {}",GS(&sortidx(&v).unindex(&v,true))); // same as sortm
   println!("Sorted:       {}",GS(&rank(&v,false).invindex().unindex(&v,false)));   
   println!("Ranks:        {}",GS(&rank(&v,true))); // how to get ranks
   println!("Ranks rev:    {}",GS(&revs(&rank(&v,true)))); // reverse funtion reverses any vector
   println!("Ranks rev:    {}",GS(&sortidx(&v).complindex().invindex()));  // via sort index
   println!("Ranks desc:   {}",GS(&rank(&v,false))); // descending ranks, not the same as ranks reversed!!   
   println!("Ranks desc:   {}",GS(&sortidx(&v).invindex().complindex())); // descending ranks, not the same as ranks reversed!!  
   println!("Sort index:   {}",GS(&sortidx(&v))); // sortindex, can be unindexed at anytime
   println!("Sortix rev:   {}",GS(&rank(&v,false).invindex())); // descending sort index from desc ranks
   println!("Sortix compl: {}",GS(&sortidx(&v).complindex())); 
   println!("Ranks to idx: {}",GS(&rank(&v,true).invindex()));  // ascending sort index from ranks 
   println!("Idx to ranks: {}",GS(&sortidx(&v).invindex()));  
   println!("Sorted rev:   {}",GS(&sortm(&v,false))); // descending sort, index lost
   println!("Sorted rev:   {}",GS(&revs(&sortm(&v,true)))); // the above simply reversed
   println!("Sorted rev:   {}",GS(&sortidx(&v).unindex(&v,false))); // more efficient reversal  
   println!("Sorted rev:   {}",GS(&sortidx(&v).invindex().complindex().invindex().unindex(&v,true)));
   println!("Sorted rev:   {}",GS(&rank(&v,true).complindex().invindex().unindex(&v,true))); // odd falses = reversed
   println!("Spearman corr against itself: {}",rank(&v,true).ucorrelation(&rank(&v,true))); //  1 for any Vec
   println!("Spearman corr against reversed: {}",rank(&v,true).ucorrelation(&rank(&v,false))); // -1 for any Vec
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v)); // merge two vecs using their sort indices
   let sorted = vi.unindex(&vm, true);
   println!("Twice sorted, Merged and Unindexed:\n{}",GS(&sorted));  
   println!("Binsearch for {}, found before: {}",15.0,GS(&[binsearch(&sorted,15.0)])); // binsearch 
   let opt = memsearch(&sorted,15.0);
   print!("Memsearch for 15, found at: ");
   if opt.is_none() { println!("{}",GS(&["not found"])) } 
    else { println!("{}",GS(&[opt.unwrap()])) } 
   println!("Memsearch_indexed for {}, found at: {:?}",14.0,memsearch_indexed(&vm,&vi,14.0)); // binsearch 
   println!("Intersect_indexed: {}",GS(&intersect_indexed(&vm, &vi, &v, &sortidx(&v))));
   println!("Diff_indexed: {}",GS(&diff_indexed(&vm, &vi, &v, &sortidx(&v))));
   println!("Sansrepeat:   {}\n",GS(&sansrepeat(&sorted)));  
   ()
}
