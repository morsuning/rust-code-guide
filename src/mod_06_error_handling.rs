// 教程里的类型、字段和方法经常只为演示语法而定义，不一定会被全部读取或调用，所以允许 dead_code；
// 其他 lint（未使用的导入、变量、可变性等）必须保持干净，不要再给模块添加新的 allow。
#![allow(dead_code)]

// Rust 错误处理
// 深入讲解 Rust 错误处理系统，包括 panic、Result、Option、错误传播和自定义错误类型

// ===========================================
// 1. panic! 宏和不可恢复错误
// ===========================================

// panic! 是 Rust 中处理不可恢复错误的机制
// 当程序遇到无法继续执行的严重错误时，会触发 panic
// panic 会打印错误信息、清理栈，然后退出程序

fn panic_and_unrecoverable_errors() {
    println!("=== panic! 和不可恢复错误 ===");

    // panic! 的基本用法：当程序遇到无法恢复的错误时使用
    // panic!("crash and burn"); // 这会导致程序立即崩溃

    // 常见的 panic 情况：数组越界访问
    // let v = vec![1, 2, 3];
    // v[99]; // 这会触发 panic，因为索引越界

    // 带有上下文信息的 panic：提供更有用的错误信息
    fn guess_number(guess: i32) {
        if guess < 1 || guess > 100 {
            panic!("猜测值必须在 1 到 100 之间，当前值为 {}", guess);
        }
        println!("猜测值 {} 有效", guess);
    }

    // guess_number(150); // 这会触发 panic
    guess_number(50); // 这是有效的猜测

    // 断言宏：用于测试和调试
    // assert_eq! 和 assert! 在测试中非常有用
    let x = 5;
    assert!(x == 5); // 通过测试
    // assert!(x == 6); // 这会 panic，因为条件不成立

    // panic 的实际应用场景：
    // 1. 程序启动时的配置错误
    // 2. 不可能发生的情况（防御性编程）
    // 3. 测试失败的情况
    // 4. 内存分配失败等系统错误

    // panic 与异常处理的区别：
    // - Java/C# 等语言使用 try-catch 处理异常
    // - Rust 使用 panic 处理不可恢复的错误，用 Result 处理可恢复的错误
    // - panic 会终止当前线程，而异常可能被捕获并继续执行

    // panic 的性能考虑：
    // - panic 有一定的运行时开销
    // - 在性能关键路径上，应考虑使用 Result 类型
    // - panic 适用于真正异常的情况，不应作为正常的控制流

    println!();
}

// ===========================================
// 2. Result 类型基础
// ===========================================

// Result<T, E> 是 Rust 中处理可恢复错误的核心类型
// 它使得错误处理变得显式和类型安全

fn result_type_basics() {
    println!("=== Result 类型基础 ===");

    // Result 类型的定义：
    // enum Result<T, E> {
    //     Ok(T),  // 表示成功，包含成功的值
    //     Err(E), // 表示失败，包含错误信息
    // }

    // Result 的基本使用：除法函数
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("不能除以零"))
        } else {
            Ok(numerator / denominator)
        }
    }

    let result1 = divide(10.0, 2.0); // Ok(5.0)
    let result2 = divide(10.0, 0.0); // Err("不能除以零")

    println!("除法结果1: {:?}", result1);
    println!("除法结果2: {:?}", result2);

    // 使用 match 处理 Result：最基础和完整的处理方式
    match result1 {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(error) => println!("错误: {}", error),
    }

    match result2 {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(error) => println!("错误: {}", error),
    }

    // unwrap()：简单的值提取，但在错误时会 panic
    let value = divide(10.0, 2.0).unwrap();
    println!("unwrap 结果: {}", value);
    // let panic_value = divide(10.0, 0.0).unwrap(); // 这会 panic

    // expect()：带自定义错误消息的 unwrap
    let value = divide(10.0, 2.0).expect("除法失败");
    println!("expect 结果: {}", value);

    // unwrap_or()：提供默认值，不会 panic
    let value = divide(10.0, 0.0).unwrap_or(0.0);
    println!("unwrap_or 结果: {}", value);

    // unwrap_or_else()：使用闭包计算默认值
    let value = divide(10.0, 0.0).unwrap_or_else(|_| 0.0);
    println!("unwrap_or_else 结果: {}", value);

    // map()：对 Ok 值进行转换，保持 Err 不变
    let doubled = divide(10.0, 2.0).map(|x| x * 2.0);
    println!("map 转换: {:?}", doubled);

    // and_then()：链式操作，类似于 flatMap
    let chained = divide(10.0, 2.0).and_then(|x| divide(x, 5.0));
    println!("链式操作: {:?}", chained);

    // Result 的设计哲学：
    // - 错误是显式的，函数签名清楚表明可能失败
    // - 错误必须被处理，不能被忽略
    // - 类型系统确保错误的安全性
    // - 支持函数式编程风格的错误处理

    // 选择合适的方法：
    // - match：需要完整控制时
    // - unwrap/expect：确定不会失败或可以 panic 时
    // - unwrap_or：有合理的默认值时
    // - map/and_then：需要链式转换时

    println!();
}

// ===========================================
// 3. Option 类型基础
// ===========================================

// Option<T> 是 Rust 中处理可能不存在的值的类型
// 它是 null 的类型安全替代方案

fn option_type_basics() {
    println!("=== Option 类型基础 ===");

    // Option 类型的定义：
    // enum Option<T> {
    //     Some(T), // 存在值
    //     None,   // 不存在值
    // }

    // Option 的基本使用：在数组中查找元素
    fn find_element(vec: &Vec<i32>, target: i32) -> Option<usize> {
        for (i, &value) in vec.iter().enumerate() {
            if value == target {
                return Some(i);
            }
        }
        None
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let found = find_element(&numbers, 3); // Some(2)
    let not_found = find_element(&numbers, 6); // None

    println!("找到元素: {:?}", found);
    println!("未找到元素: {:?}", not_found);

    // 使用 match 处理 Option
    match found {
        Some(index) => println!("元素 3 在索引 {}", index),
        None => println!("未找到元素 3"),
    }

    // unwrap()：简单的值提取，None 时会 panic
    let value = found.unwrap();
    println!("unwrap 结果: {}", value);
    // let panic_value = not_found.unwrap(); // 这会 panic

    // unwrap_or()：提供默认值
    let value = not_found.unwrap_or(0);
    println!("unwrap_or 结果: {}", value);

    // unwrap_or_else()：使用闭包计算默认值
    let value = not_found.unwrap_or_else(|| 0);
    println!("unwrap_or_else 结果: {}", value);

    // map()：对 Some 值进行转换
    let doubled = found.map(|x| x * 2);
    println!("map 转换: {:?}", doubled);

    // and_then()：链式操作
    let chained = found.and_then(|i| numbers.get(i)).copied();
    println!("链式操作: {:?}", chained);

    // is_some() 和 is_none()：检查状态
    println!("found 是否为 Some: {}", found.is_some());
    println!("not_found 是否为 None: {}", not_found.is_none());

    // or() 和 or_else()：提供备选的 Option
    let fallback = not_found.or(Some(0));
    println!("or() 结果: {:?}", fallback);

    // Option 与 Result 的关系：
    // - Option<T> 可以看作是 Result<T, ()> 的简化版
    // - Option 用于处理值的缺失，Result 用于处理错误
    // - 两者都支持类似的操作方法

    // Option 的优势：
    // - 消除了空指针异常
    // - 类型系统保证安全性
    // - 强制显式处理缺失值的情况
    // - 支持函数式编程风格

    println!();
}

// ===========================================
// 4. 错误传播
// ===========================================

// 错误传播是 Rust 错误处理的核心机制
// ? 运算符使得错误传播变得简洁和优雅

fn error_propagation() {
    println!("=== 错误传播 ===");

    // 使用 ? 运算符传播错误
    // ? 是 Rust 中最常用的错误传播方式
    fn read_username_from_file() -> Result<String, std::io::Error> {
        let mut username_file = std::fs::File::open("hello.txt")?;
        let mut username = String::new();
        std::io::Read::read_to_string(&mut username_file, &mut username)?;
        Ok(username)
    }

    // ? 运算符的工作原理：
    // - 如果结果是 Ok(value)，返回 value
    // - 如果结果是 Err(error)，立即返回 Err(error)
    // - 相当于 match 的语法糖

    // 模拟文件操作（因为文件可能不存在）
    let result = read_username_from_file();
    println!("文件读取结果: {:?}", result);

    // 自定义错误传播函数
    fn process_data(data: &str) -> Result<i32, String> {
        // 使用 map_err 转换错误类型
        let number: i32 = data.parse().map_err(|e| format!("解析错误: {}", e))?;

        if number < 0 {
            return Err("数字不能为负数".to_string());
        }
        if number > 100 {
            return Err("数字不能超过 100".to_string());
        }
        Ok(number * 2)
    }

    let inputs = vec!["50", "150", "-10", "not_a_number"];

    for input in inputs {
        match process_data(input) {
            Ok(result) => println!("处理 '{}': {}", input, result),
            Err(error) => println!("处理 '{}' 错误: {}", input, error),
        }
    }

    // 错误传播的最佳实践：
    // 1. 在函数签名中明确错误类型
    // 2. 使用 ? 运算符简化代码
    // 3. 在适当的地方转换错误类型
    // 4. 提供有意义的错误信息

    // 错误传播 vs 异常处理：
    // - Java: throw new Exception() / catch (Exception e)
    // - Rust: Err(error) / ? 操作符
    // - Rust 的方式更类型安全，编译器会检查错误处理

    // 链式操作中的错误传播：
    fn complex_pipeline(input: &str) -> Result<String, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("输入为空".to_string());
        }

        let number: i32 = trimmed.parse().map_err(|e| format!("解析失败: {}", e))?;

        let result = number * 2;
        Ok(format!("处理结果: {}", result))
    }

    println!("\n测试错误传播链:");
    for test_input in vec!["42", "", "abc"] {
        match complex_pipeline(test_input) {
            Ok(result) => println!("'{}' -> {}", test_input, result),
            Err(e) => println!("'{}' -> 错误: {}", test_input, e),
        }
    }

    println!();
}

