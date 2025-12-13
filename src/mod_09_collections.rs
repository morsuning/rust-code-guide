#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_macros, deprecated)]

// Rust 集合类型深度解析 (Rust Collections Deep Dive)
// 全面讲解 Rust 标准库中的各种集合类型、使用场景、性能特性和最佳实践
// 集合是 Rust 数据管理的核心工具，理解它们的特性对于编写高效的 Rust 程序至关重要

// ===========================================
// 1. Vector (Vec<T>) - 动态数组 (Dynamic Array)
// ===========================================

// Vector 是 Rust 中最常用的集合类型，提供了可变长度的连续内存存储
// 它在栈上存储元数据（指针、长度、容量），在堆上存储实际数据
// 这种设计结合了栈访问的效率和堆存储的灵活性

fn vector_deep_dive() {
    println!("=== Vector 深度解析 ===");

    // Vector 的内存布局理解：
    // - 栈上：指针(8字节) + 长度(8字节) + 容量(8字节) = 24字节
    // - 堆上：连续存储的元素数组
    // 这种布局使得 Vector 既安全又高效

    // 创建 Vector 的不同方式及其适用场景
    let mut v1: Vec<i32> = Vec::new(); // 空向量，需要类型注解
    println!("空向量创建: 长度={}, 容量={}", v1.len(), v1.capacity());

    // 使用 vec! 宏：这是最常用的创建方式，编译器会推断类型
    let v2 = vec![1, 2, 3, 4, 5];
    println!(
        "宏创建的向量: 长度={}, 容量={}, 内容={:?}",
        v2.len(),
        v2.capacity(),
        v2
    );

    // 使用 with_capacity 预分配容量：避免频繁的内存重分配
    let mut v3 = Vec::with_capacity(10);
    println!("预分配向量: 长度={}, 容量={}", v3.len(), v3.capacity());

    // 添加元素观察容量变化
    for i in 0..15 {
        v3.push(i);
        if v3.len() <= 15 {
            println!("添加 {}: 长度={}, 容量={}", i, v3.len(), v3.capacity());
        }
    }

    // Vector 元素访问的安全性和性能考量
    let v = vec![10, 20, 30, 40, 50];

    // 方式 1：索引访问 - 高速但不安全，会 panic 如果越界
    let element = v[0];
    println!("索引访问 v[0] = {}", element);

    // 方式 2：get 方法 - 安全，返回 Option<T>
    match v.get(10) {
        Some(element) => println!("get(10) = {}", element),
        None => println!("get(10) = None (安全处理越界)"),
    }

    // Vector 遍历的三种方式及其所有权语义
    let mut numbers = vec![1, 2, 3, 4, 5];

    // 1. 不可变借用：&T，不影响原向量
    println!("不可变遍历:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // 2. 可变借用：&mut T，可以修改元素
    println!("可变遍历(每个元素乘以2):");
    for num in &mut numbers {
        *num *= 2;
        print!("{} ", num);
    }
    println!();

    // 3. 消费性遍历：T，获取所有权，向量被消耗
    println!("消费性遍历:");
    let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
    println!("消费后新向量: {:?}", doubled);
    // numbers 已被消耗，不能再使用

    // Vector 的高级操作和性能优化
    let mut v = vec![1, 2, 3, 4, 5];

    // 批量操作
    v.extend_from_slice(&[6, 7, 8]); // 高效的批量添加
    println!("extend_from_slice 后: {:?}", v);

    v.truncate(3); // 截断到指定长度
    println!("truncate(3) 后: {:?}", v);

    v.retain(|&x| x % 2 == 0); // 保留满足条件的元素
    println!("retain(偶数) 后: {:?}", v);

    // 内存管理和容量优化
    let mut v = Vec::with_capacity(100);
    for i in 0..10 {
        v.push(i);
    }
    println!("优化前: 长度={}, 容量={}", v.len(), v.capacity());

    v.shrink_to_fit(); // 释放未使用的容量
    println!("shrink_to_fit 后: 长度={}, 容量={}", v.len(), v.capacity());

    // Vector 存储不同类型的模式
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
        Boolean(bool),
        Formula(String, Vec<String>), // 公式和依赖项
    }

    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("销售额")),
        SpreadsheetCell::Float(1234.56),
        SpreadsheetCell::Boolean(true),
        SpreadsheetCell::Formula(
            String::from("SUM(A1:A3)"),
            vec!["A1".to_string(), "A2".to_string(), "A3".to_string()],
        ),
    ];
    println!("混合类型 Vector: {:?}", row);

    // Vector 的性能特点和最佳实践：
    // 1. 预分配容量：当知道大小时，使用 with_capacity
    // 2. 批量操作：使用 extend、extend_from_slice 等
    // 3. 内存优化：使用 shrink_to_fit 释放未使用内存
    // 4. 安全访问：优先使用 get 方法处理可能的越界
    // 5. 所有权管理：理解三种遍历方式的所有权语义

    println!();
}

// ===========================================
// 2. String 类型深度解析 (String Deep Dive)
// ===========================================

// String 是 UTF-8 编码的可变文本类型，是 Rust 文本处理的核心
// 理解 String 的 UTF-8 特性对于正确处理多语言文本至关重要

