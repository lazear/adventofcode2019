fn to_digits(number: u32) -> [u8; 6] {
    let mut array = [0u8; 6];
    let digits = [100_000, 10_000, 1_000, 100, 10, 1];
    for (idx, div) in digits.iter().enumerate() {
        array[idx] = ((number / div) % 10) as u8;
    }
    array
}

fn dups(pass: &[u8; 6]) -> bool {
    let mut valid = false;
    let mut count = 0;
    let mut last_dup = 255;
    for &c in pass {
        if c == last_dup {
            count += 1;
        } else {
            last_dup = c;
            if count == 2 {
                valid = true
            }
            count = 1;
        }
    }
    valid || count == 2
}

fn validate(pass: &[u8; 6], part_2: bool) -> bool {
    let hi = pass.iter().fold((0, true), |acc, &digit| {
        if digit >= acc.0 {
            (digit, acc.1)
        } else {
            (acc.0, false)
        }
    });
    if !part_2 {
        pass.windows(2).any(|a| a[0] == a[1]) && hi.1
    } else {
        dups(pass) && hi.1
    }
}

fn part1(low: u32, hi: u32) -> usize {
    (low..hi)
        .map(|i| validate(&to_digits(i), false))
        .filter(|i| *i)
        .count()
}

fn part2(low: u32, hi: u32) -> usize {
    (low..hi)
        .map(|i| validate(&to_digits(i), true))
        .filter(|i| *i)
        .count()
}

fn main() {
    println!("Part 1: {}", part1(178416, 676461));
    println!("Part 2: {}", part2(178416, 676461));

    dbg!(dups(&[1, 2, 3, 4, 4, 4]));
    dbg!(dups(&[1, 1, 1, 1, 2, 2]));
}
