use regex::Regex;

macro_rules! print_esc {
    ($e:expr) => {
        print!("\x1b{}", $e);
    };
}

pub fn move_vert(n: i32) {
    if n >= 0 {
        print_esc!(format!("[{}A", n));
    }
    print_esc!(format!("[{}B", -n));
}

pub fn move_hor(n: i32) {
    if n >= 0 {
        print_esc!(format!("[{}C", n));
    }
    print_esc!(format!("[{}D", -n));
}

pub fn move_to_upper_left() {
    print_esc!("[H");
}

pub fn move_to(line: u32, column: u32) {
    print_esc!(format!("[{};{}H", line, column));
}

pub fn move_to_next_line() {
    print_esc!("[E");
}

pub fn scroll_up() {
    print_esc!("D");
}

pub fn scroll_down() {
    print_esc!("M");
}

pub fn clear_right() {
    print_esc!("[K");
}

pub fn clear_left() {
    print_esc!("[1K");
}

pub fn clear_line() {
    print_esc!("[2K");
}

pub fn clear_down() {
    print_esc!("[J");
}

pub fn clear_up() {
    print_esc!("[1J");
}

pub fn clear_screen() {
    print_esc!("[2J");
}

pub fn save_position() {
    print_esc!("7");
}

pub fn restore_position() {
    print_esc!("8");
}

pub fn get_position() -> (u32, u32) {
    use std::io::{self, stdin, Read};
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\x1b\[(\d+);(\d+)R").unwrap();
    }
    let mut s = String::new();
    print_esc!("[6n\r\n");
    std::io::stdin().read_line(&mut s).unwrap();
    let m = RE.captures(&s).unwrap();
    let (line, column) = (&m[1], &m[2]);
    let line: u32 = line.parse().unwrap();
    let column: u32 = column.parse().unwrap();
    (line, column)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cursor_basic() {
        clear_up();
    }

    #[test]
    fn test_move_hor() {
        print!("hello");
        move_hor(10);
        print!("world");
        move_hor(-15);
        print!(",");
        println!();
    }

    #[test]
    fn test_move_diagonal() {
        for i in 0..8 {
            move_hor(i);
            println!("{}", std::char::from_u32((i+65) as u32).unwrap());
        }
        println!();
    }

    #[test]
    fn test_get_pos() {
        let pos = get_position();
        println!("{:?}", pos);
    }
}
