use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main()  {
    println!("猜测一个数字");
    let  secret_number = rand::thread_rng().gen_range(1..3);

    let  mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("你猜测的是: {}",guess);
    // println!("随机数是: {}",secret_number);
    match  guess.cmp(&secret_number.to_string()) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too Big"),
    }

}

