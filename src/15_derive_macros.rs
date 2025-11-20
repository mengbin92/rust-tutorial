// ============================================
// 15. derive 宏详解
// ============================================
// #[derive(...)] 是 Rust 的属性宏，用于自动实现某些 Trait

fn main() {
    // ========== Debug Trait ==========

    // 1. Debug 允许使用 {:?} 或 {:#?} 格式化输出
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 使用 {:?} 打印（单行）
    println!("{:?}", person);
    // 输出: Person { name: "Alice", age: 30 }

    // 使用 {:#?} 打印（多行，更易读）
    println!("{:#?}", person);
    // 输出:
    // Person {
    //     name: "Alice",
    //     age: 30,
    // }

    // ========== PartialEq Trait ==========

    // 2. PartialEq 允许使用 == 和 != 运算符
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 != p3: {}", p1 != p3); // true

    // 在条件判断中使用
    if p1 == p2 {
        println!("两个点相同");
    }

    // ========== Eq Trait ==========

    // 3. Eq 是 PartialEq 的扩展，表示完全相等（没有 NaN 等特殊情况）
    #[derive(Debug, PartialEq, Eq)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    let c1 = Color { r: 255, g: 0, b: 0 };
    let c2 = Color { r: 255, g: 0, b: 0 };
    println!("c1 == c2: {}", c1 == c2);

    // ========== PartialOrd 和 Ord ==========

    // 4. PartialOrd 允许使用 <, >, <=, >= 运算符
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Score {
        value: f64,
    }

    let s1 = Score { value: 85.5 };
    let s2 = Score { value: 90.0 };
    println!("s1 < s2: {}", s1 < s2); // true

    // 5. Ord 是 PartialOrd 的扩展，表示完全可排序
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Student {
        id: u32,
        name: String,
    }

    let students = vec![
        Student {
            id: 3,
            name: String::from("Charlie"),
        },
        Student {
            id: 1,
            name: String::from("Alice"),
        },
        Student {
            id: 2,
            name: String::from("Bob"),
        },
    ];

    // 可以排序
    // students.sort(); // 需要 Ord

    // ========== Clone 和 Copy ==========

    // 6. Clone 允许显式克隆值
    #[derive(Debug, Clone)]
    struct Data {
        value: i32,
    }

    let d1 = Data { value: 42 };
    let d2 = d1.clone(); // 显式克隆
    println!("d1: {:?}, d2: {:?}", d1, d2);

    // 7. Copy 允许隐式复制（按位复制）
    // Copy 只能用于实现了 Copy 的类型（通常是简单类型）
    #[derive(Debug, Copy, Clone)]
    struct SimplePoint {
        x: i32,
        y: i32,
    }

    let sp1 = SimplePoint { x: 1, y: 2 };
    let sp2 = sp1; // 自动复制，sp1 仍然可用
    println!("sp1: {:?}, sp2: {:?}", sp1, sp2);

    // ========== Hash ==========

    // 8. Hash 允许类型作为 HashMap 的键
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Key {
        id: u32,
        name: String,
    }

    let mut map = HashMap::new();
    let key = Key {
        id: 1,
        name: String::from("key1"),
    };
    map.insert(key, "value1");
    println!("map: {:?}", map);

    // ========== Default ==========

    // 9. Default 提供默认值
    #[derive(Debug, Default)]
    struct Config {
        host: String,
        port: u16,
        timeout: u64,
    }

    let config = Config::default();
    println!("默认配置: {:?}", config);

    // ========== 组合使用 ==========

    // 10. 常见的组合
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    struct User {
        id: u32,
        username: String,
        email: String,
    }

    let user1 = User {
        id: 1,
        username: String::from("alice"),
        email: String::from("alice@example.com"),
    };

    let user2 = user1.clone();
    println!("user1 == user2: {}", user1 == user2);

    // ========== 枚举中的 derive ==========

    // 11. 枚举也可以使用 derive
    #[derive(Debug, PartialEq, Clone)]
    enum Status {
        Active,
        Inactive,
        Pending { days: u32 },
    }

    let status1 = Status::Active;
    let status2 = Status::Pending { days: 5 };
    println!("status1: {:?}", status1);
    println!("status2: {:?}", status2);

    // ========== 实际应用示例 ==========

    // 12. 在测试中使用
    #[derive(Debug, PartialEq)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    // 在测试中很有用
    assert_eq!(rect1, rect2);
    println!("测试通过！两个矩形相等");

    // ========== 手动实现 vs derive ==========

    // 13. 如果没有 derive，需要手动实现
    struct ManualPoint {
        x: i32,
        y: i32,
    }

    // 手动实现 Debug
    impl std::fmt::Debug for ManualPoint {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }

    // 手动实现 PartialEq
    impl PartialEq for ManualPoint {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    let mp1 = ManualPoint { x: 1, y: 2 };
    let mp2 = ManualPoint { x: 1, y: 2 };
    println!("手动实现: {:?}, 相等: {}", mp1, mp1 == mp2);

    // ========== derive 的限制 ==========

    // 14. derive 只能用于满足条件的类型
    // - 所有字段都必须实现相应的 Trait
    // - 对于某些 Trait（如 Copy），类型必须满足特定条件

    // 例如：包含 String 的类型不能实现 Copy（因为 String 不实现 Copy）
    // #[derive(Copy, Clone)]  // 这会编译错误
    // struct Bad {
    //     name: String,  // String 不实现 Copy
    // }
}

// ========== 常用 derive 组合 ==========

// 1. 用于调试和测试
#[derive(Debug, PartialEq)]
struct TestStruct {
    value: i32,
}

// 2. 用于可克隆和可比较的类型
#[derive(Debug, PartialEq, Eq, Clone)]
struct ComparableStruct {
    id: u32,
    name: String,
}

// 3. 用于作为 HashMap 键的类型
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HashableKey {
    id: u32,
}

// 4. 用于可排序的类型
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct SortableStruct {
    priority: u32,
    name: String,
}

// 5. 完整的"标准"组合（最常用）
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct StandardStruct {
    id: u32,
    data: String,
}
