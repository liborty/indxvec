#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use indxvec::{GR,UNGR,tof64,merge::*,Indices,Printing};

#[test]
fn indxvec() { 
   let v = vec![1,14,2,13,3,12,4,11,5,10,6,9,7,8,16]; 
   println!("{}",v.to_str());
   println!("Minmax:       {}",minmax(&v));
   println!("minmaxt:      {}{:?}{}",GR,minmaxt(&v),UNGR);
   println!("Ranks to f64: {}",rank(&v,true).gr());    
   println!("Sorted:       {}",sortm(&v,true).indx_to_f64().gr()); // sorted data but index lost
   println!("Sorted:       {}",sortidx(&v).unindex(&v,true).gr()); // same as sortm
   println!("Sorted:       {}",rank(&v,false).invindex().unindex(&v,false).gr());   
   println!("Ranks:        {}",rank(&v,true).gr()); // how to get ranks
   println!("Ranks:        {}",rank(&v,true).complindex().complindex().gr()); // symmetry
   println!("Ranks rev:    {}",rank(&v,true).revindex().gr()); // revindex() reverses any index 
   println!("Ranks rev:    {}",sortidx(&v).complindex().invindex().gr());  // via sortidx()  and complindex()
   println!("Ranks rev:    {}",sortidx(&v).invindex().revindex().gr()); // via revindex()
   println!("Ranks desc:   {}",rank(&v,false).gr()); // descending ranks, not the same as ranks reversed!!   
   println!("Ranks desc:   {}",rank(&v,true).complindex().gr()); // descending ranks, not the same as ranks reversed!!   
   println!("Ranks desc:   {}",sortidx(&v).invindex().complindex().gr()); // descending ranks, not the same as ranks reversed!!  
   println!("Sort index:   {}",sortidx(&v).gr()); // sortindex, can be unindexed at anytime
   println!("Sortix rev:   {}",sortidx(&v).revindex().gr()); 
   println!("Sortix rev:   {}",rank(&v,false).invindex().gr()); // descending sort index from desc ranks
   println!("Ranks to idx: {}",rank(&v,true).invindex().gr());  // ascending sort index from ranks 
   println!("Ranks to idx: {}",rank(&v,false).complindex().invindex().gr()); 
   println!("Idx to ranks: {}",sortidx(&v).invindex().gr());  
   println!("Sorted rev:   {}",sortm(&v,false).gr()); // descending sort, index lost
   println!("Sorted rev:   {}",revs(&sortm(&v,true)).gr()); // the above simply reversed
   println!("Sorted rev:   {}",sortidx(&v).unindex(&v,false).gr()); // more efficient reversal 
   println!("Sorted rev:   {}",sortidx(&v).revindex().unindex(&v,true).gr()); // by reversing the sort index 
   println!("Sorted rev:   {}",sortidx(&v).invindex().complindex().invindex().unindex(&v,true).gr());
   println!("Sorted rev:   {}",rank(&v,true).complindex().invindex().unindex(&v,true).gr()); // complindex reverses ranks
   println!("Spearman corr against itself: {}",rank(&v,true).ucorrelation(&rank(&v,true)).gr()); //  1 for any Vec
   println!("Spearman corr against reversed: {}",rank(&v,true).ucorrelation(&rank(&v,false)).gr()); // -1 for any Vec
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v)); // merge two vecs using their sort indices
   let sorted = vi.unindex(&vm, true);   
   println!("Twice sorted, Merged and Unindexed:\n{}",sorted.gr()); 
   let sorteddesc = vi.unindex(&vm,false); 
   println!("The above reversed:\n{}",sorteddesc.gr()); 
   println!("Binsearch for 15, found before: {}",binsearch(&sorted,15).gr()); // binsearch 
   println!("Binsearchdesc for 15, found before: {}",binsearchdesc(&sorteddesc,15).gr()); // binsearch 
   let opt = memsearchdesc(&revs(&sorted),14);
   print!("Memsearchdesc for 14, found at: ");
   if opt.is_none() { println!("{}","None".gr()) } else { println!("{}",opt.unwrap().gr()) } 
   println!("Memsearch_indexed for 14, found at: {}",memsearch_indexed(&vm,&vi,14).unwrap().gr());
   println!("Occurrences count of 14: {}",occurs(&sorted,&sorteddesc,14).gr());
   println!("Occurrences count of 15: {}",occurs(&sorted,&sorteddesc,15).gr());
   println!("Intersect_indexed: {}",intersect_indexed(&vm, &vi, &v, &sortidx(&v)).gr());
   println!("Diff_indexed: {}",diff_indexed(&vm, &vi, &v, &sortidx(&v)).gr());
   println!("Sansrepeat:   {}\n",sansrepeat(&sorted).gr());  
}
