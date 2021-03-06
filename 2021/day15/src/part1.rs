use crate::structs::{Point, Print};

pub fn part1(input: &[Vec<Point>]) {
    let mut grid = input.to_owned();
    // grid.print(1);
    grid[0][0].value = 0;
    grid[0][0].original_value = 0;
    for y in (0..grid.len()).rev() {
        for x in (0..grid[0].len()).rev() {
            let mut neighbors = Vec::with_capacity(2);
            if x < grid[y].len() - 1 {
                neighbors.push(grid[y][x + 1]);
            }
            if y < grid.len() - 1 {
                neighbors.push(grid[y + 1][x])
            }
            grid[y][x].set_value(&neighbors[..]);
        }
    }
    // grid.print(4);
    println!("{} * {}", grid.len(), grid[0].len());
    println!("Part 1.1: {}", grid[0][0].value);

    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for y in (0..grid.len()).rev() {
                    for x in (0..grid[0].len()).rev() {
                let mut neighbors = Vec::with_capacity(4);
                if x < grid[y].len() - 1 {
                    neighbors.push(grid[y][x + 1]);
                }
                if x > 0 {
                    neighbors.push(grid[y][x - 1]);
                }
                if y < grid.len() - 1 {
                    neighbors.push(grid[y + 1][x]);
                }
                if y > 0 {
                    neighbors.push(grid[y - 1][x])
                }
                if y < grid.len() - 1 || x < grid[y].len() - 1 {
                    let old_value = grid[y][x].value;
                    grid[y][x].set_value(&neighbors[..]);
                    if old_value != grid[y][x].value {
                        has_changed = true;
                    }
                    if old_value < grid[y][x].value {
                        println!("{:?}", grid[y][x])
                    }
                }
            }
        }
        // grid.print();
    }
    // grid.print(4);

    println!("Part 1.2: {}", grid[0][0].value)
}
