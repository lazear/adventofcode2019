use intcode::Vm;

fn part1(input: &str) -> Option<isize> {
    let mut vm = input.parse::<Vm>().ok()?;
    // vm.clone().decompile();
    dbg!(vm.data.len());
    let mut res = None;
    loop {
        match vm.run(std::iter::once(1)) {
            Ok(out) => {
                // println!("{}", out);
                res = Some(out);
                break;
            }
            Err(e) => println!("{:?}", e),
        }
    }
    res
}

fn main() {
    let input = std::fs::read_to_string("./day09/input.txt").unwrap();
    // let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    println!("Part 1: {:?}", part1(&input));
}
