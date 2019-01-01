use super::*;
use test::Bencher;

const N_TEST_RUNS: usize = 1000;

#[bench]
fn range_inclusive(bench: &mut Bencher) {
    let mut buf = Vec::<(u32, u32, u32)>::with_capacity(N_TEST_RUNS);
    bench.iter(|| range_inclusive::pythagorean_triples(N_TEST_RUNS, &mut buf));
}

#[bench]
fn range_exclusive(bench: &mut Bencher) {
    let mut buf = Vec::<(u32, u32, u32)>::with_capacity(N_TEST_RUNS);
    bench.iter(|| range_exclusive::pythagorean_triples(N_TEST_RUNS, &mut buf));
}

#[bench]
fn simple_inclusive(bench: &mut Bencher) {
    let mut buf = Vec::<(u32, u32, u32)>::with_capacity(N_TEST_RUNS);
    bench.iter(|| simple_inclusive::pythagorean_triples(N_TEST_RUNS, &mut buf));
}

#[bench]
fn simple_exclusive(bench: &mut Bencher) {
    let mut buf = Vec::<(u32, u32, u32)>::with_capacity(N_TEST_RUNS);
    bench.iter(|| simple_exclusive::pythagorean_triples(N_TEST_RUNS, &mut buf));
}
