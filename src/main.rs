mod user_info;

use std::collections::HashMap;
use user_info::user::User;
mod game;
// use game::guess_game::guess_game;
mod fn_learn;
mod slice;
mod learn_struct;
mod learn_enum;

use fn_learn::fn_learn::fn1;
use fn_learn::fn_learn::fn2;
use fn_learn::fn_learn::fn3;
use fn_learn::circle::fn1 as circle_fn1;
use fn_learn::circle::fn2 as circle_fn2;
use fn_learn::circle::fn3 as circle_fn3;
use fn_learn::circle::fn4 as circle_fn4;
use fn_learn::fn_learn::gives_ownership;
use fn_learn::fn_learn::takes_and_gives_back;
use fn_learn::fn_learn::calculate_length;
use slice::learn_slice::first_world;
use learn_struct::learn_struct::area1;
use crate::learn_struct::learn_struct::Rectangle;
use learn_enum::ipaddr_kind;


fn main()  {
    // guess_game()
    let test =fn1();
    println!("test: {}",test);
    let  condition = true;
    // let 使用if表达式 , if 中数据类型需一致
    let num = if condition{ 5 } else { 6 };
    // 变量隐藏
    let test = fn2(num);
    println!("test: {}",test);
    fn3(num);
    circle_fn1();
    circle_fn2();
    circle_fn3();
    circle_fn4();
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{}",s);
    // 值移动，移动后s不可用
    let s2 = s;
    println!("{}",s2);
    // clone 操作 s2可用，深拷贝 deep copy  堆上
    let s3 = s2.clone();
    println!("{}",s3);
    // copy trait 可以用于像整数这样完全放在stack上面的类型
    // 如果一个类型实现了copy trait，那么旧变量在赋值后仍然可用
    //  如果一个类型或该类型的一部分实现了Drop trait ，rust不允许他再实现copy trait

    let ss1 = gives_ownership();
    println!("{}",ss1);
    let ss2 = String::from("Hello");
    let ss3 = takes_and_gives_back(ss2);
    println!("{}",ss3);
    let mut  ss4 = String::from("rust");
    let len = calculate_length(&mut ss4);
    println!("{}",len);
    let ss5 = String::from("rust 111");
    let len1 = first_world(&ss5[..]);
    println!("{}",len1);
    let user1 = User::new("king".parse().unwrap(), 18, "king@com.com".parse().unwrap(), 128, true);
    println!("{:#?}",user1);
    let rectangle = Rectangle::new(30,50);
    let rect = area1(&rectangle);
    println!("{}",rect);
    let rect1 = rectangle.area();
    println!("{}",rect1);
    let four = ipaddr_kind::IpaddrKind::V4;
    let six = ipaddr_kind::IpaddrKind::V6;
    ipaddr_kind::route(four);
    ipaddr_kind::route(six);
    let test1 = String::from("hello");
    let test2 = String::from("hello1");
    // let test3 = test1 + &test2;
    // println!("{}",test3);
    // println!("{}",test2);
    let s = format!("{}{}",test1,test2);
    println!("{}",s);
    // rust 字符串不支持索引访问

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),10);
    println!("{:#?}",scores);

    //
    let teams = vec![String::from("blue"),String::from("yellow")];
    let intial_scores = vec![10,50];
    let mut scores:HashMap<_,_> = teams.iter().zip(intial_scores.iter()).collect();
    println!("{:#?}",scores);
    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{}",s),
        None => println!("team not exist!"),
    };
    for (k,v) in &scores {
        println!("{},{}",k,v)
    }
    let mut color = HashMap::new();
    //entry  or_insert不存在插入
    color.entry(String::from("yellow")).or_insert(50);
    color.entry(String::from("blue")).or_insert(10);
    color.entry(String::from("yellow")).or_insert(10);
    println!("{:#?}",color);

    // 存在更新value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{:#?}",map);
}