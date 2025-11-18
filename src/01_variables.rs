// ============================================
// 01. 变量 (Variables)
// ============================================
// Rust 中的变量默认是不可变的（immutable），这是 Rust 安全性的重要特性之一

fn main() {
    // 1. 基本变量声明
    // 使用 let 关键字声明变量，默认不可变
    let x = 5;
    println!("x = {}", x);

    // 下面的代码会编译错误，因为 x 是不可变的
    // x = 6; // error: cannot assign twice to immutable variable

    // 2. 可变变量 (Mutable Variables)
    // 使用 mut 关键字声明可变变量
    let mut y = 10;
    println!("y = {}", y);
    y = 20; // 可以修改
    println!("y = {}", y);

    // 3. 变量遮蔽 (Variable Shadowing)
    // 可以使用相同的名字重新声明变量，新变量会遮蔽旧的变量
    let spaces = "   ";
    let spaces = spaces.len(); // 类型从 &str 变为 usize
    println!("spaces = {}", spaces);

    // 4. 常量 (Constants)
    // 使用 const 关键字声明常量，必须显式指定类型
    // 常量在编译时确定值，且必须大写命名
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    // 5. 静态变量 (Static Variables)
    // 使用 static 关键字声明静态变量，生命周期为整个程序
    static LANGUAGE: &str = "Rust";
    println!("LANGUAGE = {}", LANGUAGE);

    // 6. 类型推断
    // Rust 编译器可以自动推断变量类型
    let integer = 42; // 推断为 i32
    let float = 3.14; // 推断为 f64
    let boolean = true; // 推断为 bool
    let character = 'R'; // 推断为 char

    println!(
        "integer: {}, float: {}, boolean: {}, character: {}",
        integer, float, boolean, character
    );

    // 7. 显式类型注解
    let explicit_int: i64 = 100;
    let explicit_float: f32 = 2.5;
    println!(
        "explicit_int: {}, explicit_float: {}",
        explicit_int, explicit_float
    );

    // 8. 未使用的变量
    // 如果变量未使用，编译器会警告，可以使用下划线前缀来避免警告
    let _unused = 99; // 不会产生警告

    // 9. 变量作用域
    {
        let scoped_var = 50;
        println!("scoped_var = {}", scoped_var);
    }
    // scoped_var 在这里已经不可访问
    // println!("{}", scoped_var); // 编译错误

    // 10. 变量绑定
    // let 关键字实际上是进行模式匹配（pattern matching）
    let (a, b) = (1, 2);
    println!("a = {}, b = {}", a, b);

    // 11. 可变性 vs 遮蔽
    let mut count = 0;
    count += 1; // 使用 mut 修改值

    let count = 0;
    let count = count + 1; // 使用遮蔽创建新变量
    println!("count = {}", count);
}
