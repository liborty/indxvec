#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use anyhow::{Result};
use indxvec::{Indices,GV};
use indxvec::merge::{revs,sortm,mergesort,merge_indexed,rank,binsearch};

pub const EPS:f64 = 1e-7;

#[test]
fn vecf64() -> Result<()> { 
   let v = vec![1.,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,15.];
   println!("{}",GV(&v));
   println!("Sorted:       {}",GV(&sortm(&v,true)));
   println!("Sorted:       {}",GV(&mergesort(&v,0,v.len()).unindex(true,&v))); 
   println!("Sorted:       {}",GV(&rank(&v,false).invindex().unindex(false,&v)));   
   println!("Ranks:        {}",GV(&rank(&v,true))); 
   println!("Ranks rev:    {}",GV(&rank(&v,true).iter().rev().collect::<Vec<&usize>>())); 
   println!("Ranks desc:   {}",GV(&rank(&v,false))); // not the same as ranks reversed!!   
   println!("Sort index:   {}",GV(&mergesort(&v,0,v.len()))); 
   println!("Ix from Ranks:{}",GV(&rank(&v,true).invindex()));
   println!("Sort ix desc: {}",GV(&rank(&v,false).invindex()));
   println!("Ranks from ix:{}",GV(&rank(&v,false).invindex().invindex()));
   println!("Sorted rev:   {}",GV(&sortm(&v,false)));
   println!("Sorted rev:   {}",GV(&revs(&sortm(&v,true))));
   println!("Sorted rev:   {}",GV(&mergesort(&v,0,v.len()).unindex(false,&v)));   
   println!("Sorted rev:   {}",GV(&rank(&v,false).invindex().unindex(true,&v))); 
   let (vm,vi) = merge_indexed(&v,&mergesort(&v,0,v.len()), &v, &mergesort(&v,0,v.len()));
   println!("Sorted, Concatenated and Merged:\n{}",GV(&vi.unindex(true,&vm))); 
   println!("Searched for {}, found at: {}\n",14.0,binsearch(&vi.unindex(true,&vm),14.0));   
   Ok(())
}
