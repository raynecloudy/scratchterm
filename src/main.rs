use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut search: String = "".to_string();
    parse_search(&mut stdout, &search);
    stdout.flush().unwrap();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('a') => { search.push('a'); parse_search(&mut stdout, &search); },
            Key::Char('b') => { search.push('b'); parse_search(&mut stdout, &search); },
            Key::Char('c') => { search.push('c'); parse_search(&mut stdout, &search); },
            Key::Char('d') => { search.push('d'); parse_search(&mut stdout, &search); },
            Key::Char('e') => { search.push('e'); parse_search(&mut stdout, &search); },
            Key::Char('f') => { search.push('f'); parse_search(&mut stdout, &search); },
            Key::Char('g') => { search.push('g'); parse_search(&mut stdout, &search); },
            Key::Char('h') => { search.push('h'); parse_search(&mut stdout, &search); },
            Key::Char('i') => { search.push('i'); parse_search(&mut stdout, &search); },
            Key::Char('j') => { search.push('j'); parse_search(&mut stdout, &search); },
            Key::Char('k') => { search.push('k'); parse_search(&mut stdout, &search); },
            Key::Char('l') => { search.push('l'); parse_search(&mut stdout, &search); },
            Key::Char('m') => { search.push('m'); parse_search(&mut stdout, &search); },
            Key::Char('n') => { search.push('n'); parse_search(&mut stdout, &search); },
            Key::Char('o') => { search.push('o'); parse_search(&mut stdout, &search); },
            Key::Char('p') => { search.push('p'); parse_search(&mut stdout, &search); },
            Key::Char('q') => { search.push('q'); parse_search(&mut stdout, &search); },
            Key::Char('r') => { search.push('r'); parse_search(&mut stdout, &search); },
            Key::Char('s') => { search.push('s'); parse_search(&mut stdout, &search); },
            Key::Char('t') => { search.push('t'); parse_search(&mut stdout, &search); },
            Key::Char('u') => { search.push('u'); parse_search(&mut stdout, &search); },
            Key::Char('v') => { search.push('v'); parse_search(&mut stdout, &search); },
            Key::Char('w') => { search.push('w'); parse_search(&mut stdout, &search); },
            Key::Char('x') => { search.push('x'); parse_search(&mut stdout, &search); },
            Key::Char('y') => { search.push('y'); parse_search(&mut stdout, &search); },
            Key::Char('z') => { search.push('z'); parse_search(&mut stdout, &search); },
            Key::Backspace => {
                if (search.chars().count() > 0) {
                    search = search[..search.chars().count()-1].parse().unwrap();
                    parse_search(&mut stdout, &search);
                }
            },
            Key::Ctrl('c') => break,
            _ => (),
        }

        stdout.flush().unwrap();
    }
}

fn parse_search(stdout: &mut RawTerminal<std::io::Stdout>, query: &String) {
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
    println!("{}", query);
}