// ===========================================
// 5. 多种错误类型
// ===========================================

// 在实际项目中，函数可能产生多种不同类型的错误
// Rust 提供了多种方式来处理这种情况

fn multiple_error_types() {
    println!("=== 多种错误类型 ===");

    // 定义自定义错误类型：枚举是处理多种错误的常用方式
    #[derive(Debug)]
    enum MyError {
        Io(std::io::Error),             // IO 错误
        Parse(std::num::ParseIntError), // 解析错误
        Custom(String),                 // 自定义错误
    }

    // 实现 From trait 进行错误转换：这是 ? 运算符工作的关键
    impl From<std::io::Error> for MyError {
        fn from(err: std::io::Error) -> MyError {
            MyError::Io(err)
        }
    }

    impl From<std::num::ParseIntError> for MyError {
        fn from(err: std::num::ParseIntError) -> MyError {
            MyError::Parse(err)
        }
    }

    impl From<String> for MyError {
        fn from(err: String) -> MyError {
            MyError::Custom(err)
        }
    }

    // 使用自定义错误类型
    fn complex_operation(file_path: &str) -> Result<i32, MyError> {
        let mut file = std::fs::File::open(file_path)?; // 可能产生 Io 错误
        let mut content = String::new();
        std::io::Read::read_to_string(&mut file, &mut content)?;

        let number: i32 = content.trim().parse()?; // 可能产生 Parse 错误

        if number < 0 {
            return Err("数字不能为负数".to_string().into()); // 转换为 Custom 错误
        }

        Ok(number * 2)
    }

    // 测试不同的输入
    let test_paths = vec!["nonexistent.txt", "invalid_content.txt"];

    for path in test_paths {
        match complex_operation(path) {
            Ok(result) => println!("操作成功: {}", result),
            Err(error) => println!("操作失败: {:?}", error),
        }
    }

    // 错误类型统一的好处：
    // 1. 函数签名简洁，只有一个错误类型
    // 2. 调用者只需要处理一种错误类型
    // 3. 可以保留原始错误信息
    // 4. 便于错误日志记录和调试

    // 处理多种错误的其他方式：
    // 1. 使用 Box<dyn std::error::Error>：动态错误类型
    // 2. 使用第三方库（如 thiserror、anyhow）
    // 3. 使用 String 作为通用错误类型

    // Box<dyn Error> 的示例：
    fn generic_error_operation() -> Result<i32, Box<dyn std::error::Error>> {
        let number: i32 = "42".parse()?;
        Ok(number)
    }

    println!("\n使用 Box<dyn Error>: {:?}", generic_error_operation());

    // 选择错误类型的策略：
    // - 小型项目：使用 String 或 Box<dyn Error>
    // - 中型项目：定义自定义错误枚举
    // - 大型项目：使用 thiserror 或 anyhow 库
    // - 库代码：定义具体的错误类型，便于用户处理

    println!();
}

// ===========================================
// 6. 错误处理模式
// ===========================================

// 掌握常见的错误处理模式对于编写健壮的 Rust 代码至关重要

fn error_handling_patterns() {
    println!("=== 错误处理模式 ===");

    // 模式 1: 快速失败（Fail Fast）
    // 在函数开始时验证输入，失败时立即返回
    fn validate_input(input: &str) -> Result<(), String> {
        if input.is_empty() {
            return Err("输入不能为空".to_string());
        }
        if input.len() > 100 {
            return Err("输入不能超过 100 个字符".to_string());
        }
        Ok(())
    }

    // 模式 2: 链式操作（Method Chaining）
    // 使用 ? 运算符和链式调用处理多步操作
    fn process_chain(input: &str) -> Result<i32, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("输入为空".to_string());
        }

        let number: i32 = trimmed.parse().map_err(|e| format!("解析错误: {}", e))?;

        if number < 0 {
            return Err("数字不能为负数".to_string());
        }

        Ok(number * 2)
    }

    // 模式 3: 组合多个操作
    // 处理多个可能失败的操作
    fn combine_operations(a: &str, b: &str) -> Result<i32, String> {
        let num_a = a
            .parse::<i32>()
            .map_err(|e| format!("解析 a 失败: {}", e))?;

        let num_b = b
            .parse::<i32>()
            .map_err(|e| format!("解析 b 失败: {}", e))?;

        if num_a == 0 || num_b == 0 {
            return Err("除数不能为零".to_string());
        }

        Ok(num_a / num_b)
    }

    // 模式 4: 错误上下文添加
    // 为错误添加更多上下文信息
    fn read_file_with_context(path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| format!("读取文件 '{}' 失败: {}", path, e))
    }

    // 模式 5: 可选值的错误处理
    // 将 Option 转换为 Result
    fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
        opt.ok_or_else(|| error_msg.to_string())
    }

    // 测试各种模式
    println!("测试输入验证:");
    for input in vec!["", "valid", &"a".repeat(101)] {
        match validate_input(input) {
            Ok(()) => println!("'{}' 验证通过", input),
            Err(e) => println!("'{}' 验证失败: {}", input, e),
        }
    }

    println!("\n测试链式操作:");
    for input in vec!["10", "-5", "not_a_number", ""] {
        match process_chain(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' -> 错误: {}", input, e),
        }
    }

    println!("\n测试组合操作:");
    let test_pairs = vec![("10", "2"), ("0", "5"), ("10", "0"), ("a", "b")];
    for (a, b) in test_pairs {
        match combine_operations(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(e) => println!("{} / {} -> 错误: {}", a, b, e),
        }
    }

    println!("\n测试上下文添加:");
    match read_file_with_context("nonexistent.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n测试 Option 转换:");
    let some_value = Some(42);
    let none_value: Option<i32> = None;

    println!("Some 转换: {:?}", option_to_result(some_value, "值不存在"));
    println!("None 转换: {:?}", option_to_result(none_value, "值不存在"));

    // 错误处理模式的选择指南：
    // 1. 快速失败：适合输入验证和前置条件检查
    // 2. 链式操作：适合处理流水线式的数据处理
    // 3. 组合操作：需要处理多个独立操作时
    // 4. 上下文添加：需要更好的错误信息时
    // 5. 可选值转换：需要将 Option 集成到错误处理流程中

    // 错误处理的最佳实践：
    // 1. 在函数签名中明确可能的错误
    // 2. 提供有意义的错误信息
    // 3. 在适当的层级处理错误
    // 4. 不要忽略错误（除非有意为之）
    // 5. 保持错误处理的一致性

    println!();
}

// ===========================================
// 7. 自定义错误类型
// ===========================================

