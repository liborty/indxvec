#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use core::cmp::Ordering::*;
use indxvec::{here, compare, printing::*, Binarysearch, Search, Indices, Mutops, Printing, Vecops};
use ran::*;
use std::{cmp::Ord, convert::From};
use times::*;

#[test]
fn indices() {
    let midval: u8 = 128;
    set_seeds(98777777);
    let rn = Rnum::newu8();
    let v1 = rn.ranv(20).unwrap().getvu8().unwrap();
    println!("{GR}\nv1: {}", v1.bl());
    let v2 = rn.ranv(20).unwrap().getvu8().unwrap();
    println!("{GR}v2: {}", v2.bl());
    println!("minmax v1: {}", v1.minmax());
    println!("minmaxt v1: {GR}{:?}{UN}", v1.minmaxt());
    println!("minmaxt v2: {GR}{:?}{UN}", v2.minmaxt());
    let (lset, eqset, gset) = v1.partition(midval);
    println!(
        "v1 partitioned by data value {midval}:\n{}\n{}\n{}",
        lset.gr(),
        eqset.gr(),
        gset.gr()
    );
    println!("Sorted by merge sort:\n{}", v1.sortm(true).gr()); // sorted data but index lost
    let mut vm = v1.clone();
    vm.muthashsort(&mut |&t| t as f64); // destructive (mutable) sort of vm
    println!("Sorted by muthashsort:\n{}", vm.gr()); // hashsorted
    let v1ranks = v1.rank(true); // ascending ranks
    let v1ranksd = v1.rank(false); // descending ranks
    println!(
        "Sorted via ranking:\n{}",
        v1ranks.invindex().unindex(&v1, true).gr()
    );
    println!("Ranks:        {}", v1ranks.gr()); // how to get ranks
    println!("Ranks:        {}", v1ranks.complindex().complindex().gr()); // symmetry
    println!("Ranks:        {}", v1.hashsort_indexed(&mut |&t| t as f64).invindex().gr()); // simplest ranks from sortindex
    println!("Ranks rev:    {}", v1ranks.revs().gr()); // revindex() reverses any index
    println!(
        "Ranks rev:    {}",
        v1.mergesort_indexed().complindex().invindex().gr()
    ); // via mergesort_indexed()  and complindex()
    println!(
        "Ranks rev:    {}",
        v1.hashsort_indexed(&mut |&t| t as f64).invindex().revs().gr()
    ); // via revindex()
    println!("Ranks desc:   {}", v1.rank(false).gr()); // descending ranks are not the same as ranks reversed!!
    println!("Ranks desc:   {}", v1ranks.complindex().gr()); // to make ranks descending, use complindex() instead
    println!(
        "Ranks desc:   {}",
        v1.hashsort_indexed(&mut |&t| t as f64).invindex().complindex().gr()
    ); // descending ranks from sortindex
    println!(
        "Ranks desc:   {}",
        v1.hashsort_indexed(&mut |&t| t as f64).revs().invindex().gr()
    ); // descending ranks from descending sort
    println!("Mergeort idx: {}", v1.mergesort_indexed().gr()); // can be unindexed at anytime
    println!("Hashsort idx: {}", v1.hashsort_indexed(&mut |&t| t as f64).gr());
    println!("Sortix rev:   {}", v1.mergesort_indexed().revs().gr());
    println!("Sortix rev:   {}", v1ranksd.invindex().gr()); // descending sort index from desc ranks
    println!("Sortix rev:   {}", v1ranks.complindex().invindex().gr()); // descending sort index from desc ranks
    println!("Ranks to idx: {}", v1ranks.invindex().gr()); // ascending sort index from ascending ranks
    println!("Ranks to idx: {}", v1ranksd.complindex().invindex().gr()); // sort index from ascending ranks
    println!("Idx to ranks: {}", v1.mergesort_indexed().invindex().gr());
    println!("Sortm naively reversed:\n{}", v1.sortm(true).revs().gr()); // the above simply reversed
    println!("Sortm false:\n{}", v1.sortm(false).gr()); // descending sort, index lost
    println!("Sorth false:\n{}", v1.sorth(&mut |&t| t as f64,false).gr());
    println!(
        "mergesort_indexed unindex false:\n{}",
        v1.mergesort_indexed().unindex(&v1, false).gr()
    ); // more efficient reversal
    println!(
        "hashsort_indexed unindex false:\n{}",
        v1.hashsort_indexed(&mut |&t| t as f64).unindex(&v1, false).gr()
    ); // more efficient reversal
    println!(
        "Revindex:\n{}",
        v1.mergesort_indexed().revs().unindex(&v1, true).gr()
    ); // by reversing the sort index
    println!(
        "Index-invert-compliment-invert-unindex:\n{}",
        v1.mergesort_indexed()
            .invindex()
            .complindex()
            .invindex()
            .unindex(&v1, true)
            .gr()
    );
    println!(
        "Rank-compliment-invert-unindex:\n{}",
        v1.rank(true)
            .complindex()
            .invindex()
            .unindex(&v1, true)
            .gr()
    ); // complindex reverses ranks
    println!(
        "Spearman corr v1,v2: {}",
        v1ranks.ucorrelation(&v2.rank(true)).gr()
    ); //  1 for any Vec
    println!(
        "Spearm. corr self 1: {}",
        v1ranks.ucorrelation(&v1ranks).gr()
    ); // 1 for any Vec
}

