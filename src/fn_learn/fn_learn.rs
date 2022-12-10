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

pub fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

pub fn takes_and_gives_back(a_string:String) -> String {
    a_string
}

// 可变引用
// 特定的作用域内只能有一个可变引用，防止数据竞争
// 不可以同时拥有一个可变引用和一个不可变引用
// 多个不变的引用是可以的
pub fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}