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
    exp_stm();
    b = true;
    assert_eq!(a, b);
    cal(32);
}


fn add(a:i32, b:i32) -> i32 {
    return a + b  // 这个return可以被省略, 直接写成a+b
    // Why? Because this is a **expression**, not a **statement**
    // 表达式会在求值后`返回`一个值, 而语句只是`执行操作`但是不会返回值
}

fn exp_stm() -> i32 {
    // statements
    let _a = 8;
    let _b: Vec<f64> = Vec::new();
    let (_a, _b) = ("hi", false);
    // Note this statement raise an error, because statement does not return any value
    // let a = (let b = 0);
    
    // expressions
    // 5 + 6
    // 1 + 1
    // Note there is no semicolon at the end of a expression
    // Once we add a semicolon, it becaomes a statement
    // This is also a expression:
    {
        let x = 3;
        x + 1
    }

}

fn cal(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }
    
    x + 5
}

// This function returns a ()
// () is a zero-length tuple
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
