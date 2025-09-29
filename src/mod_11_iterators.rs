// Rust 迭代器（Iterators）
// 深入讲解迭代器概念、适配器、消费者、自定义迭代器等函数式编程特性

// ===========================================
// 1. 迭代器基础 (Iterator Basics)
// ===========================================

// 迭代器是 Rust 中函数式编程的核心概念，它提供了一种统一的方式来遍历数据集合
// 迭代器模式允许我们以声明式的方式处理数据，使代码更加简洁和可读
// Rust 的迭代器是惰性的，只有在需要时才会计算下一个值，这提供了很多优化机会

fn iterator_basics() {
    println!("=== 迭代器基础 ===");

    // 创建迭代器 - iter() 方法
    // iter() 方法创建一个不可变引用的迭代器
    // 这是最常用的迭代方式，因为它不会消耗原始数据
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // 使用 next() 方法
    // next() 是迭代器的核心方法，返回 Option<Item>
    // Some(Item) 表示还有下一个元素，None 表示迭代结束
    println!("使用 next() 遍历:");
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    // for 循环遍历
    // for 循环实际上是迭代器的语法糖，它会自动调用 next() 直到 None
    // 这是使用迭代器最自然和常见的方式
    let v1 = vec![1, 2, 3];
    println!("使用 for 循环:");
    for val in v1.iter() {
        println!("Got: {}", val);
    }

    // 可变迭代 - iter_mut()
    // iter_mut() 创建一个可变引用的迭代器，允许修改集合中的元素
    // 当需要修改原始数据时使用这种方式
    let mut v1 = vec![1, 2, 3];
    println!("使用 iter_mut() 修改:");
    for val in v1.iter_mut() {
        *val += 1;
        println!("Modified to: {}", val);
    }
    println!("修改后的向量: {:?}", v1);

    // 消耗性迭代 - into_iter()
    // into_iter() 创建一个消耗所有权的迭代器，它会移动每个元素的所有权
    // 当需要转移数据的所有权时使用这种方式
    let v1 = vec![1, 2, 3];
    println!("使用 into_iter() 消耗:");
    for val in v1.into_iter() {
        println!("Consumed: {}", val);
    }
    // println!("{:?}", v1); // 编译错误，v1 已被消耗

    // 三种迭代方式的比较：
    // 1. iter() - 不可变借用 (&T)：最安全，不改变原始数据
    // 2. iter_mut() - 可变借用 (&mut T)：可以修改原始数据
    // 3. into_iter() - 所有权转移 (T)：获取元素的所有权

    // 迭代器的优势：
    // 1. 抽象性：统一的接口遍历不同类型的数据集合
    // 2. 组合性：可以通过适配器链式组合各种操作
    // 3. 惰性：只有在需要时才计算，提高效率
    // 4. 安全性：编译时保证不会越界访问

    println!();
}

// ===========================================
// 2. 迭代器适配器 (Iterator Adapters)
// ===========================================

// 迭代器适配器是返回新迭代器的方法，它们可以链式组合形成复杂的数据处理管道
// 适配器是惰性的，只有在调用消费者方法时才会真正执行计算
// 这种惰性求值特性使得迭代器链可以高效地处理无限数据流

