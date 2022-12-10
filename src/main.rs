mod user_info;
use user_info::user::User;
mod game;
// use game::guess_game::guess_game;
mod fn_learn;
use fn_learn::fn_learn::fn1;
use fn_learn::fn_learn::fn2;
use fn_learn::fn_learn::fn3;
use fn_learn::circle::fn1 as circle_fn1;
use fn_learn::circle::fn2 as circle_fn2;
use fn_learn::circle::fn3 as circle_fn3;
use fn_learn::circle::fn4 as circle_fn4;


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
}

