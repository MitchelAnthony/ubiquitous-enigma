use std::fs;

pub fn run(count: usize) -> usize {
    let mut product = 1;
    
    let mut grid: Vec<Vec<usize>> = Vec::new();
    parse_grid(&mut grid);

    for i in 0..grid.len() - count {
        for j in 0..grid[i].len() - count {
            // println!("({},{}) -> {}", i, j, grid[i][j]);
            if find_greatest_adjecent_product(&grid, count, i, j) > product {
                product = find_greatest_adjecent_product(&grid, count, i, j);
            }
        }
    }

    product
}

fn parse_grid(grid: &mut Vec<Vec<usize>>) {
    let contents = fs::read_to_string("./src/resources/eleven.txt")
        .expect("Error reading file!");
    
    for line in contents.lines() {
        let mut row: Vec<usize> = Vec::new();
        for unit in line.split_whitespace() {
            row.push(unit.parse::<usize>().unwrap());
        }
        grid.push(row);
    }
}

fn find_greatest_adjecent_product(grid: &Vec<Vec<usize>>, count: usize, x: usize, y: usize) -> usize {
    use std::cmp::max;

    let count = count - 1;
    let mut horizontal = 1;
    let mut vertical = 1;
    let mut diagonal_right = 1;
    let mut diagonal_left = 1;
    for i in 0..=count {
        horizontal *= grid[x + i][y];
        vertical *= grid[x][y + i];
        diagonal_right *= grid[x + i][y + i];
        
        if x as isize - count as isize >= 0 {
            diagonal_left *= grid[x - i][y + i];
        }
    }
    
    max(horizontal, max(vertical, max(diagonal_right, diagonal_left)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eleven() {
        assert_eq!(run(1), 99);
        assert_eq!(run(2), 9603);
        assert_eq!(run(3), 811_502);
        assert_eq!(run(4), 70_600_674);
    }
}
