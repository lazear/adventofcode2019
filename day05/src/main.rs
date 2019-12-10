use intcode::Vm;

fn part1(input: &str) -> Result<isize, intcode::Error> {
    let mut vm = input.parse::<Vm>().unwrap();
    vm.run(std::iter::repeat(1))
}

fn part2(input: &str) -> Result<isize, intcode::Error> {
    let mut vm = input.parse::<Vm>().unwrap();
    vm.run(std::iter::repeat(5))
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./day05/input.txt")?;
    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 1: {}", part2(&input).unwrap());
    Ok(())
}
