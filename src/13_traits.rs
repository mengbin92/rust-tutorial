// ============================================
// 13. Trait (特征)
// ============================================
// Trait 定义了共享的行为，类似于其他语言中的接口

fn main() {
    // ========== 基本 Trait 使用 ==========
    
    // 1. 使用实现了 Trait 的类型
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("New article available! {}", article.summarize());
    
    // ========== 默认实现 ==========
    
    // 2. 使用 Trait 的默认实现
    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("{}", tweet2.summarize());
    println!("{}", tweet2.summarize_author());
    
    // ========== Trait 作为参数 ==========
    
    // 3. 使用 impl Trait 语法
    notify(&tweet);
    notify(&article);
    
    // 4. 使用 Trait Bound 语法
    notify2(&tweet);
    
    // 5. 多个 Trait 约束
    let pair = Pair::new(3, 4);
    pair.cmp_display();
    
    // ========== 返回实现了 Trait 的类型 ==========
    
    // 6. 返回 impl Trait
    let summary = returns_summarizable();
    println!("{}", summary.summarize());
    
    // ========== 条件实现 ==========
    
    // 7. 为实现了特定 Trait 的类型实现方法
    let pair = Pair::new(5, 10);
    pair.cmp_display();
    
    // ========== 标准库中的 Trait ==========
    
    // 8. 实现标准库 Trait
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 3, y: 4 };
    let point3 = point1 + point2;
    println!("point3: {:?}", point3);
    
    // 9. 使用 Display Trait
    let point = Point { x: 1, y: 2 };
    println!("{}", point);
    
    // ========== 关联类型 ==========
    
    // 10. 使用关联类型
    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}

// ========== 定义 Trait ==========

// 1. 基本 Trait 定义
pub trait Summary {
    fn summarize(&self) -> String;
}

// 2. Trait 的默认实现
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    
    fn summarize_author(&self) -> String;
}

// ========== 实现 Trait ==========

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

impl Summary2 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

impl Summary2 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// ========== Trait 作为参数 ==========

// 3. 使用 impl Trait 语法（语法糖）
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 4. 使用 Trait Bound 语法（更明确）
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 5. 多个 Trait 约束
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Display;

// 6. 使用 where 子句（更清晰）
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

// ========== 条件实现 ==========

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只为实现了 Display 和 PartialOrd 的类型实现 cmp_display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是 x = {}", self.x);
        } else {
            println!("最大的成员是 y = {}", self.y);
        }
    }
}

// ========== 实现标准库 Trait ==========

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 实现 Add Trait 以支持 + 运算符
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 实现 Display Trait 以支持格式化输出
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ========== 关联类型 ==========

// 关联类型是 Trait 定义中的占位符类型
pub trait Iterator {
    type Item; // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // 指定关联类型
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// ========== 默认泛型类型参数 ==========

// 可以为泛型参数指定默认类型
trait Add2<Rhs = Self> {
    type Output;
    
    fn add(self, rhs: Rhs) -> Self::Output;
}

// ========== 完全限定语法 ==========

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("这是机长在说话。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("起来！");
    }
}

impl Human {
    fn fly(&self) {
        println!("*挥舞着手臂*");
    }
}

fn fly_example() {
    let person = Human;
    person.fly(); // 调用 Human 的 fly 方法
    
    // 使用完全限定语法调用特定 Trait 的方法
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// ========== 父 Trait ==========

// 一个 Trait 可以依赖于另一个 Trait（父 Trait）
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// 为 Point 实现 OutlinePrint（Point 已经实现了 Display）
impl OutlinePrint for Point {}

