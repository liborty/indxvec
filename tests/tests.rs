#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use indxvec::{wv,wi,wt,merge::*,Indices};

#[test]
fn indxvec() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,16.]; 
   println!("{}",wv(&v));
   println!("Minmax:       {}",wt(&minmax(&v)));
   println!("Ranks to f64: {}",wv(&rank(&v,true).indx_to_f64()));    
   println!("Sorted:       {}",wv(&sortm(&v,true))); // sorted data but index lost
   println!("Sorted:       {}",wv(&sortidx(&v).unindex(&v,true))); // same as sortm
   println!("Sorted:       {}",wv(&rank(&v,false).invindex().unindex(&v,false)));   
   println!("Ranks:        {}",wv(&rank(&v,true))); // how to get ranks
   println!("Ranks:        {}",wv(&rank(&v,true).complindex().complindex())); // symmetry
   println!("Ranks rev:    {}",wv(&rank(&v,true).revindex())); // revindex() reverses any index 
   println!("Ranks rev:    {}",wv(&sortidx(&v).complindex().invindex()));  // via sortidx()  and complindex()
   println!("Ranks rev:    {}",wv(&sortidx(&v).invindex().revindex())); // via revindex()
   println!("Ranks desc:   {}",wv(&rank(&v,false))); // descending ranks, not the same as ranks reversed!!   
   println!("Ranks desc:   {}",wv(&rank(&v,true).complindex())); // descending ranks, not the same as ranks reversed!!   
   println!("Ranks desc:   {}",wv(&sortidx(&v).invindex().complindex())); // descending ranks, not the same as ranks reversed!!  
   println!("Sort index:   {}",wv(&sortidx(&v))); // sortindex, can be unindexed at anytime
   println!("Sortix rev:   {}",wv(&sortidx(&v).revindex())); 
   println!("Sortix rev:   {}",wv(&rank(&v,false).invindex())); // descending sort index from desc ranks
   println!("Ranks to idx: {}",wv(&rank(&v,true).invindex()));  // ascending sort index from ranks 
   println!("Ranks to idx: {}",wv(&rank(&v,false).complindex().invindex())); 
   println!("Idx to ranks: {}",wv(&sortidx(&v).invindex()));  
   println!("Sorted rev:   {}",wv(&sortm(&v,false))); // descending sort, index lost
   println!("Sorted rev:   {}",wv(&revs(&sortm(&v,true)))); // the above simply reversed
   println!("Sorted rev:   {}",wv(&sortidx(&v).unindex(&v,false))); // more efficient reversal 
   println!("Sorted rev:   {}",wv(&sortidx(&v).revindex().unindex(&v,true))); // by reversing the sort index 
   println!("Sorted rev:   {}",wv(&sortidx(&v).invindex().complindex().invindex().unindex(&v,true)));
   println!("Sorted rev:   {}",wv(&rank(&v,true).complindex().invindex().unindex(&v,true))); // complindex reverses ranks
   println!("Spearman corr against itself: {}",wi(&rank(&v,true).ucorrelation(&rank(&v,true)))); //  1 for any Vec
   println!("Spearman corr against reversed: {}",wi(&rank(&v,true).ucorrelation(&rank(&v,false)))); // -1 for any Vec
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v)); // merge two vecs using their sort indices
   let sorted = vi.unindex(&vm, true);
   println!("Twice sorted, Merged and Unindexed:\n{}",wv(&sorted));  
   println!("Binsearch for {}, found before: {}",15.0,wi(&binsearch(&sorted,15.0))); // binsearch 
   let opt = memsearchdesc(&revs(&sorted),14.0);
   print!("Memsearchdesc for 14, found at: ");
   if opt.is_none() { println!("{}",wi(&"None")) } else { println!("{}",wi(&opt.unwrap())) } 
   println!("Memsearch_indexed for {}, found at: {}",14.0,wi(&memsearch_indexed(&vm,&vi,14.0).unwrap())); // binsearch 
   println!("Intersect_indexed: {}",wv(&intersect_indexed(&vm, &vi, &v, &sortidx(&v))));
   println!("Diff_indexed: {}",wv(&diff_indexed(&vm, &vi, &v, &sortidx(&v))));
   println!("Sansrepeat:   {}\n",wv(&sansrepeat(&sorted)));  
   ()
}
