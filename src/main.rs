use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::time;

// 初期値の設定
fn start(a: &mut Vec<isize>, n: &usize) {
    // 配列生成
    let mut rng = rand::thread_rng();
    for _i in 0..*n {
        let j: isize = rng.gen_range(0, 100000000);
        a.push(j);
    }
}

// 選択法
fn selection_sort(a: &mut Vec<isize>, n: &usize) {
    for i in 0..*n - 1 {
        let mut minj = i;
        for j in i..*n {
            if a[j] < a[minj] {
                minj = j;
            }
        }
        let t = a[i];
        a[i] = a[minj];
        a[minj] = t;
    }
}

// 挿入法
fn insertion_sort(a: &mut Vec<isize>, n: &usize) {
    for i in 1..*n {
        //未ソート部分の先頭
        let v = a[i];

        //ソート済みの部分の末尾
        let mut j = i as isize - 1;
        while j >= 0 && a[j as usize] > v {
            a[j as usize + 1] = a[j as usize];
            j -= 1;
        }

        a[(j + 1) as usize] = v
    }
}

const SENTINEL: isize = 2000000000;

// マージ
fn merge(a: &mut Vec<isize>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l = vec![0; 200000];
    let mut r = vec![0; 200000];

    for i in 0..n1 {
        l[i] = a[left + i];
    }
    for i in 0..n2 {
        r[i] = a[mid + i];
    }
    l[n1] = SENTINEL;
    r[n2] = SENTINEL;

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
    }
}

// マージソート
fn merge_sort(a: &mut Vec<isize>, left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(a, left, mid);
        merge_sort(a, mid, right);
        merge(a, left, mid, right);
    }
}

// パーティション
fn partition(a: &mut Vec<isize>, p: usize, r: usize) -> usize {
    // ピボットの決定
    let x = a[r];

    let mut i = p;
    for j in p..r {
        if a[j] <= x {
            let t = a[i];
            a[i] = a[j];
            a[j] = t;
            i += 1;
        }
    }
    let t = a[i];
    a[i] = a[r];
    a[r] = t;
    i
}

// クイックソート
fn quick_sort(a: &mut Vec<isize>, p: usize, r: usize) {
    if p < r {
        let q = partition(a, p, r);
        if q > 0 {
            quick_sort(a, p, q - 1);
        }
        quick_sort(a, q + 1, r);
    }
}

fn main() {
    // サイズの配列
    let size: Vec<usize> = vec![
        10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000,
    ];

    let mut file = File::create("output.txt").expect("Unable to create file");

    // 計測（選択法）
    let mut result: Vec<f64> = vec![];
    for i in 0..10 {
        let mut a: Vec<isize> = vec![];
        let s = size[i];
        start(&mut a, &s);
        let now = time::Instant::now();
        println!("\n実行前\n{:?}", &a[0..10]);
        selection_sort(&mut a, &s);
        // merge_sort(&mut a, 0, s);
        // quick_sort(&mut a, 0, s - 1);
        println!("実行後\n{:?}", &a[0..10]);
        result.push(now.elapsed().as_secs_f64());
    }
    // println!("{:?}", result);
    writeln!(file, "Selection Sort: {:?}", result).expect("Unable to write to file");

    //　計測（挿入ソート）
    let mut result: Vec<f64> = vec![];
    for i in 0..10 {
        let mut a: Vec<isize> = vec![];
        let s = size[i];
        start(&mut a, &s);
        let now = time::Instant::now();
        println!("\n実行前\n{:?}", &a[0..10]);
        // selection_sort(&mut a, &s);
        insertion_sort(&mut a, &s);
        // merge_sort(&mut a, 0, s);
        // quick_sort(&mut a, 0, s - 1);
        println!("実行後\n{:?}", &a[0..10]);
        result.push(now.elapsed().as_secs_f64());
    }
    // println!("{:?}", result);
    writeln!(file, "Insertion Sort: {:?}", result).expect("Unable to write to file");

    // 計測（マージソート）
    let mut result: Vec<f64> = vec![];
    for i in 0..10 {
        let mut a: Vec<isize> = vec![];
        let s = size[i];
        start(&mut a, &s);
        let now = time::Instant::now();
        println!("\n実行前\n{:?}", &a[0..10]);
        // selection_sort(&mut a, &s);
        // insertion_sort(&mut a, &s);
        merge_sort(&mut a, 0, s);
        // quick_sort(&mut a, 0, s - 1);
        println!("実行後\n{:?}", &a[0..10]);
        result.push(now.elapsed().as_secs_f64());
    }
    // println!("{:?}", result);
    writeln!(file, "Merge Sort: {:?}", result).expect("Unable to write to file");

    // 計測（クイックソート）
    let mut result: Vec<f64> = vec![];
    for i in 0..10 {
        let mut a: Vec<isize> = vec![];
        let s = size[i];
        start(&mut a, &s);
        let now = time::Instant::now();
        println!("\n実行前\n{:?}", &a[0..10]);
        // selection_sort(&mut a, &s);
        // insertion_sort(&mut a, &s);
        // merge_sort(&mut a, 0, s);
        quick_sort(&mut a, 0, s - 1);
        println!("実行後\n{:?}", &a[0..10]);
        result.push(now.elapsed().as_secs_f64());
    }
    // println!("{:?}", result);
    writeln!(file, "Quick Sort: {:?}", result).expect("Unable to write to file");
}
