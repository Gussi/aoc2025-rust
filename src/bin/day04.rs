use std::io::Read;
use aoc2025::run;
use std::io::BufRead;

struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    fn new_from_lines<R: Read>(mut input: R) -> Result<Self, std::io::Error> {
        let mut grid = Vec::new();

        for line in std::io::BufReader::new(&mut input).lines() {
            let row: Vec<char> = line?.chars().collect();
            grid.push(row);
        }

        Ok(Self { grid })
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if y < self.grid.len() && x < self.grid[y].len() {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn remove(&mut self, x: usize, y: usize) {
        if y < self.grid.len() && x < self.grid[y].len() {
            self.grid[y][x] = 'x';
        }
    }

    fn surrounded_count(&self, x: usize, y: usize, target: char) -> usize {
        let mut directions = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    directions.push((dx, dy));
                }
            }
        }

        let mut count = 0;

        for (dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                if let Some(c) = self.get(nx as usize, ny as usize) {
                    if c == target {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn main() {
    run(part01, part02)
}

fn part01<R: Read>(mut input: R) -> Result<i64, std::io::Error> {
    let map: Map = Map::new_from_lines(&mut input)?;

    let mut accessible_count = 0;

    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            if map.get(x, y) != Some('@') {
                continue;
            }

            if map.surrounded_count(x, y, '@') < 4 {
                accessible_count += 1;
            }
        }
    }

    Ok(accessible_count)
}

fn part02<R: Read>(mut _input: R) -> Result<i64, std::io::Error> {
    let mut map: Map = Map::new_from_lines(&mut _input)?;

    let mut removed_count = 0;

    loop {
        let mut toilet_paper_was_removed = false;

        for y in 0..map.grid.len() {
            for x in 0..map.grid[y].len() {
                if map.get(x, y) != Some('@') {
                    continue;
                }

                if map.surrounded_count(x, y, '@') < 4 {
                    map.remove(x, y);
                    removed_count += 1;
                    toilet_paper_was_removed = true;
                }
            }
        }

        if !toilet_paper_was_removed {
            break;
        }
    }

    Ok(removed_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static [u8] {
        b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
    }

    #[test]
    fn test_part01() {
        assert_eq!(part01(input()).unwrap(), 13);
    }

    #[test]
    fn test_part02() {
        assert_eq!(part02(input()).unwrap(), 43);
    }
}
