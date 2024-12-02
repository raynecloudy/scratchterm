fn main() {
    let (width, height) = termion::terminal_size().unwrap();
    print!("\x1b[48;2;230;240;255m");
    for y in 0..height {
        for x in 0..width {
            print!(" ");
        }
        println!();
    }
}
