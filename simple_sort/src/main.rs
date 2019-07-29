use std::time::SystemTime;

use rand::Rng;

fn main() {
    let mut arr_20 = [0; 20];
    let mut arr_160 = [0; 160];
    let mut arr_1000 = [0; 1000];
    let mut arr_10000 = [0; 10000];
    fill_arr(arr_20.as_mut());
    fill_arr(arr_160.as_mut());
    fill_arr(arr_1000.as_mut());
    fill_arr(arr_10000.as_mut());

    print_result_simple_sort(arr_20.as_mut());
    print_result_simple_sort(arr_160.as_mut());
    print_result_simple_sort(arr_1000.as_mut());
    print_result_simple_sort(arr_10000.as_mut());


    print_result_shell_sort(arr_20.as_mut(), 2);
    print_result_shell_sort(arr_160.as_mut(), 2);
    print_result_shell_sort(arr_1000.as_mut(), 2);
    print_result_shell_sort(arr_10000.as_mut(), 2);

    println!();
    print_result_shell_sort(arr_20.as_mut(), 3);
    print_result_shell_sort(arr_160.as_mut(), 3);
    print_result_shell_sort(arr_1000.as_mut(), 3);
    print_result_shell_sort(arr_10000.as_mut(), 3);

    println!();
    print_result_shell_sort(arr_20.as_mut(), 4);
    print_result_shell_sort(arr_160.as_mut(), 4);
    print_result_shell_sort(arr_1000.as_mut(), 4);
    print_result_shell_sort(arr_10000.as_mut(), 4);

    println!();
    print_result_shell_sort(arr_20.as_mut(), 5);
    print_result_shell_sort(arr_160.as_mut(), 5);
    print_result_shell_sort(arr_1000.as_mut(), 5);
    print_result_shell_sort(arr_10000.as_mut(), 5);

    println!();
    print_result_shell_sort(arr_20.as_mut(), 6);
    print_result_shell_sort(arr_160.as_mut(), 6);
    print_result_shell_sort(arr_1000.as_mut(), 6);
    print_result_shell_sort(arr_10000.as_mut(), 6);
}

fn print_result_simple_sort(arr: &mut [i32]) {
    let start = SystemTime::now();
    insert_sort(arr);
    println!(
        "Insert sort: array size = {}, total working time = {}",
        arr.len(),
        start.elapsed().unwrap().as_micros()
    );
}

fn print_result_shell_sort(arr: &mut [i32], corr: usize) {
    let start = SystemTime::now();
    shell_sort(arr, corr);
    println!(
        "Shell sort: array size = {}, total working time = {}, shift = {}",
        arr.len(),
        start.elapsed().unwrap().as_micros(),
        corr
    );
}

fn insert_sort(arr: &mut [i32]) -> &[i32] {
    let mut j;
    let mut x;
    for i in 0..arr.len() {
        x = arr[i];
        j = i;
        while j > 0 && arr[j - 1] > x {
            arr[j] = arr[j - 1];
            j = j - 1;
        }
        arr[j] = x;
    }
    arr
}

fn shell_sort(arr: &mut [i32], corr: usize) -> &[i32] {
    let mut val;
    let mut shift;
    let mut step = arr.len() / corr;
    while step >= 1 {
        for j in (0..arr.len()).step_by(step) {
            val = arr[j];
            shift = j;
            while shift > 0 && arr[shift - step] > val {
                arr[shift] = arr[shift - step];
                shift -= step;
            }
            arr[shift] = val;
        }
        step /= corr;
    }
    arr
}

fn fill_arr(arr: &mut [i32]) {
    let mut rng = rand::thread_rng();
    let size: i32 = arr.len() as i32;
    for i in 0..arr.len() {
        arr[i] = rng.gen_range(0, size);
    }
}
