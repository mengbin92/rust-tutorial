// ============================================
// 16. Rust 关键字详解
// ============================================
// Rust 关键字是语言保留的标识符，不能用作变量名、函数名等

fn main() {
    println!("Rust 关键字示例");

    // ========== 声明和定义关键字示例 ==========

    // 2. let - 声明变量（默认不可变）
    let x = 5;
    let mut y = 10; // mut 使变量可变

    // 3. const - 声明常量（必须显式指定类型，编译时确定值）
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // 4. static - 声明静态变量（生命周期为整个程序）
    static LANGUAGE: &str = "Rust";
    println!("LANGUAGE: {}", LANGUAGE);

    // ========== 控制流关键字示例 ==========

    // 11. if - 条件语句
    if x > 5 {
        println!("x 大于 5");
    }

    // 12. else - else 分支
    if x > 5 {
        println!("大于 5");
    } else {
        println!("小于等于 5");
    }

    // 13. match - 模式匹配
    match x {
        1 => println!("一"),
        2 => println!("二"),
        _ => println!("其他"),
    }

    // 14. loop - 无限循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2; // 使用 break 退出并返回值
        }
    };
    println!("loop 结果: {}", result);

    // 15. while - while 循环
    let mut i = 0;
    while i < 3 {
        println!("while: {}", i);
        i += 1;
    }

    // 16. for - for 循环
    for j in 0..3 {
        println!("for: {}", j);
    }

    // 17. break - 退出循环
    loop {
        break; // 退出循环
    }

    // 18. continue - 跳过当前循环迭代
    for k in 0..5 {
        if k % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("continue: {}", k);
    }

    // ========== 所有权和借用关键字示例 ==========

    // 20. move - 强制闭包获取所有权
    let s = String::from("hello");
    let closure = move || {
        println!("move 闭包: {}", s); // s 被移动到闭包中
    };
    closure();

    // 21. ref - 在模式匹配中获取引用
    let value = 5;
    match value {
        ref r => println!("ref 引用: {:p}", r),
    }

    // 22. mut - 使变量或引用可变
    let mut x = 5;
    let r = &mut x; // 可变引用
    *r += 1;
    println!("mut 可变引用: {}", x);

    // ========== 类型相关关键字示例 ==========

    // 23. as - 类型转换
    let integer = 65;
    let character = integer as u8 as char;
    println!("as 类型转换: {} -> {}", integer, character);

    // ========== 实际使用示例 ==========

    let point = Point::new(1, 2);
    point.display();

    // 模式匹配
    match point.x {
        1 => println!("x 是 1"),
        _ => println!("x 不是 1"),
    }

    // 循环
    for i in 0..5 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }

    // 使用 dyn Trait
    let obj: &dyn Printable = &point;
    obj.print();
}

// ========== 声明和定义关键字 ==========

// 1. fn - 定义函数
fn my_function() {
    println!("这是一个函数");
}

// 5. struct - 定义结构体
struct Person {
    name: String,
    age: u32,
}

// 6. enum - 定义枚举
enum Status {
    Active,
    Inactive,
}

// 7. impl - 为类型实现方法或 Trait
impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

// 8. trait - 定义 Trait（特征）
trait Printable {
    fn print(&self);
}

// 9. type - 定义类型别名
type Kilometers = i32;

// 10. mod - 定义模块
mod my_module {
    pub fn function() {
        println!("模块函数");
    }
}

// 19. return - 返回函数值（通常可以省略）
fn add(a: i32, b: i32) -> i32 {
    return a + b; // 可以省略 return，直接写 a + b
}

// ========== 类型相关关键字 ==========

// 24. where - 指定泛型约束
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Clone,
    U: Copy,
{
    0
}

// 25. dyn - 动态分发 Trait 对象
fn print_trait(obj: &dyn Printable) {
    obj.print();
}

// ========== 模块和可见性关键字 ==========

// 26. pub - 使项变为公有
pub struct PublicStruct {
    pub field: i32,
}

// 27. use - 引入模块或项
use std::collections::HashMap;

// 28. crate - 当前 crate 的根模块
fn use_crate() {
    crate::my_module::function();
}

// 29. super - 父模块（在子模块中使用）
// super::parent_function();

// 30. self - 当前模块或方法接收者
impl Person {
    fn introduce(&self) {
        println!("我是 {}", self.name);
    }
}

// 31. Self - 当前类型（大写）
impl Person {
    fn new_person() -> Self {
        // Self 指代 Person
        Person {
            name: String::new(),
            age: 0,
        }
    }
}

// ========== 异步和并发关键字 ==========

// 32. async - 定义异步函数
async fn fetch_data() -> String {
    String::from("data")
}

// 33. await - 等待异步操作完成（只能在 async 函数中使用）
// async fn example_async() {
//     let result = fetch_data().await;
// }

// ========== 错误处理关键字 ==========

// 34. try - 已废弃，使用 ? 运算符代替
// fn try_example() -> Result<i32, &'static str> {
//     let x = try!(some_function()); // 已废弃
//     Ok(x)
// }

// ? 运算符用于传播错误
fn example() -> Result<i32, &'static str> {
    let x = some_result_function()?; // 如果错误，自动返回
    Ok(x)
}

