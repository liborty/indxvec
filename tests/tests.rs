#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

// use anyhow::{Result};
use indxvec::{GS,merge::*,Indices};

#[test]
fn vecf64() -> () { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,15.];
   println!("{}",GS(&v)); //  uses Display impl for GS 
   println!("{:?}",*GS(&v)); //  derefs the struct GS back to slice of concrete type
   println!("Sorted:       {}",GS(&sortm(&v,true)));
   println!("Sorted:       {}",GS(&sortidx(&v).unindex(true,&v))); 
   println!("Sorted:       {}",GS(&rank(&v,false).invindex().unindex(false,&v)));   
   println!("Ranks:        {}",GS(&rank(&v,true))); 
   println!("Ranks rev:    {}",GS(&rank(&v,true).iter().rev().collect::<Vec<&usize>>())); 
   println!("Ranks desc:   {}",GS(&rank(&v,false))); // not the same as ranks reversed!!   
   println!("Sort index:   {}",GS(&sortidx(&v))); 
   println!("Ix from Ranks:{}",GS(&rank(&v,true).invindex()));
   println!("Sort ix desc: {}",GS(&rank(&v,false).invindex()));
   println!("Ranks from ix:{}",GS(&rank(&v,false).invindex().invindex()));
   println!("Sorted rev:   {}",GS(&sortm(&v,false)));
   println!("Sorted rev:   {}",GS(&revs(&sortm(&v,true))));
   println!("Sorted rev:   {}",GS(&sortidx(&v).unindex(false,&v)));   
   println!("Sorted rev:   {}",GS(&rank(&v,false).invindex().unindex(true,&v)));
   println!("Spearman corr against itself: {}",rank(&v,true).ucorrelation(&rank(&v,true))); 
   println!("Spearman corr against reversed: {}",rank(&v,true).ucorrelation(&rank(&v,false))); 
   let (vm,vi) = merge_indexed(&v,&sortidx(&v),&v,&sortidx(&v));
   println!("Twice sorted, Concatenated and Merged:\n{}",GS(&vi.unindex(true,&vm))); 
   println!("Searched for {}, found at: {}\n",14.0,binsearch(&vi.unindex(true,&vm),14.0));   
   ()
}
