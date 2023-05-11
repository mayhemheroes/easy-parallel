#![no_main]
use easy_parallel::Parallel;
use std::sync::Mutex;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|values: (Vec<i32>, i32, i32)| {
    let m = Mutex::new(0);
    let v: Vec<i32> = values.0.iter().map(|x| x >> 8).collect();

    Parallel::new()
        .add(|| *m.lock().unwrap() += values.1 >> 8)
        .add(|| *m.lock().unwrap() += values.2 >> 8)
        .each(v.iter(), |n| *m.lock().unwrap() += *n)
        .run();

    assert_eq!(m.into_inner().unwrap(), (values.1 >> 8) + (values.2 >> 8) + v.iter().sum::<i32>());
});
