use grid::*;
use intcode::*;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::iter::Iterator;
use std::usize;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    vel: Velocity,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Velocity {
    dx: isize,
    dy: isize,
    dz: isize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Deriv {
    dxdt: isize,
    dydt: isize,
    dzdt: isize,
}

impl Moon {
    pub fn apply_vel(&mut self, v: Velocity) {
        self.vel.dx += v.dx;
        self.vel.dy += v.dy;
        self.vel.dz += v.dz;
    }

    pub fn gravity(&self, other: Moon) -> (Velocity, Velocity) {
        let mut a = Velocity::default();
        let mut b = Velocity::default();
        if self.x < other.x {
            a.dx = 1;
            b.dx = -1;
        } else if self.x > other.x {
            a.dx = -1;
            b.dx = 1;
        }

        if self.y < other.y {
            a.dy = 1;
            b.dy = -1;
        } else if self.y > other.y {
            a.dy = -1;
            b.dy = 1;
        }

        if self.z < other.z {
            a.dz = 1;
            b.dz = -1;
        } else if self.z > other.z {
            a.dz = -1;
            b.dz = 1;
        }

        (a, b)
    }
}

fn part1(moons: &mut [Moon]) -> isize {
    let mut last = 0;
    for _ in 0..1000 {
        let mut updates = (0..4).map(|_| Vec::new()).collect::<Vec<_>>();
        for i in 0..4 {
            for j in i..4 {
                let (a, b) = moons[i].gravity(moons[j]);
                updates[i].push(a);
                updates[j].push(b);
            }
        }

        let mut vels = Vec::new();
        for (idx, u) in updates.into_iter().enumerate() {
            let mut vel_dt = Velocity::default();
            for x in u {
                moons[idx].apply_vel(x);
                vel_dt.dx += x.dx;
                vel_dt.dy += x.dy;
                vel_dt.dz += x.dz;
            }
            vels.push(vel_dt);
        }

        let mut total = 0;
        for m in moons.iter_mut() {
            m.x += m.vel.dx;
            m.y += m.vel.dy;
            m.z += m.vel.dz;

            let pot = m.x.abs() + m.y.abs() + m.z.abs();
            let kin = m.vel.dx.abs() + m.vel.dy.abs() + m.vel.dz.abs();
            total += pot * kin;
        }
        last = total;
    }
    last
}

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}
fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn part2(moons: &mut Vec<Moon>) -> usize {
    let mut x = HashSet::new();
    let mut y = HashSet::new();
    let mut z = HashSet::new();

    let mut stopped = (0, 0, 0);
    for step in 0..usize::MAX {
        let mut updates = (0..4).map(|_| Vec::new()).collect::<Vec<_>>();
        for i in 0..4 {
            for j in i..4 {
                let (a, b) = moons[i].gravity(moons[j]);
                updates[i].push(a);
                updates[j].push(b);
            }
        }

        let mut vels = Vec::new();
        for (idx, u) in updates.into_iter().enumerate() {
            let mut vel_dt = Velocity::default();
            for x in u {
                moons[idx].apply_vel(x);
                vel_dt.dx += x.dx;
                vel_dt.dy += x.dy;
                vel_dt.dz += x.dz;
            }
            vels.push(vel_dt);
        }

        for m in moons.iter_mut() {
            m.x += m.vel.dx;
            m.y += m.vel.dy;
            m.z += m.vel.dz;
        }
        let xs = moons.iter().map(|m| (m.x, m.vel.dx)).collect::<Vec<_>>();
        let ys = moons.iter().map(|m| (m.y, m.vel.dy)).collect::<Vec<_>>();
        let zs = moons.iter().map(|m| (m.z, m.vel.dz)).collect::<Vec<_>>();

        if x.contains(&xs) && stopped.0 == 0 {
            stopped.0 = step;
        } else {
            x.insert(xs);
        }
        if y.contains(&ys) && stopped.1 == 0 {
            stopped.1 = step;
        } else {
            y.insert(ys);
        }
        if z.contains(&zs) && stopped.2 == 0 {
            stopped.2 = step;
        } else {
            z.insert(zs);
        }

        if stopped.0 != 0 && stopped.1 != 0 && stopped.2 != 0 {
            break;
        }
    }

    lcm(lcm(stopped.0, stopped.1), stopped.2)
}

fn parse(input: &str) -> Result<Vec<Moon>, std::num::ParseIntError> {
    let v = input
        .lines()
        .flat_map(|s| {
            s.split('=').map(|s| {
                s.chars()
                    .take_while(|ch| ch.is_numeric() || *ch == '-')
                    .collect::<String>()
            })
        })
        .filter(|s| s.len() > 0)
        .map(|v| v.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(v.chunks_exact(3)
        .map(|slice| Moon {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            vel: Velocity::default(),
        })
        .collect::<Vec<_>>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./day12/input.txt")?;
    let mut data = parse(&input)?;

    println!("Part 1: {}", part1(&mut data.clone()));
    println!("Part 2: {}", part2(&mut data));
    Ok(())
}