fn iterator_adapters() {
    println!("=== 迭代器适配器 ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map - 转换每个元素
    // map 是最常用的适配器，它对每个元素应用闭包并返回新的迭代器
    // 这种函数式转换使得数据处理更加清晰和可组合
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("map 翻倍: {:?}", doubled);

    // filter - 过滤元素
    // filter 接受一个返回 bool 的闭包，只保留满足条件的元素
    // 这是函数式编程中的过滤操作
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter 偶数: {:?}", evens);

    // take - 取前 n 个元素
    // take 限制迭代器只产生前 n 个元素，之后返回 None
    // 这在处理可能无限的数据流时特别有用
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("take 前三个: {:?}", first_three);

    // skip - 跳过前 n 个元素
    // skip 忽略前 n 个元素，从第 n+1 个元素开始返回
    // 这在分页处理数据时很有用
    let after_five: Vec<&i32> = numbers.iter().skip(5).collect();
    println!("skip 跳过前五个: {:?}", after_five);

    // chain - 连接两个迭代器
    // chain 将两个迭代器按顺序连接成一个迭代器
    // 这在合并多个数据源时很有用
    let more_numbers = vec![11, 12, 13];
    let chained: Vec<&i32> = numbers.iter().chain(more_numbers.iter()).collect();
    println!("chain 连接: {:?}", chained);

    // zip - 配对两个迭代器
    // zip 将两个迭代器组合成元组的迭代器，以较短的迭代器为准
    // 这在需要同时处理两个相关序列时很有用
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("zip 配对: {:?}", pairs);

    // enumerate - 添加索引
    // enumerate 为每个元素添加索引，从 0 开始
    // 这在需要知道元素位置时很有用
    let enumerated: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();
    println!("enumerate 带索引: {:?}", enumerated);

    // rev - 反向迭代
    // rev 反转迭代器的顺序，要求迭代器实现了 DoubleEndedIterator trait
    // 这在需要反向处理数据时很有用
    let reversed: Vec<&i32> = numbers.iter().rev().collect();
    println!("rev 反向: {:?}", reversed);

    // cycle - 循环迭代
    // cycle 无限重复迭代器的元素，通常与其他适配器一起使用
    // 注意：cycle 只适用于有限迭代器，否则会导致无限循环
    let cycled: Vec<&i32> = numbers.iter().cycle().take(15).collect();
    println!("cycle 循环（前15个）: {:?}", cycled);

    // 其他有用的适配器：
    // 1. flat_map - 将每个元素转换为迭代器然后展平
    // 2. inspect - 对每个元素执行操作但不改变它
    // 3. filter_map - 结合 filter 和 map，返回 Option<T>
    // 4. while_some - 迭代直到 Some 为 None
    // 5. take_while - 条件为真时取元素
    // 6. skip_while - 条件为真时跳过元素
    // 7. fuse - 在第一次 None 后总是返回 None
    // 8. peekable - 创建可以预览下一个元素的迭代器

    // 适配器链的特性：
    // 1. 惰性求值：所有操作都是惰性的
    // 2. 链式组合：可以无限链式组合
    // 3. 零成本抽象：编译器会优化掉中间步骤
    // 4. 类型安全：编译时检查类型正确性

    println!();
}

// ===========================================
// 3. 迭代器消费者 (Iterator Consumers)
// ===========================================

// 迭代器消费者是消耗迭代器并产生最终结果的方法
// 消费者会触发惰性适配器的实际计算，将迭代器转换为具体的值
// 理解消费者对于有效使用迭代器链至关重要

fn iterator_consumers() {
    println!("=== 迭代器消费者 ===");

    let numbers = vec![1, 2, 3, 4, 5];

    // collect - 收集到集合
    // collect 是最通用的消费者，可以将迭代器转换为各种集合类型
    // 它可以推断目标类型，也可以显式指定类型注解
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("collect 收集: {:?}", doubled);

    // fold - 折叠操作
    // fold 是最强大的消费者，它使用累加器函数将所有元素折叠成一个值
    // 第一个参数是初始值，第二个参数是接受累加器和当前元素的闭包
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold 求和: {}", sum);

    // reduce - 归约操作
    // reduce 类似于 fold，但使用第一个元素作为初始值
    // 返回 Option<T> 因为迭代器可能为空
    let product: Option<i32> = numbers.iter().copied().reduce(|acc, x| acc * x);
    println!("reduce 乘积: {:?}", product);

    // find - 查找元素
    // find 返回第一个满足条件的元素，找不到时返回 None
    // 一旦找到满足条件的元素就停止迭代
    let even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("find 查找偶数: {:?}", even);

    // position - 查找位置
    // position 返回第一个满足条件的元素的索引
    // 这在需要知道元素位置时很有用
    let pos = numbers.iter().position(|&x| x == 3);
    println!("position 查找3的位置: {:?}", pos);

    // any - 任意元素满足条件
    // any 检查是否有任意元素满足条件，一旦找到就返回 true
    // 这是短路操作，不会遍历所有元素
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("any 是否有偶数: {}", has_even);

    // all - 所有元素满足条件
    // all 检查是否所有元素都满足条件，一旦发现不满足的就返回 false
    // 这也是短路操作，提高效率
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("all 都为正数: {}", all_positive);

    // count - 计数
    // count 返回迭代器中元素的数量
    // 对于有限迭代器，这会消耗整个迭代器
    let count = numbers.iter().count();
    println!("count 计数: {}", count);

    // sum - 求和
    // sum 计算所有元素的和，要求元素实现了 Sum trait
    // 这是 fold 的特化版本，专门用于求和
    let sum: i32 = numbers.iter().sum();
    println!("sum 求和: {}", sum);

    // product - 乘积
    // product 计算所有元素的乘积，要求元素实现了 Product trait
    // 这也是 fold 的特化版本
    let product: i32 = numbers.iter().product();
    println!("product 乘积: {}", product);

    // min - 最小值
    // min 找出最小值，要求元素实现了 Ord trait
    // 返回 Option<T> 因为迭代器可能为空
    let min = numbers.iter().min();
    println!("min 最小值: {:?}", min);

    // max - 最大值
    // max 找出最大值，要求元素实现了 Ord trait
    // 返回 Option<T> 因为迭代器可能为空
    let max = numbers.iter().max();
    println!("max 最大值: {:?}", max);

    // 其他有用的消费者：
    // 1. for_each - 对每个元素执行操作，但不收集结果
    // 2. try_fold - 带错误处理的 fold，遇到错误就停止
    // 3. try_for_each - 带错误处理的 for_each
    // 4. last - 返回最后一个元素
    // 5. nth - 返回第 n 个元素
    // 6. step_by - 每隔 n 个元素取一个
    // 7. partition - 将元素分为满足和不满足条件的两个集合

    // 消费者的选择策略：
    // 1. collect - 需要集合结果时
    // 2. fold/reduce - 需要聚合操作时
    // 3. find/position - 需要查找时
    // 4. any/all - 需要检查条件时
    // 5. count/sum/product - 需要统计信息时
    // 6. min/max - 需要极值时

    println!();
}

// ===========================================
// 4. 自定义迭代器 (Custom Iterators)
// ===========================================

// 自定义迭代器是 Rust 中一个强大的特性，它允许我们为任何类型实现迭代行为
// 通过实现 Iterator trait，我们可以创建符合 Rust 迭代器模式的自定义数据序列
// 自定义迭代器可以与标准库的所有适配器和消费者无缝配合使用

fn custom_iterators() {
    println!("=== 自定义迭代器 ===");

    // 示例 1: 简单的计数器迭代器
    // 这是一个自定义类型，实现了从 1 到 5 的计数功能
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    // 实现 Iterator trait
    // 这是自定义迭代器的核心，必须实现 next() 方法和定义 Item 类型
    impl Iterator for Counter {
        type Item = u32;  // 迭代器产生的元素类型

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)  // 返回 Some 表示还有元素
            } else {
                None  // 返回 None 表示迭代结束
            }
        }
    }

    // 使用自定义迭代器
    // 一旦实现了 Iterator trait，就可以使用 for 循环遍历
    let counter = Counter::new();
    println!("自定义计数器:");
    for count in counter {
        println!("count = {}", count);
    }

    // 使用适配器
    // 自定义迭代器可以与所有标准库适配器配合使用
    let counter = Counter::new();
    let sum: u32 = counter.take(3).sum();
    println!("前3个数的和: {}", sum);

    // 示例 2: 斐波那契数列迭代器
    // 这是一个更复杂的例子，展示了如何在迭代器中维护状态
    struct Fibonacci {
        a: u32,
        b: u32,
    }

    impl Fibonacci {
        fn new() -> Fibonacci {
            Fibonacci { a: 0, b: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let next = self.a;
            self.a = self.b;
            // 使用 checked_add 防止溢出，返回 Option<u32>
            self.b = self.b.checked_add(next)?;
            Some(next)
        }
    }

    let fib = Fibonacci::new();
    println!("斐波那契数列:");
    for num in fib.take(10) {
        println!("fib = {}", num);
    }

    // 示例 3: 自定义范围迭代器
    // 模拟标准库的 range 功能，展示迭代器的基本原理
    struct Range {
        start: i32,
        end: i32,
    }

    impl Range {
        fn new(start: i32, end: i32) -> Range {
            Range { start, end }
        }
    }

    impl Iterator for Range {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.start < self.end {
                let current = self.start;
                self.start += 1;
                Some(current)
            } else {
                None
            }
        }
    }

    let range = Range::new(5, 10);
    println!("自定义范围迭代器:");
    for num in range {
        println!("range = {}", num);
    }

    // 自定义迭代器的最佳实践：
    // 1. 保持简单：next() 方法应该简洁明了
    // 2. 处理边界：正确处理迭代开始和结束的条件
    // 3. 错误处理：使用 Option 处理可能的错误情况
    // 4. 性能考虑：避免在 next() 中进行昂贵的计算
    // 5. 组合性：考虑如何与其他迭代器组件配合

    // 高级迭代器模式：
    // 1. DoubleEndedIterator: 支持反向迭代
    // 2. ExactSizeIterator: 知道确切的元素数量
    // 3. FusedIterator: 在第一次 None 后总是返回 None
    // 4. TrustedLen: 提供长度信息的可信迭代器

    println!();
}

