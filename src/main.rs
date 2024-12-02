use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
    stdout.flush().unwrap();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('a') => print!("a"),
            Key::Char('b') => print!("b"),
            Key::Char('c') => print!("c"),
            Key::Char('d') => print!("d"),
            Key::Char('e') => print!("e"),
            Key::Char('f') => print!("f"),
            Key::Char('g') => print!("g"),
            Key::Char('h') => print!("h"),
            Key::Char('i') => print!("i"),
            Key::Char('j') => print!("j"),
            Key::Char('k') => print!("k"),
            Key::Char('l') => print!("l"),
            Key::Char('m') => print!("m"),
            Key::Char('n') => print!("n"),
            Key::Char('o') => print!("o"),
            Key::Char('p') => print!("p"),
            Key::Char('q') => print!("q"),
            Key::Char('r') => print!("r"),
            Key::Char('s') => print!("s"),
            Key::Char('t') => print!("t"),
            Key::Char('u') => print!("u"),
            Key::Char('v') => print!("v"),
            Key::Char('w') => print!("w"),
            Key::Char('x') => print!("x"),
            Key::Char('y') => print!("y"),
            Key::Char('z') => print!("z"),
            Key::Ctrl('c') => break,
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
