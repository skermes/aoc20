use std::collections::HashSet;
use crate::aoc_error::AocError;

pub const NAME: &str = "Conway Cubes";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec4 {
    x: isize,
    y: isize,
    z: isize,
    a: isize
}

impl Vec4 {
    fn neighbors(&self) -> Neighbors {
        Neighbors {
            point: *self,
            dx: -1,
            dy: -1,
            dz: -1,
            da: -1
        }
    }
}

struct Neighbors {
    point: Vec4,
    dx: isize,
    dy: isize,
    dz: isize,
    da: isize
}

impl Iterator for Neighbors {
    type Item = Vec4;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dx > 1 || self.dy > 1 || self.dz > 1 || self.da > 1 {
            None
        } else {
            let next = Vec4 {
                x: self.point.x + self.dx,
                y: self.point.y + self.dy,
                z: self.point.z + self.dz,
                a: self.point.a + self.da
            };

            self.da += 1;

            if self.da == 2 {
                self.da = -1;
                self.dz += 1;
            }

            if self.dz == 2 {
                self.dz = -1;
                self.dy += 1;
            }

            if self.dy == 2 {
                self.dy = -1;
                self.dx += 1;
            }

            if self.dx == 0 && self.dy == 0 && self.dz == 0 && self.da == 0 {
                self.da += 1;
            }

            Some(next)
        }
    }
}

#[derive(Debug)]
struct Conway {
    active: HashSet<Vec4>
}

impl Conway {
    fn new(initial: &str) -> Conway {
        let active = initial
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| Vec4 {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                        a: 0
                    })
            })
            .collect();

        Conway { active }
    }

    fn active_adjacent(&self, point: Vec4) -> usize {
        point
            .neighbors()
            .filter(|p| self.active.contains(p))
            .count()
    }

    fn run(&mut self, steps: usize) {
        if self.active.is_empty() {
            return;
        }

        let min_x = self.active.iter().map(|p| p.x).min().unwrap();
        let max_x = self.active.iter().map(|p| p.x).max().unwrap();
        let min_y = self.active.iter().map(|p| p.y).min().unwrap();
        let max_y = self.active.iter().map(|p| p.y).max().unwrap();
        let min_z = self.active.iter().map(|p| p.z).min().unwrap();
        let max_z = self.active.iter().map(|p| p.z).max().unwrap();
        let min_a = self.active.iter().map(|p| p.a).min().unwrap();
        let max_a = self.active.iter().map(|p| p.a).max().unwrap();

        let min_bound_x = min_x - steps as isize;
        let max_bound_x = max_x + steps as isize;
        let min_bound_y = min_y - steps as isize;
        let max_bound_y = max_y + steps as isize;
        let min_bound_z = min_z - steps as isize;
        let max_bound_z = max_z + steps as isize;
        let min_bound_a = min_a - steps as isize;
        let max_bound_a = max_a + steps as isize;

        for _ in 0..steps {
            let mut to_remove = Vec::new();
            let mut to_add = Vec::new();

            for x in min_bound_x..=max_bound_x {
                for y in min_bound_y..=max_bound_y {
                    for z in min_bound_z..=max_bound_z {
                        for a in min_bound_a..=max_bound_a {
                            let point = Vec4 { x, y, z, a };
                            let neighbor_count = self.active_adjacent(point);

                            if self.active.contains(&point) &&
                               (neighbor_count < 2 || neighbor_count > 3) {
                                to_remove.push(point);
                            }

                            if !self.active.contains(&point) &&
                               neighbor_count == 3 {
                                to_add.push(point);
                            }
                        }
                    }
                }
            }

            for point in to_remove {
                self.active.remove(&point);
            }

            for point in to_add {
                self.active.insert(point);
            }
        }
    }
}

pub fn part_one(_input: &str) -> Result<String, AocError> {
    Ok("Not implemented".to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut conway = Conway::new(input);
    conway.run(6);

    Ok(conway.active.len().to_string())
}
