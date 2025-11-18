// ============================================
// 14. 生命周期 (Lifetimes)
// ============================================
// 生命周期是 Rust 中确保引用有效的机制

fn main() {
    // ========== 生命周期基础 ==========
    
    // 1. 生命周期确保引用有效
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串是 {}", result);
    }
    
    // ========== 生命周期注解语法 ==========
    
    // 2. 生命周期注解
    // 生命周期参数以单引号开始，通常是小写字母，如 'a
    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用
    
    // ========== 函数签名中的生命周期 ==========
    
    // 3. 函数中的生命周期注解
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
    
    // ========== 结构体中的生命周期 ==========
    
    // 4. 结构体定义中的生命周期注解
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到 '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("重要摘录: {}", i.part);
    
    // ========== 生命周期省略规则 ==========
    
    // 5. 生命周期省略规则（Lifetime Elision Rules）
    // Rust 编译器在某些情况下可以自动推断生命周期
    // 规则 1: 每个引用参数都有自己的生命周期参数
    // 规则 2: 如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数
    // 规则 3: 如果方法有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
    //         则 self 的生命周期被赋予所有输出生命周期参数
    
    // 示例：这些函数不需要显式生命周期注解
    let s = String::from("hello");
    let len = first_word(&s);
    println!("第一个单词: {}", len);
    
    // ========== 方法定义中的生命周期 ==========
    
    // 6. 方法中的生命周期注解
    let i = ImportantExcerpt {
        part: "hello world",
    };
    let announcement = "重要通知";
    let result = i.announce_and_return_part(announcement);
    println!("{}", result);
    
    // ========== 静态生命周期 ==========
    
    // 7. 'static 生命周期
    // 'static 生命周期表示整个程序的持续时间
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
    
    // ========== 泛型、Trait Bound 和生命周期 ==========
    
    // 8. 结合泛型类型参数、Trait Bound 和生命周期
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let ann = "重要通知";
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), ann);
    println!("{}", result);
    
    // ========== 生命周期子类型 ==========
    
    // 9. 生命周期子类型
    // 'a: 'b 表示 'a 的生命周期至少和 'b 一样长
    struct Context<'a>(&'a str);
    
    struct Parser<'c> {
        context: &'c Context<'c>,
    }
    
    impl<'c> Parser<'c> {
        fn parse(&self) -> Result<(), &'c str> {
            Ok(())
        }
    }
    
    // ========== 高阶生命周期 (HRTB) ==========
    
    // 10. 高阶 Trait Bound (Higher-Ranked Trait Bounds)
    // 用于处理闭包和函数指针的生命周期
    fn call_on_ref_zero<F>(f: F)
    where
        F: for<'a> Fn(&'a i32),
    {
        let zero = 0;
        f(&zero);
    }
    
    call_on_ref_zero(|x| println!("{}", x));
}

// ========== 函数中的生命周期 ==========

// 1. 带生命周期注解的函数
// 这个函数返回的引用的生命周期与两个参数中较短的那个相同
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ========== 结构体中的生命周期 ==========

// 2. 结构体中的生命周期注解
// 这个结构体有一个字段 part，它存放了一个字符串 slice 的引用
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 3. 为结构体实现方法
impl<'a> ImportantExcerpt<'a> {
    // 规则 3 适用：因为 &self 是参数，返回值的生命周期被赋予 &self 的生命周期
    fn level(&self) -> i32 {
        3
    }
    
    // 显式生命周期注解
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意! {}", announcement);
        self.part
    }
}

// ========== 生命周期省略示例 ==========

// 4. 生命周期省略规则应用
// 规则 1: 每个参数都有自己的生命周期
// fn first_word<'a>(s: &'a str) -> &str
// 规则 2: 只有一个输入生命周期，所以输出生命周期被赋予输入生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// ========== 泛型和生命周期结合 ==========

use std::fmt::Display;

// 5. 结合泛型类型参数、Trait Bound 和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("通知! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ========== 多个生命周期参数 ==========

// 6. 多个生命周期参数
// 如果函数有多个引用参数，可以指定不同的生命周期
fn longest_of_three<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a, // 'b 的生命周期至少和 'a 一样长
{
    if x.len() > y.len() {
        x
    } else {
        x // 必须返回 'a 生命周期的引用
    }
}

// ========== 生命周期和可变性 ==========

// 7. 可变引用和生命周期
fn change_string<'a>(s: &'a mut String) {
    s.push_str(" changed");
}

// ========== 生命周期和迭代器 ==========

// 8. 迭代器中的生命周期
fn first_word_iter<'a>(words: &'a [&str]) -> Option<&'a str> {
    words.first().copied()
}

// ========== 常见生命周期错误 ==========

// 9. 避免悬垂引用
// 下面的代码会编译错误，因为返回的引用可能指向无效数据
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 错误！s 在这里离开作用域
// }

// 正确的做法是返回拥有所有权的值
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回所有权
}

