use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<usize>, Box<dyn Error>> {
    let s = fs::read_to_string(path.as_ref())?;
    s.lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}

fn memoized(cache: &mut BTreeMap<usize, usize>, mass: usize) -> usize {
    match cache.get(&mass) {
        Some(cached) => dbg!(*cached),
        None => {
            let mut cost = (mass / 3).saturating_sub(2);
            if cost > 0 {
                cost += memoized(cache, cost);
            }
            cache.insert(mass, cost);
            cost
        }
    }
}

fn fuel_cost(mass: usize) -> usize {
    mass / 3 - 2
}

fn part1(input: &[usize]) -> usize {
    input.into_iter().map(|el| fuel_cost(*el)).sum::<usize>()
}

fn part2(input: &[usize]) -> usize {
    let mut cache = BTreeMap::new();
    input
        .into_iter()
        .map(|el| memoized(&mut cache, *el))
        .sum::<usize>()
}

fn main() {
    let data = parse("./day01/input.txt").unwrap();
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_1() {
        assert_eq!(fuel_cost(12), 2);
        assert_eq!(fuel_cost(14), 2);
        assert_eq!(fuel_cost(1969), 654);
        assert_eq!(fuel_cost(100756), 33583);
    }

    #[test]
    fn part_2() {
        let mut cache = BTreeMap::new();
        assert_eq!(memoized(&mut cache, 14), 2);
        assert_eq!(memoized(&mut cache, 1969), 966);
        assert_eq!(memoized(&mut cache, 100756), 50346);
    }
}
