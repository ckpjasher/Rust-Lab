use std::io;
use rand::Rng;

// 生成一个随机数
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("这个随机数是: {secret_number}");

    println!("输入你的数字");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("你输入的数字是: {guess}");
}
