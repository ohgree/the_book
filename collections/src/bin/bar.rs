use std::io;
fn main() {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let n: i32 = buf.trim().parse().unwrap();
    }
}
