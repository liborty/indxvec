#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{merge::*, random::*, Indices, Printing, GR, UN};

#[test]
fn indxvec() {
    let mut seed: u64 = 555;
    let v1 = ranvu8(20, &mut seed);
    println!("\nv1: {}", v1.gr());
    let v2 = ranvu8(20, &mut seed);
    println!("v2: {}", v2.gr());
    println!("Minmax:       {}", minmax(&v1));
    println!("minmaxt:      {GR}{:?}{UN}", minmaxt(&v1));
    let (lset,gset) = partition_indexed(&v1, 128);
    println!( "Partition indices around 128:\n{}\n{}", lset.gr(),gset.gr() );
    println!("Ranks to f64: {}", rank(&v1, true).gr());
    println!("Sorted:       {}", sortm(&v1, true).gr()); // sorted data but index lost
    println!("Sorted:       {}", sortidx(&v1).unindex(&v1, true).gr()); // same as sortm
    println!("Sorted:       {}", rank(&v1, false).invindex().unindex(&v1, false).gr() );
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
    println!("Sortix rev:   {}", sortidx(&v1).revindex().gr());
    println!("Sortix rev:   {}", rank(&v1, false).invindex().gr()); // descending sort index from desc ranks
    println!("Ranks to idx: {}", rank(&v1, true).invindex().gr()); // ascending sort index from ascending ranks
    println!("Ranks to idx: {}", rank(&v1, false).complindex().invindex().gr()); // from ascending ranks
    println!("Idx to ranks: {}", sortidx(&v1).invindex().gr());
    println!("Sorted rev:   {}", sortm(&v1, false).gr()); // descending sort, index lost
    println!("Sorted rev:   {}", revs(&sortm(&v1, true)).gr()); // the above simply reversed
    println!("Sorted rev:   {}", sortidx(&v1).unindex(&v1, false).gr()); // more efficient reversal
    println!("Sorted rev:   {}", sortidx(&v1).revindex().unindex(&v1, true).gr()); // by reversing the sort index
    println!("Sorted rev:   {}", sortidx(&v1).invindex().complindex().invindex().unindex(&v1, true).gr());
    println!("Sorted rev:   {}", rank(&v1, true).complindex().invindex().unindex(&v1, true).gr()); // complindex reverses ranks
    println!("Spearman corr v1,v2: {}",rank(&v1, true).ucorrelation(&rank(&v2, true)).gr()); //  1 for any Vec
    //println!("Spearman corr against reversed: {}",
    //    rank(&v1, true).ucorrelation(&rank(&v1, false)).gr()); // -1 for any Vec
    let (vm, vi) = merge_indexed(&v1, &sortidx(&v1), &v2, &sortidx(&v2)); // merge two vecs using their sort indices
    let sorted = vi.unindex(&vm, true);
    println!("v1 and v2 sorted, merged and unindexed:\n{}", sorted.gr());
    let sorteddesc = vi.unindex(&vm, false);
    println!("The above reversed:\n{}", sorteddesc.gr());
    println!("Binsearch for 15, found before: {}",binsearch(&sorted, 15).gr()); // binsearch
    println!("Binsearchdesc for 15, found before: {}",binsearchdesc(&sorteddesc, 15).gr()); // binsearch
    println!("Memsearchdesc for 14, found at: {}",
        memsearchdesc(&revs(&sorted), 14).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Memsearch_indexed for 105, found at: {}",
        memsearch_indexed(&vm, &vi, 105).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Memsearch_indexed for 105, found at: {}",
        memsearchdesc_indexed(&vm, &vi.revindex(), 105).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Occurrences count of 105: {}",occurs(&sorted, 105).gr());
    println!("Occurrences count of 105: {}",occurs_multiple(&sorted,&sorteddesc,105).gr());
    println!("Intersect_indexed: {}",intersect_indexed(&vm, &vi, &v1, &sortidx(&v1)).gr());
    println!("Diff_indexed: {}",diff_indexed(&vm, &vi, &v1, &sortidx(&v1)).gr());
    println!("Sansrepeat:   {}\n", sansrepeat(&sorted).gr());
}
