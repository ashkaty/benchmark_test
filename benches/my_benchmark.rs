use criterion::{black_box, criterion_group, criterion_main, Criterion};


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
        println!("{}", i);
    }
}

fn iterate_array(array: &[i32]){
    for i in array {
        println!("{}", i);
    }
}

fn iterate_slice(slice: &[i32]){
    for i in slice {
        println!("{}", i);
    }
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

criterion_group!(benches, bench_insert, bench_iterate);
criterion_main!(benches);