// ===========================================
// 5. 迭代器链式调用 (Iterator Chaining)
// ===========================================

// 迭代器链式调用是 Rust 函数式编程的精髓，它允许我们将多个操作组合成一个数据处理管道
// 每个适配器都返回一个新的迭代器，可以继续链式调用其他适配器
// 这种方式使得复杂的转换逻辑变得清晰、可读且高效

fn iterator_chaining() {
    println!("=== 迭代器链式调用 ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 复杂的链式操作
    // 演示了多个适配器的组合使用，每个步骤都有明确的语义
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x > 3)           // 过滤大于3的
        .map(|&x| x * 2)               // 翻倍
        .take(4)                       // 取前4个
        .collect();                    // 收集结果
    println!("复杂链式操作: {:?}", result);

    // 数据处理管道 - 学生成绩分析
    // 展示了迭代器在数据处理中的实际应用
    let data = vec![
        ("Alice", 85),
        ("Bob", 92),
        ("Charlie", 78),
        ("David", 88),
        ("Eve", 95),
    ];

    // 找出成绩大于80的学生姓名
    // 展示了数据筛选和提取的链式操作
    let excellent_students: Vec<&str> = data.iter()
        .filter(|&&(_, score)| score > 80)
        .map(|&(name, _)| name)
        .collect();
    println!("优秀学生: {:?}", excellent_students);

    // 计算平均成绩
    // 展示了数值计算和类型转换的链式操作
    let average_score: f64 = data.iter()
        .map(|&(_, score)| score as f64)
        .sum::<f64>() / data.len() as f64;
    println!("平均成绩: {:.2}", average_score);

    // 找出最高分学生
    // 展示了最大值查找和结果格式化
    let top_student = data.iter()
        .max_by_key(|&&(_, score)| score)
        .map(|&(name, score)| format!("{}: {}", name, score));
    println!("最高分学生: {:?}", top_student);

    // 文本处理链 - 词频统计
    // 展示了迭代器在文本处理中的强大能力
    let text = "Hello world! Rust is awesome. Hello iterators!";
    let word_count: std::collections::HashMap<String, usize> = text
        .split_whitespace()                          // 分割单词
        .map(|word| word.to_lowercase())             // 转换为小写
        .map(|word| word.trim_end_matches(&['.', '!', '?', ',', ';', ':'][..]).to_string())  // 移除标点并转换为String
        .filter(|word| !word.is_empty())             // 过滤空单词
        .fold(std::collections::HashMap::new(), |mut map, word| {  // 统计词频
            *map.entry(word).or_insert(0) += 1;
            map
        });

    println!("词频统计: {:?}", word_count);

    // 数字处理链 - 条件求和
    // 展示了数学计算的链式操作
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = numbers.iter()
        .filter(|&&x| x % 2 == 0)        // 过滤偶数
        .map(|&x| x * x)                // 计算平方
        .take(3)                        // 取前3个
        .sum::<i32>();                  // 求和
    println!("偶数平方和（前3个）: {}", result);

    // 链式调用的优势：
    // 1. 可读性：每个步骤都有明确的语义
    // 2. 组合性：可以自由组合各种操作
    // 3. 效率：编译器会优化链式调用
    // 4. 惰性：只有在需要时才执行计算
    // 5. 类型安全：编译时检查类型正确性

    // 链式调用的最佳实践：
    // 1. 过滤在前：先过滤数据再进行转换，减少不必要的计算
    // 2. 转换居中：对过滤后的数据进行转换
    // 3. 限制在后：使用 take 等限制处理的数据量
    // 4. 最后收集：使用 collect 或其他消费者获取结果

    // 常见的链式调用模式：
    // 1. 过滤 + 转换 + 收集
    // 2. 映射 + 折叠 + 聚合
    // 3. 分组 + 统计 + 分析
    // 4. 验证 + 转换 + 处理

    println!();
}

