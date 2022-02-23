fn main() {
    println!("Each value in Rust can only have one owner at a time!");
    println!("When the owner goes out of scope, the value will be dropped.");

    let x = 5;
    let _y = x; // 自动copy, 将x的值拷贝一份给y, i32是基本类型

    let s1 = String::from("hello");
    let _s2 = s1;
    // let _s2 = s1.clone();
    // String is a compound value that data stored in heap.
    // 如果此时, s1和s2都是"hello" String的所有者, 那么当变量离开作用域的时候
    // Rust会drop掉其堆内存, 那么就会出现二次释放 (double drop) 的问题

    // println!("{}", s1); // error, borrow of moved value
    // recap:
    //      shallow copy in rust is acutally **move**
    //      rust will never auto does deep copy, we can use clone()


    // Rust有一个叫做Copy trait的annotation, 基本类型都是可以的

    let mut s = String::from("ownership"); // s becomes an owner, s is in scope

    s = take_ownership_then_give_back(s); // "ownership" moves to this function and then comes back

    take_ownership(s); // "ownership" moves to this function
    // no owner for "ownership"

    let x = 5;

    makes_copy(x); // 5 is copied to this function
    // still x is usable

    println!("{}", x);

    //========
    let mut s2 = String::from("world");
    change(&mut s2); // this is a mutable reference
    
    // mutable reference can only have one at a time in the same scope
    // let r1 = &mut s2;
    // let r2 = &mut s2; // raise error
    // println!("{}, {}", r1, r2);

    let len = cal_len(&s2);

    println!("The length of '{}' is {}", &s2, &len);
}
// x is out of scope, drop
// then s. however, the value of s has been moved away, thus no action

fn makes_copy(int: i32) {
    println!("{}", int);
}

fn take_ownership(str: String) {
    println!("{}", str);
} // str out of scope, dropped

fn take_ownership_then_give_back(str: String) -> String {
    println!("{}", str);
    str
}


// 我们可以通过"借用", 来减少啰里啰嗦的语句(meaning 总是使用一个值还要传来传去)
// s 将会指向传入的s2的ptr, 我们仅仅使用值
fn cal_len(s: &String) -> usize {
    s.len()
}

// 引用默认也不可变
// fn change_wrong_version(s: &String) {
//     // s.push_str("!");
// }


fn change(s: &mut String) {
    s.push_str("!");
}

// wrong function
// fn dangle() -> &String {
//     let s = String::from("wrong function");
//     &s
// } // s is leaving the scope and will be dropped, but we return a reference to it.

fn _no_dangle() -> String {
    let s = String::from("correct");
    s
}

// recap:
//      we can only have one mutable reference or several immutable references at a time
//      the references must be legal (no dangling)
