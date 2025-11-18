// ============================================
// 09. 常见集合及操作 (Collections)
// ============================================
// Rust 标准库提供了多种集合类型

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    // ========== Vector (Vec<T>) ==========

    // 1. 创建 Vector
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // 使用宏创建

    // 2. 添加元素
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    // 3. 访问元素
    let third = &v[2]; // 使用索引（可能 panic）
    println!("第三个元素: {}", third);

    let third = v.get(2); // 使用 get（返回 Option）
    match third {
        Some(value) => println!("第三个元素: {}", value),
        None => println!("没有第三个元素"),
    }

    // 4. 遍历 Vector
    for i in &v {
        println!("{}", i);
    }

    // 5. 遍历并修改
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 1; // 解引用并修改
    }
    println!("修改后: {:?}", v);

    // 6. 使用枚举存储不同类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // ========== String ==========

    // 7. 创建 String
    let mut s = String::new();
    let s1 = String::from("hello");
    let s2 = "world".to_string();

    // 8. 更新 String
    s.push_str("hello");
    s.push('!');
    println!("s: {}", s);

    // 9. 字符串连接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 被移动，不能再使用
    println!("s3: {}", s3);

    // 10. 使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // 11. 字符串索引（Rust 字符串不支持直接索引）
    let hello = "Здравствуйте";
    // let h = hello[0]; // 错误！

    // 12. 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 必须是字符边界
    println!("切片: {}", s);

    // 13. 遍历字符串
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // ========== HashMap ==========

    // 14. 创建 HashMap
    let mut scores = HashMap::new();

    // 15. 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 16. 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue 队的分数: {}", s),
        None => println!("没有找到"),
    }

    // 17. 遍历 HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 18. 更新 HashMap
    // 覆盖现有值
    scores.insert(String::from("Blue"), 25);

    // 只在键不存在时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(30);

    // 19. 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // ========== HashSet ==========

    // 20. 创建 HashSet
    let mut set = HashSet::new();

    // 21. 插入值
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2); // 重复值，不会插入

    println!("set: {:?}", set);

    // 22. 检查值是否存在
    if set.contains(&2) {
        println!("包含 2");
    }

    // 23. 集合操作
    let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

    // 并集
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("并集: {:?}", union);

    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("交集: {:?}", intersection);

    // 差集
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("差集: {:?}", difference);

    // ========== VecDeque (双端队列) ==========

    // 24. 创建 VecDeque
    let mut deque = VecDeque::new();

    // 25. 从两端添加元素
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);

    println!("deque: {:?}", deque);

    // 26. 从两端移除元素
    let front = deque.pop_front();
    let back = deque.pop_back();
    println!("front: {:?}, back: {:?}", front, back);

    // ========== BinaryHeap (优先队列) ==========

    // 27. 创建 BinaryHeap（最大堆）
    let mut heap = BinaryHeap::new();

    // 28. 插入元素
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(2);

    // 29. 获取最大元素
    while let Some(max) = heap.pop() {
        println!("{}", max); // 输出: 4, 3, 2, 1
    }
}
