#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    let s1 = String::from("Waynex");

    // so that is why a str can not change the value
    // because &str is an immutable reference
    let s = "hello world"; // &str 字符串切片类型

    let hello = &s[..5]; // 0..5
    let world = &s[6..]; // 6..$len
    // caution: 切片所以必须落在字符之间的边界位置, 中文UTF8占用三个字节
    // panic code:
    //      let s = "中国";
    //      let a = &s[0..2]; // panic

    greeting(&s1);

    let word = first_word(&s1);

    assert_eq!(hello, &s[..5]); // ok
    assert_eq!(hello, "hello"); // ok
    
    string_lab();
    tuple_lab();
    struct_lab();
    enum_lab();
    array_lab();
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn greeting(name: &String) {
    println!("Hello, {}!", &name);
}


fn string_lab() {
    let mut s = String::new();
    s.push_str("hello,world");
    s.push('!');
    assert_eq!(s, "hello,world!");

    let mut s = "hello,world".to_string();
    s.push('!');
    assert_eq!(s, "hello,world!");

    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    // let s3 = s1 + s2; // mismatched types. why?
    /*
    impl Add<&str> for String {
    type Output = String;

    #[inline]
    fn add(mut self, other: &str) -> String {
        self.push_str(other);
        self
    }
    */
    // so we should use:
    let s3 = s1 + &s2;

    // Still, add takes the value of s1
    // so s1 is no longer accessible
    // println!("{}", s1); // error

    // String ==> &str
    let s = String::from("hello,world!");
    say_hello(&s); // why?  **deref**


    /*
    pub struct String {
        vec: Vec<u8>,
    }
    */
    let s = String::from("中文");
    for u in s.bytes() {
        print!("{} ", &u);
    }
    // so it is not possible to access &s[0]
    // we should use chars()
    println!();
    for c in s.chars() {
        print!("{} ", &c);
    }
    println!();
}

fn say_hello(s: &str) {
    println!("{}!", s);
}

fn tuple_lab() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is:{}", y);
    println!("The value of y is:{}", tup.1); // meaning? u will be confused. here comes struct
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn struct_lab() {
    // each field should be initialized when create the struct
    let user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        sign_in_count: 0
    };

    // update a struct:
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // can simplify it by:
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // noted here a ownership transfer happened, user1 and user1.username
    }; // bool and u64 can copy, but String not

    println!("{}", user1.active); // ok
    // println!("{}", user1.username); // wrong code
    // println!("{}", user1); // wrong code

    // we can also have a tuple struct
    struct Color(u8, u8, u8);
    let black = Color(0,0,0);
    let white = Color(255,255,255);
}

// or, we can simplify the struct creation by:

// fn build_user(email: String, username: String) -> User {
//     User { active: true, username: username, email: email, sign_in_count: 1 }
// }

// can simplify it if the field has same name with func args:

fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}

// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }

// We can link the value type to the member of enum
enum PokerSuit {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(u8),
}

fn enum_lab() {
    let heart = PokerSuit::Hearts(5);
    let diamond = PokerSuit::Diamonds('A');

    print_suit(heart);
}

fn print_suit(card: PokerSuit) {
    // println!("{}", card);
}

// pub enum Option<T> {
//     /// No value
//     #[lang = "None"]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     None,
//     /// Some value `T`
//     #[lang = "Some"]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
// }

fn option_lab() {
    let some_number = Some(5);
    let some_string = Some("string");
    // let absent_number = None; // error, we need to declare the type explicitly
    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);
    // x + y // error, cannot add Option<Integer> to Integer
    let six = plus_one(y);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}


// array - fixed length - stack
// vector - dynamicly expandable - heap
fn array_lab() {
    let a = [1,2,2,3,4,5];
    let b: [u8; 3] = [1,2,3];
    let repeat_3_for_5_times = [3; 5];

    for number in &a {
        print!("{}:", &number);
    }
    println!();
    for n in a.iter() {
        print!("{}:", &n);
    }
    println!();
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i]
    }
    println!("a.sum = {}", &sum);
}
