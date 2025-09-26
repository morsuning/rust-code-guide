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

    let result1 = divide(10.0, 2.0);  // Ok(5.0)
    let result2 = divide(10.0, 0.0);  // Err("不能除以零")

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
    let doubled = divide(10.0, 2.0).map(|x| x * 2);
    println!("map 转换: {:?}", doubled);

    // and_then()：链式操作，类似于 flatMap
    let chained = divide(10.0, 2.0)
        .and_then(|x| divide(x, 5.0));
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
    let found = find_element(&numbers, 3);    // Some(2)
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
    let chained = found
        .and_then(|i| numbers.get(i))
        .copied();
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
        let number: i32 = data.parse()
            .map_err(|e| format!("解析错误: {}", e))?;

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

        let number: i32 = trimmed.parse()
            .map_err(|e| format!("解析失败: {}", e))?;

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
        Io(std::io::Error),        // IO 错误
        Parse(std::num::ParseIntError), // 解析错误
        Custom(String),            // 自定义错误
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
        let mut file = std::fs::File::open(file_path)?;      // 可能产生 Io 错误
        let mut content = String::new();
        std::io::Read::read_to_string(&mut file, &mut content)?;

        let number: i32 = content.trim().parse()?;           // 可能产生 Parse 错误

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

        let number: i32 = trimmed.parse()
            .map_err(|e| format!("解析错误: {}", e))?;

        if number < 0 {
            return Err("数字不能为负数".to_string());
        }

        Ok(number * 2)
    }

    // 模式 3: 组合多个操作
    // 处理多个可能失败的操作
    fn combine_operations(a: &str, b: &str) -> Result<i32, String> {
        let num_a = a.parse::<i32>()
            .map_err(|e| format!("解析 a 失败: {}", e))?;

        let num_b = b.parse::<i32>()
            .map_err(|e| format!("解析 b 失败: {}", e))?;

        if num_a == 0 || num_b == 0 {
            return Err("除数不能为零".to_string());
        }

        Ok(num_a / num_b)
    }

    // 模式 4: 错误上下文添加
    // 为错误添加更多上下文信息
    fn read_file_with_context(path: &str) -> Result<String, String> {
        std::fs::read_to_string(path)
            .map_err(|e| format!("读取文件 '{}' 失败: {}", path, e))
    }

    // 模式 5: 可选值的错误处理
    // 将 Option 转换为 Result
    fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
        opt.ok_or_else(|| error_msg.to_string())
    }

    // 测试各种模式
    println!("测试输入验证:");
    for input in vec!["", "valid", "a".repeat(101)] {
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
                AppError::Io(err) => Some(err),      // 返回底层错误
                AppError::ParseInt(err) => Some(err), // 返回底层错误
                _ => None,                           // 没有底层错误
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
    println!("{}", user_friendly_error("计算", Err("无效输入".to_string())));

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
        let content = std::fs::read_to_string(file_path)
            .map_err(|_| ProcessingError::FileNotFound)?;

        // 解析内容
        let mut numbers = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let number: i32 = line.parse()
                .map_err(|e| ProcessingError::ParseError(
                    format!("第 {} 行 '{}': {}", line_num + 1, line, e)
                ))?;

            if number < 0 {
                return Err(ProcessingError::ValidationError(
                    format!("第 {} 行: 负数不允许: {}", line_num + 1, number)
                ));
            }

            numbers.push(number);
        }

        if numbers.is_empty() {
            return Err(ProcessingError::ValidationError(
                "文件不包含有效数字".to_string()
            ));
        }

        Ok(numbers)
    }

    // 计算统计信息
    fn calculate_statistics(numbers: &[i32]) -> (f64, f64, i32) {
        let sum: i32 = numbers.iter().sum();
        let avg = sum as f64 / numbers.len() as f64;
        let max = numbers.iter().max().copied().unwrap_or(0);
        (avg, max, sum)
    }

    // 批量处理文件
    fn batch_process_files(file_paths: &[&str]) -> Vec<(String, Result<Vec<i32>, ProcessingError>)> {
        file_paths.iter()
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
        let number: i32 = input.trim()
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
        "42",
        "150",
        "-10",
        "abc",
        "",
        " 25  ", // 包含空格
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
                _ => Err(NetworkError::InvalidResponse("500 Internal Server Error".to_string())),
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
// 9. Rust 1.76+ core::error::Error trait 改进（Enhanced Error Trait）
// ===========================================

// Rust 1.76 开始对 core::error::Error trait 进行了重大改进
// 这些改进持续到后续版本，提供了更强大的错误处理能力
// 包括更好的错误链、源码位置信息、上下文信息等

// Error trait 改进的核心价值：
// 1. 增强的错误链：提供更清晰的错误溯源能力
// 2. 源码位置信息：自动记录错误发生的位置
// 3. 上下文信息：为错误添加更多的上下文数据
// 4. 更好的诊断：改进的错误显示和调试信息
// 5. 向后兼容：保持与现有代码的兼容性

fn enhanced_error_trait() {
    println!("=== Rust 1.76+ core::error::Error trait 改进 ===");

    // 1. 基础 Error trait 使用（Rust 1.76+）
    // 新版本的 Error trait 提供了更好的默认实现和扩展能力

    use std::error::Error;
    use std::fmt;

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
            self.source.as_ref().map(|s| s.as_ref() as &(dyn Error + 'static))
        }

        // 提供更多的错误详情（Rust 1.76+ 改进）
        fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
            // 在实际应用中，这里可以提供更多的上下文信息
            request.provide_value("query", &self.query);
            request.provide_value("error_type", "database");
        }
    }

    // 2. 错误链和上下文（Rust 1.76+ 改进）
    // 新的 Error trait 提供了更好的错误链处理能力

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

    impl Error for NetworkError {
        fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
            request.provide_value("url", &self.url);
            if let Some(code) = self.status_code {
                request.provide_value("status_code", code);
            }
            request.provide_value("error_type", "network");
        }
    }

    // 创建嵌套的错误链
    fn fetch_user_data(user_id: u32) -> Result<String, DatabaseError> {
        // 模拟数据库查询失败
        if user_id == 999 {
            Err(DatabaseError::new("用户不存在",
                format!("SELECT * FROM users WHERE id = {}", user_id))
                .with_source(NetworkError {
                    message: "连接超时".to_string(),
                    url: "tcp://database:5432".to_string(),
                    status_code: None,
                })
            )
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

            // 使用增强的错误信息（Rust 1.76+）
            println!("  错误详情:");
            if let Some(query) = error::request_value::<String>(&e, "query") {
                println!("    查询: {}", query);
            }
            if let Some(error_type) = error::request_value::<&str>(&e, "error_type") {
                println!("    错误类型: {}", error_type);
            }
        }
    }

    // 3. 错误转换和包装（Rust 1.76+ 改进）
    // 更好的错误类型转换和包装机制

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

        fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
            request.provide_value("operation", &self.operation);
            request.provide_value("context", &self.context);
            request.provide_value("error_type", "service");

            // 将源错误的信息也传递出去
            self.source.provide(request);
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

    // 4. 错误报告和诊断（Rust 1.76+ 改进）
    // 更好的错误报告和诊断信息

    #[derive(Debug)]
    struct ConfigError {
        key: String,
        value: String,
        expected_type: String,
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "配置错误: '{}' 应该是 {} 类型，但得到 '{}'",
                   self.key, self.expected_type, self.value)
        }
    }

    impl Error for ConfigError {
        fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
            request.provide_value("config_key", &self.key);
            request.provide_value("config_value", &self.value);
            request.provide_value("expected_type", &self.expected_type);
            request.provide_value("error_type", "configuration");
        }
    }

    fn parse_config_value(key: &str, value: &str, expected_type: &str) -> Result<(), ConfigError> {
        match expected_type {
            "integer" => {
                value.parse::<i32>().map_err(|_| {
                    ConfigError {
                        key: key.to_string(),
                        value: value.to_string(),
                        expected_type: expected_type.to_string(),
                    }
                })?;
            }
            "boolean" => {
                value.parse::<bool>().map_err(|_| {
                    ConfigError {
                        key: key.to_string(),
                        value: value.to_string(),
                        expected_type: expected_type.to_string(),
                    }
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

                // 使用增强的错误信息
                if let Some(config_key) = error::request_value::<String>(&e, "config_key") {
                    println!("  配置键: {}", config_key);
                }
                if let Some(config_value) = error::request_value::<String>(&e, "config_value") {
                    println!("  配置值: {}", config_value);
                }
            }
        }
    }

    // 5. 错误日志和监控（Rust 1.76+ 改进）
    // 更好的错误日志记录和监控支持

    fn log_error_enhanced(error: &dyn Error) {
        println!("[ERROR] {}", error);

        // 记录错误源信息
        if let Some(source) = error.source() {
            println!("[ERROR]   源: {}", source);
        }

        // 记录结构化的错误信息
        let mut request = error::Request::new();
        error.provide(&mut request);

        println!("[ERROR]   详细信息:");
        if let Some(operation) = request.value::<String>() {
            println!("[ERROR]     操作: {}", operation);
        }
        if let Some(error_type) = request.value::<&str>() {
            println!("[ERROR]     类型: {}", error_type);
        }
    }

    // 模拟一个复杂错误
    let complex_error = ServiceError::new(
        "data_processing",
        "数据处理失败",
        DatabaseError::new("查询超时", "SELECT * FROM large_table")
            .with_source(NetworkError {
                message: "网络不稳定".to_string(),
                url: "tcp://primary-db:5432".to_string(),
                status_code: Some(503),
            })
    );

    log_error_enhanced(&complex_error);

    // Rust 1.76+ Error trait 改进的最佳实践：
    // 1. 实现provide方法：为错误提供丰富的上下文信息
    // 2. 正确的错误链：使用source()方法提供错误溯源
    // 3. 结构化错误信息：使用request_value等方法提供结构化数据
    // 4. 清晰的错误消息：Display trait 实现应该提供用户友好的信息
    // 5. 错误分类：通过错误类型和上下文进行分类处理

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

    println!("错误处理演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(validate_input(&"a".repeat(101)), Err("输入不能超过 100 个字符".to_string()));
    }

    #[test]
    fn test_process_chain() {
        fn process_chain(input: &str) -> Result<i32, String> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err("输入为空".to_string());
            }

            let number: i32 = trimmed.parse()
                .map_err(|e| format!("解析错误: {}", e))?;

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
        assert_eq!(calculate_average(&[]), Err("不能计算空数组的平均值".to_string()));
    }

    #[test]
    fn test_option_to_result() {
        fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
            opt.ok_or_else(|| error_msg.to_string())
        }

        let some_value = Some(42);
        let none_value: Option<i32> = None;

        assert_eq!(option_to_result(some_value, "值不存在"), Ok(42));
        assert_eq!(option_to_result(none_value, "值不存在"), Err("值不存在".to_string()));
    }

    #[test]
    fn test_user_input_processing() {
        fn process_user_input(input: &str) -> Result<i32, String> {
            if input.trim().is_empty() {
                return Err("输入不能为空".to_string());
            }

            let number: i32 = input.trim()
                .parse()
                .map_err(|e| format!("'{}' 不是有效的整数: {}", input, e))?;

            if number < 1 || number > 100 {
                return Err(format!("数字 {} 不在 1-100 范围内", number));
            }

            Ok(number * 2)
        }

        assert_eq!(process_user_input("42"), Ok(84));
        assert_eq!(process_user_input("150"), Err("数字 150 不在 1-100 范围内".to_string()));
        assert!(process_user_input("abc").is_err());
        assert_eq!(process_user_input(""), Err("输入不能为空".to_string()));
    }

    #[test]
    fn test_error_context() {
        fn read_file_with_context(path: &str) -> Result<String, String> {
            std::fs::read_to_string(path)
                .map_err(|e| format!("读取文件 '{}' 失败: {}", path, e))
        }

        let result = read_file_with_context("nonexistent.txt");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("nonexistent.txt"));
        assert!(error_msg.contains("读取文件"));
    }

    // Rust 1.76+ 增强的 Error trait 特性测试
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
                self.source.as_deref()
            }

            fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
                request
                    .provide_value::<&str>(&self.message)
                    .provide_value("error_code", &500u16)
                    .provide_value("context_data", &self.context_data);
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

        // 测试结构化错误数据访问
        let mut request = error::Request::new();
        error.provide(&mut request);

        // 验证提供了结构化数据
        assert!(request.value::<&str>().is_some());
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

        impl Error for NetworkError {
            fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
                request
                    .provide_value("url", &self.url)
                    .provide_value("status_code", &self.status_code)
                    .provide_value("error_type", "network");
            }
        }

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

            fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
                request
                    .provide_value("service", &self.service)
                    .provide_value("error_type", "service");
                // 委托给源错误以提供更多信息
                self.source.provide(request);
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
        assert_eq!(service_error.source().unwrap().to_string(),
                   "Network Error: https://example.com - 404");
    }
}