fn string_deep_dive() {
    println!("=== String 深度解析 ===");

    // String 的内存结构：
    // - 栈上：指针 + 长度 + 容量（与 Vector 相同）
    // - 堆上：UTF-8 编码的字节数组
    // 这种设计使得 String 既支持 Unicode 又保持内存效率

    // String 创建的多种方式
    let mut s1 = String::new(); // 空字符串
    s1.push_str("Hello");
    println!("String::new(): {}", s1);

    let s2 = "initial content".to_string(); // 从 &str 转换
    println!("to_string(): {}", s2);

    let s3 = String::from("literal content"); // 从字面量创建
    println!("String::from(): {}", s3);

    let s4 = String::with_capacity(20); // 预分配容量
    println!("预分配容量: {}, {}", s4.len(), s4.capacity());

    // String 更新操作的性能和语义
    let mut s = String::from("Hello");

    // push_str：高效地添加字符串切片，不获取所有权
    s.push_str(", world!");
    println!("push_str: {}", s);

    // push：添加单个字符
    s.push('!');
    println!("push: {}", s);

    // 使用 + 运算符：注意所有权转移
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 的所有权被转移
    println!("+ 运算符: {}", s3);
    // println!("{}", s1); // 编译错误：s1 不再有效

    // format! 宏：创建新字符串，不转移所有权
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format! 宏: {}", s);
    println!("原字符串仍然可用: {}, {}, {}", s1, s2, s3);

    // Unicode 和 UTF-8 的深度理解
    let hindi = "नमस्ते"; // 印地语 "你好"
    let chinese = "你好"; // 中文 "你好"
    let emoji = "😊🌍"; // 表情符号

    println!("Unicode 字符串示例:");
    println!("印地语: {}", hindi);
    println!("中文: {}", chinese);
    println!("表情符号: {}", emoji);

    // 字节 vs 字符 vs 字形簇（grapheme clusters）
    println!("UTF-8 编码分析:");
    println!("'नमस्ते' 字节: {:?}", hindi.as_bytes());
    println!("'नमस्ते' 字符: {:?}", hindi.chars().collect::<Vec<char>>());
    println!("'你好' 字节: {:?}", chinese.as_bytes());
    println!("'你好' 字符: {:?}", chinese.chars().collect::<Vec<char>>());

    // 字符串切片的安全处理
    let hello = "Здравствуйте"; // 俄语 "你好"

    // 直接按字节切片可能不安全
    // let s = &hello[0..4]; // 这样是安全的，因为 4 是字符边界
    println!("安全的字节切片: {}", &hello[0..4]);

    // 安全的字符串切片函数
    fn safe_slice(s: &str, start: usize, end: usize) -> Option<&str> {
        if !s.is_char_boundary(start) || !s.is_char_boundary(end) {
            return None;
        }
        Some(&s[start..end])
    }

    match safe_slice(hello, 0, 4) {
        Some(slice) => println!("安全切片结果: {}", slice),
        None => println!("切片边界无效"),
    }

    // 字符串遍历的不同粒度
    let text = "你好世界 Hello World 😊";

    println!("字符串遍历示例:");
    println!("按字节遍历:");
    for b in text.bytes() {
        print!("{} ", b);
    }
    println!("\n按字符遍历:");
    for c in text.chars() {
        print!("{} ", c);
    }
    println!("\n按字形簇遍历（需要 unicode-segmentation crate）:");

    // String 的常用方法和模式
    let s = String::from("  Hello, Rust World!  ");

    // 查询和检查
    println!("原始字符串: '{}'", s);
    println!("长度: {}", s.len());
    println!("字符数: {}", s.chars().count());
    println!("是否为空: {}", s.is_empty());
    println!("包含 'Rust': {}", s.contains("Rust"));
    println!("以 'Hello' 开头: {}", s.starts_with("Hello"));
    println!("以 'World' 结尾: {}", s.ends_with("World!"));

    // 修改和转换
    let trimmed = s.trim(); // 去除首尾空白
    println!("去除空白: '{}'", trimmed);

    let replaced = s.replace("Hello", "Hi");
    println!("替换: {}", replaced);

    let upper = s.to_uppercase();
    let lower = s.to_lowercase();
    println!("大写: {}", upper);
    println!("小写: {}", lower);

    // 字符串分割和连接
    let data = "name:张三,age:25,city:北京";
    let parts: Vec<&str> = data.split(',').collect();
    println!("分割结果: {:?}", parts);

    let joined = parts.join(" | ");
    println!("连接结果: {}", joined);

    // String 性能优化的最佳实践：
    // 1. 使用 &str 进行函数参数：避免不必要的 String 创建
    // 2. 预分配容量：当知道最终大小时使用 with_capacity
    // 3. 使用 push_str 而非 +：避免所有权转移
    // 4. 批量操作：使用 extend 而非多次 push
    // 5. 谨重处理 Unicode：使用 chars() 而非字节索引

    println!();
}

// ===========================================
// 3. HashMap 深度解析 (HashMap Deep Dive)
// ===========================================

// HashMap 是基于哈希表的键值对集合，提供了 O(1) 平均时间复杂度的查找、插入和删除
// 理解 HashMap 的工作原理对于正确使用和避免性能陷阱至关重要

