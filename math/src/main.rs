

fn main() {
    println!("evklid minus = {}", evklid_munis(1234567890, 12));
    println!("evklid divide = {}", evklid_divide(1234567890, 12));
    println!("pow dummy = {}", pow_dummy(2, 32));
    println!("pow multiply = {}", pow_multiply(2, 16));
    println!("pow multiply binary = {}", pow_multiply_binary(2, 16));
    println!("simple numbers size = {}", simple_numbers(100).len());
    println!("simple numbers ext size = {:?}", simple_numbers_ext(120));
    println!("eratospene simple = {:?}", eratosphene_simple(120));
    println!("fibonachi recursion = {}", fibonachi_recursion(10));
    println!("fibonachi loop = {}", fibonachi_loop(10));
    println!("fibonachi golden = {}", fibonachi_golden(10));
}

fn evklid_munis(a: u32, b: u32) -> u32 {
    let mut ax = a;
    let mut bx = b;
    while ax != bx {
        if ax > bx {
            ax = ax - bx;
        } else {
            bx = bx - ax
        }
    }
    ax
}

fn evklid_divide(a: u32, b: u32) -> u32 {
    let mut ax = a;
    let mut bx = b;

    while ax != 0 && bx != 0 {
        if ax > bx {
            ax = ax % bx;
        } else {
            bx = bx % ax;
        }
    }
    ax
}

fn pow_dummy(a: u32, b: u32) -> u64 {
    let mut ax = a as u64;
    let mul = a as u64;
    for i in 1..b {
        ax = ax * mul;
    }
    ax
}

fn pow_multiply(base: u32, pow: u32) -> u64 {
    let mut temp: u64 = 2;
    let mul: u64 = pow as u64;
    let mut result = base as u64;
    let mut middle: u64;
    let mut data: Vec<u64> = vec![base as u64];
    if temp == mul { return result * result; }
    result *= result;
    while temp != mul {
        if temp < mul {
            result = result * result;
            data.push(result);
            temp *= 2;
        } else {
            temp /= 2;
            middle = (mul - temp) / 2;
            result = result * data.get(middle as usize).unwrap().clone();
            if middle == 0 {
                temp += 1;
            } else {
                temp += (middle * 2);
            }
            if temp != mul { temp *= 2; }
        }
    }
    result
}

fn pow_multiply_binary(base: u64, pow: u32) -> u64 {
    let mut abase = base;
    let mut apow = pow;
    let mut result: u64 = 1;
    while apow > 1 {
        if apow % 2 == 1 {
            result *= abase;
        }
        abase *= abase;
        apow /= 2;
    }
    if apow > 0 { result *= abase }
    result
}

fn simple_numbers(limit: u32) -> Vec<u32> {
    let mut data = vec![1];
    if limit == 1 { return data; }
    let mut counter: u32;
    for i in 2..limit + 1 {
        counter = 2;
        loop {
            if i == counter {
                data.push(i);
                break;
            } else if i % counter == 0 && i != counter {
                break;
            }
            counter += 1;
        }
    }
    data
}

fn simple_numbers_ext(limit: u32) -> Vec<u32> {
    let mut data = vec![1];
    if limit == 1 { return data; }
    let mut counter: u32 = 0;
    for a in 2..limit {
        if a == 2 || a == 3 || a == 5 {
            data.push(a);
            counter = a;
        } else {
            counter = 7;
            loop {
                if a % 2 == 0 || a % 3 == 0 || a % 5 == 0 || a < counter {
                    break;
                }
                if a % counter == 0 && a == counter {
                    data.push(a);
                    break;
                } else if a % counter == 0 {
                    break;
                }
                counter += 1;
            }
        }
    }
    data
}

fn eratosphene_simple(limit: u32) -> Vec<u32> {
    let mut list: Vec<bool> = Vec::new();
    let border = (limit + 1) as usize;
    let mut result = Vec::new();

    for i in 0..border {
        list.push(true);
    }

    for i in 2..border {
        if i * i <= border - 1 {
            for j in (i * i..border).step_by(i) {
                if list[j] != false && j % i == 0 {
                    list[j] = false;
                }
            }
        }
    }
    for (i, v) in list.into_iter().enumerate() {
        if v == true && i > 1 {
            result.push(i as u32);
        }
    }
    result
}

fn fibonachi_recursion(limit: u32) -> u32 {
    if limit < 2 {
        return 1;
    }
    fibonachi_recursion(limit - 1) + fibonachi_recursion(limit - 2)
}

fn fibonachi_loop(limit: u32) -> u128 {
    let mut result = 1;
    if limit > 2 {
        let mut prev = 1;
        let mut current = 1;
        for i in 2..limit + 1 {
            result = prev + current;
            prev = current;
            current = result;
        }
    }
    result
}

fn fibonachi_golden(limit: u32) -> u128 {
    let constant = (5 as f64).sqrt();
    let fi = (1 as f64 + constant) / 2 as f64;
    let step = fi.powi(limit as i32);
    let result = step / constant;
    return result as u128;
}