// 自定义错误类型是构建健壮 Rust 应用的重要组成部分
// 实现标准错误 trait 可以让你的错误更好地与生态系统集成

fn custom_error_types() {
    println!("=== 自定义错误类型 ===");

    // 完整的自定义错误类型：实现标准错误 trait
    #[derive(Debug)]
    enum AppError {
        Io(std::io::Error),
        ParseInt(std::num::ParseIntError),
        InvalidInput(String),
        NetworkError(String),
    }

    // 实现 Display trait：提供用户友好的错误信息
    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                AppError::Io(err) => write!(f, "IO 错误: {}", err),
                AppError::ParseInt(err) => write!(f, "整数解析错误: {}", err),
                AppError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
                AppError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            }
        }
    }

    // 实现 Error trait：支持错误链和标准错误处理
    impl std::error::Error for AppError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AppError::Io(err) => Some(err),       // 返回底层错误
                AppError::ParseInt(err) => Some(err), // 返回底层错误
                _ => None,                            // 没有底层错误
            }
        }
    }

    // 实现 From trait：支持 ? 运算符和自动错误转换
    impl From<std::io::Error> for AppError {
        fn from(err: std::io::Error) -> Self {
            AppError::Io(err)
        }
    }

    impl From<std::num::ParseIntError> for AppError {
        fn from(err: std::num::ParseIntError) -> Self {
            AppError::ParseInt(err)
        }
    }

    // 使用自定义错误类型
    fn process_file_with_custom_error(file_path: &str) -> Result<i32, AppError> {
        let mut file = std::fs::File::open(file_path)?;
        let mut content = String::new();
        std::io::Read::read_to_string(&mut file, &mut content)?;

        let number: i32 = content.trim().parse()?;

        if number < 0 {
            return Err(AppError::InvalidInput("数字不能为负数".to_string()));
        }

        Ok(number)
    }

    // 测试自定义错误
    let test_file = "nonexistent.txt";
    match process_file_with_custom_error(test_file) {
        Ok(result) => println!("处理成功: {}", result),
        Err(e) => println!("处理失败: {} - {:?}", e, e),
    }

    // 自定义错误的最佳实践：
    // 1. 为错误类型提供清晰的名称
    // 2. 实现 Display trait 提供用户友好的信息
    // 3. 实现 Error trait 支持错误链
    // 4. 实现 From trait 支持 ? 运算符
    // 5. 在适当的地方添加上下文信息

    // 错误链（Error Chaining）的重要性：
    // - 保留原始错误信息
    // - 提供上下文信息
    // - 便于调试和问题定位
    // - 支持错误日志记录

    // 使用 thiserror 简化错误定义（如果可用）：
    /*
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum MyError {
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        #[error("Parse error: {0}")]
        Parse(#[from] std::num::ParseIntError),
        #[error("Invalid input: {0}")]
        InvalidInput(String),
    }
    */

    println!();
}

// ===========================================
// 8. 错误处理最佳实践
// ===========================================

// 掌握错误处理的最佳实践可以显著提高代码质量和可维护性

fn error_handling_best_practices() {
    println!("=== 错误处理最佳实践 ===");

    // 实践 1: 使用具体的错误类型
    // 避免使用过于通用的错误类型
    fn read_config() -> Result<String, std::io::Error> {
        std::fs::read_to_string("config.txt")
    }

    // 实践 2: 提供有用的错误信息
    // 错误信息应该清晰、具体、有行动指导
    fn calculate_average(numbers: &[f64]) -> Result<f64, String> {
        if numbers.is_empty() {
            return Err("不能计算空数组的平均值".to_string());
        }

        let sum: f64 = numbers.iter().sum();
        Ok(sum / numbers.len() as f64)
    }

    // 实践 3: 适当的错误处理层级
    // 在合适的层级处理错误，避免过早或过晚处理
    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("除数不能为零".to_string());
        }
        Ok(a / b)
    }

    fn complex_calculation(values: &[f64]) -> Result<f64, String> {
        if values.len() < 2 {
            return Err("至少需要 2 个值".to_string());
        }

        let avg = calculate_average(values)?;
        let first = safe_divide(values[0], avg)?;
        let second = safe_divide(values[1], avg)?;

        Ok(first + second)
    }

    // 实践 4: 使用日志记录错误
    // 在错误发生时记录有用的调试信息
    fn logged_operation(value: &str) -> Result<i32, String> {
        match value.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(e) => {
                eprintln!("警告: 无法解析 '{}' 为整数: {}", value, e);
                Err(format!("无效的数字格式: {}", value))
            }
        }
    }

    // 实践 5: 错误处理的上下文感知
    // 根据调用上下文决定如何处理错误
    fn user_friendly_error(operation: &str, result: Result<i32, String>) -> String {
        match result {
            Ok(value) => format!("{} 成功: {}", operation, value),
            Err(error) => format!("{} 失败: {}", operation, error),
        }
    }

    // 实践 6: 错误恢复策略
    // 提供合理的错误恢复机制
    fn robust_divide(a: f64, b: f64) -> f64 {
        a / b.max(0.001) // 避免除零，使用小的正数
    }

    // 测试最佳实践
    println!("配置读取: {:?}", read_config());

    let numbers = vec![1.0, 2.0, 3.0];
    println!("平均值计算: {:?}", calculate_average(&numbers));
    println!("空数组平均值: {:?}", calculate_average(&[]));

    let values = vec![10.0, 20.0, 30.0];
    println!("复杂计算: {:?}", complex_calculation(&values));

    for test_value in vec!["42", "not_a_number"] {
        match logged_operation(test_value) {
            Ok(result) => println!("'{}' -> {}", test_value, result),
            Err(e) => println!("'{}' -> 错误: {}", test_value, e),
        }
    }

    println!("\n用户友好的错误信息:");
    println!("{}", user_friendly_error("计算", Ok(42)));
    println!(
        "{}",
        user_friendly_error("计算", Err("无效输入".to_string()))
    );

    println!("\n健壮的除法:");
    println!("10 / 0 = {}", robust_divide(10.0, 0.0));
    println!("10 / 2 = {}", robust_divide(10.0, 2.0));

    // 错误处理策略总结：
    // 1. **防御性编程**：预见可能的错误情况
    // 2. **清晰的错误信息**：帮助用户理解问题
    // 3. **适当的抽象层级**：在正确的位置处理错误
    // 4. **日志记录**：便于调试和维护
    // 5. **错误恢复**：提供合理的回退机制
    // 6. **一致性**：保持整个项目的错误处理风格一致

    // 常见的错误处理反模式：
    // 1. 过度使用 unwrap()
    // 2. 忽略错误（let _ = result;）
    // 3. 错误信息不够具体
    // 4. 错误处理逻辑过于复杂
    // 5. 在错误的层级处理错误

    println!();
}

// ===========================================
// 9. 错误处理示例程序
// ===========================================

// 通过实际的示例程序展示错误处理的完整应用

