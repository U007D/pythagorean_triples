pub fn pythagorean_triples(n: usize, triples: &mut Vec<(u32, u32, u32)>) {
    for z in 1_u32.. {
        #[allow(clippy::integer_arithmetic, clippy::range_plus_one)]
        for x in 1..z + 1 {
            for y in x..z + 1 {
                if x * x + y * y == z * z {
                    triples.push((x, y, z));
                    if triples.len() >= n {
                        return;
                    }
                }
            }
        }
    }
    unreachable!();
}
