use intcode::Vm;

fn main() {
    let input = std::fs::read_to_string("./day09/input.txt").unwrap();

    // let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut vm = input.parse::<Vm>().unwrap();
    println!("Part 1: {:?}", vm.clone().run(std::iter::once(1)));
    println!("Part 2: {:?}", vm.clone().run(std::iter::once(2)));
}