#[test]
fn vecops() {
    let midval: u8 = 128;
    let rn = Rnum::newu8();
    let v1 = rn.ranv(20).unwrap().getvu8().unwrap();    
    println!("{GR}\nv1: {}", v1.bl());
    let v2 = rn.ranv(20).unwrap().getvu8().unwrap();
    println!("{GR}v2: {}", v2.bl());
    let (vm, mut vi) = v1.merge_indexed(
        // merge two vecs using their sort indices
        &v1.hashsort_indexed(&mut |&t| t as f64),
        &v2,
        &v2.hashsort_indexed(&mut |&t| t as f64),
    );
    println!("\nv1 and v2 appended:\n{}", vm.gr());
    println!("20 smallest items:\n{BL}{:?}{UN}", vm.as_slice().smallest_k(20));
    println!("21 smallest items:\n{BL} {:?}{UN}",vm.as_slice().smallest_k(21));
    println!(
        "Number of occurrences of {BL}89{UN}: {GR}{}{UN}",
        vm.occurs(89)
    );
    println!(
        "Number of occurrences of {BL}128{UN}: {GR}{}{UN}",
        vm.occurs(128)
    );
    println!(
        "Number of occurrences of {BL}199{UN}: {GR}{}{UN}",
        vm.revs().occurs(199)
    );

    let mut sorted = vi.unindex(&vm, true);
    println!("v1 and v2 sorted, merged and unindexed:\n{}", sorted.mg());
    println!(
        "Binary_search for {BL}199{UN}: {GR}{:?}{UN}",
        (0..=sorted.len()-1).binary_all(&mut |&probe| sorted[probe].cmp(&199),true));

    println!(
        "Binsearch_indexed for {BL}{midval}{UN}: {GR}{:?}{UN}",
        vm.binsearch_indexed(&vi,&midval)
    ); // binsearch_indexed, ascending
    
    println!(
        "Nearest equal or greater item from {BL}{midval}{UN} is: {GR}{:?}{UN}",
        vm[vi[vm.binsearch_indexed(&vi,&midval).start]]
    );

    println!(
        "Forwards member index of {BL}199{UN}, is in 'sorted' at: {}",
        sorted
            .member(199, true)
            .map_or_else(|| "None".rd(), |x| x.gr())
    );
    println!(
        "Backwards member index for {BL}199{UN}, is in 'sorted' at: {}",
        sorted
            .member(199, false)
            .map_or_else(|| "None".rd(), |x| x.gr())
    );

    let sorteddesc = vi.unindex(&vm, false);
    vi.mutrevs();

    println!(
        "\nThe above unindexed into descending order:\n{}",
        sorteddesc.mg()
    );
    println!(
        "Binsearch for {BL}199{UN} (two methods): {GR}{:?}{UN} = {GR}{:?}{UN}",
        (0..=sorteddesc.len()-1).find_all(&mut |&probe| sorteddesc[probe],199),
        sorteddesc.binsearch(&199)); 
    println!(
        "Binsearchdesc_indexed for {BL}{midval}{UN}: {GR}{:?}{UN} = {GR}{:?}{UN}",
        (0..=sorteddesc.len()-1).find_all(&mut |&probe| vm[vi[probe]],midval),
        vm.binsearch_indexed(&vi, &midval)
    ); // binsearch_indexed, descending
    println!(
        "Nearest equal or smaller item from {BL}{midval}{UN} is: {GR}{}{UN}",
        vm[vi[vm.binsearch_indexed(&vi, &midval).start]]
    );
    println!(
        "Intersect_indexed:\n{}",
        vm.intersect_indexed(&vi, &v1, &v1.mergesort_indexed()).gr()
    );
    println!(
        "Diff_indexed:\n{}",
        vm.diff_indexed(&vi, &v1, &v1.mergesort_indexed()).gr()
    );
    println!("Sansrepeat:\n{}\n", sorted.sansrepeat().gr());
    sorted.dedup();
    println!("Dedup:\n{}\n", sorted.gr());
}