fn error_handling_example_program() {
    println!("=== 错误处理示例程序 ===");

    // 示例：文件处理系统
    #[derive(Debug)]
    enum ProcessingError {
        FileNotFound,
        ParseError(String),
        ValidationError(String),
    }

    impl std::fmt::Display for ProcessingError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ProcessingError::FileNotFound => write!(f, "文件未找到"),
                ProcessingError::ParseError(msg) => write!(f, "解析错误: {}", msg),
                ProcessingError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            }
        }
    }

    impl std::error::Error for ProcessingError {}

    // 处理包含数字的文件
    fn process_numeric_file(file_path: &str) -> Result<Vec<i32>, ProcessingError> {
        // 读取文件
        let content =
            std::fs::read_to_string(file_path).map_err(|_| ProcessingError::FileNotFound)?;

        // 解析内容
        let mut numbers = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let number: i32 = line.parse().map_err(|e| {
                ProcessingError::ParseError(format!("第 {} 行 '{}': {}", line_num + 1, line, e))
            })?;

            if number < 0 {
                return Err(ProcessingError::ValidationError(format!(
                    "第 {} 行: 负数不允许: {}",
                    line_num + 1,
                    number
                )));
            }

            numbers.push(number);
        }

        if numbers.is_empty() {
            return Err(ProcessingError::ValidationError(
                "文件不包含有效数字".to_string(),
            ));
        }

        Ok(numbers)
    }

    // 计算统计信息
    fn calculate_statistics(numbers: &[i32]) -> (f64, f64, i32) {
        let sum: i32 = numbers.iter().sum();
        let avg = sum as f64 / numbers.len() as f64;
        let max = numbers.iter().max().copied().unwrap_or(0) as f64;
        (avg, max, sum)
    }

    // 批量处理文件
    fn batch_process_files(
        file_paths: &[&str],
    ) -> Vec<(String, Result<Vec<i32>, ProcessingError>)> {
        file_paths
            .iter()
            .map(|&path| {
                let result = process_numeric_file(path);
                (path.to_string(), result)
            })
            .collect()
    }

    // 创建测试文件并处理
    let test_cases = vec![
        ("valid_data.txt", "10\n20\n30\n40\n50"),
        ("empty_file.txt", ""),
        ("invalid_numbers.txt", "10\nabc\n30"),
        ("negative_numbers.txt", "10\n-5\n30"),
    ];

    println!("=== 批量文件处理 ===");
    for (filename, content) in test_cases {
        println!("\n处理文件: {}", filename);

        // 创建临时文件
        if let Err(_) = std::fs::write(filename, content) {
            println!("无法创建测试文件");
            continue;
        }

        match process_numeric_file(filename) {
            Ok(numbers) => {
                let (avg, max, sum) = calculate_statistics(&numbers);
                println!("✓ 成功处理 {} 个数字", numbers.len());
                println!("  数字列表: {:?}", numbers);
                println!("  统计: 平均值={:.2}, 最大值={}, 总和={}", avg, max, sum);
            }
            Err(e) => println!("✗ 处理失败: {}", e),
        }

        // 清理测试文件
        let _ = std::fs::remove_file(filename);
    }

    // 示例：用户输入处理系统
    println!("\n=== 用户输入处理系统 ===");
    fn process_user_input(input: &str) -> Result<i32, String> {
        // 验证输入不为空
        if input.trim().is_empty() {
            return Err("输入不能为空".to_string());
        }

        // 解析为数字
        let number: i32 = input
            .trim()
            .parse()
            .map_err(|e| format!("'{}' 不是有效的整数: {}", input, e))?;

        // 验证范围
        if number < 1 || number > 100 {
            return Err(format!("数字 {} 不在 1-100 范围内", number));
        }

        // 处理成功
        Ok(number * 2)
    }

    let test_inputs = vec![
        "42", "150", "-10", "abc", "", " 25  ", // 包含空格
    ];

    for input in test_inputs {
        match process_user_input(input) {
            Ok(result) => println!("输入 '{}' -> 处理结果: {}", input, result),
            Err(e) => println!("输入 '{}' -> 错误: {}", input, e),
        }
    }

    // 示例：网络请求模拟
    println!("\n=== 网络请求模拟 ===");
    #[derive(Debug)]
    enum NetworkError {
        Timeout,
        ConnectionFailed,
        InvalidResponse(String),
    }

    impl std::fmt::Display for NetworkError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                NetworkError::Timeout => write!(f, "请求超时"),
                NetworkError::ConnectionFailed => write!(f, "连接失败"),
                NetworkError::InvalidResponse(msg) => write!(f, "无效响应: {}", msg),
            }
        }
    }

    fn simulate_network_request(success_rate: f64) -> Result<String, NetworkError> {
        use std::time::Duration;
        std::thread::sleep(Duration::from_millis(100)); // 模拟网络延迟

        if rand::random::<f64>() > success_rate {
            let error_type = rand::random::<u8>() % 3;
            match error_type {
                0 => Err(NetworkError::Timeout),
                1 => Err(NetworkError::ConnectionFailed),
                _ => Err(NetworkError::InvalidResponse(
                    "500 Internal Server Error".to_string(),
                )),
            }
        } else {
            Ok("请求成功".to_string())
        }
    }

    // 模拟多次请求，展示错误处理
    for i in 1..=5 {
        match simulate_network_request(0.7) {
            Ok(response) => println!("请求 {}: ✓ {}", i, response),
            Err(e) => println!("请求 {}: ✗ {}", i, e),
        }
    }

    // 实际应用中的错误处理要点：
    // 1. **分层处理**：在不同层级处理不同类型的错误
    // 2. **用户友好**：向用户提供清晰的错误信息
    // 3. **调试信息**：为开发者提供详细的调试信息
    // 4. **错误恢复**：提供合理的错误恢复机制
    // 5. **日志记录**：记录错误以便后续分析
    // 6. **监控告警**：对严重错误进行监控和告警

    println!();
}

// ===========================================
// 10. Rust 1.81 core::error::Error trait（Enhanced Error Trait）
// ===========================================

// Rust 1.81 把 Error trait 稳定地放进了 core（`core::error::Error`），
// `std::error::Error` 现在只是它的重新导出——两者是同一个 trait。
// 这让无标准库（no_std）的库也能给自己的错误类型实现标准的 `Error`，
// 并且与使用 std 的应用无缝互通（应用里可以直接把它装进 `Box<dyn std::error::Error>`）。
//
// 需要澄清的是：1.81 只做了“搬家 + 稳定”这一件事。下面这些常被误认为是它带来的能力，其实都不是：
// - 错误链（`source()` 方法）自 Rust 1.30 起就有，与 1.81 无关
// - 让错误附带“类型化的上下文数据”（`provide` / `Request`）至今仍是不稳定特性，只能在 nightly 上使用
// - Error 不会自动记录错误发生的位置；要调用栈请用 `std::backtrace::Backtrace`（1.65 稳定），
//   要位置信息请自己在错误类型里保存（例如 `std::panic::Location::caller()`）
//
// 所以本节代码的主体，讲的是怎样写一个好的自定义错误类型、怎样用 `source()` 串起错误链，
// 这些在所有 stable 版本里都成立。

