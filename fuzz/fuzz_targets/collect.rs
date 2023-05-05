#![no_main]
use easy_parallel::Parallel;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|values: (Vec<i32>, i32)| {
    let res = Parallel::new()
        .each(0..values.0.len(), |i| values.0[i])
        .add(|| values.1)
        .collect::<Vec<i32>>();
    assert_eq!(res[values.0.len()], values.1);
});