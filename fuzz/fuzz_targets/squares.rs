#![no_main]
use easy_parallel::Parallel;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|values: Vec<i32>| {
    let v: Vec<i32> = values.iter().map(|x| x >> 16).collect();
    let true_squares: Vec<i32> = v.iter().map(|x| x * x).collect();
    let squares = Parallel::new().each(0..v.len(), |i| v[i] * v[i]).run();
    assert_eq!(squares, true_squares);
});
