// ============================================
// 04. 流程控制 (Control Flow)
// ============================================

fn main() {
    // ========== if 表达式 ==========

    // 1. 基本的 if 语句
    let number = 3;
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    // 2. if-else if-else
    let number = 6;
    if number % 4 == 0 {
        println!("数字可以被 4 整除");
    } else if number % 3 == 0 {
        println!("数字可以被 3 整除");
    } else if number % 2 == 0 {
        println!("数字可以被 2 整除");
    } else {
        println!("数字不能被 4、3 或 2 整除");
    }

    // 3. if 作为表达式使用
    // 注意：if 的每个分支必须返回相同类型的值
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);

    // ========== 循环 ==========

    // 4. loop 循环（无限循环）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 可以返回值
        }
    };
    println!("result = {}", result);

    // 5. while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 6. for 循环遍历集合
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值为: {}", element);
    }

    // 7. for 循环使用范围
    // (1..4) 生成 1, 2, 3
    // (1..=4) 生成 1, 2, 3, 4
    for number in 1..4 {
        println!("{}!", number);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // ========== match 表达式 ==========

    // 8. match 基本用法
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"), // 默认分支
    }

    // 9. match 返回值
    let number = 2;
    let description = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "其他",
    };
    println!("描述: {}", description);

    // 10. match 多个值
    let number = 5;
    match number {
        1 | 2 | 3 => println!("小"),
        4 | 5 | 6 => println!("中"),
        7 | 8 | 9 => println!("大"),
        _ => println!("其他"),
    }

    // 11. match 范围匹配
    let number = 42;
    match number {
        1..=10 => println!("1 到 10"),
        11..=20 => println!("11 到 20"),
        21..=50 => println!("21 到 50"),
        _ => println!("其他"),
    }

    // 12. match 解构
    let point = (0, 5);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在 y 轴上，y = {}", y),
        (x, 0) => println!("在 x 轴上，x = {}", x),
        (x, y) => println!("({}, {})", x, y),
    }

    // ========== if let ==========

    // 13. if let 语法糖
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

    // ========== while let ==========

    // 14. while let 循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ========== 标签循环 ==========

    // 15. 循环标签（用于 break 和 continue）
    'outer: loop {
        println!("进入外层循环");
        'inner: loop {
            println!("进入内层循环");
            break 'outer; // 跳出外层循环
        }
        println!("这行不会执行");
    }
    println!("退出外层循环");

    // ========== continue ==========

    // 16. continue 语句
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("{}", i);
    }

    // ========== 守卫 (Guards) ==========

    // 17. match 守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("大于等于 5: {}", x),
        None => (),
    }

    // ========== @ 绑定 ==========

    // 18. @ 绑定（在模式匹配中绑定值）
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("找到范围内的 id: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("找到另一个范围内的 id")
        }
        Message::Hello { id } => {
            println!("找到其他 id: {}", id)
        }
    }
}
