fn main() {

    let post = Post{title: "Rust".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "weibo is trash".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
    println!("{}",post.preview());


    let p = Pair { x: 3, y: 4 };
    p.cmp_display();


    println!("weibo: {:#?}", weibo);
}


pub trait Summary {
    fn summarize(&self) -> String;
}

trait Preview {
    // default implementation
    fn preview(&self) -> String {
        String::from("Read more")
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// 孤儿规则, to implement this, there must be 
// at least one Summary or Post exists in current scope
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("article: {}, author: {}", self.title, self.author)
    }
}

impl Preview for Post {
    // use the default implementation
    // we can also reload the preview method here
}

#[derive(Debug)]
pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} post a weibo{}", self.username, self.content)
    }
}

// use trait as the function argument
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}
// |
// |    upper is the syntatic sugar of the below version
// V
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking News! {}", item.summarize());
// }

// if we have multi traits need to be bound
// fn notify2(item: &(impl Summary + Preview)) {}

// use std::fmt::{Display, Debug};


// if we have a lot of traits....
// fn some_function<T, U>(_t: &T, _u: &U) where T: Display + Clone, U: Clone + Debug {}

use std::fmt::Display;


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}


impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl<T> Pair<T> {
    // fn cmp_display(&self) {
    //     println!("not implemented yet")
    // }
}