// ===========================================
// 6. 迭代器性能优化 (Iterator Performance)
// ===========================================

// 迭代器在 Rust 中被设计为零成本抽象，但在实际使用中仍有一些性能考虑
// 理解这些性能特性有助于编写更高效的 Rust 代码
// 本节将探讨迭代器与循环的比较、惰性求值的优势以及优化技巧

fn iterator_performance() {
    println!("=== 迭代器性能优化 ===");

    let large_vec: Vec<i32> = (0..10000).collect();

    // 迭代器 vs for 循环性能比较
    // 测试两种不同风格的性能差异
    let start = std::time::Instant::now();
    let sum_iter: i32 = large_vec.iter().sum();
    let iter_time = start.elapsed();

    let start = std::time::Instant::now();
    let mut sum_for = 0;
    for &x in &large_vec {
        sum_for += x;
    }
    let for_time = start.elapsed();

    println!("迭代器求和: {}, 耗时: {:?}", sum_iter, iter_time);
    println!("for循环求和: {}, 耗时: {:?}", sum_for, for_time);

    // 惰性求值的优势
    // 惰性求值可以避免不必要的计算和中间存储
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 链式操作（惰性）- 效率更高
    // 所有操作在一个遍历中完成，没有中间分配
    let start = std::time::Instant::now();
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x > 5)
        .map(|&x| x * 2)
        .take(2)
        .collect();
    let chain_time = start.elapsed();

    // 非惰性操作（多次分配）- 效率较低
    // 每个步骤都需要分配中间集合
    let start = std::time::Instant::now();
    let filtered: Vec<&i32> = numbers.iter().filter(|&&x| x > 5).collect();
    let mapped: Vec<i32> = filtered.iter().map(|&&x| x * 2).collect();
    let taken: Vec<i32> = mapped.into_iter().take(2).collect();
    let manual_time = start.elapsed();

    println!("惰性链式操作: {:?}, 耗时: {:?}", result, chain_time);
    println!("手动分步操作: {:?}, 耗时: {:?}", taken, manual_time);

    // 迭代器内联优化
    // 对比传统的 for 循环和函数式迭代器风格的性能
    fn process_with_for_loop(data: &[i32]) -> i32 {
        let mut sum = 0;
        for &x in data {
            if x > 5 {
                sum += x * 2;
            }
        }
        sum
    }

    fn process_with_iterators(data: &[i32]) -> i32 {
        data.iter()
            .filter(|&&x| x > 5)
            .map(|&x| x * 2)
            .sum()
    }

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let start = std::time::Instant::now();
    let for_result = process_with_for_loop(&data);
    let for_duration = start.elapsed();

    let start = std::time::Instant::now();
    let iter_result = process_with_iterators(&data);
    let iter_duration = start.elapsed();

    println!("for循环结果: {}, 耗时: {:?}", for_result, for_duration);
    println!("迭代器结果: {}, 耗时: {:?}", iter_result, iter_duration);

    // 迭代器性能的关键因素：
    // 1. 惰性求值：避免不必要的计算和中间分配
    // 2. 内联优化：编译器会内联小的闭包和适配器
    // 3. 单态化：泛型迭代器会为每个类型生成专门代码
    // 4. 零成本抽象：迭代器调用没有运行时开销

    // 性能优化策略：
    // 1. 优先使用迭代器链：避免中间集合分配
    // 2. 合理使用 take/skip：限制处理的数据量
    // 3. 过滤在前：先过滤再转换，减少计算量
    // 4. 避免过度收集：只在需要时调用 collect
    // 5. 使用专用消费者：sum/product 等比 fold 更高效

    // 性能陷阱：
    // 1. 过早收集：在不需要时调用 collect
    // 2. 重复计算：对相同数据多次遍历
    // 3. 复杂闭包：在适配器中使用复杂的闭包
    // 4. 不必要的克隆：避免在闭包中克隆数据
    // 5. 动态分发：过度使用 trait 对象

    println!();
}

