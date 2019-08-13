fn main() {
    let mut array = [4, 1, 8, 10, 16, 20, 11, 5, 2];
    sort(array.as_mut());
    println!("{:?}", array);
}

fn build_heap(arr: &mut [i32]) {
    for i in (0..(arr.len() / 2) + 1).rev() {
        heapify(arr, i, arr.len());
//        heapify_iter(arr, i, arr.len());
    }
}

// построение узла рекурсией
fn heapify(arr: &mut [i32], i: usize, heap_size: usize) {
    let left = if i == 0 { get_left(i) + 1 } else { get_left(i) };
    let right = if i == 0 { get_right(i) + 1 } else { get_right(i) };
    let mut largest;
    let x = arr[i];
    if left < heap_size && arr[left] > x {
        largest = left;
    } else {
        largest = i;
    }

    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    if i != largest {
        arr.swap(i, largest);
        heapify(arr, largest, heap_size);
    }
}

// построение узла без рекурсии
fn heapify_iter(arr: &mut [i32], i: usize, heap_size: usize) {
    let mut left;
    let mut right;
    let mut index = i;
    let mut largest;
    loop {
        left = get_left(index);
        right = get_right(index);
        let x = arr[index];
        if left < heap_size && arr[left] > x {
            largest = left;
        } else {
            largest = index;
        }

        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if index != largest {
            arr.swap(index, largest);
            index = largest;
        } else { break; }
    }
}

fn sort(arr: &mut [i32]) {
    let mut size = arr.len();
    build_heap(arr);
    for i in (1..arr.len()).rev() {
        println!("Iter = {}, arr = {:?}", i, arr);
        arr.swap(i, 0);
        size -= 1;
        heapify(arr, 0, size);
//        heapify_iter(arr, 0, size);
    }
}

fn get_left(index: usize) -> usize {
    index * 2
}

fn get_right(index: usize) -> usize {
    index * 2 + 1
}

#[cfg(tests)]

mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let mut arr = vec![50, 25, 1, 5, 3, 7, 15];
        sort(arr.as_mut());
        assert_eq!(vec![1, 3, 5, 7, 15, 25, 50], arr.as_mut());
    }
}