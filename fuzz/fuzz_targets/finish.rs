#![no_main]
use easy_parallel::Parallel;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|values: Vec<String>| {
    let true_lens: Vec<usize> = values.iter().map(|x| x.len()).collect();
    let (lens, len) = Parallel::new().each(0..values.len(), |i| values[i].len()).finish(|| values.len());
    assert_eq!(lens, true_lens);
    assert_eq!(len, values.len());
});