fn enhanced_error_trait() {
    println!("=== Rust 1.81 core::error::Error trait ===");

    // 1. 基础 Error trait 使用（Rust 1.81）
    // core::error::Error 与 std::error::Error 同源，
    // no_std 环境下可以直接 use core::error::Error

    use std::error::Error;
    use std::fmt;

    // 验证“两者是同一个 trait”：把标准库的 io::Error 当作 `core::error::Error` 的 trait 对象使用，
    // 不需要任何转换。这说明在 no_std 库里按 core::error::Error 写的实现，可以直接用在 std 应用中。
    let as_core_error: &dyn core::error::Error = &std::io::Error::other("磁盘已满");
    println!("io::Error 同时就是 core::error::Error: {as_core_error}");

    // 基础自定义错误类型
    #[derive(Debug)]
    struct DatabaseError {
        message: String,
        query: String,
        source: Option<Box<dyn Error + Send + Sync>>,
    }

    impl DatabaseError {
        fn new(message: &str, query: &str) -> Self {
            DatabaseError {
                message: message.to_string(),
                query: query.to_string(),
                source: None,
            }
        }

        fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
            self.source = Some(Box::new(source));
            self
        }
    }

    impl fmt::Display for DatabaseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "数据库错误: {} (查询: {})", self.message, self.query)
        }
    }

    impl Error for DatabaseError {
        // 提供错误源信息
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source
                .as_ref()
                .map(|s| s.as_ref() as &(dyn Error + 'static))
        }

        // 【仅 nightly】`provide` 是尚未稳定的方法（feature = error_generic_member_access，
        // 需要 nightly 工具链，并在 crate 根部写 `#![feature(error_generic_member_access)]`）。
        // 它允许错误对象向外“按类型”提供附加数据——注意是按**类型**索引，不是按字符串键：
        //
        //     fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {
        //         request.provide_ref::<str>(&self.query); // 以 &str 的形式提供查询语句
        //     }
        //
        // 调用方也按类型取用：
        //     let query: Option<&str> = std::error::request_ref::<str>(&error);
        // （按值提供数据用 `provide_value::<T>(value)` / `request_value::<T>(&error)`。）
        // 在 stable 上，需要结构化信息时请直接给自己的错误类型加访问器方法。
    }

    // 2. 错误链和上下文
    // `source()` 把底层错误串起来，形成一条可以逐层回溯的错误链

    #[derive(Debug)]
    struct NetworkError {
        message: String,
        url: String,
        status_code: Option<u16>,
    }

    impl fmt::Display for NetworkError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "网络错误: {} (URL: {})", self.message, self.url)
        }
    }

    // 没有嵌套的源错误，也不需要附加数据：空实现，直接使用所有方法的默认实现即可
    impl Error for NetworkError {}

    // 创建嵌套的错误链
    fn fetch_user_data(user_id: u32) -> Result<String, DatabaseError> {
        // 模拟数据库查询失败
        if user_id == 999 {
            let query = format!("SELECT * FROM users WHERE id = {}", user_id);
            let network_error = NetworkError {
                message: "连接超时".to_string(),
                url: "tcp://database:5432".to_string(),
                status_code: None,
            };
            Err(DatabaseError::new("用户不存在", &query).with_source(network_error))
        } else {
            Ok(format!("用户数据: {}", user_id))
        }
    }

    // 测试错误链
    match fetch_user_data(999) {
        Ok(data) => println!("成功: {}", data),
        Err(e) => {
            println!("错误链:");
            println!("  主要错误: {}", e);

            if let Some(source) = e.source() {
                println!("  源错误: {}", source);
            }

            // 需要结构化的错误详情时，在 stable 上最直接的办法是访问错误类型自己的字段或方法
            // （按类型向外提供数据的 `provide` / `request_ref` 机制仍是 nightly-only，见上文）：
            println!("  出错的查询语句: {}", e.query);
        }
    }

    // 3. 错误包装：把底层错误包进更高层的错误里，同时保留原因
    // 高层错误说明“在做什么时失败了”，`source()` 指向“为什么失败”

    #[derive(Debug)]
    struct ServiceError {
        operation: String,
        context: String,
        source: Box<dyn Error + Send + Sync>,
    }

    impl ServiceError {
        fn new(operation: &str, context: &str, source: impl Error + Send + Sync + 'static) -> Self {
            ServiceError {
                operation: operation.to_string(),
                context: context.to_string(),
                source: Box::new(source),
            }
        }
    }

    impl fmt::Display for ServiceError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "服务错误: {} - {}", self.operation, self.context)
        }
    }

    impl Error for ServiceError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(self.source.as_ref() as &(dyn Error + 'static))
        }
    }

    // 使用错误包装
    fn process_user_request(user_id: u32) -> Result<String, ServiceError> {
        fetch_user_data(user_id)
            .map_err(|e| ServiceError::new("process_user_request", "获取用户数据失败", e))
    }

    // 测试错误包装
    match process_user_request(999) {
        Ok(result) => println!("处理成功: {}", result),
        Err(e) => {
            println!("包装错误:");
            println!("  服务错误: {}", e);

            // 遍历错误链
            let mut current_error: &dyn Error = &e;
            let mut level = 0;

            loop {
                println!("  层级 {}: {}", level, current_error);

                if let Some(source) = current_error.source() {
                    current_error = source;
                    level += 1;
                } else {
                    break;
                }
            }
        }
    }

    // 4. 错误报告：在错误里保留足够的诊断信息
    // Display 给用户看（说清楚哪里错了、期望什么），Debug 给开发者看（保留完整的字段）

    #[derive(Debug)]
    struct ConfigError {
        key: String,
        value: String,
        expected_type: String,
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "配置错误: '{}' 应该是 {} 类型，但得到 '{}'",
                self.key, self.expected_type, self.value
            )
        }
    }

    impl Error for ConfigError {}

    fn parse_config_value(key: &str, value: &str, expected_type: &str) -> Result<(), ConfigError> {
        match expected_type {
            "integer" => {
                value.parse::<i32>().map_err(|_| ConfigError {
                    key: key.to_string(),
                    value: value.to_string(),
                    expected_type: expected_type.to_string(),
                })?;
            }
            "boolean" => {
                value.parse::<bool>().map_err(|_| ConfigError {
                    key: key.to_string(),
                    value: value.to_string(),
                    expected_type: expected_type.to_string(),
                })?;
            }
            _ => {
                return Err(ConfigError {
                    key: key.to_string(),
                    value: value.to_string(),
                    expected_type: expected_type.to_string(),
                });
            }
        }
        Ok(())
    }

    // 测试配置解析错误
    let config_tests = vec![
        ("port", "8080", "integer"),
        ("debug", "true", "boolean"),
        ("timeout", "30s", "integer"), // 这个会失败
    ];

    for (key, value, expected_type) in config_tests {
        match parse_config_value(key, value, expected_type) {
            Ok(_) => println!("配置 '{}' = '{}' (类型: {}) ✓", key, value, expected_type),
            Err(e) => {
                println!("配置 '{}' = '{}' (类型: {}) ✗", key, value, expected_type);
                println!("  错误: {}", e);
            }
        }
    }

    // 5. 错误日志：沿着 source() 记录整条错误链
    // 只打印最外层错误会丢掉真正的根因，日志里应当把每一层原因都记录下来

    fn log_error_enhanced(error: &dyn Error) {
        println!("[ERROR] {}", error);

        // 一层层向下追溯，直到没有更底层的原因
        let mut current = error.source();
        while let Some(cause) = current {
            println!("[ERROR]   caused by: {}", cause);
            current = cause.source();
        }
    }

    // 模拟一个复杂错误
    let complex_error = ServiceError::new(
        "data_processing",
        "数据处理失败",
        DatabaseError::new("查询超时", "SELECT * FROM large_table").with_source(NetworkError {
            message: "网络不稳定".to_string(),
            url: "tcp://primary-db:5432".to_string(),
            status_code: Some(503),
        }),
    );

    log_error_enhanced(&complex_error);

    // 自定义错误类型的最佳实践：
    // 1. 实现 Display（给用户看的一句话）和 Debug（给开发者看的完整字段），再实现 Error
    // 2. 用 source() 提供底层原因，让调用方和日志能回溯整条错误链
    // 3. Display 里不要把 source 的内容再重复拼一遍：
    //    要么放进 Display，要么通过 source() 暴露，二选一，否则整条链打印时同一个原因会出现两次
    // 4. 错误种类多时，库用 `thiserror` 自动生成 Display / Error / From，
    //    应用层用 `anyhow` 汇总各种错误并附加上下文，不要重复造轮子
    // 5. 需要让调用方按种类做不同处理时，用枚举表示错误种类，而不是解析错误消息字符串

    println!();
}

// ===========================================
// 11. Rust 1.98 Option / Result 便利方法
// ===========================================

// 下面几个小而实用的方法在 Rust 1.98 稳定：
// - `bool::ok_or(err)` / `bool::ok_or_else(f)`：把“条件是否成立”直接变成 `Result<(), E>`
// - `Option::map_or_default(f)` / `Result::map_or_default(f)`：
//   “有值就映射，没值就用返回类型的默认值”
//
// 它们没有引入新概念，只是把两类很常见的样板写法收进了标准库。
// 学会它们的意义在于：看到旧代码里的 `if !cond { return Err(..) }` 或
// `opt.map(f).unwrap_or_default()` 时，知道有更贴近意图的写法。