// ===========================================
// 7. 迭代器模式 (Iterator Patterns)
// ===========================================

// 迭代器模式是在实际编程中反复出现的问题解决方案
// 这些模式利用了迭代器的强大能力，提供了优雅而高效的代码实现
// 掌握这些模式有助于我们更好地应用迭代器来解决实际问题

fn iterator_patterns() {
    println!("=== 迭代器模式 ===");

    // 模式 1: 数据转换管道
    // 将原始数据通过一系列转换步骤，最终得到所需的结果
    // 这是最常见的迭代器模式，特别适合数据处理工作流
    fn process_numbers(numbers: &[i32]) -> Vec<String> {
        numbers.iter()
            .filter(|&&x| x > 0)                    // 过滤负数
            .map(|&x| x * 2)                        // 数值翻倍
            .map(|x| format!("处理后的数字: {}", x))  // 格式化输出
            .collect()                               // 收集结果
    }

    let numbers = vec![-1, 2, -3, 4, -5, 6];
    let processed = process_numbers(&numbers);
    println!("数据处理管道: {:?}", processed);

    // 模式 2: 分组统计
    // 使用 fold 和 HashMap 对数据进行分组统计
    // 这种模式在数据分析和统计中非常常见
    let data = vec!["apple", "banana", "apple", "orange", "banana", "apple"];
    let counts = data.iter()
        .fold(std::collections::HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;  // 统计每个元素的出现次数
            map
        });
    println!("分组统计: {:?}", counts);

    // 模式 3: 扁平化嵌套结构
    // 使用 flatten 将嵌套的数据结构展平
    // 这种模式在处理树形结构或嵌套集合时很有用
    let nested = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let flattened: Vec<i32> = nested.into_iter().flatten().collect();
    println!("扁平化嵌套结构: {:?}", flattened);

    // 模式 4: 窗口滑动
    // 使用 windows 方法创建滑动窗口，用于分析连续的数据序列
    // 这种模式在时间序列分析、信号处理等领域很常见
    let data = vec![1, 2, 3, 4, 5];
    let windows: Vec<Vec<i32>> = data.windows(2).map(|w| w.to_vec()).collect();
    println!("滑动窗口: {:?}", windows);

    // 模式 5: 条件求和
    // 使用 take_while 和 filter 结合实现复杂的条件聚合
    // 这种模式在满足特定条件的数据聚合中很有用
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let conditional_sum = numbers.iter()
        .take_while(|&&x| x < 8)               // 只处理小于8的数
        .filter(|&&x| x % 2 == 0)              // 只处理偶数
        .sum::<i32>();                         // 求和
    println!("条件求和（小于8的偶数）: {}", conditional_sum);

    // 模式 6: 数据验证
    // 使用 filter_map 结合解析操作，安全地验证和转换数据
    // 这种模式在处理外部数据输入时非常有用
    let inputs = vec!["42", "123", "abc", "789", "xyz"];
    let valid_numbers: Vec<i32> = inputs.iter()
        .filter_map(|s| s.parse().ok())         // 只保留成功解析的数字
        .collect();
    println!("数据验证和转换: {:?}", valid_numbers);

    // 模式 7: 批处理
    // 使用 chunks 方法将数据分批处理
    // 这种模式在处理大量数据时很有用，可以避免内存问题
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let batches: Vec<Vec<i32>> = items.chunks(3).map(|c| c.to_vec()).collect();
    println!("批处理（每批3个）: {:?}", batches);

    // 更多有用的迭代器模式：
    // 8. 数据去重：使用 collect 到 HashSet 再转换回 Vec
    // 9. 排序和分组：使用 sort_by 和 group_by
    // 10. 数据标准化：使用 map 进行数据转换和规范化
    // 11. 缓存计算结果：使用 memoization 模式
    // 12. 流式处理：使用迭代器处理无限数据流
    // 13. 并行处理：使用 par_iter 进行并行计算
    // 14. 惰性评估：延迟计算直到真正需要结果

    // 模式选择的考虑因素：
    // 1. 数据规模：大数据集优先使用惰性操作
    // 2. 内存使用：避免不必要的中间集合分配
    // 3. 计算复杂度：选择最合适的算法和操作顺序
    // 4. 代码可读性：优先选择最清晰的表达方式
    // 5. 性能要求：根据性能瓶颈选择优化策略

    println!();
}