fn hashmap_deep_dive() {
    println!("=== HashMap 深度解析 ===");

    use std::collections::HashMap;

    // HashMap 的工作原理：
    // - 使用 SipHash 1-3 哈希算法（防碰撞攻击）
    // - 使用 Robin Hood 哈希处理碰撞
    // - 自动扩容：当元素数量超过容量时重新分配
    // - 负载因子：元素数量与桶数量的比率

    // HashMap 创建和基本操作
    let mut scores = HashMap::new();

    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 30);
    println!("初始 HashMap: {:?}", scores);

    // 访问值的几种方式
    let team_name = String::from("Blue");

    // 方式 1：直接访问，返回 Option<&V>
    match scores.get(&team_name) {
        Some(score) => println!("Blue 队得分: {}", score),
        None => println!("Blue 队不存在"),
    }

    // 方式 2：entry API 提供更灵活的操作
    let entry = scores.entry(String::from("Green"));
    let score = entry.or_insert(0); // 如果不存在则插入默认值
    println!("Green 队得分: {}", score);

    // HashMap 更新模式
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);

    // 模式 1：直接覆盖
    scores.insert("Blue", 25); // 无论是否存在都会覆盖
    println!("直接覆盖后: {:?}", scores);

    // 模式 2：基于旧值更新
    let old_score = scores.get("Blue").copied().unwrap_or(0);
    scores.insert("Blue", old_score + 5);
    println!("基于旧值更新后: {:?}", scores);

    // 模式 3：entry API 高效更新
    let score = scores.entry("Blue").or_insert(0);
    *score += 5;
    println!("entry API 更新后: {:?}", scores);

    // 复杂的 HashMap 操作示例：词频统计
    let text = "hello world wonderful world hello rust hello programming";
    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    println!("词频统计: {:?}", word_counts);

    // HashMap 遍历和操作
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 5);

    // 不可变遍历
    println!("键值对遍历:");
    for (key, value) in &map {
        println!("  {}: {}", key, value);
    }

    // 可变遍历：修改值
    println!("数量加倍:");
    for (_, value) in &mut map {
        *value *= 2;
    }
    for (key, value) in &map {
        println!("  {}: {}", key, value);
    }

    // HashMap 的所有权语义
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 所有权转移到 HashMap

    // field_name 和 field_value 不再有效
    // println!("{}", field_name); // 编译错误

    // 只能通过引用访问
    if let Some(color) = map.get("Favorite color") {
        println!("收藏的颜色: {}", color);
    }

    // HashMap 的性能考虑和优化
    // 预分配容量：避免多次扩容
    let mut optimized_map = HashMap::with_capacity(1000);
    println!("预分配的 HashMap: 容量={}", optimized_map.capacity());

    // 批量插入：比多次 insert 更高效
    let data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
    optimized_map.extend(data);
    println!("批量插入后: {:?}", optimized_map);

    // 查找和删除操作
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    // 检查存在性
    println!("包含 key1: {}", map.contains_key("key1"));

    // 删除键值对
    let removed = map.remove("key1");
    println!("删除 key1: {:?}", removed);
    println!("删除后: {:?}", map);

    // 保留满足条件的键值对
    map.retain(|key, value| key.starts_with("key"));
    println!("保留后: {:?}", map);

    // 自定义哈希和比较
    use std::hash::{Hash, Hasher};

    struct CustomKey {
        id: u32,
        name: String,
    }

    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 只使用 id 进行哈希，忽略 name
            self.id.hash(state);
        }
    }

    impl PartialEq for CustomKey {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for CustomKey {}

    let key1 = CustomKey {
        id: 1,
        name: "Item1".to_string(),
    };
    let key2 = CustomKey {
        id: 1,
        name: "Item2".to_string(),
    };

    let mut map = HashMap::new();
    map.insert(key1, "Value1");

    // key2 会被视为与 key1 相同的键
    println!("相同 id 的键被视为相同: {:?}", map.get(&key2));

    // HashMap 使用场景和替代方案：
    // 1. 普通映射：HashMap<K, V>
    // 2. 需要有序访问：BTreeMap<K, V>
    // 3. 需要插入顺序：使用 Vec 或 LinkedHashMap
    // 4. 简单查找：对于小数据集，Vec<(K, V)> 可能更快

    println!();
}

// ===========================================
// 4. 其他集合类型详解 (Other Collection Types)
// ===========================================

// Rust 标准库提供了多种集合类型，每种都有其特定的使用场景和性能特性
// 理解这些集合的差异对于选择合适的数据结构至关重要