fn option_result_conveniences() {
    println!("=== Rust 1.98 Option/Result 便利方法 ===");

    // ---------- bool::ok_or / bool::ok_or_else ----------
    //
    // 校验函数里最常见的需求是：“条件成立就继续，不成立就返回错误”。
    // 以前要写 `if !cond { return Err(..); }`，
    // 或者绕一圈：`cond.then_some(()).ok_or(..)`（先变成 Option 再变成 Result）。
    // 现在 `bool` 自己就有 `ok_or`：true 得到 `Ok(())`，false 得到 `Err(err)`。
    // 配合 `?` 运算符，一行就能完成“不满足就提前返回”。
    //
    // 错误值是 `&'static str` 这类几乎零成本的值时，直接用 `ok_or`：
    fn check_name(name: &str) -> Result<(), &'static str> {
        (!name.is_empty()).ok_or("名字不能为空")
    }

    // 错误信息要现拼（例如 `format!` 会分配 String）时，用 `ok_or_else`：
    // `ok_or` 的参数会“先求值”，哪怕条件成立、错误根本用不上；
    // `ok_or_else` 接收闭包，只有条件不成立时才会执行，成功路径上没有多余开销。
    fn check_age(age: u8) -> Result<(), String> {
        (age >= 18).ok_or_else(|| format!("年龄 {age} 未满 18 岁"))
    }

    // 两个校验可以用 `?` 串起来。它们的错误类型并不相同（`&'static str` 与 `String`），
    // 但 `?` 会自动通过 `From` 把错误转换成函数返回的错误类型——标准库有 `impl From<&str> for String`，
    // 所以这里不需要手写 `map_err`。
    fn register(name: &str, age: u8) -> Result<String, String> {
        check_name(name)?;
        check_age(age)?;
        Ok(format!("{name} 注册成功"))
    }

    println!("register(\"Ferris\", 20) = {:?}", register("Ferris", 20));
    println!("register(\"\", 20) = {:?}", register("", 20));
    println!("register(\"Ferris\", 15) = {:?}", register("Ferris", 15));

    // ---------- map_or_default ----------
    //
    // 旧写法：`opt.map(f).unwrap_or_default()`，或 `opt.map_or(Default::default(), f)`。
    // 新写法：`opt.map_or_default(f)`。
    // 语义：`Some(v)` 得到 `f(v)`；`None` 得到返回类型的 `Default::default()`
    // （数字是 0，字符串是空串，Vec 是空 Vec……）。要求返回类型实现 `Default`。
    let name: Option<&str> = Some("Rust");
    let nothing: Option<&str> = None;
    println!(
        "Option::map_or_default: Some(\"Rust\") -> {}，None -> {}",
        name.map_or_default(|s| s.len()),
        nothing.map_or_default(|s| s.len())
    );

    // `Result` 上也有同名方法：`Ok(v)` 得到 `f(v)`，`Err(_)` 得到默认值。
    // 注意这会把错误信息直接丢掉，只适合“出错了也无所谓，按默认值处理”的场合；
    // 如果需要区分错误原因，请继续使用 `match` 或 `map_err`。
    let good: Result<u32, _> = "21".parse::<u32>();
    let bad: Result<u32, _> = "abc".parse::<u32>();
    println!(
        "Result::map_or_default: Ok(21) -> {}，Err(_) -> {}",
        good.map_or_default(|n| n * 2),
        bad.map_or_default(|n| n * 2)
    );

    // 小结：
    // 1. `bool::ok_or`：把“条件校验”接入 `Result` 与 `?` 的世界
    // 2. `ok_or` 先求值、`ok_or_else` 惰性求值——错误值有开销时选后者
    // 3. `map_or_default`：默认值恰好就是 `Default::default()` 时，比 `map_or` 更简洁
    println!();
}

// ===========================================
// 12. #[must_use]：不要悄悄忽略返回值（Rust 1.27 起，标准库在 1.58 补全）
// ===========================================

// 有些返回值一旦被忽略，几乎一定是 bug：
// - 返回 `Result` 的函数：忽略它，就等于忽略了错误
// - 纯函数（只有返回值有意义）：`s.trim();` 并不会修改 `s`，单独写一行毫无作用
// - 惰性的迭代器：`v.iter().map(f);` 不消费的话，什么都不会发生
// `#[must_use]` 让编译器在“返回值被直接丢弃”时发出警告（lint 名叫 `unused_must_use`，默认 warn）。
// 它可以标在函数 / 方法上（1.27）、类型上（结构体、枚举），也可以标在 trait 上（1.32）。
// `Result`、各种迭代器适配器本来就标了；1.57–1.58 又把标准库里所有“该标”的函数补标齐全。
// 标在 `async fn` 上时，对它 `.await` 之后得到的结果同样生效（1.67 起）。
// 带消息的写法 `#[must_use = "原因"]` 会把原因显示在警告里，告诉使用者“为什么不能忽略”。

// 标在函数上：校验结果必须有人看
#[must_use = "忽略校验结果，非法的用户名就会悄悄溜过去"]
fn is_valid_username(name: &str) -> bool {
    (3..=16).contains(&name.chars().count()) && name.chars().all(char::is_alphanumeric)
}

// 标在类型上：这个值创建之后必须被使用，否则几乎肯定是漏写了后续步骤
#[must_use = "事务没有提交就被丢弃了"]
struct Transaction {
    amount: i64,
}

impl Transaction {
    fn new(amount: i64) -> Self {
        Transaction { amount }
    }

    // 消耗事务并返回提交的金额
    fn commit(self) -> i64 {
        self.amount
    }
}

fn must_use_attribute() {
    println!("=== #[must_use]：不要悄悄忽略返回值（Rust 1.27 起，标准库在 1.58 补全） ===");

    // 正确用法：使用返回值
    if is_valid_username("rustacean") {
        println!("用户名合法");
    }
    let committed = Transaction::new(100).commit();
    println!("已提交的金额: {committed}");

    // 下面这些写法都会触发 unused_must_use 警告，所以只写在注释里：
    //     is_valid_username("ab");           // warning: unused return value of `is_valid_username` that must be used
    //     "  hi ".trim();                    // warning: unused return value of `core::str::<impl str>::trim` that must be used
    //     vec![1, 2].iter().map(|x| x + 1);  // warning: unused `Map` that must be used
    //     Transaction::new(5);               // warning: unused `Transaction` that must be used
    // 带消息的属性，警告里会附上我们写下的那句原因。

    // 确实想忽略返回值时，要明确表态：`let _ = ...;`。读代码的人由此知道你是有意为之，而不是忘了。
    let _ = is_valid_username("ab");

    // 最佳实践：
    // 1. 给“忽略了就一定出错”的函数和类型加 `#[must_use]`，并写上原因
    // 2. 不要用 `#[allow(unused_must_use)]` 压警告；真的要忽略，写 `let _ = ...;`
    // 3. 返回 `Result` 的函数不需要再标，`Result` 本身就是 must_use
    println!();
}

// ===========================================
// 13. #[track_caller]：让 panic 指向调用者，而不是内部实现（Rust 1.46）
// ===========================================

// panic 的报错信息里带有出错的文件和行号。默认情况下，它指向 `panic!` 所在的那一行。
// 但对一个“检查参数，不满足就 panic”的辅助函数来说，真正出问题的是**调用它的那一行**，
// 指向辅助函数内部毫无帮助——你还得顺着调用栈自己去找。
// 标准库的 `unwrap()`、`expect()`、`RefCell::borrow_mut()`、切片下标越界等，报错时指向的都是**你的代码**，
// 靠的就是 `#[track_caller]`。
//
// 给函数加上 `#[track_caller]` 之后：
// - 函数里 panic（或调用 `Location::caller()`）所看到的位置，是调用这个函数的位置
// - 如果它又调用了另一个带 `#[track_caller]` 的函数，位置会一路向上传递到最外层的调用者
// - 代价是每次调用多传一个隐藏参数，几乎可以忽略
// 典型用法：给自己写的 `unwrap` 风格辅助函数、断言函数、检查参数的构造函数加上它。

use std::panic::Location;

// 带 `#[track_caller]`：返回的是“调用本函数的那一行”
#[track_caller]
fn caller_location() -> &'static Location<'static> {
    Location::caller()
}

// 不带：返回的是本函数里 `Location::caller()` 所在的那一行，与谁调用无关
fn plain_location() -> &'static Location<'static> {
    Location::caller()
}

fn track_caller_attribute() {
    println!("=== #[track_caller]：让 panic 指向调用者，而不是内部实现（Rust 1.46） ===");

    let tracked = caller_location(); // 返回的行号，就是上面这一行
    let untracked = plain_location();
    println!("带 #[track_caller] 的函数报告的位置: {}:{}", tracked.file(), tracked.line());
    println!("不带的函数报告的位置: {}:{}（指向它自己内部）", untracked.file(), untracked.line());

    // 最佳实践：
    // 1. 凡是“检查参数不满足就 panic”的公开辅助函数，都加上 `#[track_caller]`，让使用者一眼看到是哪一行出的错
    // 2. 它只改变“报告的位置”，不改变程序行为
    println!();
}

// ===========================================
// 14. 调用栈回溯：RUST_BACKTRACE 与 std::backtrace::Backtrace（Rust 1.65）
// ===========================================

// panic 时，默认只打印错误信息和位置，并提示：
//     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// 设置环境变量后重新运行，就会打印调用栈，一路追到出错的位置：
// - `RUST_BACKTRACE=1`：简短回溯，隐藏运行时内部的栈帧，只留下和你的代码相关的部分（1.47 起输出更紧凑）
// - `RUST_BACKTRACE=full`：完整回溯，包含所有栈帧
//
// 1.65 起，你的代码也能主动捕获回溯：`std::backtrace::Backtrace`。
// 典型用途是把回溯存进自定义错误里。因为 `?` 传播错误时，错误本身并不记录“从哪里一路传上来的”，
// 事后看日志就只剩一句错误信息；带上回溯，就能直接看到源头。
//
// 两种捕获方式：
// - `Backtrace::capture()`：受环境变量控制（先看 `RUST_LIB_BACKTRACE`，再看 `RUST_BACKTRACE`）；
//   没有启用时几乎没有开销，所以可以放心地写在错误的构造函数里
// - `Backtrace::force_capture()`：无视环境变量，一定捕获，代价较高（要遍历调用栈）
// 回溯的状态用 `status()` 查询：`Unsupported`（平台不支持）、`Disabled`（环境变量没开）、`Captured`（已捕获）。

