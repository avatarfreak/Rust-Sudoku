#![allow(unused_variables)]
fn print_sud(board:[[i32;9]; 9]) {
    println!("Before solution!!!");
    println!("*********************");
    for x in 0..9{
        if x == 3 || x == 6{
            println!("*********************");
        }
        for y in 0..9{
            if y == 3 || y == 6{
                print!("* ");
            }
            print!("{} ",board[x][y]);
        }
        println!("{}", " ");
    }
    println!("*********************");
}

fn print_solved_sud(board:[[i32;9]; 9]) {
    println!("Solution");
    println!("*********************");
    for x in 0..9{
        if x == 3 || x == 6{
            println!("*********************");
        }
        for y in 0..9{
            if y == 3 || y == 6{
                print!("* ");
            }
            print!("{} ",board[x][y]);
        }
        println!("{}", " ");
    }
    println!("*********************");
}

fn is_safe(row: usize, col: usize, val:i32, board: &mut [[i32; 9]; 9]) -> bool {
    //check for duplicate element row. If found, return false.
    for y in 0..9{
        if board[y][col] == val{
            return false;
        }
    }

    //check for duplicate element in col. If found, return false.
    for x in 0..9{
        if board[row][x] == val{
            return false;
        }
    }

    //check for duplicate element in 3x3 box> If found, return false.
    let box_row= (row  / 3) * 3;
    let box_col = (col /3) * 3;
    for i in 0..3{
        for j in 0..3{
            if board[box_row + i][box_col + j] == val{
                return false;
            }
        }
    }
    true
}

fn solve(row:usize, col:usize, board: &mut [[i32; 9]; 9])-> bool {
    let mut row = row;
    let mut col = col;
    if row == 9{
        row = 0;
        col += 1;
        if col == 9{
            return true;
        }
    }

    // skip to the next row if it is not empty
    if board[row][col] != 0{
        return solve(row + 1, col, board);
    }

    for val in 1..10{
        if is_safe(row, col, val, board){
            board[row][col] = val;
            if solve(row + 1, col, board){
                return true;
            }
        }
    }

    board[row][col] = 0;
    false

}
fn main() {
    let mut board: [[i32; 9]; 9] = [[5, 3, 0, 0, 7, 0, 0, 0, 0],
                                    [6, 0, 0, 1, 9, 5, 0, 0, 0],
                                    [0, 9, 8, 0, 0, 0, 0, 6, 0],
                                    [8, 0, 0, 0, 6, 0, 0, 0, 3],
                                    [4, 0, 0, 8, 0, 3, 0, 0, 1],
                                    [7, 0, 0, 0, 2, 0, 0, 0, 6],
                                    [0, 6, 0, 0, 0, 0, 2, 8, 0],
                                    [0, 0, 0, 4, 1, 9, 0, 0, 5],
                                    [0, 0, 0, 0, 8, 0, 0, 7, 9]];

    print_sud(board);
    solve(0, 0, &mut board);
    print_solved_sud(board);
}
