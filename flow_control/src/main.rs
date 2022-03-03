fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    }; // if block is a expression and have a return value
    // below code will raise an error
    // let wrong = if condition {
    //     5
    // } else {
    //     "six"
    // };
    println!("The number is {}", number);

    for i in 1..=5 {
        println!("{}", &i);
    }
    // tip: we should use the reference of the collection if we still want
    // to use this collection later. (collection not impl copy trait)

    // if we want to modify element in a for loop, we can use mut:
    // for item in &mut collection {
    //      ...
    // }

    let a = [5,4,3,2,1];
    for (i, v) in a.iter().enumerate() {
        println!("element at {} is {}", &i, &v);
    }

    // continue and break

    // while
    let mut n = 0;
    while n < 5 {
        n += 1;
    }
    println!("while finished. n is {}", &n); // n is 5

    // loop
    // loop is the simplest loop that can be used under any sceanario. However,
    // for and while is always a better option.

    // we need to use break and continue to control the loop
    let mut counter = 0;
    let final_counter = loop {
        if counter == 5 {
            break counter; // return this value
        }
        counter += 1;
    };

    println!("The result is {}", final_counter);
    

    // match
    // match is very similiar to switch, we use _ stands for default situation

    enum Direction {
        East,
        West,
        North,
        South,
    }

    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    }

    // match can also assign a value
    enum IpAddr {
        Ipv4,
        Ipv6
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    // more often, we only want to care some certain value but not cover all of the possibilities.
    // under this circumstance, we can use if let (concise control flow)
    let v = Some(3u8);
    
    match v {
        Some(3) => println!("three"),
        _ => ()
    }
    // if let can do the same job here:
    if let Some(3) = v {
        println!("three");
    }

    // matches! macro
    let a = 5;
    if matches!(a, 5) {
        println!("true")
    }


    // variable match
    let age = Some(30);
    println!("age is {:?}", age);
    if let Some(age) = age {
        println!("age is {}", age); 
    }

    println!("age is {:?}", age);

    // match guard
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    option_lab();
}

fn option_lab() {
    let five = Some(5);
    plus_one(five);
    plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
