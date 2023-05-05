#![no_main]
use easy_parallel::Parallel;
use libfuzzer_sys::fuzz_target;

fn par_sum(v: &[i32]) -> i32 {
    const THRESHOLD: usize = 2;

    if v.len() <= THRESHOLD {
        v.iter().copied().sum()
    } else {
        let half = (v.len() + 1) / 2;
        let sums = Parallel::new().each(v.chunks(half), par_sum).run();
        sums.into_iter().sum()
    }
}

fuzz_target!(|values: Vec<i32>| {
    let v: Vec<i32> = values.iter().map(|x| x >> 16).collect();
    assert_eq!(par_sum(&v), v.iter().copied().sum());
});
