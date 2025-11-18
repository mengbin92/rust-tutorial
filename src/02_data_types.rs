// ============================================
// 02. 数据类型 (Data Types)
// ============================================
// Rust 是静态类型语言，编译时必须知道所有变量的类型

fn main() {
    // ========== 标量类型 (Scalar Types) ==========

    // 1. 整数类型 (Integer Types)
    // 有符号整数: i8, i16, i32, i64, i128, isize
    // 无符号整数: u8, u16, u32, u64, u128, usize
    let small: i8 = -128;
    let large: u64 = 18_446_744_073_709_551_615;
    let default: i32 = 42; // 默认整数类型是 i32
    let size: isize = 100; // 取决于架构（32位或64位）

    println!(
        "small: {}, large: {}, default: {}, size: {}",
        small, large, default, size
    );

    // 整数字面量可以使用不同进制
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // 仅限 u8

    println!(
        "decimal: {}, hex: {}, octal: {}, binary: {}, byte: {}",
        decimal, hex, octal, binary, byte
    );

    // 2. 浮点类型 (Floating-Point Types)
    // f32: 32位浮点数（单精度）
    // f64: 64位浮点数（双精度，默认）
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {}, y: {}", x, y);

    // 3. 布尔类型 (Boolean Type)
    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    // 4. 字符类型 (Character Type)
    // Rust 的 char 类型是 4 字节，表示 Unicode 标量值
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // ========== 复合类型 (Compound Types) ==========

    // 5. 元组 (Tuples)
    // 元组是固定长度的，一旦声明，长度不能改变
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 通过解构获取元组的值
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 通过索引访问元组元素
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "five_hundred: {}, six_point_four: {}, one: {}",
        five_hundred, six_point_four, one
    );

    // 空元组 () 被称为单元类型 (unit type)
    let unit = ();
    println!("unit: {:?}", unit);

    // 6. 数组 (Arrays)
    // 数组长度固定，所有元素必须是相同类型
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // 指定类型和长度
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 创建相同元素的数组
    let b = [3; 5]; // 等同于 [3, 3, 3, 3, 3]

    println!("arr: {:?}, a: {:?}, b: {:?}", arr, a, b);

    // 访问数组元素
    let first = arr[0];
    let second = arr[1];
    println!("first: {}, second: {}", first, second);

    // 数组越界会在运行时 panic（如果使用编译时检查，会在编译时发现）
    // let invalid = arr[10]; // 运行时 panic

    // ========== 字符串类型 ==========

    // 7. 字符串切片 (&str)
    // &str 是不可变的字符串切片，通常用于字符串字面量
    let s1: &str = "Hello, world!";
    println!("s1: {}", s1);

    // 8. String 类型
    // String 是可变的、拥有所有权的字符串类型
    let mut s2 = String::from("Hello");
    s2.push_str(", world!");
    println!("s2: {}", s2);

    // ========== 类型转换 ==========

    // 9. 类型转换
    let integer = 65;
    let character = integer as u8 as char;
    println!("integer {} as char: {}", integer, character);

    // ========== 类型别名 ==========

    // 10. 类型别名 (Type Aliases)
    type Kilometers = i32;
    let distance: Kilometers = 100;
    println!("distance: {} km", distance);
}