fn other_collections_detailed() {
    println!("=== 其他集合类型详解 ===");

    use std::collections::{BTreeMap, BTreeSet, HashSet, LinkedList, VecDeque};

    // LinkedList - 双向链表
    // 特点：O(1) 的插入和删除，但 O(n) 的随机访问
    // 适用场景：需要频繁在中间插入删除元素的情况

    let mut list: LinkedList<String> = LinkedList::new();

    // 两端操作
    list.push_back("end".to_string());
    list.push_front("start".to_string());
    println!("LinkedList 初始状态: {:?}", list);

    // 中间插入（需要遍历）- linked_list_cursors 是 unstable 特性
    // let mut cursor = list.cursor_front_mut();
    // if let Some(node) = cursor.current() {
    //     cursor.insert_after("middle".to_string());
    // }
    // println!("中间插入后: {:?}", list);

    // 前后删除
    let front = list.pop_front();
    let back = list.pop_back();
    println!("删除操作: front={:?}, back={:?}", front, back);
    println!("最终状态: {:?}", list);

    // VecDeque - 双端队列
    // 特点：环形缓冲区实现，两端操作都是 O(1)，支持随机访问
    // 适用场景：需要从两端频繁添加删除元素的队列

    let mut deque: VecDeque<i32> = VecDeque::with_capacity(5);

    // 前端操作
    deque.push_front(1);
    deque.push_front(2);
    println!("前端添加: {:?}", deque);

    // 后端操作
    deque.push_back(3);
    deque.push_back(4);
    println!("后端添加: {:?}", deque);

    // 随机访问
    if let Some(item) = deque.get(1) {
        println!("索引 1 的元素: {}", item);
    }

    // 中间操作（相对低效）
    deque.insert(2, 99); // 在索引 2 处插入
    println!("中间插入: {:?}", deque);

    // BTreeMap - 有序映射
    // 特点：键自动排序，提供 O(log n) 的所有操作
    // 适用场景：需要有序遍历或范围查询

    let mut scores = BTreeMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    scores.insert("David", 95); // 相同分数，按名字排序

    println!("BTreeMap 自动排序:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // 范围查询
    let range = scores.range("Bob"..="David");
    println!("范围查询 (Bob..=David):");
    for (name, score) in range {
        println!("  {}: {}", name, score);
    }

    // 第一个和最后一个
    if let Some((first_name, first_score)) = scores.first_key_value() {
        println!("第一个: {} - {}", first_name, first_score);
    }
    if let Some((last_name, last_score)) = scores.last_key_value() {
        println!("最后一个: {} - {}", last_name, last_score);
    }

    // BTreeSet - 有序集合
    // 特点：元素自动排序，去重，O(log n) 的操作
    // 适用场景：需要有序的唯一元素集合

    let mut set = BTreeSet::new();

    // 插入元素（自动排序和去重）
    set.insert(5);
    set.insert(2);
    set.insert(8);
    set.insert(2); // 重复元素会被忽略
    set.insert(7);

    println!("BTreeSet 自动排序: {:?}", set);

    // 集合操作
    let set1: BTreeSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: BTreeSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    // 并集
    let union: Vec<i32> = set1.union(&set2).cloned().collect();
    println!("并集: {:?}", union);

    // 交集
    let intersection: Vec<i32> = set1.intersection(&set2).cloned().collect();
    println!("交集: {:?}", intersection);

    // 差集
    let difference: Vec<i32> = set1.difference(&set2).cloned().collect();
    println!("差集 (set1 - set2): {:?}", difference);

    // HashSet - 无序哈希集合
    // 特点：O(1) 平均时间的查找、插入、删除，但无序
    // 适用场景：快速查找，不需要顺序

    let mut hash_set = HashSet::new();

    // 插入元素（自动去重）
    hash_set.insert("apple");
    hash_set.insert("banana");
    hash_set.insert("orange");
    hash_set.insert("apple"); // 重复

    println!("HashSet: {:?}", hash_set);

    // 快速查找
    println!("包含 'apple': {}", hash_set.contains("apple"));
    println!("包含 'grape': {}", hash_set.contains("grape"));

    // 集合操作
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    let union: HashSet<i32> = set1.union(&set2).cloned().collect();
    println!("HashSet 并集: {:?}", union);

    // 集合类型选择指南：
    // 1. 需要随机访问：Vec
    // 2. 需要两端操作：VecDeque
    // 3. 需要中间插入删除：LinkedList
    // 4. 需要有序映射：BTreeMap
    // 5. 需要快速查找映射：HashMap
    // 6. 需要有序集合：BTreeSet
    // 7. 需要快速查找集合：HashSet

    println!();
}

// ===========================================
// 5. 迭代器深入解析 (Iterators Deep Dive)
// ===========================================

// 迭代器是 Rust 中处理集合序列的核心抽象，提供了惰性求值和链式操作的能力
// 理解迭代器的适配器和消费者对于编写高效、简洁的 Rust 代码至关重要

fn iterators_deep_dive() {
    println!("=== 迭代器深入解析 ===");

    // 迭代器的核心概念：
    // - Iterator trait：定义了 next() 方法
    // - 惰性求值：链式调用不会立即执行
    // - 所有权：迭代器可以获取所有权或借用

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 迭代器的三种创建方式
    // 1. iter() - 不可变借用
    let sum1: i32 = numbers.iter().sum();
    println!("iter() 求和: {}, 原向量仍可用: {:?}", sum1, numbers);

    // 2. iter_mut() - 可变借用
    let mut numbers2 = vec![1, 2, 3];
    let doubled: Vec<_> = numbers2
        .iter_mut()
        .map(|x| {
            *x *= 2;
            *x
        })
        .collect();
    println!("iter_mut() 双倍后: {:?}", numbers2);

    // 3. into_iter() - 消费所有权
    let numbers3 = vec![1, 2, 3];
    let squares: Vec<_> = numbers3.into_iter().map(|x| x * x).collect();
    println!("into_iter() 平方后: {:?}", squares);
    // numbers3 已被消耗

    // 常用迭代器适配器（返回新的迭代器）
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter - 过滤元素
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter(偶数): {:?}", evens);

    // map - 转换元素
    let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("map(平方): {:?}", squares);

    // take - 取前 n 个元素
    let first_three: Vec<_> = numbers.iter().take(3).collect();
    println!("take(3): {:?}", first_three);

    // skip - 跳过前 n 个元素
    let skip_three: Vec<_> = numbers.iter().skip(3).collect();
    println!("skip(3): {:?}", skip_three);

    // take_while - 满足条件时取元素
    let take_while: Vec<_> = numbers.iter().take_while(|&&x| x <= 5).collect();
    println!("take_while(<=5): {:?}", take_while);

    // 链式操作示例
    let result: Vec<_> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // 过滤偶数
        .map(|&x| x * x) // 计算平方
        .take(3) // 取前3个
        .collect();
    println!("链式操作: {:?}", result);

    // 迭代器消费者（执行迭代并产生结果）
    let numbers = vec![1, 2, 3, 4, 5];

    // collect - 收集到集合
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("collect: {:?}", doubled);

    // fold - 折叠（从左到右）
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold 求和: {}", sum);

    // reduce - 减少（fold 的特例，初始值为第一个元素）
    let product = numbers.iter().cloned().reduce(|acc, x| acc * x);
    println!("reduce 乘积: {:?}", product);

    // any/all - 存在性检查
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("any(有偶数): {}, all(全正数): {}", has_even, all_positive);

    // count - 计数
    let count = numbers.iter().filter(|&&x| x > 3).count();
    println!("count(>3): {}", count);

    // find - 查找
    let found = numbers.iter().find(|&&x| x % 2 == 0);
    println!("find(第一个偶数): {:?}", found);

    // position - 位置查找
    let position = numbers.iter().position(|&x| x == 3);
    println!("position(3的位置): {:?}", position);

    // max/min - 极值查找
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("max: {:?}, min: {:?}", max, min);

    // 自定义迭代器
    struct Fibonacci {
        a: u64,
        b: u64,
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let next = self.a;
            self.a = self.b;
            self.b = self.b.checked_add(next)?;
            Some(next)
        }
    }

    let fib = Fibonacci { a: 0, b: 1 };
    let fibonacci_numbers: Vec<_> = fib.take(10).collect();
    println!("斐波那契数列: {:?}", fibonacci_numbers);

    // 迭代器性能优化技巧：
    // 1. 使用 by_ref() 避免所有权问题
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x > 2).sum();
    let count = numbers.iter().filter(|&&x| x > 2).count();
    println!("重复使用 iter(): sum={}, count={}", sum, count);

    // 2. 使用 peek() 预览下一个元素
    let mut iter = numbers.iter().peekable();
    while let Some(&x) = iter.next() {
        if let Some(&next) = iter.peek() {
            println!("当前: {}, 下一个: {}", x, next);
        } else {
            println!("当前: {}, 无下一个", x);
        }
    }

    // 3. 使用 cycle() 无限循环（谨慎使用）
    let endless = vec![1, 2, 3].into_iter().cycle().take(8);
    println!("循环取8个: {:?}", endless.collect::<Vec<_>>());

    println!();
}

