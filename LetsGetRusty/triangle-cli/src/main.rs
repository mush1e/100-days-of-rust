use std::io;
use std::io::Write;

fn display_triangle(n : u32){
    let mut row_count = 0;
    'outer_loop : loop {
        let mut column_count = 0;

        'inner_loop: loop {
            if column_count > row_count {
                break 'inner_loop;
            }
            print!("*");
            column_count += 1;
        }

        print!("\n");

        if row_count == n {
            break 'outer_loop;
        }
        row_count += 1;
    }
}


fn main() {
    loop {
        let mut buffer = String::new();
        print!("Enter the desired size for the triangle: ");
        io::stdout().flush().expect("Error flushing stdout");
        io::stdin().read_line(&mut buffer).expect("Error reading line");
        let mut max_lines : u32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break display_triangle(max_lines);
    }
}
