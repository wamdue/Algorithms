fn main() {
    let mut array = [4, 1, 8, 10, 16, 20, 11, 5, 2];
    build_heap(array.as_mut());
    println!("{:?}", array);
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
    let left = get_left(i);
    let right = get_right(i);
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
        let temp = arr[i];
        arr[i] = arr[largest];
        arr[largest] = temp;
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
            let temp = arr[index];
            arr[index] = arr[largest];
            arr[largest] = temp;
            index = largest;
        } else { break; }
    }
}

fn sort(arr: &mut [i32]) {
    let mut temp;
    let mut size = arr.len();
    for i in (1..arr.len()).rev() {
        temp = arr[i];
        arr[i] = arr[0];
        arr[0] = temp;
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