fn backtrace_support() {
    println!("=== 调用栈回溯：RUST_BACKTRACE 与 std::backtrace::Backtrace（Rust 1.65） ===");

    use std::backtrace::{Backtrace, BacktraceStatus};

    // 典型用法：在自定义错误的构造函数里捕获回溯
    #[derive(Debug)]
    struct AppError {
        message: String,
        backtrace: Backtrace,
    }
    impl AppError {
        fn new(message: &str) -> Self {
            AppError { message: message.to_string(), backtrace: Backtrace::capture() }
        }
    }

    let error = AppError::new("配置文件格式错误");
    let status_text = match error.backtrace.status() {
        BacktraceStatus::Captured => "已捕获（环境变量已启用）",
        BacktraceStatus::Disabled => "未启用（设置 RUST_BACKTRACE=1 后重新运行即可）",
        // BacktraceStatus 标了 #[non_exhaustive]，所以必须有 `_` 分支
        _ => "当前平台不支持",
    };
    println!("错误「{}」的回溯状态: {status_text}", error.message);

    // 强制捕获：无视环境变量，一定拿到回溯
    let forced = Backtrace::force_capture();
    println!("force_capture 的状态: {:?}，回溯共 {} 行", forced.status(), forced.to_string().lines().count());

    // 最佳实践：
    // 1. 自定义错误的构造函数里用 `capture()`，平时零开销，排查问题时开环境变量就有回溯
    // 2. 线上服务不要常开 `force_capture()`，它会遍历调用栈，在高频错误路径上开销可观
    println!();
}

// ===========================================
// 15. 顺便看一眼，不改变值：Option / Result 的 inspect（Rust 1.76）
// ===========================================

// 在一串链式调用中间，想“顺便打印或记录一下当前的值”，但又不能改变它。
// 迭代器里早就有 `Iterator::inspect`；1.76 给 `Option` 和 `Result` 也补上了：
// - `Option::inspect(|值| ..)`：是 `Some` 时调用闭包（拿到 `&T`），然后原样返回这个 Option
// - `Result::inspect(|值| ..)`：是 `Ok` 时调用闭包；`Result::inspect_err(|错误| ..)`：是 `Err` 时调用闭包
// 在这之前只能用 `map` 再把原值返回（`map(|v| { log(&v); v })`），既啰嗦，又要把引用换成移动。
// 典型场景：在 `?` 链、`and_then` 链的中间加调试日志或指标统计，而不打断数据流。

// 解析端口号：成功和失败时各记一条日志，值本身原样往下传
fn parse_port_with_log(text: &str, log: &mut Vec<String>) -> Result<u16, std::num::ParseIntError> {
    text.parse::<u16>()
        .inspect(|port| log.push(format!("解析成功: {port}")))
        .inspect_err(|error| log.push(format!("解析失败: {error}")))
}

fn option_result_inspect() {
    println!("=== 顺便看一眼，不改变值：Option / Result 的 inspect（Rust 1.76） ===");

    let mut log = Vec::new();
    println!("{:?}", parse_port_with_log("8080", &mut log));
    println!("{:?}", parse_port_with_log("http", &mut log));
    println!("日志: {log:?}");

    // Option 版本：在链条中间看一眼找到的元素
    let first_even = [1, 3, 4, 6].iter().find(|n| *n % 2 == 0).inspect(|n| println!("找到偶数 {n}")).copied();
    println!("find 的结果仍然原样传了下来: {first_even:?}");

    // 最佳实践：
    // 1. 调试日志、指标统计放进 inspect，而不是 `map(|v| { log(&v); v })`
    // 2. inspect 的闭包只能拿到引用，不能修改值；要修改就老老实实用 map
    println!();
}

// ===========================================
// 16. main 返回自定义退出码：ExitCode（Rust 1.61）
// ===========================================

// 命令行程序要用进程的退出码告诉调用者（shell、脚本、CI）：成功了，还是失败了、是哪一类失败。
// 1.61 之前，`main` 只能返回 `()` 或 `Result<(), E>`（出错时退出码固定为 1）；
// 想要自定义退出码，只能调用 `std::process::exit(码)`——它会立刻终止进程，局部变量的析构函数（drop）都不会运行，
// 缓冲区里没写完的数据可能就丢了。
// 1.61 稳定了 `std::process::ExitCode` 和 `Termination` trait：
// - `main` 可以返回 `ExitCode`：`ExitCode::SUCCESS`、`ExitCode::FAILURE`，或者 `ExitCode::from(0..=255)`
// - 函数正常返回，所有析构照常运行，退出码由你决定
// - 能用作 `main` 返回值的类型，都实现了 `Termination`：`()`、`ExitCode`、`Result<T, E: Debug>` 等
//   （`Err` 时会把错误的 Debug 输出打印到 stderr，退出码为 1）

use std::process::ExitCode;

// 应用自己的错误分类，每一类对应一个退出码（命令行工具的常见约定：2 = 用法错误，3 = 配置错误……）
#[derive(Debug)]
enum CliError {
    Usage(String),
    Config(String),
}

impl CliError {
    fn exit_code(&self) -> u8 {
        match self {
            CliError::Usage(_) => 2,
            CliError::Config(_) => 3,
        }
    }
}

fn run_cli(args: &[&str]) -> Result<(), CliError> {
    match args.first() {
        None => Err(CliError::Usage("缺少参数".to_string())),
        Some(&"bad-config") => Err(CliError::Config("配置文件格式错误".to_string())),
        Some(_) => Ok(()),
    }
}

// 这就是 `fn main() -> ExitCode` 的典型写法：把业务错误转换成退出码，错误信息打印到 stderr
fn exit_code_of(result: Result<(), CliError>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("错误: {error:?}");
            ExitCode::from(error.exit_code())
        }
    }
}

