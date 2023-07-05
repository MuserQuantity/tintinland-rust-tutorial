// 循环打印从’a’~’Z’ 之间的所有字符
pub fn print_range1() {
    let mut c = 'a';
    loop {
        print!("{}", c);
        if c == 'Z' {
            break;
        }
        c = (c as u8 - 1) as char;
    }
    println!();
}

pub mod printer2;