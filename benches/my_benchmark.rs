use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn access_array(array: &[i32]) -> i32 {
    return array[43];
}

fn access_slice(slice: &[i32]) -> i32 {
    return slice[43];
}

fn access_vec(vec: &Vec<i32>) -> i32 {
    return vec[43];
}

fn iterate_vec(vec: &Vec<i32>){
    for i in vec {
        continue;
    }
}

fn iterate_array(array: &[i32]){
    for i in array {
        continue;
    }
}

fn iterate_slice(slice: &[i32]){
    for i in slice {
        continue;
    }
}



fn sort_array(array: &mut [i32]) {
    array.sort()
}

fn sort_slice(slice: &mut [i32]) {
    slice.sort();
}

fn sort_vec(vec: &mut Vec<i32>) {
    vec.sort();
}

fn bench_insert(c: &mut Criterion){
    let array = [1; 100];
    let array_slice = &array[..];
    let vec = vec![1; 100];
    let vec_slice = &vec[..];
    c.bench_function("Array Access", |b| b.iter(|| access_array(black_box(&array))));
    c.bench_function("Array Slice Access", |b| b.iter(|| access_slice(black_box(&array_slice))));
    c.bench_function("Vector Slice Access", |b| b.iter(|| access_slice(black_box(&vec_slice))));
    c.bench_function("Vector Access", |b| b.iter(|| access_vec(black_box(&vec))));

}

fn bench_iterate(c: &mut Criterion){
    let array = [1; 100];
    let array_slice = &array[..];
    let vec = vec![1; 100];
    let vec_slice = &vec[..];
    c.bench_function("Array Iterate", |b| b.iter(|| iterate_array(black_box(&array))));
    c.bench_function("Array Slice Iterate", |b| b.iter(|| iterate_slice(black_box(&array_slice))));
    c.bench_function("Vector Slice Iterate", |b| b.iter(|| iterate_slice(black_box(&vec_slice))));
    c.bench_function("Vector Iterate", |b| b.iter(|| iterate_vec(black_box(&vec))));
}

// fn bench_sort(c: &mut Criterion){
//     let array: [i32; 100];
//     let vec: Vec<i32>;
//     for i in 1..100 {
//         let mut rng = rand::thread_rng();
//         let num = rng.gen_range(0..100);
//         array[i] = num;
//         vec.push(num);
//     }
//     let array_slice = &mut array[..];
//     let vec_slice = &mut vec[..];
//     c.bench_function("Array Iterate", |b| b.iter(|| sort_array(black_box(&array))));
//     c.bench_function("Array Slice Iterate", |b| b.iter(|| sort_slice(black_box(&array_slice))));
//     c.bench_function("Vector Slice Iterate", |b| b.iter(|| sort_slice(black_box(&vec_slice))));
//     c.bench_function("Vector Iterate", |b| b.iter(|| sort_vec(black_box(&vec))));
// }

criterion_group!(benches, bench_insert, bench_iterate);
criterion_main!(benches);