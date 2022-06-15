#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutsort };
use ran::*;

#[test]
fn indxvec() {
    let min = 0.;
    let max = 255.;
    let midval:u8 = 128;
    set_seeds(987654321);
    let v1 = ranvu8(19);
    let mut vm = v1.clone();
    println!("{GR}\nv1: {}", v1.bl());  
    let v2 = ranvu8(19);
    println!("{GR}v2: {}", v2.bl());    
    println!("minmax v1:       {}", v1.minmax());
    println!("minmaxt v1:      {GR}{:?}{UN}", v1.minmaxt()); 
    let (lset,eqset,gset) = v1.partition_indexed(midval);
    println!( "v1 indices partitioned by data value {midval}:\n{}\n{}\n{}",
        lset.gr(),eqset.gr(),gset.gr() );
    println!("Sorted by merge sort:\n{}", v1.sortm(true).gr()); // sorted data but index lost
    vm.hashsort(0.,255.); // destructive (mutable) sort of wm
    println!("Sorted by hash sort:\n{}", vm.gr()); // hashsorted
    println!("Sorted via ranking:\n{}", v1.rank(false).invindex().unindex(&v1, false).gr() );
    println!("Ranks:        {}", v1.rank(true).gr()); // how to get ranks
    println!("Ranks:        {}", v1.rank(true).complindex().complindex().gr() ); // symmetry
    println!("Ranks:        {}", v1.sortidx().invindex().gr()); // simplest ranks from sortindex
    println!("Ranks rev:    {}", v1.rank(true).revindex().gr()); // revindex() reverses any index
    println!("Ranks rev:    {}", v1.sortidx().complindex().invindex().gr()); // via sortidx()  and complindex()
    println!("Ranks rev:    {}", v1.sortidx().invindex().revindex().gr()); // via revindex()
    println!("Ranks desc:   {}", v1.rank(false).gr()); // descending ranks, not the same as ranks reversed!!
    println!("Ranks desc:   {}", v1.rank(true).complindex().gr()); // descending ranks, not the same as ranks reversed!!
    println!("Ranks desc:   {}", v1.sortidx().invindex().complindex().gr()); // descending ranks, not the same as ranks reversed!!
    println!("Sort index:   {}", v1.sortidx().gr()); // sortindex, can be unindexed at anytime
    println!("Sort index:   {}", v1.hashsort_indexed(min,max).gr()); 
    // println!("Sortix compl: {}", hashsort_indexed(&v1,min,max).complindex().gr());    
    println!("Sortix rev:   {}", v1.sortidx().revindex().gr());
    println!("Sortix rev:   {}", v1.rank(false).invindex().gr()); // descending sort index from desc ranks
    println!("Sortix rev:   {}", v1.rank(true).complindex().invindex().gr()); // descending sort index from desc ranks    println!("Ranks to idx: {}", rank(&v1, true).invindex().gr()); // ascending sort index from ascending ranks
    println!("Ranks to idx: {}", v1.rank(false).complindex().invindex().gr()); // from ascending ranks
    println!("Idx to ranks: {}", v1.sortidx().invindex().gr());
    println!("Sortm naively reversed:\n{}",v1.sortm(true).revs().gr()); // the above simply reversed
    println!("Sortm false:\n{}", v1.sortm(false).gr()); // descending sort, index lost
    println!("Hashsort unindex false:\n{}", v1.hashsort_indexed(min,max).unindex(&v1,false).gr()); // more efficient reversal
    println!("Sortidx unindex false:\n{}", v1.sortidx().unindex(&v1, false).gr()); // more efficient reversal
    println!("Revindex:\n{}", v1.sortidx().revindex().unindex(&v1, true).gr()); // by reversing the sort index
    println!("Sortindx-invert-compliment-invert-unindex:\n{}", v1.sortidx().invindex().complindex().invindex().unindex(&v1, true).gr());
    println!("Rank-compliment-invert-unindex:\n{}", v1.rank(true).complindex().invindex().unindex(&v1, true).gr()); // complindex reverses ranks
    println!("Spearman corr v1,v2: {}", v1.rank(true).ucorrelation(&v2.rank(true)).gr()); //  1 for any Vec
    //println!("Spearman corr against reversed: {}",
    //    rank(&v1, true).ucorrelation(&rank(&v1, false)).gr()); // -1 for any Vec
    let (vm, vi) = v1.merge_indexed( &v1.hashsort_indexed(min,max), 
        &v2, &v2.hashsort_indexed(min,max)); // merge two vecs using their sort indices
    let sorted = vi.unindex(&vm, true);
    println!("v1 and v2 sorted, merged and unindexed:\n{}", sorted.gr());
    let sorteddesc = vi.unindex(&vm, false);
    println!("The above reversed:\n{}", sorteddesc.gr());
    println!("Binsearch for {BL}{midval}{UN}, found before: {GR}{}{UN}",sorted.binsearch(midval)); // binsearch
    println!("Binsearchdesc for {BL}{midval}{UN}, found before: {GR}{}{UN}",sorteddesc.binsearchdesc(midval)); // binsearch
    println!("Memsearch for {BL}{midval}{UN}, found at: {}",
        sorted.memsearch(midval).map_or_else(||"None".rd(),|x| x.gr()));
    println!("Memsearch_indexed for {BL}{midval}{UN}, found at: {}",
        vm.memsearch_indexed(&vi,midval).map_or_else(||"None".rd(),|x| x.gr()));
    println!("Memsearch_indexed (reversed index) for {BL}{midval}{UN}, found at: {}",
        vm.memsearchdesc_indexed(&vi.revindex(),midval).map_or_else(||"None".rd(),|x| x.gr()));
    println!("Occurrences count of {BL}{midval}{UN}: {GR}{}{UN}",sorted.occurs(midval));
    println!("Occurrences count of {BL}{midval}{UN}: {GR}{}{UN}",sorted.occurs_multiple(&sorteddesc,midval));
    println!("Intersect_indexed: {}", vm.intersect_indexed(&vi, &v1, &v1.sortidx()).gr());
    println!("Diff_indexed: {}", vm.diff_indexed(&vi, &v1, &v1.sortidx()).gr());
    println!("Sansrepeat:   {}\n", sorted.sansrepeat().gr());
}

#[test]
fn printing() {
    set_seeds(123456789);
    let v1 = ranvu8(20); 
    println!("\n{}",v1.rd());
    println!("\n{}",v1.gr());
    println!("\n{}",v1.yl());
    println!("\n{}",v1.bl());
    println!("\n{}",v1.mg());
    println!("\n{}",v1.cy());
    println!("\n{}",v1.to_str());
    println!("\n{}\n",v1.to_plainstr());
    let mut f = std::fs::File::create("/dev/stdout")
        .unwrap_or_else(|e| 
            panic!("{} {} Failed to open stdout File. Works on Linux.",here!(),e));
    v1.wvec(&mut f)
        .unwrap_or_else(|e| panic!("{} {} failed to write",here!(),e));
    println!()
}
