#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, F64, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use times::*;
use std::convert::From;

#[test]
fn indices() {
    let midval:u8 = 128;
    set_seeds(98777777);
    let rn = Rnum::newu8();
    let v1 = rn.ranv(20).getvu8(); 
    let mut vm = v1.clone();
    println!("{GR}\nv1: {}", v1.bl()); 
    let v2 = rn.ranv(20).getvu8(); 
    println!("{GR}v2: {}", v2.bl());    
    println!("minmax v1: {}", v1.minmax());
    println!("minmaxt v1: {GR}{:?}{UN}", v1.minmaxt()); 
    println!("minmaxt v2: {GR}{:?}{UN}", v2.minmaxt()); 
    let (lset,eqset,gset) = v1.partition(midval);
    println!( "v1 partitioned by data value {midval}:\n{}\n{}\n{}",
        lset.gr(),eqset.gr(),gset.gr() );
    println!("Sorted by merge sort:\n{}", v1.sortm(true).gr()); // sorted data but index lost
    vm.muthashsort(); // destructive (mutable) sort of wm
    println!("Sorted by muthashsort:\n{}", vm.gr()); // hashsorted
    let v1ranks = v1.rank(true); // ascending ranks
    let v1ranksd = v1.rank(false); // descending ranks
    println!("Sorted via ranking:\n{}", v1ranks.invindex().unindex(&v1,true).gr() );
    println!("Ranks:        {}", v1ranks.gr()); // how to get ranks
    println!("Ranks:        {}", v1ranks.complindex().complindex().gr() ); // symmetry
    println!("Ranks:        {}", v1.hashsort_indexed().invindex().gr()); // simplest ranks from sortindex
    println!("Ranks rev:    {}", v1ranks.revs().gr()); // revindex() reverses any index
    println!("Ranks rev:    {}", v1.mergesort_indexed().complindex().invindex().gr()); // via mergesort_indexed()  and complindex()
    println!("Ranks rev:    {}", v1.hashsort_indexed().invindex().revs().gr()); // via revindex()
    println!("Ranks desc:   {}", v1.rank(false).gr()); // descending ranks are not the same as ranks reversed!!
    println!("Ranks desc:   {}", v1ranks.complindex().gr()); // to make ranks descending, use complindex() instead
    println!("Ranks desc:   {}", v1.hashsort_indexed().invindex().complindex().gr()); // descending ranks from sortindex
    println!("Ranks desc:   {}", v1.hashsort_indexed().revs().invindex().gr()); // descending ranks from descending sort
    println!("Mergeort idx: {}", v1.mergesort_indexed().gr()); // can be unindexed at anytime
    println!("Hashsort idx: {}", v1.hashsort_indexed().gr());    
    println!("Sortix rev:   {}", v1.mergesort_indexed().revs().gr());
    println!("Sortix rev:   {}", v1ranksd.invindex().gr()); // descending sort index from desc ranks
    println!("Sortix rev:   {}", v1ranks.complindex().invindex().gr()); // descending sort index from desc ranks    
    println!("Ranks to idx: {}", v1ranks.invindex().gr()); // ascending sort index from ascending ranks
    println!("Ranks to idx: {}", v1ranksd.complindex().invindex().gr()); // sort index from ascending ranks
    println!("Idx to ranks: {}", v1.mergesort_indexed().invindex().gr());
    println!("Sortm naively reversed:\n{}",v1.sortm(true).revs().gr()); // the above simply reversed
    println!("Sortm false:\n{}", v1.sortm(false).gr()); // descending sort, index lost
    println!("Sorth false:\n{}", v1.sorth(false).gr()); 
    println!("mergesort_indexed unindex false:\n{}", v1.mergesort_indexed().unindex(&v1, false).gr()); // more efficient reversal
    println!("hashsort_indexed unindex false:\n{}", v1.hashsort_indexed().unindex(&v1, false).gr()); // more efficient reversal
    println!("Revindex:\n{}", v1.mergesort_indexed().revs().unindex(&v1, true).gr()); // by reversing the sort index
    println!("Index-invert-compliment-invert-unindex:\n{}", v1.mergesort_indexed().invindex().complindex().invindex().unindex(&v1, true).gr());
    println!("Rank-compliment-invert-unindex:\n{}", v1.rank(true).complindex().invindex().unindex(&v1, true).gr()); // complindex reverses ranks
    println!("Spearman corr v1,v2: {}", v1ranks.ucorrelation(&v2.rank(true)).gr()); //  1 for any Vec
    println!("Spearm. corr self 1: {}", v1ranks.ucorrelation(&v1ranks).gr()); // 1 for any Vec
}

