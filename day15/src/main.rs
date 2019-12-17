use grid::{Coord, Direction, Grid};
use intcode::Vm;
use std::collections::HashMap;

enum Status {
    Unchanged,
    Success,
    Final,
}

struct Game {
    map: HashMap<Coord, char>,
    last: Coord,
    vm: Vm,
}

impl Game {
    pub fn new(vm: Vm) -> Game {
        Game {
            map: HashMap::default(),
            last: Coord::new(0, 0),
            vm,
        }
    }

    pub fn step(&mut self, dir: Direction) -> Result<Status, intcode::Error> {
        let i = match dir {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        };

        let new_pos = self.last.move_one(dir);

        let stat = match self.vm.run(std::iter::once(i), false)? {
            0 => {
                self.map.insert(new_pos, '#');
                Status::Unchanged
            }
            1 => {
                self.map.insert(new_pos, '.');
                self.last = new_pos;
                Status::Success
            }
            2 => {
                self.map.insert(new_pos, 'o');
                self.last = new_pos;
                Status::Final
            }
            _ => panic!("Invalid game state!"),
        };
        Ok(stat)
    }

    pub fn grid(&self) -> Grid<char> {
        Coord::to_grid(self.map.clone())
    }

    pub fn sequence(&mut self, steps: &[Direction]) -> Result<usize, intcode::Error> {
        steps.iter().try_fold(0, |acc, &d| match self.step(d) {
            Ok(Status::Success) => Ok(acc + 1),
            Err(e) => Err(e),
            _ => Ok(acc),
        })
    }

    pub fn find_edge(&mut self, edge: Direction, opp: Direction) -> usize {
        let mut total = 0;
        loop {
            let mut steps = 0;
            while let Ok(Status::Success) = self.step(edge) {
                steps += 1;
            }
            if steps > 1 {
                match self.step(opp) {
                    Ok(Status::Success) => steps += 1,
                    _ => return total + steps,
                }
            }
            total += steps;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./day15/input.txt").unwrap();
    let mut vm = input.parse::<Vm>().unwrap();

    let mut game = Game::new(vm);

    while game.find_edge(Direction::Up, Direction::Down) > 1 {
        let _ = game.step(Direction::Left);
    }

    while game.find_edge(Direction::Left, Direction::Right) > 1 {
        let _ = game.step(Direction::Down);
    }

    println!("\n\n{}", game.grid());
}
