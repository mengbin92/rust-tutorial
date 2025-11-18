// ============================================
// 03. 函数 (Functions)
// ============================================
// Rust 代码中的函数和变量名使用 snake_case 命名规范

fn main() {
    println!("这是 main 函数");

    // 调用函数
    another_function();
    function_with_parameters(5, 6);

    // 函数可以有返回值
    let result = add(3, 4);
    println!("3 + 4 = {}", result);

    // 使用表达式作为返回值
    let result2 = multiply(5, 6);
    println!("5 * 6 = {}", result2);

    // 提前返回
    let result3 = divide(10, 2);
    println!("10 / 2 = {}", result3);

    // 函数指针
    let func: fn(i32, i32) -> i32 = add;
    let result4 = func(7, 8);
    println!("7 + 8 = {}", result4);

    // 高阶函数示例
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
}

// 1. 基本函数定义
fn another_function() {
    println!("这是另一个函数");
}

// 2. 带参数的函数
// 参数必须显式声明类型
fn function_with_parameters(x: i32, y: i32) {
    println!("参数值: x = {}, y = {}", x, y);
}

// 3. 带返回值的函数
// 使用 -> 指定返回类型
fn add(x: i32, y: i32) -> i32 {
    return x + y; // 显式返回
}

// 4. 表达式作为返回值
// Rust 中，最后一个表达式（没有分号）会自动作为返回值
fn multiply(x: i32, y: i32) -> i32 {
    x * y // 注意：没有分号，这是一个表达式
}

// 5. 提前返回
fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0; // 提前返回
    }
    x / y // 正常返回
}

// 6. 多个返回值（使用元组）
fn calculate(x: i32, y: i32) -> (i32, i32, i32) {
    (x + y, x - y, x * y)
}

// 7. 无返回值函数
// 实际上返回单元类型 ()
fn no_return() {
    println!("这个函数没有返回值");
    // 隐式返回 ()
}

// 8. 函数作为参数
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// 9. 闭包 (Closures)
// 闭包是匿名函数，可以捕获环境中的变量
fn closure_example() {
    let x = 4;

    // 闭包语法：|参数| 表达式
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    // 闭包可以捕获变量的三种方式：
    // FnOnce: 获取所有权
    // FnMut: 可变借用
    // Fn: 不可变借用

    // 移动语义的闭包
    let s = String::from("hello");
    let takes_ownership = move || {
        println!("{}", s);
    };
    takes_ownership();
    // s 在这里已经不可用了

    // 不可变借用的闭包
    let list = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// 10. 泛型函数（将在泛型章节详细讲解）
fn identity<T>(x: T) -> T {
    x
}
