use rand;
use rand::Rng;
use std::time::SystemTime;
use rand::prelude::ThreadRng;
use std::borrow::BorrowMut;

fn main() {
    let mut arr = [0; 10000];
    let mut arr2 = [0; 10000];
    let mut rand = rand::thread_rng();
    for i in 0..arr.len() {
        arr[i] = rand.gen();
        arr2[i] = rand.gen();
    }
    let size = arr.len();
    let start = SystemTime::now();
    quick_sort(arr.as_mut(), 0, size);
    println!("quick_sort sort time for 1000 elements = {:?}", start.elapsed().unwrap().as_micros());
    println!("arr = {:?}", arr.as_mut());

    let start2 = SystemTime::now();
    randomized_quicksort(arr2.as_mut(), 0, size, rand.borrow_mut());
    println!("randomized_quick_sort sort time for 1000 elements = {:?}", start2.elapsed().unwrap().as_micros());
    println!("arr = {:?}", arr2.as_mut());

}

fn randomized_partition(arr: &mut [i32], low: usize, high: usize, rng: &mut ThreadRng) -> usize {
    if low < high - 1 {
        let random = rng.gen_range(low, high);
        arr.swap(random,high - 1);
    }
    partition(arr, low, high)
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high - 1];
    let mut i = low;
    for j in low..high - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high - 1);
    i
}

fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let q = partition(arr, low, high) as usize;
        quick_sort(arr, low, q);
        quick_sort(arr, q + 1, high);
    }
}

fn randomized_quicksort(arr: &mut [i32], low: usize, high: usize, rng: &mut ThreadRng) {
    if low < high {
        let q = randomized_partition(arr, low, high, rng);
        randomized_quicksort(arr, low, q, rng);
        randomized_quicksort(arr, q + 1, high, rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn quick_sort_result() {
        let mut array = [10, 5, 3, 6, 4, 2, 8, 0, 11];
        let result = [0, 2, 3, 4, 5, 6, 8, 10, 11];
        let size = array.len();
        quick_sort(array.as_mut(), 0, size);
        assert_eq!(array, result);
    }

    #[test]
    fn randomized_quick_sort_result() {
        let mut array = [10, 5, 3, 6, 4, 2, 8, 0, 11, 1];
        let result = [0, 1, 2, 3, 4, 5, 6, 8, 10, 11];
        let size = array.len();
        let mut rand = rand::thread_rng();
        randomized_quicksort(array.as_mut(), 0, size, rand.borrow_mut());
        assert_eq!(array, result);
    }

}
