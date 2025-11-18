// ============================================
// 12. 泛型 (Generics)
// ============================================
// 泛型允许我们编写可以处理多种数据类型的代码

fn main() {
    // ========== 函数中的泛型 ==========

    // 1. 泛型函数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大数字: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大字符: {}", result);

    // ========== 结构体中的泛型 ==========

    // 2. 泛型结构体
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed_point = Point2 { x: 5, y: 4.0 };

    println!("integer_point: {:?}", integer_point);
    println!("float_point: {:?}", float_point);
    println!("mixed_point: {:?}", mixed_point);

    // 3. 使用泛型方法
    println!("integer_point.x = {}", integer_point.x());
    println!(
        "float_point.distance_from_origin() = {}",
        float_point.distance_from_origin()
    );

    // ========== 枚举中的泛型 ==========

    // 4. 泛型枚举（标准库中的例子）
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!(
        "some_number: {:?}, some_string: {:?}, absent_number: {:?}",
        some_number, some_string, absent_number
    );

    // ========== 方法中的泛型 ==========

    // 5. 为泛型结构体实现方法
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // 6. 为特定类型实现方法
    let p = Point { x: 5.0, y: 10.0 };
    println!("距离原点: {}", p.distance_from_origin());

    // ========== 性能 ==========

    // 7. 单态化（Monomorphization）
    // Rust 在编译时会将泛型代码转换为具体类型的代码
    // 这意味着使用泛型没有运行时开销

    // ========== Trait 约束 ==========

    // 8. 使用 Trait 约束泛型
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_with_trait(&number_list);
    println!("最大数字: {}", result);

    // ========== 多个 Trait 约束 ==========

    // 9. 多个 Trait 约束
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    // ========== 生命周期参数 ==========

    // 10. 泛型和生命周期一起使用
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest_with_an_announcement(&string1, &string2, "重要通知");
        println!("最长的字符串: {}", result);
    }
}

// ========== 泛型函数 ==========

// 1. 基本泛型函数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 2. 使用 Trait 约束的泛型函数
fn largest_with_trait<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// ========== 泛型结构体 ==========

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 为泛型结构体实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// ========== Trait 约束示例 ==========

// 定义 Trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Display {
    fn display(&self) -> String;
}

// 实现 Trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 使用 Trait 作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 使用 Trait Bound 语法（更明确）
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 多个 Trait 约束
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item.display());
}

// 使用 where 子句（更清晰）
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}

// ========== 返回实现了 Trait 的类型 ==========

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// ========== 使用 Trait Bound 有条件地实现方法 ==========

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是 x = {}", self.x);
        } else {
            println!("最大的成员是 y = {}", self.y);
        }
    }
}

// ========== 泛型和生命周期一起使用 ==========

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("通知! {}", ann);
    if x.len() > y.len() { x } else { y }
}
