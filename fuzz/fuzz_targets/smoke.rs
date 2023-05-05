#![no_main]
use easy_parallel::Parallel;
use std::sync::Mutex;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|values: (Vec<i32>, i32, i32)| {
    let m = Mutex::new(0);
    let v: Vec<i32> = values.0.iter().map(|x| x >> 16).collect();

    Parallel::new()
        .add(|| *m.lock().unwrap() += values.1 >> 16)
        .add(|| *m.lock().unwrap() += values.2 >> 16)
        .each(v.iter(), |n| *m.lock().unwrap() += *n)
        .run();

    assert_eq!(m.into_inner().unwrap(), (values.1 >> 16) + (values.2 >> 16) + v.iter().sum::<i32>());
});
