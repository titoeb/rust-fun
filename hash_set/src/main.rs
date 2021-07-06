use std::collections::HashSet;

fn main() {
    // let's create our first Hashset!
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");

    // Let's have a look.
    // Observe that there are no ordering guarantees!
    println!("{:?}", greeks);

    // We can add stuff twice without changing the hash set!
    greeks.insert("gamma");
    println!("{:?}", greeks);

    // But we can also observe if the insertion was
    // "sucessful"!
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("We added vega!");
    } else {
        println!("vega was not added, already in the HashSet.")
    }
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("We added vega!");
    } else {
        println!("vega was not added, already in the HashSet.");
    }

    // We can also remove entries from our HashSet!
    let removed = greeks.remove("delta");
    if removed {
        println!("'delta' could be remove from HashSet.");
    }

    // We can also use typical set-operations like comparison, subsets, supersets ect.
    let _1_5: HashSet<i32> = (1..=5).collect();
    let _6_10: HashSet<i32> = (6..=10).collect();
    let _1_10: HashSet<i32> = (1..=10).collect();
    let _2_8: HashSet<i32> = (2..=8).collect();

    // Test for supersets
    println!(
        "is {:?} a subset of {:?}? => {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // Test for disjoint sets.
    println!(
        "is {:?} a disjoint of {:?}? => {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // Get all entries from both sets.
    println!(
        "The union of {:?} and {:?} is  {:?}",
        _1_5,
        _6_10,
        _1_5.union(&_6_10)
    );
    // Only get the elemetns both in a and b
    println!(
        "The elemnts in {:?} and {:?} are {:?}",
        _2_8,
        _1_10,
        _2_8.intersection(&_1_10)
    );
}
