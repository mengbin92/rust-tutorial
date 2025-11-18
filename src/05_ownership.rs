// ============================================
// 05. 所有权 (Ownership)
// ============================================
// 所有权是 Rust 最独特的特性，它让 Rust 无需垃圾回收就能保证内存安全

fn main() {
    // ========== 所有权规则 ==========
    // 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量
    // 2. 值在任一时刻有且只有一个所有者
    // 3. 当所有者（变量）离开作用域，这个值将被丢弃

    // ========== 变量作用域 ==========
    {
        let s = String::from("hello"); // s 在这里开始有效
        // 使用 s
    } // s 在这里离开作用域，内存被释放

    // ========== 移动 (Move) ==========

    // 2. 对于堆上分配的数据，赋值会移动所有权
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被移动到 s2
    // println!("{}", s1); // 错误！s1 不再有效

    println!("s2 = {}", s2);

    // 3. 对于栈上分配的数据（实现了 Copy trait），会进行复制
    let x = 5;
    let y = x; // x 的值被复制到 y
    println!("x = {}, y = {}", x, y); // x 仍然有效

    // Copy trait 的类型包括：
    // - 所有整数类型
    // - 布尔类型
    // - 浮点类型
    // - 字符类型
    // - 只包含 Copy 类型的元组

    // ========== 克隆 (Clone) ==========

    // 4. 如果需要深度复制堆上的数据，使用 clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深度复制
    println!("s1 = {}, s2 = {}", s1, s2); // 两者都有效

    // ========== 所有权和函数 ==========

    // 5. 将值传递给函数会移动或复制
    let s = String::from("hello");
    takes_ownership(s); // s 的所有权被移动
    // println!("{}", s); // 错误！s 不再有效

    let x = 5;
    makes_copy(x); // x 被复制
    println!("x = {}", x); // x 仍然有效

    // 6. 返回值也可以转移所有权
    let s1 = gives_ownership(); // 获得所有权
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 被移动，然后返回

    println!("s1 = {}, s3 = {}", s1, s3);

    // ========== 引用和借用 (References & Borrowing) ==========

    // 7. 使用引用，不获取所有权
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递引用
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然有效

    // 8. 可变引用
    let mut s = String::from("hello");
    change(&mut s); // 传递可变引用
    println!("s = {}", s);

    // 9. 引用规则：
    // - 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
    // - 引用必须总是有效的

    let mut s = String::from("hello");
    let r1 = &s; // 不可变引用
    let r2 = &s; // 不可变引用
    // let r3 = &mut s; // 错误！不能同时有可变和不可变引用
    println!("{}, {}", r1, r2);

    // r1 和 r2 的作用域在这里结束
    let r3 = &mut s; // 现在可以了
    println!("{}", r3);

    // 10. 悬垂引用（Dangling References）
    // Rust 编译器会防止悬垂引用
    // let reference_to_nothing = dangle(); // 编译错误

    // ========== 切片 (Slices) ==========

    // 11. 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5]; // 或 &s[..5]
    let world = &s[6..11]; // 或 &s[6..]
    let whole = &s[..]; // 整个字符串

    println!("hello: {}, world: {}, whole: {}", hello, world, whole);

    // 12. 字符串字面量就是切片
    let s: &str = "Hello, world!";

    // 13. 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
    println!("slice: {:?}", slice);
}

// 函数获取所有权
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 在这里离开作用域，内存被释放

// 函数复制值
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer 在这里离开作用域，但因为是 Copy 类型，没有特殊操作

// 函数返回所有权
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回并移动所有权
}

// 函数获取并返回所有权
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回并移动所有权
}

// 使用引用（借用）
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为只是引用，不会释放内存

// 使用可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 这个函数会产生悬垂引用（编译错误）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回 s 的引用，但 s 在这里离开作用域
// } // s 在这里被释放，引用无效
