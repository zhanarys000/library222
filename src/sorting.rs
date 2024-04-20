pub fn quick_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let pivot_index = partition(array);
    quick_sort(&mut array[..pivot_index]);
    quick_sort(&mut array[pivot_index + 1..]);
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let pivot_index = array.len() / 2;
    array.swap(pivot_index, array.len() - 1);
    let mut i = 0;
    for j in 0..array.len() - 1 {
        if array[j] <= array[array.len() - 1] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, array.len() - 1);
    i
}

pub fn selection_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut min_index = i;
        for j in i + 1..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            array.swap(i, min_index);
        }
    }
}


pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j); 
            j -= 1;
        }
    }
}


pub fn merge_sort<T: Ord + Clone>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let mid = array.len() / 2;
    let mut left = array[..mid].to_vec();
    let mut right = array[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            array[k] = left[i].clone();
            i += 1;
        } else {
            array[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        array[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        array[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
