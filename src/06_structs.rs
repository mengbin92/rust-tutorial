// ============================================
// 06. 结构体 (Structs)
// ============================================
// 结构体让你可以创建自定义数据类型

fn main() {
    // ========== 定义和实例化结构体 ==========

    // 1. 基本结构体定义
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 2. 创建结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 3. 访问结构体字段
    user1.email = String::from("anotheremail@example.com");
    println!("user1 email: {}", user1.email);

    // 4. 使用结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 使用 user1 的其余字段
    };
    println!("user2: {} ({})", user2.username, user2.email);

    // 5. 元组结构体 (Tuple Structs)
    // 元组结构体有名称，但字段没有名称
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 6. 类单元结构体 (Unit-like Structs)
    // 没有任何字段的结构体
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // ========== 结构体方法 ==========

    // 7. 定义方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 的面积: {}", rect1.area());
    println!(
        "rect1 可以容纳 rect2: {}",
        rect1.can_hold(&Rectangle {
            width: 10,
            height: 40
        })
    );

    // 8. 关联函数（类似静态方法）
    let sq = Rectangle::square(3);
    println!("正方形: {:?}", sq);

    // ========== 使用其他结构体的例子 ==========

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    person.introduce();
    println!("{} 的年龄: {}", person.name, person.get_age());
}

// ========== 带方法的结构体示例 ==========

#[derive(Debug)] // 允许使用 {:?} 打印
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块用于定义方法
impl Rectangle {
    // 方法：第一个参数是 &self（不可变引用）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法：第一个参数是 &mut self（可变引用）
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // 方法：第一个参数是 self（获取所有权）
    fn into_tuple(self) -> (u32, u32) {
        (self.width, self.height)
    }

    // 方法：可以有多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数：没有 self 参数（类似静态方法）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 可以有多个 impl 块
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// ========== 另一个结构体示例 ==========

struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 方法示例
    fn introduce(&self) {
        println!("我是 {}, {} 岁", self.name, self.age);
    }

    fn get_age(&self) -> u32 {
        self.age
    }

    // 可变方法
    fn have_birthday(&mut self) {
        self.age += 1;
    }

    // 关联函数：构造函数
    fn new(name: String, age: u32) -> Person {
        Person { name, age } // 字段初始化简写语法
    }
}
