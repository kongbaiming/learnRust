use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn guess_game() {
    println!("猜数字游戏!!");
    let  secret_number = rand::thread_rng().gen_range(1..10);
    println!("神秘数字是: {}",secret_number);
    loop {
        println!("猜测一个数字:");
        let  mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("你猜测的是: {}",guess);
        // 可以使用新变量隐藏就变量
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字!!");
                continue;
            }
        };
        match  guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break
            },
            Ordering::Greater => println!("Too Big"),
        }
    }
}