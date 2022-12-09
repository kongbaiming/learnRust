use std::io;

fn main()  {
    println!("猜测一个数字");
    let  mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("你猜测的是: {}",guess);
}

