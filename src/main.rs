mod sorting;

use sorting::{quick_sort, selection_sort,insertion_sort, merge_sort};

fn main() {
    let mut numbers = vec![9, 7, 5, 3, 0, 1, 4, 2, 8];
    quick_sort(&mut numbers);
    println!("Numbers Quick Sorted: {:?}", numbers);

    let mut words = vec!["mustang", "camaro", "corvette", "challenger", "civic"];
    selection_sort(&mut words);
    println!("Words Selection Sorted: {:?}", words);

    let mut floats = vec![1.5, 2.5, 3.5, 4.5, 6.5];
    insertion_sort(&mut floats);
    println!("Insertion Sorted: {:?}", floats);

    let mut characters = vec!['z', 'y', 'x', 'w', 'v'];
    merge_sort(&mut characters);
    println!("Characters Merge Sorted: {:?}", characters);
}
