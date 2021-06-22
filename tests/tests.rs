#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use anyhow::{Result};
use indxvec::{Indices,GV};
use indxvec::merge::{revs,sortm,sortidx,merge_indexed,rank,binsearch};

#[test]
fn vecf64() -> Result<()> { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,15.];
   println!("{}",GV(&v));
   println!("Sorted:       {}",GV(&sortm(&v,true)));
   println!("Sorted:       {}",GV(&sortidx(&v).unindex(true,&v))); 
   println!("Sorted:       {}",GV(&rank(&v,false).invindex().unindex(false,&v)));   
   println!("Ranks:        {}",GV(&rank(&v,true))); 
   println!("Ranks rev:    {}",GV(&rank(&v,true).iter().rev().collect::<Vec<&usize>>())); 
   println!("Ranks desc:   {}",GV(&rank(&v,false))); // not the same as ranks reversed!!   
   println!("Sort index:   {}",GV(&sortidx(&v))); 
   println!("Ix from Ranks:{}",GV(&rank(&v,true).invindex()));
   println!("Sort ix desc: {}",GV(&rank(&v,false).invindex()));
   println!("Ranks from ix:{}",GV(&rank(&v,false).invindex().invindex()));
   println!("Sorted rev:   {}",GV(&sortm(&v,false)));
   println!("Sorted rev:   {}",GV(&revs(&sortm(&v,true))));
   println!("Sorted rev:   {}",GV(&sortidx(&v).unindex(false,&v)));   
   println!("Sorted rev:   {}",GV(&rank(&v,false).invindex().unindex(true,&v)));
   println!("Spearman corr against itself: {}",rank(&v,true).ucorrelation(&rank(&v,true))); 
   println!("Spearman corr against reversed: {}",rank(&v,true).ucorrelation(&rank(&v,false))); 
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v));
   println!("Twice sorted, Concatenated and Merged:\n{}",GV(&vi.unindex(true,&vm))); 
   println!("Searched for {}, found at: {}\n",14.0,binsearch(&vi.unindex(true,&vm),14.0));   
   Ok(())
}