fn custom_exit_code() {
    println!("=== main 返回自定义退出码：ExitCode（Rust 1.61） ===");

    for args in [&["run"][..], &[], &["bad-config"]] {
        let result = run_cli(args);
        let expected = match &result {
            Ok(()) => 0,
            Err(error) => error.exit_code(),
        };
        let code = exit_code_of(result);
        println!("参数 {args:?} -> 退出码 {expected}（等于 SUCCESS: {}）", code == ExitCode::SUCCESS);
    }

    // 真实程序里，main 直接返回它即可：
    //     fn main() -> ExitCode {
    //         exit_code_of(run_cli(&args))
    //     }

    // 最佳实践：
    // 1. 想要自定义退出码时返回 ExitCode，不要在函数中间调用 `process::exit`（它会跳过所有析构）
    // 2. 约定好每个退出码的含义并写进文档，脚本和 CI 才能据此区分失败原因
    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 错误处理演示");
    println!("=================");

    panic_and_unrecoverable_errors();
    result_type_basics();
    option_type_basics();
    error_propagation();
    multiple_error_types();
    error_handling_patterns();
    custom_error_types();
    error_handling_best_practices();
    enhanced_error_trait();
    error_handling_example_program();
    option_result_conveniences();
    must_use_attribute();
    track_caller_attribute();
    backtrace_support();
    option_result_inspect();
    custom_exit_code();

    println!("错误处理演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_ok_or_and_map_or_default() {
        use std::cell::Cell;

        // ok_or：true -> Ok(())，false -> Err(err)
        assert_eq!(true.ok_or("错误"), Ok(()));
        assert_eq!(false.ok_or("错误"), Err("错误"));

        // ok_or_else 是惰性的：条件成立时闭包不会被调用
        let calls = Cell::new(0);
        let build_err = || {
            calls.set(calls.get() + 1);
            "构造出的错误".to_string()
        };
        assert_eq!(true.ok_or_else(build_err), Ok(()));
        assert_eq!(calls.get(), 0);
        assert_eq!(false.ok_or_else(build_err), Err("构造出的错误".to_string()));
        assert_eq!(calls.get(), 1);

        // map_or_default：None / Err 时得到返回类型的默认值
        assert_eq!(Some("Rust").map_or_default(|s| s.len()), 4);
        assert_eq!(None::<&str>.map_or_default(|s| s.len()), 0);
        assert_eq!(Ok::<u32, ()>(21).map_or_default(|n| n * 2), 42);
        assert_eq!(Err::<u32, ()>(()).map_or_default(|n| n * 2), 0);
        assert_eq!(None::<&str>.map_or_default(|s| s.to_string()), String::new());
    }

    #[test]
    fn test_divide_function() {
        fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
            if denominator == 0.0 {
                Err(String::from("不能除以零"))
            } else {
                Ok(numerator / denominator)
            }
        }

        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(10.0, 0.0), Err("不能除以零".to_string()));
    }

    #[test]
    fn test_find_element() {
        fn find_element(vec: &Vec<i32>, target: i32) -> Option<usize> {
            for (i, &value) in vec.iter().enumerate() {
                if value == target {
                    return Some(i);
                }
            }
            None
        }

        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_element(&numbers, 3), Some(2));
        assert_eq!(find_element(&numbers, 6), None);
    }

    #[test]
    fn test_validate_input() {
        fn validate_input(input: &str) -> Result<(), String> {
            if input.is_empty() {
                return Err("输入不能为空".to_string());
            }
            if input.len() > 100 {
                return Err("输入不能超过 100 个字符".to_string());
            }
            Ok(())
        }

        assert!(validate_input("valid").is_ok());
        assert_eq!(validate_input(""), Err("输入不能为空".to_string()));
        assert_eq!(
            validate_input(&"a".repeat(101)),
            Err("输入不能超过 100 个字符".to_string())
        );
    }

    #[test]
    fn test_process_chain() {
        fn process_chain(input: &str) -> Result<i32, String> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("输入为空".to_string());
            }

            let number: i32 = trimmed.parse().map_err(|e| format!("解析错误: {}", e))?;

            if number < 0 {
                return Err("数字不能为负数".to_string());
            }

            Ok(number * 2)
        }

        assert_eq!(process_chain("10"), Ok(20));
        assert_eq!(process_chain("-5"), Err("数字不能为负数".to_string()));
        assert!(process_chain("not_a_number").is_err());
    }

    #[test]
    fn test_calculate_average() {
        fn calculate_average(numbers: &[f64]) -> Result<f64, String> {
            if numbers.is_empty() {
                return Err("不能计算空数组的平均值".to_string());
            }

            let sum: f64 = numbers.iter().sum();
            Ok(sum / numbers.len() as f64)
        }

        let numbers = vec![1.0, 2.0, 3.0];
        assert_eq!(calculate_average(&numbers), Ok(2.0));
        assert_eq!(
            calculate_average(&[]),
            Err("不能计算空数组的平均值".to_string())
        );
    }

    #[test]
    fn test_option_to_result() {
        fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
            opt.ok_or_else(|| error_msg.to_string())
        }

        let some_value = Some(42);
        let none_value: Option<i32> = None;

        assert_eq!(option_to_result(some_value, "值不存在"), Ok(42));
        assert_eq!(
            option_to_result(none_value, "值不存在"),
            Err("值不存在".to_string())
        );
    }

    #[test]
    fn test_user_input_processing() {
        fn process_user_input(input: &str) -> Result<i32, String> {
            if input.trim().is_empty() {
                return Err("输入不能为空".to_string());
            }

            let number: i32 = input
                .trim()
                .parse()
                .map_err(|e| format!("'{}' 不是有效的整数: {}", input, e))?;

            if number < 1 || number > 100 {
                return Err(format!("数字 {} 不在 1-100 范围内", number));
            }

            Ok(number * 2)
        }

        assert_eq!(process_user_input("42"), Ok(84));
        assert_eq!(
            process_user_input("150"),
            Err("数字 150 不在 1-100 范围内".to_string())
        );
        assert!(process_user_input("abc").is_err());
        assert_eq!(process_user_input(""), Err("输入不能为空".to_string()));
    }

    #[test]
    fn test_error_context() {
        fn read_file_with_context(path: &str) -> Result<String, String> {
            std::fs::read_to_string(path).map_err(|e| format!("读取文件 '{}' 失败: {}", path, e))
        }

        let result = read_file_with_context("nonexistent.txt");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("nonexistent.txt"));
        assert!(error_msg.contains("读取文件"));
    }

    // 自定义错误类型：Display 文本与 source() 错误链
    #[test]
    fn test_enhanced_error_trait() {
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        struct EnhancedError {
            message: String,
            source: Option<Box<dyn Error + Send + Sync>>,
            context_data: u32,
        }

        impl fmt::Display for EnhancedError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Enhanced Error: {}", self.message)
            }
        }

        impl Error for EnhancedError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                self.source
                    .as_ref()
                    .map(|s| s.as_ref() as &(dyn Error + 'static))
            }
        }

        let error = EnhancedError {
            message: "Test error".to_string(),
            source: None,
            context_data: 42,
        };

        // 测试错误显示
        assert_eq!(error.to_string(), "Enhanced Error: Test error");

        // 测试错误源
        assert!(error.source().is_none());

        // 结构化数据直接通过字段访问（按类型向外提供数据的 `provide` 机制仍是 nightly-only）
        assert_eq!(error.context_data, 42);
    }

    #[test]
    fn test_error_chaining_with_enhanced_trait() {
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        struct NetworkError {
            url: String,
            status_code: u16,
        }

        impl fmt::Display for NetworkError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Network Error: {} - {}", self.url, self.status_code)
            }
        }

        impl Error for NetworkError {}

        #[derive(Debug)]
        struct ServiceError {
            service: String,
            source: NetworkError,
        }

        impl fmt::Display for ServiceError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Service Error: {}", self.service)
            }
        }

        impl Error for ServiceError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                Some(&self.source)
            }
        }

        let network_error = NetworkError {
            url: "https://example.com".to_string(),
            status_code: 404,
        };

        let service_error = ServiceError {
            service: "user_service".to_string(),
            source: network_error,
        };

        // 测试错误链
        assert_eq!(service_error.to_string(), "Service Error: user_service");
        assert_eq!(
            service_error.source().unwrap().to_string(),
            "Network Error: https://example.com - 404"
        );
    }

    #[test]
    fn test_must_use_attribute() {
        assert!(is_valid_username("rustacean"));
        assert!(!is_valid_username("ab")); // 太短
        assert!(!is_valid_username("has space")); // 含空格
        assert_eq!(Transaction::new(100).commit(), 100);
    }

    #[test]
    fn test_track_caller_attribute() {
        // 带 #[track_caller] 的函数：报告的是调用点所在的行
        let (tracked, line) = (caller_location(), line!());
        assert_eq!(tracked.line(), line);
        assert!(tracked.file().ends_with("mod_06_error_handling.rs"));

        // 不带的函数：报告的是它内部 Location::caller() 所在的行，与调用点不同
        let untracked = plain_location();
        assert_ne!(untracked.line(), line);
    }

    #[test]
    fn test_backtrace_support() {
        use std::backtrace::{Backtrace, BacktraceStatus};
        // force_capture 无视环境变量，一定捕获
        assert_eq!(Backtrace::force_capture().status(), BacktraceStatus::Captured);
    }

    #[test]
    fn test_option_result_inspect() {
        // inspect / inspect_err 看一眼，原值原样返回
        let mut log = Vec::new();
        assert_eq!(parse_port_with_log("8080", &mut log), Ok(8080));
        assert!(parse_port_with_log("http", &mut log).is_err());
        assert_eq!(log.len(), 2);
        assert_eq!(log[0], "解析成功: 8080");
        assert!(log[1].starts_with("解析失败"));

        let mut seen = Vec::new();
        assert_eq!(Some(3).inspect(|v| seen.push(*v)), Some(3));
        assert_eq!(None::<i32>.inspect(|v| seen.push(*v)), None);
        assert_eq!(seen, [3]);
    }

    #[test]
    fn test_custom_exit_code() {
        use std::process::ExitCode;

        assert_eq!(exit_code_of(run_cli(&["run"])), ExitCode::SUCCESS);
        assert_eq!(exit_code_of(run_cli(&[])), ExitCode::from(2));
        assert_eq!(exit_code_of(run_cli(&["bad-config"])), ExitCode::from(3));
        assert_ne!(ExitCode::SUCCESS, ExitCode::FAILURE);
    }
}