#[test]
fn text() {
    let sentence = "Humpty Dumpty sat on a wall, \
        Humpty Dumpty had a great fall, \
        and all the king's horses and all the king's men \
        could not put Humpty together again";
    let v = sentence.split(' ').collect::<Vec<_>>();
    println!("{}", v.gr()); // Display
                            // using cloning mergesort with implied alphabetic partial order
    let mut sorted = v.sorth(&mut |&s| s.len() as f64, true);
    println!("Ascending sorted by word length:\n{}", sorted.gr());
    // Binary search 
    println!(
        "Binary_search for {BL}word length 8{UN}: {YL}{:?}{UN}",
        (0..=sorted.len()-1).find_all(&mut |&probe| sorted[probe].len(),8)
    );
    sorted = v.sortm(true);
    println!("Ascending sorted by lexicon:\n{}", sorted.gr());
    println!(
        "Binary_search for {BL}Humpty{UN}: {YL}{:?}{UN}",
        (0..=sorted.len()-1).find_all(&mut |&probe| sorted[probe],"Humpty")
    );
    println!(
        "Binary_search for {BL}'Humpty'{UN} in range 5..end: {YL}{:?}{UN}",
        (5..=sorted.len()-1).find_all(&mut |&probe| sorted[probe],"Humpty")
    );
    println!(
        "Binary_search for {BL}'the'{UN}: {YL}{:?}{UN}",
        (0..=sorted.len()-1).find_all(&mut |&probe| sorted[probe],"the")
    );
    println!(
        "Binary_search for {BL}'the'{UN} in range 0..=23: {YL}{:?}{UN}",
        (0..=23).find_all(&mut |&probe| sorted[probe],"the")
    );
    println!(
        "Binary_search for {BL}'queen's'{UN}: {YL}{:?}{UN}",
        (0..=sorted.len()-1).find_all(&mut |&probe| sorted[probe],"queen's")
    );
    sorted.dedup();
    println!("Ascending deduplicated:\n{}\n", sorted.gr());

    let mut dsorted = v.sortm(false);
    println!("Descending sorted:\n{}", dsorted.gr());
    println!(
        "Binary_search for {BL}'Humpty'{UN}: {YL}{:?}{UN}",
        (0..=dsorted.len()-1).find_all(&mut |&probe| dsorted[probe],"Humpty") 
    );
    println!(
        "Binary_search for {BL}'Humpty'{UN} in range 0..=21: {YL}{:?}{UN}",
        (0..=21).find_all(&mut |&probe| dsorted[probe],"Humpty")
    );
    println!(
        "Binary_search for {BL}'the'{UN}: {YL}{:?}{UN}",
        (0..=dsorted.len()-1).find_all(&mut |&probe| dsorted[probe],"the") 
    );
    println!(
        "Binary_search for {BL}'the'{UN} in range 5..end: {YL}{:?}{UN}",
        (5..=sorted.len()-1).find_all(&mut |&probe| dsorted[probe],"the")
    );
    println!(
        "Binary_search for {BL}'queen's'{UN}: {YL}{:?}{UN}",
        (0..=dsorted.len()-1).find_all(&mut |&probe| dsorted[probe],"queen's") 
    );
    dsorted.dedup();
    println!("Descending deduplicated:\n{}\n", dsorted.gr());
}

