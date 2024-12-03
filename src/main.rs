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
            Key::Char('(') => { search.push('('); parse_search(&mut stdout, &search); },
            Key::Char(')') => { search.push(')'); parse_search(&mut stdout, &search); },
            Key::Char('[') => { search.push('['); parse_search(&mut stdout, &search); },
            Key::Char(']') => { search.push(']'); parse_search(&mut stdout, &search); },
            Key::Char('{') => { search.push('{'); parse_search(&mut stdout, &search); },
            Key::Char('}') => { search.push('}'); parse_search(&mut stdout, &search); },
            Key::Char('<') => { search.push('<'); parse_search(&mut stdout, &search); },
            Key::Char('>') => { search.push('>'); parse_search(&mut stdout, &search); },
            Key::Char('#') => { search.push('#'); parse_search(&mut stdout, &search); },
            Key::Char('%') => { search.push('%'); parse_search(&mut stdout, &search); },
            Key::Char('?') => { search.push('?'); parse_search(&mut stdout, &search); },
            Key::Char('+') => { search.push('+'); parse_search(&mut stdout, &search); },
            Key::Char('-') => { search.push('-'); parse_search(&mut stdout, &search); },
            Key::Char('*') => { search.push('*'); parse_search(&mut stdout, &search); },
            Key::Char('/') => { search.push('/'); parse_search(&mut stdout, &search); },
            Key::Char(' ') => { search.push(' '); parse_search(&mut stdout, &search); },
            Key::Backspace => {
                if search.chars().count() > 0 {
                    search = search[..search.chars().count()-1].parse().unwrap();
                    parse_search(&mut stdout, &search);
                }
            },
            Key::Ctrl('c') | Key::Esc => {
                write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
                break;
            },
            _ => (),
        }

        stdout.flush().unwrap();
    }
}

