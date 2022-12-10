mod user_info;
use user_info::user::User;
mod game;
// use game::guess_game::guess_game;
mod fn_learn;
use fn_learn::fn_learn::fn1;
use fn_learn::fn_learn::fn2;
use fn_learn::fn_learn::fn3;

fn main()  {
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));
    // guess_game()
    let test =fn1();
    println!("test: {}",test);
    let  condition = true;
    // let 使用if表达式
    let num = if condition{ 5 } else { 6 };
    // 变量隐藏
    let test = fn2(num);
    println!("test: {}",test);
    fn3(num);
}

