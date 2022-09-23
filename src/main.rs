fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let name = line.trim().to_string();

    println!("こんにちわ{}さん", name);

    let _t1 = (1, 2);
}