fn parse_search(stdout: &mut RawTerminal<std::io::Stdout>, query: &String) {
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
    println!("{}\x1b[{}D", query, query.len());

    let category_names =    vec!["motion",               "looks",                    "sound",                    "events",                   "control",                  "sensing",                  "operators",                "variables",                "lists",                    "my blocks",                "extensions"];
    let category_colours =  vec!["\x1b[38;2;76;151;255m",   "\x1b[38;2;153;102;255m",   "\x1b[38;2;207;99;207m",    "\x1b[38;2;255;191;0m",     "\x1b[38;2;255;171;25m",   "\x1b[38;2;92;177;214m",     "\x1b[38;2;89;192;89m",     "\x1b[38;2;255;140;26m",    "\x1b[38;2;255;102;26m",    "\x1b[38;2;255;102;128m",   "\x1b[38;2;15;189;140m"];

    let blocks = [
        ["move () steps", "motion"],
        ["turn clockwise () degrees", "motion"],
        ["turn counterclockwise () degrees", "motion"],
        ["go to [ v]", "motion"],
        ["go to x: () y: ()", "motion"],
        ["glide () secs to [ v]", "motion"],
        ["glide () secs to x: () y: ()", "motion"],
        ["point in direction ()", "motion"],
        ["point towards [ v]", "motion"],
        ["change x by ()", "motion"],
        ["set x to ()", "motion"],
        ["change y by ()", "motion"],
        ["set y to ()", "motion"],
        ["if on edge, bounce", "motion"],
        ["set rotation style [ v]", "motion"],
        ["(x position)", "motion"],
        ["(y position", "motion"],
        ["(direction)", "motion"],
        ["say [] for () seconds", "looks"],
        ["say []", "looks"],
        ["think [] for () seconds", "looks"],
        ["think []", "looks"],
        ["switch costume to [ v]", "looks"],
        ["next costume", "looks"],
        ["switch backdrop to [ v]", "looks"],
        ["next backdrop", "looks"],
        ["change size by ()", "looks"],
        ["set size to () %", "looks"],
        ["change [ v] effect by ()", "looks"],
        ["set [ v] effect to ()", "looks"],
        ["clear graphic effects", "looks"],
        ["show", "looks"],
        ["hide", "looks"],
        ["go to [ v] layer", "looks"],
        ["go [ v] () layers", "looks"],
        ["(costume [ v])", "looks"],
        ["(backdrop [ v])", "looks"],
        ["(size)", "looks"],
        ["play sound [ v] until done", "sound"],
        ["start sound [ v]", "sound"],
        ["stop all sounds", "sound"],
        ["change [ v] effect by ()", "sound"],
        ["set [ v] effect to ()", "sound"],
        ["clear sound effects", "sound"],
        ["change volume by ()", "sound"],
        ["set volume to ()", "sound"],
        ["(volume)", "sound"],
        ["when green flag clicked", "events"],
        ["when [ v] key pressed", "events"],
        ["when this sprite clicked", "events"],
        ["when backdrop switches to [ v]", "events"],
        ["when [ v] > ()", "events"],
        ["when i receive [ v]", "events"],
        ["broadcast [ v]", "events"],
        ["broadcast [ v] and wait", "events"],
        ["wait () seconds", "control"],
        ["repeat () {}", "control"],
        ["forever {}", "control"],
        ["if <> then {}", "control"],
        ["if <> then {} else {}", "control"],
        ["wait until <>", "control"],
        ["repeat until <>", "control"],
        ["stop [ v]", "control"],
        ["when i start as a clone", "control"],
        ["create clone of [ v]", "control"],
        ["delete this clone", "control"],
        ["<touching [ v]?>", "sensing"],
        ["<touching color (#)?>", "sensing"],
        ["<color (#) is touching (#)?>", "sensing"],
        ["(distance to [ v])", "sensing"],
        ["ask [] and wait", "sensing"],
        ["(answer)", "sensing"],
        ["<key [ v] pressed?>", "sensing"],
        ["<mouse down?>", "sensing"],
        ["(mouse x)", "sensing"],
        ["(mouse y)", "sensing"],
        ["set drag mode [ v]", "sensing"],
        ["(loudness)", "sensing"],
        ["(timer)", "sensing"],
        ["reset timer", "sensing"],
        ["([ v] of [ v])", "sensing"],
        ["(current [ v])", "sensing"],
        ["(days since 2000)", "sensing"],
        ["(username)", "sensing"],
        ["(() + ())", "operators"],
        ["(() - ())", "operators"],
        ["(() * ())", "operators"],
        ["(() / ())", "operators"],
        ["(pick random () to ())", "operators"],
        ["<[] > []>", "operators"],
        ["<[] < []>", "operators"],
        ["<[] = []>", "operators"],
        ["<<> and <>>", "operators"],
        ["<<> or <>>", "operators"],
        ["<not <>>", "operators"],
        ["(join [] [])", "operators"],
        ["(letter () of [])", "operators"],
        ["(length of [])", "operators"],
        ["<[] contains []?>", "operators"],
        ["(() mod ())", "operators"],
        ["(round ())", "operators"],
        ["([ v] of ())", "operators"],
        ["(my variable)", "variables"],
        ["set [ v] to []", "variables"],
        ["change [ v] by ()", "variables"],
        ["show variable [ v]", "variables"],
        ["hide variable [ v]", "variables"],
    ];

    let mut results = 0;
    if query.len() > 0 {
        for i in 0..blocks.len() {
            if blocks[i][0].contains(query) && results < 9 {
                println!("{}{}\x1b[0m\x1b[{}D", category_colours[category_names.iter().position(|&r| r == blocks[i][1]).unwrap()], blocks[i][0], blocks[i][0].len());
                results += 1;
            }
        }
        if results >= 9 {
            println!("...");
        }
    }

    print!("{}", termion::cursor::Goto(query.len() as u16 + 1, 1));
    stdout.flush().unwrap();
}
