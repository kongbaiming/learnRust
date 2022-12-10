use std::cmp::Ordering;

pub fn fn1() -> i32 {
    5
}

pub fn fn2(x:i32) -> i32 {
    return if x <= 5 {
        println!("大于等于5");
        1
    } else {
        println!("小于5");
        2
    }
}

pub fn fn3(x:i32) {
    let num = 5;
    match  x.cmp(&num) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("======"),
        Ordering::Greater => println!("Too Big"),
    }
}