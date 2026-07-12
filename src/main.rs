use std::io::{self, Write};

fn get_spiral_numbers(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let row_size: usize = matrix.len();
    let col_size: usize = matrix[0].len();
    let mut spiral_arr: Vec<i32> = Vec::with_capacity(row_size * col_size);

    if row_size == 1 {
        return Vec::clone(&matrix[0]);
    } else if col_size == 1 {
        let mut r: usize = 0;

        while r < row_size {
            spiral_arr.push(i32::clone(&matrix[r][0]));
            r += 1;
        }

        return spiral_arr;
    }

    let mut is_visited: Vec<Vec<bool>> = vec![vec![false; col_size]; row_size];
    let mut r: usize = 0;
    let mut c: usize = 0;
    let mut count: usize = 0;
    let spiral_count: usize = row_size * col_size;

    loop {

        // println!("\nloop1");
        while c < col_size {

            // println!("current_row: {} | current_col: {}", r, c);
            if is_visited[r][c] {break;}

            spiral_arr.push(i32::clone(&matrix[r][c]));
            is_visited[r][c] = true;

            c += 1;
            count += 1;
        }

        c -= 1;
        r += 1;

        // println!("\nloop2");
        while r < row_size {

            // println!("current_row: {} | current_col: {}", r, c);
            if is_visited[r][c] {break;}

            spiral_arr.push(i32::clone(&matrix[r][c]));
            is_visited[r][c] = true;

            r += 1;
            count += 1;
        }

        r -= 1;
        c -= 1;

        // println!("\nloop3");
        loop {

            // println!("current_row: {} | current_col: {}", r, c);
            if is_visited[r][c] {c += 1; break;}

            spiral_arr.push(i32::clone(&matrix[r][c]));
            is_visited[r][c] = true; 

            count += 1;
            if c == 0 {
                break;
            } 

            c -= 1;
        }

        r -= 1;

        // println!("\nloop4");
        loop {

            // println!("current_row: {} | current_col: {}", r, c);
            if !is_visited[r][c] {
                spiral_arr.push(i32::clone(&matrix[r][c]));
                is_visited[r][c] = true;
            } else {
                r += 1;
                c += 1;
                // println!("last current_row: {} | current_col: {}", r, c);
                break;
            }

            count += 1;
            if r == 0 {break;}

            r -= 1;
        }

        if count >= spiral_count {break;}
    }

    spiral_arr
}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the row size: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let row: usize = if let Ok(xr) = buf.trim().parse() {xr} else {0};
    buf.clear();

    print!("Enter the column size: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let col: usize = if let Ok(xc) = buf.trim().parse() {xc} else {0};

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; col]; row];
    
    for xr in 0..row {
        println!("Enter the values of row: {} below", xr + 1);
        
        for xc in 0..col {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            matrix[xr][xc] = if let Ok(xcv) = buf.trim().parse() {xcv} else {0};
        }

    }

    println!("The Matrix: {:?}", &matrix);
    // let matrix: Vec<Vec<i32>> = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16],vec![17,18,19,20],vec![21,22,23,24]];
    let spiral_arr: Vec<i32> = get_spiral_numbers(&matrix);
    println!("The spiral numbers are: {:?}", spiral_arr);
}