// ===========================================
// 6. 集合性能优化实战 (Collection Performance Optimization)
// ===========================================

// 理解集合的性能特性对于编写高性能 Rust 代码至关重要
// 本节将深入探讨各种优化技术和最佳实践

fn collection_performance_optimization() {
    println!("=== 集合性能优化实战 ===");

    use std::collections::{HashMap, HashSet};
    use std::time::Instant;

    // 1. Vec 性能优化
    println!("1. Vec 性能优化:");

    // 预分配容量避免重分配
    let start = Instant::now();
    let mut vec_with_capacity = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        vec_with_capacity.push(i);
    }
    let duration1 = start.elapsed();
    println!("  预分配耗时: {:?}", duration1);

    let start = Instant::now();
    let mut vec_without_capacity = Vec::new();
    for i in 0..100_000 {
        vec_without_capacity.push(i);
    }
    let duration2 = start.elapsed();
    println!("  不预分配耗时: {:?}", duration2);
    println!(
        "  性能提升: {:.2}x",
        duration2.as_nanos() as f64 / duration1.as_nanos() as f64
    );

    // 批量操作 vs 单个操作
    let start = Instant::now();
    let mut vec = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec.push(i);
    }
    let single_duration = start.elapsed();

    let start = Instant::now();
    let mut vec = Vec::with_capacity(1000);
    vec.extend(0..1000);
    let batch_duration = start.elapsed();
    println!("  单个 push 耗时: {:?}", single_duration);
    println!("  批量 extend 耗时: {:?}", batch_duration);

    // 2. HashMap 性能优化
    println!("\n2. HashMap 性能优化:");

    // 预分配容量
    let start = Instant::now();
    let mut map_with_capacity = HashMap::with_capacity(100_000);
    for i in 0..100_000 {
        map_with_capacity.insert(i.to_string(), i);
    }
    let duration1 = start.elapsed();
    println!("  预分配耗时: {:?}", duration1);

    let start = Instant::now();
    let mut map_without_capacity = HashMap::new();
    for i in 0..100_000 {
        map_without_capacity.insert(i.to_string(), i);
    }
    let duration2 = start.elapsed();
    println!("  不预分配耗时: {:?}", duration2);

    // 3. 迭代器链式优化
    println!("\n3. 迭代器链式优化:");

    let large_vec: Vec<i32> = (0..10_000).collect(); // 减小数据量避免溢出

    // 多次遍历
    let start = Instant::now();
    let sum = large_vec.iter().filter(|&&x| x % 2 == 0).map(|&x| x as i64).sum::<i64>(); // 使用i64避免溢出
    let count = large_vec.iter().filter(|&&x| x % 2 == 0).count();
    let max = large_vec.iter().filter(|&&x| x % 2 == 0).max();
    let multiple_pass_duration = start.elapsed();

    // 单次遍历
    let start = Instant::now();
    let (sum2, count2, max2) = large_vec.iter().filter(|&&x| x % 2 == 0).fold(
        (0i64, 0, None::<i32>), // 使用i64避免溢出
        |(sum, count, max), &x| {
            let new_max = match max {
                Some(m) => Some(m.max(x)),
                None => Some(x),
            };
            (sum + x as i64, count + 1, new_max) // 转换为i64
        },
    );
    let single_pass_duration = start.elapsed();

    println!("  多次遍历耗时: {:?}", multiple_pass_duration);
    println!("  单次遍历耗时: {:?}", single_pass_duration);
    println!("  结果验证: sum={}, count={}, max={:?}", sum, count, max);

    // 4. 内存使用优化
    println!("\n4. 内存使用优化:");

    // shrink_to_fit 释放未使用内存
    let mut vec = Vec::with_capacity(1000);
    vec.extend(0..10);
    println!("  优化前: 长度={}, 容量={}", vec.len(), vec.capacity());
    vec.shrink_to_fit();
    println!(
        "  shrink_to_fit 后: 长度={}, 容量={}",
        vec.len(),
        vec.capacity()
    );

    // 5. 查找算法选择
    println!("\n5. 查找算法选择:");

    let sorted_vec: Vec<i32> = (0..10_000).collect();
    let unsorted_vec: Vec<i32> = (0..10_000).rev().collect();
    let target = 9999;

    // 有序数组上的二分查找
    let start = Instant::now();
    let binary_result = sorted_vec.binary_search(&target);
    let binary_duration = start.elapsed();

    // 无序数组上的线性查找
    let start = Instant::now();
    let linear_result = unsorted_vec.iter().find(|&&x| x == target);
    let linear_duration = start.elapsed();

    // HashMap 查找
    let mut hash_map = HashMap::new();
    for &i in &unsorted_vec {
        hash_map.insert(i, ());
    }
    let start = Instant::now();
    let hash_result = hash_map.get(&target);
    let hash_duration = start.elapsed();

    println!(
        "  二分查找耗时: {:?}, 结果: {:?}",
        binary_duration, binary_result
    );
    println!(
        "  线性查找耗时: {:?}, 结果: {:?}",
        linear_duration, linear_result
    );
    println!(
        "  HashMap 查找耗时: {:?}, 结果: {:?}",
        hash_duration, hash_result
    );

    // 6. 集合选择建议
    println!("\n6. 集合选择建议:");
    println!("  - 需要随机访问：选择 Vec");
    println!("  - 需要快速查找：选择 HashMap/HashSet");
    println!("  - 需要有序遍历：选择 BTreeMap/BTreeSet");
    println!("  - 需要两端操作：选择 VecDeque");
    println!("  - 需要频繁中间插入：选择 LinkedList");
    println!("  - 小数据集：考虑 Vec<(K, V)>");
    println!("  - 预先知道大小：使用 with_capacity()");

    // 7. 避免常见性能陷阱
    println!("\n7. 常见性能陷阱:");
    println!("  - 在循环中重复分配：重用集合");
    println!("  - 不必要的克隆：使用引用");
    println!("  - 过度的集合转换：保持数据结构");
    println!("  - 忽略迭代器链：使用链式操作");
    println!("  - 不合理的查找算法：根据数据特征选择");

    println!();
}

