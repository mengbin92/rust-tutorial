// ============================================
// 08. 模式匹配 (Pattern Matching)
// ============================================
// Rust 的模式匹配功能非常强大，主要用于 match 表达式和 if let

fn main() {
    // ========== match 表达式 ==========

    // 1. 基本 match 用法
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"), // 默认分支，必须覆盖所有可能
    }

    // 2. match 返回值
    let number = 2;
    let text = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "其他",
    };
    println!("{}", text);

    // ========== 匹配 Option ==========

    // 3. 匹配 Option<T>
    let some_number = Some(5);
    let result = match some_number {
        Some(i) => i * 2,
        None => 0,
    };
    println!("result: {}", result);

    // 4. 匹配嵌套的 Option
    let some_value = Some(Some(42));
    match some_value {
        Some(Some(v)) => println!("嵌套值: {}", v),
        Some(None) => println!("内部是 None"),
        None => println!("外部是 None"),
    }

    // ========== 匹配 Result ==========

    // 5. 匹配 Result<T, E>
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }

    // ========== 多个值匹配 ==========

    // 6. 使用 | 匹配多个值
    let number = 5;
    match number {
        1 | 2 | 3 => println!("小"),
        4 | 5 | 6 => println!("中"),
        7 | 8 | 9 => println!("大"),
        _ => println!("其他"),
    }

    // ========== 范围匹配 ==========

    // 7. 使用 ..= 匹配范围
    let number = 42;
    match number {
        1..=10 => println!("1 到 10"),
        11..=20 => println!("11 到 20"),
        21..=50 => println!("21 到 50"),
        _ => println!("其他"),
    }

    // ========== 解构 ==========

    // 8. 解构元组
    let point = (0, 5);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在 y 轴上，y = {}", y),
        (x, 0) => println!("在 x 轴上，x = {}", x),
        (x, y) => println!("({}, {})", x, y),
    }

    // 9. 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y } => println!("({}, {})", x, y),
    }

    // 10. 解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("改变颜色: RGB({}, {}, {})", r, g, b);
        }
    }

    // ========== 守卫 (Guards) ==========

    // 11. match 守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("大于等于 5: {}", x),
        None => (),
    }

    // 12. 多个守卫条件
    let point = (4, 5);
    match point {
        (x, y) if x == y => println!("在对角线上"),
        (x, y) if x > y => println!("x > y"),
        (x, y) if x < y => println!("x < y"),
        _ => (),
    }

    // ========== @ 绑定 ==========

    // 13. @ 绑定：在匹配模式时绑定值
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("找到范围内的 id: {}", id_variable)
        }
        Message2::Hello { id: 10..=12 } => {
            println!("找到另一个范围内的 id")
        }
        Message2::Hello { id } => {
            println!("找到其他 id: {}", id)
        }
    }

    // ========== if let ==========

    // 14. if let 语法糖
    // 用于只关心一个匹配分支的情况
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("值是 3");
    }

    // 等价于：
    match some_value {
        Some(3) => println!("值是 3"),
        _ => (),
    }

    // 15. if let else
    let some_number = Some(7);
    if let Some(i) = some_number {
        println!("匹配到: {}", i);
    } else {
        println!("没有匹配");
    }

    // ========== while let ==========

    // 16. while let 循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ========== for 循环中的模式匹配 ==========

    // 17. for 循环解构
    let v = vec![(1, 2), (3, 4), (5, 6)];
    for (x, y) in v {
        println!("({}, {})", x, y);
    }

    // ========== let 语句中的模式匹配 ==========

    // 18. let 语句使用模式
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 19. 忽略值
    let (x, _, z) = (1, 2, 3); // 忽略中间的值
    println!("x: {}, z: {}", x, z);

    // 20. 忽略多个值
    let (x, ..) = (1, 2, 3, 4, 5);
    println!("x: {}", x);

    // ========== 匹配引用 ==========

    // 21. 匹配引用
    let reference = &4;
    match reference {
        &val => println!("通过解构获得值: {}", val),
    }

    // 22. 使用 ref 关键字
    let value = 5;
    match value {
        ref r => println!("获得引用: {:p}", r),
    }

    // ========== 可反驳和不可反驳模式 ==========

    // 23. 不可反驳模式（总是匹配成功）
    // let 语句使用不可反驳模式
    let x = 5; // 总是成功

    // 24. 可反驳模式（可能匹配失败）
    // if let 使用可反驳模式
    if let Some(x) = Some(5) {
        println!("{}", x);
    }
}