#[test]
fn vecops() { 
    let midval:u8 = 128;
    let rn = Rnum::newu8();
    let v1 = rn.ranv(20).getvu8();
    println!("{GR}\nv1: {}", v1.bl());  
    let v2 = rn.ranv(20).getvu8();
    println!("{GR}v2: {}", v2.bl());    
    let (vm, mut vi) = v1.merge_indexed( // merge two vecs using their sort indices
        &v1.hashsort_indexed(), &v2,&v2.hashsort_indexed());  
    println!("\nv1 and v2 appended:\n{}",vm.gr());
    println!("Number of occurrences of {BL}89{UN}: {GR}{}{UN}",vm.occurs(89));  
    println!("Number of occurrences of {BL}128{UN}: {GR}{}{UN}",vm.occurs(128));  
    println!("Number of occurrences of {BL}199{UN}: {GR}{}{UN}",vm.revs().occurs(199));  

    let mut sorted = vi.unindex(&vm, true);    
    println!("v1 and v2 sorted, merged and unindexed:\n{}", sorted.mg()); 
    let (idx,count) = sorted.binsearch(&199);
    println!("Binary_search for {BL}199{UN}: {GR}({},{}){UN}",idx,count); 
    println!("Binsearch_indexed for {BL}{midval}{UN}, fits in sort position before: {GR}{}{UN}",
        vm.binsearch_indexed(&vi,midval)); // binsearch_indexed
    println!("Nearest greater item from {BL}{midval}{UN} is: {GR}{}{UN}",
        vm[vi[vm.binsearch_indexed(&vi,midval)]]);
    println!("Member forwards for {BL}199{UN}, is in sorted at: {}",
        sorted.member(199,true).map_or_else(||"None".rd(),|x| x.gr()));
    println!("Member backwards for {BL}199{UN}, is in sorted at: {}",
        sorted.member(199,false).map_or_else(||"None".rd(),|x| x.gr()));
    // println!("Memsearch_indexed for {BL}199{UN}, is in sorted at: {}",
    //    vm.memsearch_indexed(&vi,199).map_or_else(||"None".rd(),|x| x.gr()));

    let sorteddesc = vi.unindex(&vm, false);    
    vi.mutrevs();
    println!("\nThe above unindexed into descending order:\n{}", sorteddesc.mg()); 
    let (idx,count) = sorteddesc.binsearchdesc(&199);
    println!("Binsearchdesc for {BL}199{UN}: {GR}({},{}){UN}",idx,count); 
    println!("Binsearchdesc_indexed for {BL}{midval}{UN}, fits in descending before: {GR}{}{UN}",
        vm.binsearchdesc_indexed(&vi,midval)); // binsearch_indexed
    println!("Nearest smaller item from {BL}{midval}{UN} is: {GR}{}{UN}",
        vm[vi[vm.binsearchdesc_indexed(&vi,midval)]]);
    // println!("Memsearchdesc for {BL}161{UN}, found in descending at: {}",
    //    sorteddesc.memsearchdesc(161).map_or_else(||"None".rd(),|x| x.gr()));    
    // println!("Memsearchdesc_indexed for {BL}161{UN}, found in descending at: {}",
    //    vm.memsearchdesc_indexed(&vi,161).map_or_else(||"None".rd(),|x| x.gr()));  
 
    println!("Intersect_indexed:\n{}", vm.intersect_indexed(&vi, &v1, &v1.mergesort_indexed()).gr());
    println!("Diff_indexed:\n{}", vm.diff_indexed(&vi, &v1, &v1.mergesort_indexed()).gr());
    sorted.dedup();
    println!("Dedup:\n{}\n",sorted.gr());
}

#[test]
fn text() {
    let sentence = "Oh what a bunch of doodaas , doodaa , doodaa - daa";
    let mut v = sentence.split(' ').collect::<Vec<_>>();
    println!("{}",v.gr()); // Display
    v.muthashsort();
    println!("Ascending: {}",v.gr()); // Display 
    v.mutrevs(); 
    println!("Descending: {}",v.gr()); // Display     
} 
#[test]
fn printing() {
    set_seeds(123456789);
    let rn = Rnum::newu8();
    let v1 = rn.ranv(20).getvu8();
    println!("\n{}",v1.rd());
    println!("\n{}",v1.gr());
    println!("\n{}",v1.yl());
    println!("\n{}",v1.bl());
    println!("\n{}",v1.mg());
    println!("\n{}",v1.cy());
    println!("\n{}",v1.to_str());
    println!("\n{}\n",v1.to_plainstr()); // no brackets
    let mut f = std::fs::File::create("/dev/stdout")
        .unwrap_or_else(|e| 
            panic!("{} {} Failed to open stdout File. Works on Linux.",here!(),e));
    v1.wvec(&mut f)
        .unwrap_or_else(|e| panic!("{} {} failed to write",here!(),e));
    println!() // blank line to mark the end of the test
}

#[test]
fn sorts()
{ 
    const NAMES:[&str;6] = 
        [ "sortm","sorth","mergesort_indexed","hashsort_indexed","mutquicksort","muthashsort"];
    // Here we found it necessary to declare the data argument v as mutable in all closures,
    // even though only the last two require it.
    // The Rust compiler would throw a fit otherwise.
    let closures = [
        |v:&mut [u8]| { v.sortm(true); }, 
        |v:&mut [u8]| { v.sorth(true); }, 
        |v:&mut [u8]| { v.mergesort_indexed(); },
        |v:&mut [u8]| { v.hashsort_indexed(); },
        |v:&mut [u8]| { v.mutquicksort(); },
        |v:&mut [u8]| { v.muthashsort(); }
    ];

    set_seeds(7777777777_u64);   // intialise the random numbers generator
    let rn = Rnum::newu8(); // specifies the type of data items
    mutbenchu8(rn,4,10,&NAMES,&closures); 
}
