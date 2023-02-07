use std::fmt;

const SIZE: usize = 8;

struct Square<'a> {
    color: &'a str,
}

impl<'a> fmt::Display for Square<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.color)
    }
}

fn main() {
    let mut grid = [[Square { color: "" }; SIZE]; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            grid[i][j].color = if (i + j) % 2 == 0 {
                "black"
            } else {
                "white"
            };
        }
    }

    for row in grid.iter() {
        for square in row.iter() {
            print!("{} ", square);
        }
        println!("");
    }
}