// ===========================================
// 7. 综合实例：数据流处理系统 (Comprehensive Example: Data Processing Pipeline)
// ===========================================

// 展示如何综合运用各种集合类型构建一个完整的数据处理系统
// 这个示例将展示集合在实际应用中的协作和优化

use std::collections::{HashMap, HashSet, VecDeque};

// 定义数据结构（在模块级别以便测试访问）
#[derive(Debug, Clone)]
struct Event {
    id: String,
    event_type: String,
    timestamp: u64,
    user_id: String,
    data: HashMap<String, String>,
}

#[derive(Debug)]
struct ProcessingRule {
    name: String,
    conditions: Vec<String>,
    actions: Vec<String>,
    priority: u32,
}

// 事件处理器
struct EventProcessor {
    // 待处理事件队列（需要处理顺序）
    event_queue: VecDeque<Event>,

    // 已处理事件缓存（快速查找）
    processed_cache: HashSet<String>,

    // 用户事件统计（快速聚合）
    user_stats: HashMap<String, (u32, u64)>, // (event_count, last_timestamp)

    // 事件类型统计（有序遍历）
    event_type_stats: HashMap<String, u32>,

    // 处理规则（有序优先级）
    rules: Vec<ProcessingRule>,
}

impl EventProcessor {
    fn new() -> Self {
        EventProcessor {
            event_queue: VecDeque::with_capacity(1000),
            processed_cache: HashSet::with_capacity(1000),
            user_stats: HashMap::with_capacity(100),
            event_type_stats: HashMap::with_capacity(50),
            rules: Vec::new(),
        }
    }

