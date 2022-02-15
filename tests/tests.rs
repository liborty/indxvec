#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{merge::*, Indices, Printing, GR, RD, UN};
use ran::*;

#[test]
fn indxvec() {
    let min = 0.;
    let max = 255.;
    let midval:u8 = 128;
    set_seeds(9876543210);
    let v1 = ranvu8(19);
    let mut vm = v1.clone();
    println!("{GR}\nv1: {}", v1.red());  
    let v2 = ranvu8(19);
    println!("{GR}v2: {}{UN}", v2.to_str());    
    println!("minmax v1:       {}", minmax(&v1));
    println!("minmaxt v1:      {GR}{:?}{UN}", minmaxt(&v1)); 
    let (lset,gset) = partition_indexed(&v1, midval);
    println!( "v1 indices partitioned by data value {midval}:\n{}\n{}", lset.gr(),gset.gr() );
    println!("Sorted by merge sort:\n{}", sortm(&v1, true).gr()); // sorted data but index lost
    hashsort(&mut vm,0,255);
    println!("Sorted by hash sort:\n{}", vm.gr()); // new hashsort
    println!("Sorted via ranking:\n{}", rank(&v1, false).invindex().unindex(&v1, false).gr() );
    println!("Ranks:        {}", rank(&v1, true).gr()); // how to get ranks
    println!("Ranks:        {}", rank(&v1, true).complindex().complindex().gr() ); // symmetry
    println!("Ranks:        {}", sortidx(&v1).invindex().gr()); // simplest ranks from sortindex
    println!("Ranks rev:    {}", rank(&v1, true).revindex().gr()); // revindex() reverses any index
    println!("Ranks rev:    {}", sortidx(&v1).complindex().invindex().gr()); // via sortidx()  and complindex()
    println!("Ranks rev:    {}", sortidx(&v1).invindex().revindex().gr()); // via revindex()
    println!("Ranks desc:   {}", rank(&v1, false).gr()); // descending ranks, not the same as ranks reversed!!
    println!("Ranks desc:   {}", rank(&v1, true).complindex().gr()); // descending ranks, not the same as ranks reversed!!
    println!("Ranks desc:   {}", sortidx(&v1).invindex().complindex().gr()); // descending ranks, not the same as ranks reversed!!
    println!("Sort index:   {}", sortidx(&v1).gr()); // sortindex, can be unindexed at anytime
    println!("Sort index:   {}", hashsort_indexed(&v1,min,max).gr()); 
    println!("Sortix rev:   {}", sortidx(&v1).revindex().gr());
    println!("Sortix rev:   {}", rank(&v1, false).invindex().gr()); // descending sort index from desc ranks
    println!("Ranks to idx: {}", rank(&v1, true).invindex().gr()); // ascending sort index from ascending ranks
    println!("Ranks to idx: {}", rank(&v1, false).complindex().invindex().gr()); // from ascending ranks
    println!("Idx to ranks: {}", sortidx(&v1).invindex().gr());
    println!("Sortm naively reversed:\n{}", revs(&sortm(&v1, true)).gr()); // the above simply reversed
    println!("Sortm false:\n{}", sortm(&v1, false).gr()); // descending sort, index lost
    println!("Hashsort unindex false:\n{}", hashsort_indexed(&v1,min,max).unindex(&v1, false).gr()); // more efficient reversal
    println!("Sortidx unindex false:\n{}", sortidx(&v1).unindex(&v1, false).gr()); // more efficient reversal
    println!("Revindex:\n{}", sortidx(&v1).revindex().unindex(&v1, true).gr()); // by reversing the sort index
    println!("Invert-compliment-invert:\n{}", sortidx(&v1).invindex().complindex().invindex().unindex(&v1, true).gr());
    println!("Rank-compliment-invert:\n{}", rank(&v1, true).complindex().invindex().unindex(&v1, true).gr()); // complindex reverses ranks
    println!("Spearman corr v1,v2: {}",rank(&v1, true).ucorrelation(&rank(&v2, true)).gr()); //  1 for any Vec
    //println!("Spearman corr against reversed: {}",
    //    rank(&v1, true).ucorrelation(&rank(&v1, false)).gr()); // -1 for any Vec
    let (vm, vi) = merge_indexed(&v1, &hashsort_indexed(&v1,min,max),
        &v2, &hashsort_indexed(&v2,min,max)); // merge two vecs using their sort indices
    let sorted = vi.unindex(&vm, true);
    println!("v1 and v2 sorted, merged and unindexed:\n{}", sorted.gr());
    let sorteddesc = vi.unindex(&vm, false);
    println!("The above reversed:\n{}", sorteddesc.gr());
    println!("Binsearch for {midval}, found before: {}",binsearch(&sorted,midval).gr()); // binsearch
    println!("Binsearchdesc for {midval}, found before: {}",binsearchdesc(&sorteddesc,midval).gr()); // binsearch
    println!("Memsearchdesc for {midval}, found at: {}",
        memsearchdesc(&revs(&sorted),midval).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Memsearch_indexed for {midval}, found at: {}",
        memsearch_indexed(&vm, &vi,midval).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Memsearch_indexed (reversed index) for {midval}, found at: {}",
        memsearchdesc_indexed(&vm, &vi.revindex(),midval).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Occurrences count of {midval}: {}",occurs(&sorted, midval).gr());
    println!("Occurrences count of {midval}: {}",occurs_multiple(&sorted,&sorteddesc,midval).gr());
    println!("Intersect_indexed: {}",intersect_indexed(&vm, &vi, &v1, &sortidx(&v1)).gr());
    println!("Diff_indexed: {}",diff_indexed(&vm, &vi, &v1, &sortidx(&v1)).gr());
    println!("Sansrepeat:   {}\n", sansrepeat(&sorted).gr());
}
