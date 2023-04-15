use super::RunConfig;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;
use std::cmp::Ordering;

/// Problem 11: Largest product in a grid
///
/// In a 20x20 grid, what is the greatest product of four adjacent numbers in the same direction
/// (up, down, left, right, or diagonally)?
///
/// NOTE: the usage for this problem is:
///
/// `cargo run 11 filename`
///
/// where `filename` is the path to a text file containing the grid. The data provided by Project
/// Euler is located in `data/p011-grid.txt`.
pub fn run(config: RunConfig) {
    if config.params.len() < 1 {
        eprintln!("Problem 11 requires a filename");
        return;
    }

    let filename = &config.params[0];
    let file = File::open(filename).unwrap();
    let re = Regex::new(r"^(\d{2}(\s\d{2})*)?$").unwrap();
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        assert!(re.is_match(&line));

        let mut row: Vec<u8> = Vec::new();
        let mut temp_width: usize = 0;
        for elem in line.trim().split(" ") {
            let elem: u8 = elem.parse().unwrap();
            row.push(elem);
            temp_width += 1;
        }
        grid.push(row);

        height += 1;
        if width == 0 {
            width = temp_width;
        } else if width != temp_width {
            panic!("Input file must have consistent width!")
        }
    }

    let mut greatest_product = 0;
    for y in 0..height {
        for x in 0..width {
            // only look for E, S, SE, NE. The other directions are redundant

            if x < width - 3 {
                let prod_e = get_product_for_direction(&grid, &x, &y, 1, 0);
                if prod_e > greatest_product {
                    greatest_product = prod_e;
                }

                if y < height - 3 {
                    let prod_se = get_product_for_direction(&grid, &x, &y, 1, 1);
                    if prod_se > greatest_product {
                        greatest_product = prod_se;
                    }
                }

                if y >= 3 {
                    let prod_ne = get_product_for_direction(&grid, &x, &y, 1, -1);
                    if prod_ne > greatest_product {
                        greatest_product = prod_ne;
                    }
                }
            }

            if y < height - 3 {
                let prod_s = get_product_for_direction(&grid, &x, &y, 0, 1);
                if prod_s > greatest_product {
                    greatest_product = prod_s;
                }
            }
        }
    }

    println!("Answer: {}", greatest_product);
}

fn get_product_for_direction(grid: &Vec<Vec<u8>>, x: &usize, y: &usize, dx: isize, dy: isize) -> u64 {
    let mut result: u64 = 1;
    for i in 0..4 {
        let sx: usize = match dx.cmp(&0) {
            Ordering::Less => x - i,
            Ordering::Greater => x + i,
            Ordering::Equal => *x,
        };
        let sy: usize = match dy.cmp(&0) {
            Ordering::Less => y - i,
            Ordering::Greater => y + i,
            Ordering::Equal => *y,
        };
        result *= grid[sy][sx] as u64;
    }
    result
}