    // 添加事件到队列
    fn add_event(&mut self, event: Event) {
        // 检查是否已处理
        if self.processed_cache.contains(&event.id) {
            println!("事件 {} 已处理，跳过", event.id);
            return;
        }

        self.event_queue.push_back(event);
    }

    // 批量添加事件
    fn add_events(&mut self, events: Vec<Event>) {
        for event in events {
            self.add_event(event);
        }
    }

    // 添加处理规则
    fn add_rule(&mut self, rule: ProcessingRule) {
        self.rules.push(rule);
        // 按优先级排序
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    // 处理单个事件
    fn process_event(&mut self, event: &Event) -> Result<String, String> {
        let mut results = Vec::new();

        // 应用处理规则
        for rule in &self.rules {
            let mut conditions_met = true;

            // 检查所有条件
            for condition in &rule.conditions {
                if condition.starts_with("type:") {
                    let expected_type = &condition[5..];
                    if event.event_type != expected_type {
                        conditions_met = false;
                        break;
                    }
                } else if condition.starts_with("user:") {
                    let user_prefix = &condition[5..];
                    if !event.user_id.starts_with(user_prefix) {
                        conditions_met = false;
                        break;
                    }
                }
            }

            // 如果条件满足，执行动作
            if conditions_met {
                for action in &rule.actions {
                    let result = self.execute_action(event, action);
                    results.push(result);
                }
            }
        }

        // 更新统计信息
        self.update_stats(event);

        // 标记为已处理
        self.processed_cache.insert(event.id.clone());

        Ok(results.join(", "))
    }

    // 执行动作
    fn execute_action(&self, event: &Event, action: &str) -> String {
        match action {
            "log" => format!("记录事件: {}", event.id),
            "alert" => format!("警报: 用户 {} 的 {} 事件", event.user_id, event.event_type),
            "count" => format!("计数: {}", event.event_type),
            _ => format!("未知动作: {}", action),
        }
    }

    // 更新统计信息
    fn update_stats(&mut self, event: &Event) {
        // 更新用户统计
        let user_entry = self
            .user_stats
            .entry(event.user_id.clone())
            .or_insert((0, 0));
        user_entry.0 += 1;
        user_entry.1 = event.timestamp;

        // 更新事件类型统计
        *self
            .event_type_stats
            .entry(event.event_type.clone())
            .or_insert(0) += 1;
    }

    // 处理所有待处理事件
    fn process_all_events(&mut self) -> Vec<Result<String, String>> {
        let mut results = Vec::new();

        while let Some(event) = self.event_queue.pop_front() {
            let result = self.process_event(&event);
            results.push(result);
        }

        results
    }

    // 获取统计报告
    fn get_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== 数据处理报告 ===\n");

        // 处理的事件数量
        report.push_str(&format!("已处理事件数: {}\n", self.processed_cache.len()));

        // 事件类型统计（按数量排序）
        let mut type_stats: Vec<_> = self.event_type_stats.iter().collect();
        type_stats.sort_by(|a, b| b.1.cmp(a.1));

        report.push_str("事件类型统计:\n");
        for (event_type, count) in type_stats {
            report.push_str(&format!("  {}: {}\n", event_type, count));
        }

        // 用户活动统计（按时间排序）
        let mut user_stats: Vec<_> = self.user_stats.iter().collect();
        user_stats.sort_by(|a, b| b.1.1.cmp(&a.1.1));

        report.push_str("用户活动统计:\n");
        for (user_id, (count, timestamp)) in user_stats.iter().take(5) {
            report.push_str(&format!(
                "  {}: {} 事件，最后活动: {}\n",
                user_id, count, timestamp
            ));
        }

        // 队列状态
        report.push_str(&format!("待处理队列大小: {}\n", self.event_queue.len()));
        report.push_str(&format!("活跃规则数: {}\n", self.rules.len()));

        report
    }
}

fn data_processing_pipeline_system() {
    println!("=== 综合实例：数据流处理系统 ===");

    use std::collections::{HashMap, HashSet, VecDeque};
    use std::time::{SystemTime, UNIX_EPOCH};

    // 创建模拟数据
    let generate_events = || {
        let mut events = Vec::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for i in 0..20 {
            let event = Event {
                id: format!("event_{}", i),
                event_type: if i % 3 == 0 {
                    "login".to_string()
                } else if i % 3 == 1 {
                    "purchase".to_string()
                } else {
                    "logout".to_string()
                },
                timestamp: current_time + i,
                user_id: format!("user_{}", i % 5),
                data: {
                    let mut data = HashMap::new();
                    data.insert("value".to_string(), i.to_string());
                    data
                },
            };
            events.push(event);
        }
        events
    };

    // 创建处理规则
    let create_rules = || {
        vec![
            ProcessingRule {
                name: "登录记录".to_string(),
                conditions: vec!["type:login".to_string()],
                actions: vec!["log".to_string(), "count".to_string()],
                priority: 1,
            },
            ProcessingRule {
                name: "购买警报".to_string(),
                conditions: vec!["type:purchase".to_string()],
                actions: vec!["alert".to_string(), "log".to_string()],
                priority: 2,
            },
            ProcessingRule {
                name: "VIP用户处理".to_string(),
                conditions: vec!["user:user_0".to_string(), "type:purchase".to_string()],
                actions: vec!["alert".to_string()],
                priority: 3,
            },
        ]
    };

    // 运行数据处理系统
    let mut processor = EventProcessor::new();

    // 添加规则
    for rule in create_rules() {
        processor.add_rule(rule);
    }

    // 添加事件
    let events = generate_events();
    processor.add_events(events);

    println!("初始状态:");
    println!("队列大小: {}", processor.event_queue.len());
    println!("规则数量: {}", processor.rules.len());

    // 处理事件
    println!("\n处理事件:");
    let results = processor.process_all_events();

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(msg) => println!("事件 {} 处理成功: {}", i, msg),
            Err(e) => println!("事件 {} 处理失败: {}", i, e),
        }
    }

    // 显示报告
    println!("\n{}", processor.get_report());

    // 系统特点展示：
    // 1. 使用 VecDeque 保证处理顺序
    // 2. 使用 HashSet 快速检测重复事件
    // 3. 使用 HashMap 高效统计和查找
    // 4. 使用 Vec 存储有序规则
    // 5. 展示了集合类型的协作和优化

    println!();
}





