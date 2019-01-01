pub fn pythagorean_triples(n: usize, buf: &mut Vec<(u32, u32, u32)>) {
    #[allow(clippy::integer_arithmetic, clippy::range_plus_one)]
    (1..n).flat_map(|z| {
                (1..z + 1).flat_map(move |y| {
                        #[allow(clippy::integer_arithmetic)]
                    (1..z + 1).filter_map(move |x| {
                        #[allow(clippy::cast_possible_truncation)]
                            match x * x + y * y == z * z {
                                true => Some((x as u32, y as u32 , z as u32)),
                                false => None,
                        }})})})
           .take(n)
           .for_each(|triple| buf.push(triple));
}
