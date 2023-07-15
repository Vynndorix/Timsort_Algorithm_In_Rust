use rand::Rng;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use std::cmp::PartialOrd;

pub fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, (len - 1) as isize);
    }
}



fn main() {


    let mut map:HashMap<i128, Vec<i128>> = HashMap::new();
    let mut rng = rand::thread_rng();

    for i in 1..=10 {
        let mut my_vector: Vec<i128> = Vec::new();
        for _ in 0..20 {
            let random_number:i128 = rng.gen_range(-100000..=100000);
            my_vector.push(random_number);
        }

        println!("\n\t\tBEFORE (vector {})", i);

        thread::sleep(Duration::from_secs(2));

        for num in &my_vector {
            println!("{num}", num = num);
        }

        thread::sleep(Duration::from_secs(3));

        quick_sort(&mut my_vector);

        println!("\n\t\tAFTER (vector {})", i);

        thread::sleep(Duration::from_secs(2));

        for num in &my_vector {
            println!("{num}", num = num);
        }

        thread::sleep(Duration::from_secs(3));

        map.insert(i, my_vector);
    }

    println!("\n\nAccessing sorted vectors in the hashmap:");
    for (i, sorted_vector) in map.iter() {
        println!("vector {}: {:?}", i, sorted_vector);
    }
}



