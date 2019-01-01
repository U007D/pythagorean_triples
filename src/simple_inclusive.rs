pub fn pythagorean_triples(n: usize, triples: &mut Vec<(u32, u32, u32)>) {
    for z in 1_u32.. {
        for x in 1..=z {
            #[allow(clippy::integer_arithmetic)]
            for y in x..=z {
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