// ===========================================
// 8. 迭代器示例程序 (Iterator Example Program)
// ===========================================

// 通过完整的示例程序来展示迭代器在实际应用中的强大能力
// 这些示例涵盖了文本分析、数据处理、日志分析等常见场景
// 每个示例都展示了迭代器的不同特性和最佳实践

fn iterator_example_program() {
    println!("=== 迭代器示例程序 ===");

    // 示例 1: 文本分析工具
    // 展示了迭代器在文本处理中的全面应用
    // 这个工具可以分析文本的各种统计信息
    struct TextAnalyzer {
        text: String,
    }

    impl TextAnalyzer {
        fn new(text: String) -> Self {
            TextAnalyzer { text }
        }

        // 计算单词数量
        fn word_count(&self) -> usize {
            self.text.split_whitespace().count()
        }

        // 计算字符数量（正确处理 Unicode）
        fn character_count(&self) -> usize {
            self.text.chars().count()
        }

        // 计算行数
        fn line_count(&self) -> usize {
            self.text.lines().count()
        }

        // 找出最常见的单词
        fn most_common_word(&self) -> Option<(String, usize)> {
            let mut counts = std::collections::HashMap::new();
            self.text.split_whitespace()
                .map(|word| word.to_lowercase())                 // 小写化
                .map(|word| word.trim_end_matches(&['.', '!', '?', ',', ';', ':'][..]).to_string())  // 移除标点并转换为String
                .filter(|word| !word.is_empty())                 // 过滤空单词
                .for_each(|word| {
                    *counts.entry(word).or_insert(0) += 1;       // 统计词频
                });

            counts.into_iter().max_by_key(|&(_, count)| count)    // 找出最大值
        }

        // 计算每个单词的长度
        fn word_lengths(&self) -> Vec<usize> {
            self.text.split_whitespace()
                .map(|word| word.chars().count())             // 正确计算 Unicode 字符数
                .collect()
        }

        // 获取不重复的单词列表
        fn unique_words(&self) -> Vec<String> {
            let mut words: Vec<String> = self.text.split_whitespace()
                .map(|word| word.to_lowercase())
                .map(|word| word.trim_end_matches(&['.', '!', '?', ',', ';', ':'][..]).to_string())
                .filter(|word| !word.is_empty())
                .collect();

            words.sort();                                      // 排序
            words.dedup();                                    // 去重
            words
        }
    }

    let sample_text = r#"
Rust is a systems programming language that runs blazingly fast,
prevents segfaults, and guarantees thread safety.
Rust is memory safe without garbage collection.
Rust empowers everyone to build reliable and efficient software.
    "#.trim();

    let analyzer = TextAnalyzer::new(sample_text.to_string());

    println!("=== 文本分析工具 ===");
    println!("词数: {}", analyzer.word_count());
    println!("字符数: {}", analyzer.character_count());
    println!("行数: {}", analyzer.line_count());

    if let Some((word, count)) = analyzer.most_common_word() {
        println!("最常见单词: '{}' (出现{}次)", word, count);
    }

    let word_lengths = analyzer.word_lengths();
    let avg_length = word_lengths.iter().sum::<usize>() as f64 / word_lengths.len() as f64;
    println!("平均词长: {:.2} 个字符", avg_length);

    let unique_words = analyzer.unique_words();
    println!("不重复单词数: {}", unique_words.len());
    println!("不重复单词: {:?}", unique_words);

    // 示例 2: 数据流处理
    // 展示了迭代器在数据处理中的错误处理和过滤能力
    fn process_sensor_data(data: &[f64]) -> (f64, f64, f64) {
        let valid_data = data.iter()
            .filter(|&&x| x >= 0.0 && x <= 100.0);  // 过滤有效数据

        let count = valid_data.clone().count() as f64;
        let average = valid_data.clone().sum::<f64>() / count;
        let max = valid_data.clone().fold(0.0, |a, &b| f64::max(a, b));
        let min = valid_data.fold(100.0, |a, &b| f64::min(a, b));

        (min, max, average)
    }

    let sensor_readings = vec![
        23.5, 24.1, 22.8, 25.3, 101.5, // 101.5 无效
        23.9, 24.5, -1.0, 22.1, 24.8,  // -1.0 无效
        23.2, 24.0, 23.7, 24.3, 23.8,
    ];

    println!("\n=== 传感器数据处理 ===");
    let (min_temp, max_temp, avg_temp) = process_sensor_data(&sensor_readings);
    println!("最低温度: {:.1}°C", min_temp);
    println!("最高温度: {:.1}°C", max_temp);
    println!("平均温度: {:.1}°C", avg_temp);

    // 示例 3: 日志分析
    // 展示了迭代器在日志处理和分析中的应用
    let log_lines = vec![
        "INFO: Application started",
        "DEBUG: Loading configuration",
        "WARN: Configuration file not found, using defaults",
        "INFO: Database connection established",
        "ERROR: Failed to connect to external service",
        "INFO: User login successful",
        "DEBUG: Processing request",
        "WARN: High memory usage detected",
        "ERROR: Request timeout",
        "INFO: Application shutting down",
    ];

    println!("\n=== 日志分析 ===");

    let log_counts = log_lines.iter()
        .map(|line| {
            if let Some(level) = line.split(':').next() {
                level
            } else {
                "UNKNOWN"
            }
        })
        .fold(std::collections::HashMap::new(), |mut map, level| {
            *map.entry(level).or_insert(0) += 1;
            map
        });

    println!("日志级别统计: {:?}", log_counts);

    let error_messages: Vec<&str> = log_lines.iter()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line| line.split(':').nth(1).unwrap_or("").trim())
        .collect();

    println!("错误消息: {:?}", error_messages);

    // 迭代器在实际项目中的应用场景：
    // 1. 数据分析：统计、聚合、过滤大量数据
    // 2. 文本处理：搜索、替换、格式化文本
    // 3. 网络编程：处理数据流和协议消息
    // 4. 数据库：查询结果的处理和转换
    // 5. 文件系统：遍历文件和目录结构
    // 6. 并发编程：处理消息队列和事件流
    // 7. 游戏开发：处理游戏状态和事件
    // 8. 科学计算：数值计算和数据分析

    // 迭代器编程的最佳实践：
    // 1. 优先使用迭代器而不是手动循环
    // 2. 合理使用链式调用避免中间分配
    // 3. 在适配器中使用简单的闭包
    // 4. 处理错误和异常情况
    // 5. 编写测试验证迭代器行为

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 迭代器演示");
    println!("===============");

    iterator_basics();
    iterator_adapters();
    iterator_consumers();
    custom_iterators();
    iterator_chaining();
    iterator_performance();
    iterator_patterns();
    iterator_example_program();

    println!("迭代器演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_next() {
        let v = vec![1, 2, 3];
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_adapters() {
        let v = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_iterator_consumers() {
        let v = vec![1, 2, 3, 4, 5];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 15);

        let count = v.iter().count();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_custom_iterator() {
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let counter = Counter::new();
        let values: Vec<u32> = counter.collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iterator_chaining() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result: Vec<i32> = v.iter()
            .filter(|&&x| x > 5)
            .map(|&x| x * 2)
            .take(3)
            .collect();
        assert_eq!(result, vec![12, 14, 16]);
    }

    #[test]
    fn test_text_analyzer() {
        struct TextAnalyzer {
            text: String,
        }

        impl TextAnalyzer {
            fn new(text: String) -> Self {
                TextAnalyzer { text }
            }

            fn word_count(&self) -> usize {
                self.text.split_whitespace().count()
            }

            fn character_count(&self) -> usize {
                self.text.chars().count()
            }
        }

        let analyzer = TextAnalyzer::new("Hello world Rust".to_string());
        assert_eq!(analyzer.word_count(), 3);
        assert_eq!(analyzer.character_count(), 16);
    }
}