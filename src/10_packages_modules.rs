// ============================================
// 10. 包和模块 (Packages and Modules)
// ============================================
// Rust 的模块系统帮助组织代码并控制私有性

// ========== 模块基础 ==========

// 1. 定义模块
mod front_of_house {
    // 模块可以包含其他项：函数、结构体、枚举、常量、trait 等

    // 2. 嵌套模块
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }

        fn seat_at_table() {
            println!("安排座位");
        }
    }

    mod serving {
        fn take_order() {
            println!("接受订单");
        }

        fn serve_order() {
            println!("上菜");
        }

        fn take_payment() {
            println!("收银");
        }
    }
}

// 3. 使用模块
fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// ========== 使用 use 关键字 ==========

// 4. 使用 use 引入路径
use crate::front_of_house::hosting;

fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}

// 5. 使用 use 引入函数（不推荐，除非是 trait 方法）
// use crate::front_of_house::hosting::add_to_waitlist;
// fn eat_at_restaurant3() {
//     add_to_waitlist();
// }

// 6. 使用 use 引入结构体、枚举等
use std::collections::HashMap;

fn use_hashmap() {
    let mut map = HashMap::new();
    map.insert(1, "a");
}

// 7. 使用 as 关键字创建别名
use std::fmt::Result;
use std::io::Result as IoResult;

// 8. 使用 pub use 重新导出
pub use crate::front_of_house::hosting;

// ========== 模块文件系统 ==========

// 9. 将模块分离到不同文件
// 假设有 src/lib.rs 或 src/main.rs
// mod front_of_house; // 这会在 front_of_house.rs 或 front_of_house/mod.rs 中查找

// ========== 私有性规则 ==========

// 10. 私有性规则：
// - 所有项（函数、方法、结构体、枚举、模块和常量）默认是私有的
// - 可以使用 pub 关键字使项变为公有
// - 父模块中的项不能使用子模块中的私有项
// - 子模块中的项可以使用其父模块中的项

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // 公有字段
        seasonal_fruit: String, // 私有字段
    }

    impl Breakfast {
        // 关联函数必须是 pub 才能从外部调用
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举的所有变体默认都是公有的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_at_restaurant4() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("我要 {} 吐司", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // 错误！私有字段

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// ========== super 关键字 ==========

// 11. 使用 super 访问父模块
mod back_of_house2 {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // 访问父模块的函数
    }

    fn cook_order() {}
}

fn serve_order() {
    println!("上菜");
}

// ========== 模块组织示例 ==========

// 12. 典型的模块结构
mod network {
    pub fn connect() {
        println!("网络连接");
    }

    pub mod server {
        pub fn serve() {
            println!("服务器服务");
        }
    }
}

mod client {
    pub fn connect() {
        println!("客户端连接");
    }
}

fn main() {
    // 使用模块
    network::connect();
    network::server::serve();
    client::connect();

    // 使用 use
    use network::server;
    server::serve();

    // 使用嵌套路径
    use std::collections::{HashMap, HashSet};
    use std::io::{self, Write};

    // 使用 glob 运算符
    use std::collections::*; // 引入所有公有项（谨慎使用）
}
