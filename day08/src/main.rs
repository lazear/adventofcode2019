fn parse(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_digit() {
                Some(ch as u8 - '0' as u8)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn part1(size: usize, data: &[u8]) -> Option<usize> {
    let images = data.chunks_exact(size).collect::<Vec<_>>();
    let min = images
        .iter()
        .map(|img| img.iter().filter(|&&d| d == 0).count())
        .min()?;
    images
        .iter()
        .filter(|img| img.iter().filter(|&&d| d == 0).count() == min)
        .map(|img| {
            img.iter().filter(|&&d| d == 1).count() * img.iter().filter(|&&d| d == 2).count()
        })
        .next()
}

fn part2(w: usize, h: usize, data: &[u8]) -> Vec<String> {
    let images = data.chunks_exact(w * h).collect::<Vec<_>>();
    let mut output = std::iter::repeat(0).take(w * h).collect::<Vec<_>>();
    for layer in images.iter().rev() {
        for (idx, pixel) in layer.iter().enumerate() {
            output[idx] = match pixel {
                0 => 0,
                1 => 1,
                _ => output[idx],
            }
        }
    }
    output
        .chunks_exact(w)
        .map(|ch| {
            ch.iter()
                .map(|ch| match ch {
                    0 => " ",
                    _ => "@",
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect()
}

fn main() {
    let input = parse(&std::fs::read_to_string("./day08/input.txt").unwrap());
    println!("Part 1: {:?}", part1(25 * 6, &input));
    println!("Part 2:");
    for line in part2(25, 6, &input) {
        println!("{}", line);
    }
}

#[test]
fn examples_part1() {
    assert_eq!(part1(6, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]), Some(1));
}

#[test]
fn examples_part2() {
    assert_eq!(
        part2(2, 2, &[0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0]),
        vec![" @".to_string(), "@ ".to_string()]
    );
}