// ===========================================
// Rust 集合教程
// ===========================================

pub fn main() {
    println!("Rust 集合类型深度解析");
    println!("=====================");

    vector_deep_dive();
    string_deep_dive();
    hashmap_deep_dive();
    other_collections_detailed();
    iterators_deep_dive();
    collection_performance_optimization();
    data_processing_pipeline_system();

    println!("集合类型解析完成！");
    println!("\n关键要点总结:");
    println!("1. Vec 是最常用的集合，提供连续内存存储和随机访问");
    println!("2. String 是 UTF-8 编码的文本类型，正确处理 Unicode 很重要");
    println!("3. HashMap 提供快速键值查找，但需要注意哈希碰撞处理");
    println!("4. 迭代器提供了惰性求值和链式操作的能力");
    println!("5. 选择合适的集合类型对性能至关重要");
    println!("6. 预分配容量和批量操作可以显著提升性能");
    println!("7. 理解所有权语义是正确使用集合的基础");
    println!("8. 实际应用中通常需要多种集合类型协作");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let mut v = Vec::with_capacity(5);
        assert_eq!(v.capacity(), 5);

        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);

        let popped = v.pop();
        assert_eq!(popped, Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_string_operations() {
        let mut s = String::with_capacity(10);
        assert_eq!(s.capacity(), 10);

        s.push_str("hello");
        assert_eq!(s, "hello");
        assert_eq!(s.len(), 5);

        s.push('!');
        assert_eq!(s, "hello!");

        assert!(s.contains("hello"));
        assert!(!s.contains("world"));
    }

    #[test]
    fn test_hashmap_operations() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert("Alice".to_string(), 95);
        scores.insert("Bob".to_string(), 87);

        assert_eq!(scores.len(), 2);
        assert_eq!(scores.get("Alice"), Some(&95));

        // 测试 entry API
        let score = scores.entry("Charlie".to_string()).or_insert(0);
        assert_eq!(*score, 0);
        *score += 90;
        assert_eq!(scores.get("Charlie"), Some(&90));
    }

    #[test]
    fn test_iterator_chaining() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let result: Vec<i32> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * x)
            .take(3)
            .collect();

        assert_eq!(result, vec![4, 16, 36]);
    }

    #[test]
    fn test_set_operations() {
        use std::collections::HashSet;
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

        let union: HashSet<i32> = set1.union(&set2).cloned().collect();
        assert_eq!(union.len(), 5);

        let intersection: HashSet<i32> = set1.intersection(&set2).cloned().collect();
        assert_eq!(intersection.len(), 1);
        assert!(intersection.contains(&3));
    }

    #[test]
    fn test_string_unicode() {
        let text = "你好世界 Hello World 😊";

        // 测试字符计数
        let char_count = text.chars().count();
        assert!(char_count > 10);

        // 测试字节访问
        let bytes = text.as_bytes();
        assert!(bytes.len() > char_count);

        // 测试安全切片
        assert!(text.is_char_boundary(0));
        assert!(text.is_char_boundary(3)); // "你" 占 3 字节
    }

    #[test]
    fn test_performance_optimization() {
        // 测试预分配容量的效果
        let mut vec1 = Vec::new();
        for i in 0..1000 {
            vec1.push(i);
        }

        let mut vec2 = Vec::with_capacity(1000);
        for i in 0..1000 {
            vec2.push(i);
        }

        // 预分配的容量应该 >= 元素数量
        assert!(vec2.capacity() >= 1000);
    }

    #[test]
    fn test_event_processor() {
        use std::collections::HashMap;

        let mut processor = EventProcessor::new();

        // 添加测试事件
        let event = Event {
            id: "test_event".to_string(),
            event_type: "test".to_string(),
            timestamp: 12345,
            user_id: "test_user".to_string(),
            data: HashMap::new(),
        };

        processor.add_event(event.clone());
        assert_eq!(processor.event_queue.len(), 1);

        // 处理事件
        let results = processor.process_all_events();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        // 测试重复检测
        processor.add_event(event);
        assert_eq!(processor.event_queue.len(), 0); // 应该被跳过
    }
}
