// ============================================
// 07. 枚举 (Enums)
// ============================================
// 枚举允许你定义一个类型，它可以是几个不同的变体之一

fn main() {
    // ========== 基本枚举 ==========

    // 1. 定义枚举
    enum IpAddrKind {
        V4,
        V6,
    }

    // 2. 使用枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // ========== 枚举关联数据 ==========

    // 3. 枚举变体可以关联数据
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // 4. 枚举变体可以有不同的数据类型
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    // 5. 枚举变体可以包含结构体
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(0, 160, 255);

    // ========== Option 枚举 ==========

    // 6. Option<T> 是 Rust 标准库中的枚举
    // 用于表示值可能存在或不存在的情况
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("absent_number: {:?}", absent_number);

    // Option<T> 的使用示例
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // 错误！不能直接相加
    // 需要使用 match 或其他方法处理 Option

    // ========== Result 枚举 ==========

    // 7. Result<T, E> 是另一个重要的枚举
    // 用于表示操作可能成功或失败
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出错了");

    println!("success: {:?}, failure: {:?}", success, failure);

    // ========== 枚举方法 ==========

    // 8. 枚举也可以有方法
    let msg = Message::Write(String::from("hello"));
    msg.call();

    // ========== 实际应用示例 ==========

    // 9. 使用枚举表示状态
    let state = State::Running;
    state.execute();

    // 10. 使用枚举表示事件
    let event = Event::Click { x: 100, y: 200 };
    handle_event(event);

    let event2 = Event::KeyPress('A');
    handle_event(event2);
}

// ========== 函数使用枚举 ==========

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}

// ========== 枚举方法示例 ==========

impl Message {
    fn call(&self) {
        // 方法实现
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(s) => println!("写入: {}", s),
            Message::ChangeColor(r, g, b) => println!("改变颜色: RGB({}, {}, {})", r, g, b),
        }
    }
}

// ========== 状态机示例 ==========

enum State {
    Idle,
    Running,
    Paused,
    Stopped,
}

impl State {
    fn execute(&self) {
        match self {
            State::Idle => println!("状态：空闲"),
            State::Running => println!("状态：运行中"),
            State::Paused => println!("状态：已暂停"),
            State::Stopped => println!("状态：已停止"),
        }
    }
}

// ========== 事件处理示例 ==========

enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    MouseMove { x: i32, y: i32 },
    Quit,
}

fn handle_event(event: Event) {
    match event {
        Event::Click { x, y } => {
            println!("点击位置: ({}, {})", x, y);
        }
        Event::KeyPress(key) => {
            println!("按键: {}", key);
        }
        Event::MouseMove { x, y } => {
            println!("鼠标移动: ({}, {})", x, y);
        }
        Event::Quit => {
            println!("退出事件");
        }
    }
}
