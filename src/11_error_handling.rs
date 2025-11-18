// ============================================
// 11. 错误处理 (Error Handling)
// ============================================
// Rust 将错误分为两大类：可恢复错误和不可恢复错误

use std::fs::File;
use std::io::{self, Read};
use std::num;

fn main() {
    // ========== panic! 宏（不可恢复错误） ==========

    // 1. 使用 panic! 宏
    // panic!("crash and burn");

    // 2. 使用 RUST_BACKTRACE=1 环境变量查看回溯
    // let v = vec![1, 2, 3];
    // v[99]; // 会 panic

    // ========== Result<T, E>（可恢复错误） ==========

    // 3. Result 枚举
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // 4. 处理 Result
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("打开文件时出错: {:?}", error);
        }
    };

    // 5. 匹配不同的错误
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件时出错: {:?}", e),
            },
            other_error => {
                panic!("打开文件时出错: {:?}", other_error);
            }
        },
    };

    // ========== unwrap 和 expect ==========

    // 6. unwrap：如果 Result 是 Ok，返回 Ok 中的值；如果是 Err，调用 panic!
    // let f = File::open("hello.txt").unwrap();

    // 7. expect：类似 unwrap，但可以指定错误消息
    // let f = File::open("hello.txt").expect("无法打开 hello.txt");

    // ========== 传播错误 ==========

    // 8. 使用 ? 运算符传播错误
    // ? 运算符只能用于返回 Result 或 Option 的函数
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("错误: {}", e),
    }

    // 9. 使用 ? 运算符的简化版本
    match read_username_from_file_simple() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("错误: {}", e),
    }

    // ========== 自定义错误类型 ==========

    // 10. 定义自定义错误类型
    #[derive(Debug)]
    enum MyError {
        Io(io::Error),
        Parse(num::ParseIntError),
    }

    impl From<io::Error> for MyError {
        fn from(error: io::Error) -> Self {
            MyError::Io(error)
        }
    }

    impl From<num::ParseIntError> for MyError {
        fn from(error: num::ParseIntError) -> Self {
            MyError::Parse(error)
        }
    }

    // ========== Option<T> 的错误处理 ==========

    // 11. Option 也可以使用 ? 运算符
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    match last_char_of_first_line("hello\nworld") {
        Some(c) => println!("第一个字符: {}", c),
        None => println!("没有找到"),
    }

    // ========== 错误处理最佳实践 ==========

    // 12. 何时使用 panic!
    // - 示例、原型代码、测试
    // - 确定不会发生的错误
    // - 程序状态损坏，无法恢复

    // 13. 何时返回 Result
    // - 错误是预期的，应该被处理
    // - 调用者可能有处理错误的方法

    // ========== 实际示例 ==========

    // 14. 链式调用和错误处理
    match divide(10, 2) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }
}

// ========== 传播错误示例 ==========

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符简化
fn read_username_from_file_simple() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更简洁的版本
fn read_username_from_file_simpler() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// ========== 自定义错误示例 ==========

#[derive(Debug)]
enum DivisionError {
    DivisionByZero,
}

fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// ========== 错误处理模式 ==========

// 15. 使用 map 和 map_err 转换错误
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("解析错误: {}", e))
}

// 16. 使用 and_then 链式操作
fn double_parse(s: &str) -> Result<i32, String> {
    parse_number(s).and_then(|n| Ok(n * 2))
}

// 17. 使用 or_else 提供默认值
fn get_number(s: &str) -> Result<i32, String> {
    parse_number(s).or_else(|_| Ok(0)) // 如果失败，返回 0
}