use core::ops::Range;

#[test]
fn solvetest() {
    let num: f64 = 1234567890.0;
    let root: f64 = 5.3;
    let (res,rng) = (1_f64..=num).find_any(&mut |&x| x.powf(root),num);
    println!(
        "{} to the power of {YL}1/({}){UN}\nsolve:\t{} error: {RD}{:e}{UN}\npowf:\t{} error:{RD}{:e}{UN}\n",
        num.yl(),
        root,
        res.gr(), 
        rng.end-rng.start,    
        num.powf(1. / root).gr(),
        (num - num.powf(1. / root).powf(root))
    );
    let (quarterpi,rng) = (0.5..=1_f64).find_any(&mut |&x| x.tan(),1_f64);
    println!("pi:\t{GR}{}{UN} err:  {RD}{:e}{UN}", 4.0*quarterpi, rng.end-rng.start);
}

#[test]
fn printing() {
    println!("\n{}", &("pair","tuple").gr());
    println!("\n{}", &(1,2,3).mg()); 
    println!("\n{}", &("plain","triplet","tuple").to_plainstr().yl());  
    set_seeds(123456789);
    let rn = Rnum::newu8();
    let v1 = rn.ranv(20).unwrap().getvu8().unwrap();
    println!("\n{}", v1.rd());
    println!("\n{}", v1.gr());
    println!("\n{}", v1.yl());
    println!("\n{}", v1.bl());
    println!("\n{}", v1.mg());
    println!("\n{}", v1.cy());
    println!("\n{}", v1.to_str());
    println!("\n{}\n", v1.to_plainstr()); // no brackets
    let mut f = std::fs::File::create("/dev/stdout").unwrap_or_else(|e| {
        panic!(
            "{} {} Failed to open stdout File. Works on Linux.",
            here!(),
            e
        )
    });
    v1.wvec(&mut f)
        .unwrap_or_else(|e| panic!("{} {} failed to write", here!(), e));
    println!() // blank line to mark the end of the test
}

#[test]
fn sorts() {
    const NAMES: [&str; 6] = [
        "sortm",
        "sorth",
        "mergesort_indexed",
        "hashsort_indexed",
        "mutquicksort",
        "muthashsort",
    ];
    // Here we found it necessary to declare the data argument v as mutable in all closures,
    // even though only the last two require it.
    // The Rust compiler would throw a fit otherwise.
    let closures = [
        |v: &mut [u8]| {
            v.sortm(true);
        },
        |v: &mut [u8]| {
            v.sorth(&mut |&t| t as f64, true);
        },
        |v: &mut [u8]| {
            v.mergesort_indexed();
        },
        |v: &mut [u8]| {
            v.hashsort_indexed(&mut |&t| t as f64);
        },
        |v: &mut [u8]| {
            v.mutquicksort();
        },
        |v: &mut [u8]| {
            v.muthashsort(&mut |&t| t as f64);
        },
    ];

    set_seeds(7777777777_u64); // intialise the random numbers generator
    let rn = Rnum::newu8(); // specifies the type of data items
    mutbenchu8(rn, 10..3000, 1000, 10, &NAMES, &closures);
}
