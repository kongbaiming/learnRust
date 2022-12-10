pub fn  fn1() {
    let mut num = 3;
    while num != 0 {
        println!("{}!",num);
        num = num - 1;
    }
}

pub fn fn2() {
    let a = [10,20,30,40,50,60];
    for i in a.iter() {
        println!("i:{}",i)
    }
}

pub fn fn3() {
    // rev 反转
    for i in (1..10).rev() {
        println!("{}!",i);
    }
    println!("haha!!!")
}

pub fn fn4() {
    for i in 1..10 {
        println!("{}!",i);
    }
    println!("haha!!!")
}