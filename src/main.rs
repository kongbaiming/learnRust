mod user_info;
use user_info::user::User;
mod game;
// use game::guess_game::guess_game;
mod fn_learn;
mod slice;

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


fn main()  {
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));
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
    let len1 = first_world(&ss5);
    println!("{}",len1);
}

