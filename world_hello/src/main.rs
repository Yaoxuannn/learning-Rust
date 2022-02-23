use std::mem::size_of_val;

const FOO: i32 = 1;

fn main() {
    let a = 3;
    let b = 4;
    println!("Hello, world!");
    println!("The result of a + b is {}", add(a, b));
    var();
    println!("FOO is {}", FOO);
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {}, b = {}", a, b);

    b = true;
    assert_eq!(a, b);

}


fn add(a:i32, b:i32) -> i32 {
    return a + b  // 这个return可以被省略, 直接写成a+b
}

fn var() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    {
        let x = x * 2;
        println!("The value of x in this inner scope is {}.", x);
    }
    println!("The value of x is {}", x);
    let x = "6";
    let x = x.parse::<i32>().expect("Not a number");
    println!("The value of x is {}", x);


    // let twenty = 20; // 编译器自行推导
    // let twenty = 20i32; // 后缀方式标注
    // let twenty:f64 = 20.0; // 类型标注
    // let one_million:i64 = 1_000_000; // 可以使用下划线_来分割较长的数字, 以提升可读性
    let forty_twos = [
        42.042,
        42f32,
        42.0_f32
    ]; // 定义数组
    println!("{:.2}", forty_twos[0]); // 控制小数位为2位

    let c = '国';
    let emoji = '😴';
    println!("{}: The size of char '国' is {}", emoji, size_of_val(&c));

}
