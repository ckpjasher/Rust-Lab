use std::io;


// 输入字符
fn main() {
    println!("输入字符！");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("你输入了: {guess}");
}