// ========== 生命周期关键字 ==========

// 35. 'a, 'b, 'c... - 生命周期参数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 36. 'static - 静态生命周期
fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

// ========== 泛型关键字 ==========

// 37. impl - 在函数签名中使用（impl Trait 语法）
fn notify(item: &impl Printable) {
    item.print();
}

// ========== 属性关键字 ==========

// 38. derive - 派生宏（通过属性使用）
#[derive(Debug, PartialEq)]
struct MyStruct {
    value: i32,
}

// ========== 测试关键字 ==========

// 39. test - 测试函数（通过属性使用）
#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}

// ========== 条件编译关键字 ==========

// 40. cfg - 条件编译（通过属性使用）
#[cfg(target_os = "linux")]
fn linux_only_function() {}

// ========== 其他关键字 ==========

// 41. extern - 声明外部函数或变量
extern "C" {
    fn abs(input: i32) -> i32;
}

// 42. unsafe - 标记不安全代码块
fn unsafe_example() {
    unsafe {
        // 不安全的代码
        // 例如：调用外部函数、解引用裸指针等
    }
}

// 43. union - 定义联合体（不安全）
union MyUnion {
    f1: u32,
    f2: f32,
}

// 44. extern crate - 引入外部 crate（Rust 2018+ 中通常不需要）
// extern crate serde;

// ========== 宏关键字 ==========

// 45. macro_rules! - 定义声明宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// ========== 未使用的关键字（保留） ==========

// 以下关键字目前未使用，但被保留，不能用作标识符：
// abstract, become, box, do, final, macro, override, priv, try, typeof,
// unsized, virtual, yield, async, await

// ========== 关键字分类总结 ==========

/*
========== 严格关键字（Strict Keywords）==========

1. 声明和定义（10个）：
   - fn: 定义函数
   - let: 声明变量
   - const: 声明常量
   - static: 声明静态变量
   - struct: 定义结构体
   - enum: 定义枚举
   - impl: 实现方法或 Trait
   - trait: 定义 Trait
   - type: 定义类型别名
   - mod: 定义模块

2. 控制流（9个）：
   - if: 条件语句
   - else: else 分支
   - match: 模式匹配
   - loop: 无限循环
   - while: while 循环
   - for: for 循环
   - break: 退出循环
   - continue: 跳过当前迭代
   - return: 返回函数值

3. 所有权和借用（3个）：
   - move: 强制闭包获取所有权
   - ref: 在模式匹配中获取引用
   - mut: 使变量或引用可变

4. 类型相关（3个）：
   - as: 类型转换
   - where: 指定泛型约束
   - dyn: 动态分发 Trait 对象（Rust 2018+）

5. 模块和可见性（6个）：
   - pub: 使项变为公有
   - use: 引入模块或项
   - crate: 当前 crate 的根模块
   - super: 父模块
   - self: 当前模块或方法接收者
   - Self: 当前类型（大写）

6. 异步（2个，Rust 2018+）：
   - async: 定义异步函数
   - await: 等待异步操作完成

7. 其他（4个）：
   - extern: 声明外部函数或变量
   - unsafe: 标记不安全代码块
   - in: 用于 for 循环中，表示迭代范围
   - false/true: 布尔值字面量

========== 保留关键字（Reserved Keywords）==========

这些关键字目前未使用，但被保留，不能用作标识符：
- abstract: 抽象（保留）
- become: 保留
- box: 保留（曾经用于堆分配）
- do: 保留
- final: 保留
- macro: 保留（用于宏定义）
- override: 保留
- priv: 保留（曾经用于私有可见性）
- try: 保留（曾经用于错误处理，现用 ? 运算符）
- typeof: 保留
- unsized: 保留
- virtual: 保留
- yield: 保留

========== 弱关键字（Weak Keywords）==========

这些关键字仅在特定上下文中具有特殊含义：
- macro_rules: 用于定义宏规则
- union: 用于定义联合体（不安全）
- 'static: 表示静态生命周期

========== 特殊运算符（虽然不是关键字，但很重要）==========

- ?: 错误传播运算符
- :: 路径分隔符
- -> 函数返回类型
- => match 分支
- @ 绑定模式
- _ 通配符模式

========== 常用关键字使用频率 ==========

最常用（几乎每个程序都会用到）：
- fn, let, mut, if, else, match, return, struct, enum, impl

常用（经常使用）：
- pub, use, mod, for, while, loop, break, continue, trait, Self, self

中等使用：
- const, static, as, where, dyn, move, ref, async, await

较少使用：
- extern, unsafe, union, type, crate, super, in

========== 关键字使用建议 ==========

1. 不要将关键字用作标识符（变量名、函数名等）
2. 如果必须使用关键字作为标识符，可以使用原始标识符：r#fn
3. 了解关键字的分类有助于理解 Rust 的语法结构
4. 某些关键字（如 async/await）只在特定上下文中有效
*/

// ========== 实际使用示例 ==========

// 综合示例：使用多个关键字
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn display(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

// 为 Point 实现 Printable trait
impl Printable for Point {
    fn print(&self) {
        println!("Point: ({}, {})", self.x, self.y);
    }
}

fn some_result_function() -> Result<i32, &'static str> {
    Ok(42)
}
