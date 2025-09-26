// Rust 基础语法特性
// 包含变量、数据类型、函数、注释和控制流的深入讲解

// ===========================================
// 1. 变量和可变性 (Variables and Mutability)
// ===========================================

// Rust 的变量系统是其内存安全特性的核心基础
// 通过默认不可变的机制，Rust 在编译时就能避免许多并发错误
// 这是 Rust 区别于其他系统编程语言的关键设计哲学

fn variables_and_mutability() {
    println!("=== 变量和可变性 ===");

    // 不可变变量：Rust 的默认选择
    // 这不是限制，而是安全性的保障。通过默认不可变，
    // Rust 可以确保数据在多线程环境下不会被意外修改，
    // 从而避免了数据竞争（data race）等问题。
    let x = 5;
    println!("不可变变量 x 的值: {}", x);
    // x = 6; // 编译错误：不能二次赋值给不可变变量 x
    // 这个编译错误是 Rust 编译器在保护我们免于潜在的 bug

    // 可变变量：需要明确声明 mut 关键字
    // 当你确实需要修改变量时，必须明确表达这个意图
    // 这种明确的声明让代码的意图更加清晰，便于理解和维护
    let mut y = 5;
    println!("可变变量 y 的初始值: {}", y);
    y = 6;
    println!("可变变量 y 修改后的值: {}", y);

    // 最佳实践：优先使用不可变变量，只在必要时使用可变变量
    // 这样可以减少代码的复杂性，提高可维护性和安全性

    // 常量：编译时常量，永远不可变
    // 常量与变量的区别：
    // 1. 常量必须在声明时初始化，且值必须是编译时可确定的
    // 2. 常量在任何地方都不可变，没有 mut 选项
    // 3. 常量可以定义在任何作用域，包括全局作用域
    // 4. 常量命名约定使用全大写字母，单词间用下划线分隔
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS: {}", MAX_POINTS);

    // 变量遮蔽（Shadowing）：允许重新声明同名变量
    // 这是一个强大的特性，可以在不改变变量可变性的情况下修改变量的值或类型
    let z = 5;
    let z = z + 1; // 遮蔽前面的 z，但仍然是不可变的
    println!("第一次遮蔽后的 z: {}", z);

    {
        let z = z * 2; // 在内部作用域中遮蔽
        println!("内部作用域的 z: {}", z);
    }
    println!("外部作用域的 z: {}", z); // 恢复为外部作用域的值

    // 遮蔽的实用场景：
    // 1. 转换数据类型时保持变量名不变
    let spaces = "   ";
    let spaces = spaces.len(); // 从 &str 转换为 usize
    println!("字符串长度: {}", spaces);

    // 2. 在不同作用域中使用相同变量名
    // 3. 逐步处理数据时保持代码的连续性

    // 注意：遮蔽与 mut 的区别
    // mut 是修改同一个内存位置的值
    // 遮蔽是创建一个新的变量绑定，可能使用不同的内存位置和类型

    println!();
}

// ===========================================
// 2. 数据类型 (Data Types)
// ===========================================

// Rust 是静态类型语言，但具有强大的类型推断能力
// 类型系统是 Rust 内存安全和性能优化的另一个关键支柱

fn data_types() {
    println!("=== 数据类型 ===");

    // 标量类型（Scalar Types）：表示单个值

    // 整数类型：根据大小和符号有不同的选择
    // Rust 的整数类型设计考虑了不同的使用场景：
    // - 位宽：决定能表示的数值范围
    // - 符号：决定是否支持负数
    // - 架构：isize 和 usize 的大小与目标架构一致

    let a: i8 = -128;        // 8位有符号整数：-128 到 127
    let b: u8 = 255;         // 8位无符号整数：0 到 255
    let c: i16 = -32768;     // 16位有符号整数：-32768 到 32767
    let d: u16 = 65535;      // 16位无符号整数：0 到 65535
    let e: i32 = -2147483648; // 32位有符号整数：-2147483648 到 2147483647
    let f: u32 = 4294967295;  // 32位无符号整数：0 到 4294967295
    let g: i64 = -9223372036854775808; // 64位有符号整数
    let h: u64 = 18446744073709551615;  // 64位无符号整数
    let i: i128 = -170141183460469231731687303715884105728; // 128位有符号整数
    let j: u128 = 340282366920938463463374607431768211455;  // 128位无符号整数

    // 架构相关类型：大小取决于目标平台
    let k: isize = -4;       // 有符号整数，大小与指针相同
    let l: usize = 4;        // 无符号整数，大小与指针相同

    // 整数字面量的多种表示方式
    let decimal = 98_222;     // 十进制，下划线提高可读性
    let hex = 0xff;          // 十六进制
    let octal = 0o77;        // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A';         // 字节字面量（仅适用于 u8）

    println!("各种整数类型和表示方式");

    // 浮点类型：IEEE 754 标准的浮点数
    // f32：单精度浮点数，精度约为 6-9 位十进制数
    // f64：双精度浮点数，精度约为 15-17 位十进制数（默认）
    let m: f32 = 3.1415926;  // 单精度
    let n: f64 = 2.718281828459045; // 双精度

    // 浮点数的特殊值
    let infinity: f64 = f64::INFINITY;
    let negative_infinity: f64 = f64::NEG_INFINITY;
    let nan: f64 = f64::NAN;

    println!("浮点类型示例: {}, {}", m, n);

    // 布尔类型：逻辑运算的基础
    // 占用 1 字节内存，只能取 true 或 false
    let p = true;
    let q: bool = false;  // 显式类型标注

    // 布尔运算
    let r = p && q;       // 逻辑与
    let s = p || q;       // 逻辑或
    let t = !p;           // 逻辑非

    println!("布尔运算: &&={}, ||={}, !={}", r, s, t);

    // 字符类型：Unicode 标量值
    // Rust 的 char 类型占用 4 字节，可以表示任何 Unicode 字符
    // 这与许多其他语言中 1 字节的 char 类型不同
    let heart_eyed_cat = '😻';
    let z = 'ℤ';
    let heart = '❤';

    println!("Unicode 字符: {}, {}, {}", heart_eyed_cat, z, heart);

    // 字符类型与字符串字面量的区别
    // char 是单个 Unicode 字符
    // "hello" 是字符串字面量（string literal），类型为 &str
    let single_char = 'A';
    let string_slice = "hello";

    println!("字符类型: '{}', 字符串切片: \"{}\"", single_char, string_slice);

    // 复合类型（Compound Types）：可以将多个值组合成一个类型

    // 元组（Tuple）：固定大小的有序集合
    // 元组的长度在创建时确定，不能改变
    // 元组在函数返回多个值时特别有用
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 访问元组元素的方式
    let (x, y, z) = tup; // 模式解构
    println!("解构后的元组: x={}, y={}, z={}", x, y, z);

    let five_hundred = tup.0; // 使用索引访问
    let six_point_four = tup.1;
    let one = tup.2;
    println!("索引访问元组: {}, {}, {}", five_hundred, six_point_four, one);

    // 单元元组：特殊的空元组
    // 在函数不返回值时隐式使用
    let unit = ();
    println!("单元元组: {:?}", unit);

    // 数组（Array）：固定大小的同类型元素集合
    // 与元组不同，数组的所有元素必须是相同类型
    // 数组在栈上分配，大小在编译时确定
    let a = [1, 2, 3, 4, 5]; // 类型推断为 [i32; 5]
    let months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "October", "November", "December"];

    // 显式类型标注
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // 创建包含 5 个 3 的数组

    // 访问数组元素
    let first = a[0];
    let second = a[1];
    println!("数组访问: first={}, second={}", first, second);

    // 数组的内存表示
    // 数组在内存中是连续存储的，这使得访问非常高效
    // Rust 会进行边界检查，防止缓冲区溢出

    // 数组越界访问的安全性
    // let invalid = a[10]; // 这会导致编译时错误（如果索引是字面量）
    // 或运行时 panic（如果索引是变量）

    println!("数组和元组的区别：");
    println!("- 数组：所有元素类型相同，固定大小，连续内存");
    println!("- 元组：元素类型可不同，固定大小，可能不连续内存");

    println!();
}

// ===========================================
// 3. 函数 (Functions)
// ===========================================

// Rust 的函数是一等公民，支持闭包、高阶函数等函数式编程特性
// 函数设计遵循清晰、明确、安全的哲学

// 函数的基本结构：fn 关键字 + 函数名 + 参数 + 返回类型 + 函数体
fn add_one(x: i32) -> i32 {
    // 在 Rust 中，函数体最后一个表达式（没有分号）就是返回值
    // 这称为"表达式"（expression），而不是"语句"（statement）
    x + 1
}

// 使用 return 关键字的显式返回
fn add_two(x: i32) -> i32 {
    return x + 2;
}

// 无返回值的函数
fn print_message(message: &str) {
    println!("消息: {}", message);
    // 函数没有返回值时，隐式返回单元类型 ()
}

// 无参数函数
fn get_pi() -> f64 {
    3.141592653589793
}

// 函数参数的规则
// 1. 必须声明每个参数的类型
// 2. 多个参数用逗号分隔
// 3. 参数所有权会被移动到函数中（对于非 Copy 类型）

fn functions() {
    println!("=== 函数 ===");

    // 基本函数调用
    let result1 = add_one(5);
    let result2 = add_two(5);
    println!("函数调用结果: add_one(5)={}, add_two(5)={}", result1, result2);

    // 无返回值函数
    print_message("Hello, Functions!");

    // 常量函数
    let pi = get_pi();
    println!("圆周率: {}", pi);

    // 语句 vs 表达式
    // 语句（Statement）：执行操作但不返回值的指令
    // 表达式（Expression）：计算并返回值的代码片段

    // 语句示例
    let y = 6; // 这是语句，它不返回值

    // 表达式示例
    let x = 5; // 5 是表达式
    let y = {
        let x = 3;
        x + 1 // 这是表达式（没有分号）
    };
    println!("块表达式结果: {}", y);

    // 函数作为参数（高阶函数）
    fn apply_function<F>(x: i32, f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let result = apply_function(10, add_one);
    println!("高阶函数调用: apply_function(10, add_one) = {}", result);

    // 闭包作为参数
    let result2 = apply_function(10, |x| x * 2);
    println!("闭包作为参数: apply_function(10, |x| x * 2) = {}", result2);

    println!();
}

// ===========================================
// 4. 注释 (Comments)
// ===========================================

// 注释是代码文档的重要组成部分，帮助理解代码的设计意图
// Rust 支持多种注释形式，每种都有其特定的用途

fn comments_example() {
    println!("=== 注释示例 ===");

    // 这是单行注释：解释单行代码或提供简短说明

    /*
    这是多行注释：适用于较长的说明
    可以跨越多行，用于解释复杂的逻辑
    或提供详细的文档
    */

    /// 这是文档注释：用于生成 HTML 文档
    /// 支持 Markdown 格式，可以包含代码示例
    /// 使用 `cargo doc` 命令生成文档
    /// # 标题
    /// ## 子标题
    /// ```rust
    /// let x = 5;
    /// ```
    fn documented_function() {
        // 这是一个有文档的函数
    }

    //! 这是模块级文档注释：用于描述整个模块
    //! 通常放在文件的开头，描述模块的用途和功能

    // 注释的最佳实践：
    // 1. 解释"为什么"而不是"是什么"
    // 2. 保持注释与代码同步更新
    // 3. 使用文档注释为公共 API 生成文档
    // 4. 在复杂算法处添加详细注释
    // 5. 避免显而易见的注释

    // 代码示例：良好的注释风格
    let mut counter = 0; // 计数器，跟踪处理的项目数量

    // 计算平均值，处理可能的除零错误
    let average = if counter > 0 {
        // 使用浮点数除法获得精确结果
        (total as f64) / (counter as f64)
    } else {
        0.0 // 没有项目时返回 0
    };

    println!("注释示例完成");
    println!();
}

// ===========================================
// 5. 控制流 (Control Flow)
// ===========================================

// 控制流是编程语言的核心功能，Rust 提供了丰富的控制流结构
// 这些结构既保持了安全性，又提供了强大的表达能力

fn control_flow() {
    println!("=== 控制流 ===");

    // if 表达式：条件分支
    // Rust 的 if 是表达式，可以返回值
    let number = 6;

    if number % 4 == 0 {
        println!("{} 可以被 4 整除", number);
    } else if number % 3 == 0 {
        println!("{} 可以被 3 整除", number);
    } else {
        println!("{} 不能被 4 或 3 整除", number);
    }

    // if 作为表达式返回值
    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("if 表达式结果: {}", x);

    // 注意：if 表达式的所有分支必须返回相同类型
    // let x = if condition { 5 } else { "six" }; // 编译错误：类型不匹配

    // 循环（loop）：无限循环，直到显式 break
    // loop 是表达式，可以返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以带返回值
        }
    };
    println!("loop 循环结果: {}", result);

    // while 循环：条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for 循环：遍历迭代器
    // for 循环是 Rust 中最常用的循环方式
    // 它安全、高效，不会出现索引越界错误
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("元素值: {}", element);
    }

    // 使用范围（range）进行 for 循环
    // Range 类型提供了多种范围表示方式
    for number in (1..4).rev() { // 反向迭代
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // 不同范围的类型：
    // 1..5   : 包含 1，不包含 5 [1, 5)
    // 1..=5  : 包含 1 和 5 [1, 5]
    // ..5    : 从开始到 5 (不包含 5)
    // ..=5   : 从开始到 5 (包含 5)

    // for 循环与索引的对比
    let array = [10, 20, 30, 40, 50];

    // 推荐：使用迭代器
    for element in array.iter() {
        println!("安全访问: {}", element);
    }

    // 不推荐：使用索引（虽然安全，但不如迭代器优雅）
    for index in 0..array.len() {
        println!("索引访问: {}", array[index]);
    }

    // 控制流的高级特性
    // continue：跳过当前迭代
    // break：退出循环
    // 标签（labels）：控制嵌套循环的退出

    'outer: for x in 0..10 {
        for y in 0..10 {
            if x + y == 15 {
                break 'outer; // 退出外层循环
            }
        }
    }

    // match 表达式：强大的模式匹配
    // match 是 Rust 的模式匹配工具，比 C 语言的 switch 更强大
    let number = 13;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"), // 通配符模式
    }

    // match 作为表达式
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("布尔转二进制: {}", binary);

    // match 的穷尽性检查
    // Rust 编译器会确保 match 覆盖所有可能的情况
    // 这使得代码更加安全和完整

    println!();
}

// ===========================================
// 6. Rust 1.65+ let-else 语句（let-else Statement）
// ===========================================

// let-else 语句是 Rust 1.65 引入的重要语法特性
// 它允许在变量绑定失败时直接执行 else 分支，大大简化了错误处理代码
// 这个特性使 Rust 的控制流更加优雅和表达力更强

// let-else 语句的核心价值：
// 1. 简化错误处理：将模式匹配和错误处理合并为单一语句
// 2. 提高代码可读性：减少嵌套的 if-let 结构
// 3. 提前返回：在 else 分支中可以提前返回或中断执行
// 4. 保持作用域：绑定的变量在外部作用域中可用

fn let_else_statement() {
    println!("=== Rust 1.65+ let-else 语句 ===");

    // 1. 基础 let-else 语法
    // let-else 的基本形式：let PATTERN = EXPR else { BLOCK };
    // 如果模式匹配成功，变量被绑定到当前作用域
    // 如果模式匹配失败，执行 else 块中的代码

    fn parse_number(s: &str) -> Option<i32> {
        s.parse().ok()
    }

    // 传统方式：使用 if-let
    let input = "123";
    if let Some(number) = parse_number(input) {
        println!("传统方式解析成功：{}", number);
    } else {
        println!("传统方式解析失败");
        return;  // 提前返回
    }
    println!("传统方式继续执行");

    // Rust 1.65+ let-else 方式
    let input = "456";
    let Some(number) = parse_number(input) else {
        println!("let-else 解析失败");
        return;  // 提前返回
    };
    println!("let-else 解析成功：{}", number);
    println!("let-else 继续执行");

    // 2. 在错误处理中的应用
    // let-else 特别适合处理 Option 和 Result 类型的错误情况

    fn get_user_by_id(id: u32) -> Option<&'static str> {
        match id {
            1 => Some("Alice"),
            2 => Some("Bob"),
            3 => Some("Charlie"),
            _ => None,
        }
    }

    fn get_user_age(user: &str) -> Option<u32> {
        match user {
            "Alice" => Some(25),
            "Bob" => Some(30),
            "Charlie" => Some(35),
            _ => None,
        }
    }

    // 嵌套的错误处理
    let user_id = 2;
    let Some(user) = get_user_by_id(user_id) else {
        println!("用户 ID {} 不存在", user_id);
        return;
    };

    let Some(age) = get_user_age(user) else {
        println!("用户 {} 的年龄信息不可用", user);
        return;
    };

    println!("用户 {} 的年龄是 {}", user, age);

    // 3. 在函数参数验证中的应用
    fn validate_config(config: &str) -> Result<&'static str, String> {
        if config.is_empty() {
            Err("配置不能为空".to_string())
        } else if config.len() > 100 {
            Err("配置过长".to_string())
        } else {
            Ok("配置有效")
        }
    }

    fn process_config(config: &str) -> String {
        let Ok(validation_msg) = validate_config(config) else {
            return format!("配置验证失败: {}", config);
        };

        format!("处理成功: {}", validation_msg)
    }

    println!("配置处理结果: {}", process_config("valid_config"));
    println!("配置处理结果: {}", process_config(""));

    // 4. 在文件处理中的应用
    fn read_file_content(filename: &str) -> Option<&'static str> {
        match filename {
            "config.txt" => Some("database_url=localhost\nport=5432"),
            "settings.json" => Some("{\"theme\": \"dark\", \"lang\": \"zh\"}"),
            _ => None,
        }
    }

    fn extract_database_url(content: &str) -> Option<&str> {
        content.lines().find(|line| line.starts_with("database_url"))
    }

    let filename = "config.txt";
    let Some(content) = read_file_content(filename) else {
        println!("文件 {} 不存在", filename);
        return;
    };

    let Some(db_url) = extract_database_url(content) else {
        println!("文件中未找到数据库 URL");
        return;
    };

    println!("找到数据库 URL: {}", db_url);

    // 5. 在网络请求处理中的应用
    #[derive(Debug)]
    enum HttpResponse {
        Success { data: String, status: u16 },
        Error { message: String, code: u32 },
    }

    fn make_api_request(endpoint: &str) -> HttpResponse {
        match endpoint {
            "/api/users" => HttpResponse::Success {
                data: "[{\"id\": 1, \"name\": \"Alice\"}]".to_string(),
                status: 200,
            },
            "/api/error" => HttpResponse::Error {
                message: "Internal Server Error".to_string(),
                code: 500,
            },
            _ => HttpResponse::Error {
                message: "Not Found".to_string(),
                code: 404,
            },
        }
    }

    let endpoint = "/api/users";
    let HttpResponse::Success { data, status } = make_api_request(endpoint) else {
        println!("API 请求失败: {}", endpoint);
        return;
    };

    println!("API 响应成功 (状态码 {}): {}", status, data);

    // 6. 在配置解析中的应用
    #[derive(Debug)]
    struct ServerConfig {
        host: String,
        port: u16,
        max_connections: u32,
    }

    fn parse_server_config(config_str: &str) -> Option<ServerConfig> {
        let parts: Vec<&str> = config_str.split(':').collect();
        if parts.len() != 3 {
            return None;
        }

        let host = parts[0].to_string();
        let port: u16 = parts[1].parse().ok()?;
        let max_connections: u32 = parts[2].parse().ok()?;

        Some(ServerConfig {
            host,
            port,
            max_connections,
        })
    }

    let config_str = "localhost:8080:100";
    let Some(config) = parse_server_config(config_str) else {
        println!("配置解析失败: {}", config_str);
        return;
    };

    println!("服务器配置: {:?}", config);

    // let-else 语句的最佳实践：
    // 1. 适合提前返回的场景：当模式匹配失败时希望立即返回或中断
    // 2. 保持 else 块简洁：else 块通常应该包含返回、break 或 continue
    // 3. 合理使用作用域：绑定的变量在 else 块外部可用
    // 4. 避免过度嵌套：let-else 本身就是为了减少嵌套，不要在 else 中再嵌套
    // 5. 错误信息清晰：在 else 块中提供有用的错误信息

    println!();
}

// ===========================================
// 7. 实际应用示例
// ===========================================

fn practical_examples() {
    println!("=== 实际应用示例 ===");

    // 示例 1：简单的计算器
    fn calculate(a: i32, b: i32, operation: &str) -> Result<i32, String> {
        match operation {
            "+" => Ok(a + b),
            "-" => Ok(a - b),
            "*" => Ok(a * b),
            "/" => {
                if b != 0 {
                    Ok(a / b)
                } else {
                    Err("除数不能为零".to_string())
                }
            }
            _ => Err("未知操作".to_string()),
        }
    }

    match calculate(10, 5, "+") {
        Ok(result) => println!("10 + 5 = {}", result),
        Err(e) => println!("错误: {}", e),
    }

    // 示例 2：FizzBuzz 问题
    // 使用 if-else 结构解决经典编程问题
    for number in 1..=100 {
        if number % 15 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }

    // 示例 3：数据验证
    fn validate_age(age: i32) -> Result<(), String> {
        if age < 0 {
            return Err("年龄不能为负数".to_string());
        }
        if age > 150 {
            return Err("年龄过大".to_string());
        }
        Ok(())
    }

    match validate_age(25) {
        Ok(_) => println!("年龄有效"),
        Err(e) => println!("年龄无效: {}", e),
    }

    // 示例 4：使用循环处理用户输入（模拟）
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        attempts += 1;
        println!("尝试 {} / {}", attempts, max_attempts);

        if attempts >= max_attempts {
            println!("达到最大尝试次数");
            break;
        }

        // 模拟成功条件
        if attempts == 2 {
            println!("成功！");
            break;
        }
    }

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 基础语法演示");
    println!("=================");

    variables_and_mutability();
    data_types();
    functions();
    comments_example();
    control_flow();
    let_else_statement();
    practical_examples();
    compiler_attributes();

    println!("基础语法演示完成！");
}

// ===========================================
// 13. Rust 1.78 #[expect(lint)] 属性
// ===========================================

// Rust 1.78 引入了 #[expect(lint)] 属性，这是一个重要的编译器改进
// 它允许开发者明确预期某些 lint 警告，从而提供更好的代码维护体验
// 与 #[allow] 不同，#[expect] 会确保如果预期的警告没有出现，编译器会报错

fn compiler_attributes() {
    println!("=== Rust 1.78 #[expect(lint)] 属性 ===");

    // #[expect(lint)] 属性的背景和意义：
    // 1. 显式意图：明确表达对特定 lint 的预期
    // 2. 维护性保障：当代码修改导致预期警告消失时，编译器会提醒
    // 3. 文档作用：告诉其他开发者这个警告是预期的，不是遗漏
    // 4. 渐进式改进：可以逐步清理代码中的警告
    // 5. 团队协作：统一团队对 lint 警告的处理策略

    // 传统方法与 #[expect] 的对比：
    // #[allow(dead_code)] - 静默警告，没有维护保障
    // #[expect(dead_code)] - 预期警告，如果警告消失会报错

    // 示例 1: 预期 dead_code 警告
    #[expect(dead_code, reason = "这个函数用于测试目的，故意未使用")]
    fn unused_function() {
        println!("这是一个故意未使用的函数");
    }

    // 示例 2: 预期 unused_variables 警告
    fn function_with_unused_variables() {
        #[expect(unused_variables, reason = "变量预留用于未来扩展")]
        let _future_feature = "预留变量";

        println!("函数中包含预期的未使用变量");
    }

    // 示例 3: 预期 deprecated 警告
    #[deprecated(since = "1.0.0", note = "使用 new_function 替代")]
    #[expect(deprecated, reason = "测试已弃用函数的预期警告")]
    fn old_function() {
        println!("这是一个已弃用的函数");
    }

    fn new_function() {
        println!("这是新的替代函数");
    }

    // 演示调用已弃用函数
    old_function();

    // 示例 4: 在模块级别使用 #[expect]
    mod example_module {
        #![expect(unused_imports, reason = "此模块包含用于演示的导入")]

        use std::collections::HashMap;
        use std::fs::File;
        use std::io::Read;

        // 这些导入是为了演示 #[expect] 的使用
        // 在实际应用中，这些导入可能会被使用
    }

    // 示例 5: 与 #[allow] 的对比
    fn compare_expect_vs_allow() {
        println!("--- #[expect] vs #[allow] 对比 ---");

        // 使用 #[allow] - 静默警告，无维护保障
        #[allow(unused_variables)]
        let _allowed_variable = "这不会产生警告";

        // 使用 #[expect] - 预期警告，有维护保障
        #[expect(unused_variables, reason = "演示预期警告")]
        let _expected_variable = "这应该产生警告";

        println!("#[expect] 提供了更好的维护保障");
    }

    // 示例 6: 多重 lint 预期
    #[expect(
        dead_code,
        unused_variables,
        reason = "演示同时预期多个 lint"
    )]
    fn multiple_lints_example() {
        let _unused = "多个 lint 预期";
        println!("同时预期多个 lint 警告");
    }

    // 示例 7: 条件性预期警告
    #[cfg(debug_assertions)]
    #[expect(dead_code, reason = "调试模式下的测试代码")]
    fn debug_only_function() {
        println!("仅在调试模式下使用的函数");
    }

    // 实际应用场景
    practical_expect_examples();

    println!("编译器属性演示完成");
    println!();
}

fn practical_expect_examples() {
    println!("=== #[expect] 实际应用示例 ===");

    // 场景 1: API 向后兼容性
    mod legacy_api {
        #[expect(deprecated, reason = "保持向后兼容性")]
        #[deprecated(since = "2.0.0", note = "使用 new_api() 替代")]
        pub fn old_api() {
            println!("遗留 API 实现");
        }

        pub fn new_api() {
            println!("新 API 实现");
        }

        // 在某些情况下，我们可能需要保留旧的 API 调用
        pub fn transition_code() {
            #[expect(deprecated, reason = "过渡期代码，将在下个版本移除")]
            old_api(); // 临时调用旧 API
            new_api(); // 同时使用新 API
        }
    }

    // 场景 2: 测试代码中的预期警告
    mod test_utilities {
        #[expect(dead_code, reason = "测试工具函数，按需使用")]
        pub fn setup_test_environment() {
            println!("设置测试环境");
        }

        #[expect(dead_code, reason = "测试数据生成器")]
        pub fn generate_test_data() -> Vec<i32> {
            vec![1, 2, 3, 4, 5]
        }

        #[expect(unused_variables, reason = "测试占位符变量")]
        pub fn test_placeholder() {
            let _placeholder = "测试占位符";
        }
    }

    // 场景 3: 条件编译的预期警告
    #[cfg(feature = "experimental")]
    mod experimental_features {
        #![expect(dead_code, reason = "实验性功能，按需启用")]

        pub fn experimental_feature() {
            println!("实验性功能实现");
        }
    }

    // 场景 4: 代码生成器的预期警告
    mod generated_code {
        // 在实际应用中，这可能是由代码生成器产生的代码
        #[expect(dead_code, reason = "由代码生成器产生，保留供未来使用")]
        fn generated_placeholder() {
            // 占位符实现
        }

        #[expect(unused_variables, reason = "生成代码中的预留变量")]
        fn generated_with_placeholders() {
            let _future_impl = "预留实现";
        }
    }

    // 场景 5: 性能优化相关的预期警告
    mod performance_code {
        #[expect(clippy::modulo_one, reason = "性能优化相关的数学运算")]
        pub fn optimized_calculation() {
            // 某些性能优化可能会触发 lint
            let _result = 100 % 1; // 故意的模1运算
        }
    }

    println!("实际应用示例演示完成");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_mutability() {
        let x = 5;
        assert_eq!(x, 5);

        let mut y = 5;
        y = 6;
        assert_eq!(y, 6);
    }

    #[test]
    fn test_data_types() {
        let tup = (500, 6.4, 1);
        assert_eq!(tup.0, 500);
        assert_eq!(tup.1, 6.4);
        assert_eq!(tup.2, 1);
    }

    #[test]
    fn test_functions() {
        assert_eq!(add_one(5), 6);
        assert_eq!(add_two(5), 7);
    }

    #[test]
    fn test_control_flow() {
        let x = if true { 5 } else { 6 };
        assert_eq!(x, 5);

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 3 {
                break counter * 2;
            }
        };
        assert_eq!(result, 6);
    }

    #[test]
    fn test_calculator() {
        assert_eq!(calculate(10, 5, "+"), Ok(15));
        assert_eq!(calculate(10, 5, "/"), Ok(2));
        assert_eq!(calculate(10, 0, "/"), Err("除数不能为零".to_string()));
    }

    #[test]
    fn test_age_validation() {
        assert_eq!(validate_age(25), Ok(()));
        assert_eq!(validate_age(-1), Err("年龄不能为负数".to_string()));
        assert_eq!(validate_age(200), Err("年龄过大".to_string()));
    }

    #[test]
    fn test_let_else_statement() {
        // 测试基础的 let-else 语法
        fn parse_number(s: &str) -> Option<i32> {
            s.parse().ok()
        }

        // 测试成功情况
        let input = "123";
        let Some(number) = parse_number(input) else {
            panic!("测试失败：应该成功解析数字");
        };
        assert_eq!(number, 123);

        // 测试失败情况（通过函数测试）
        fn test_parse_failure() -> bool {
            let input = "invalid";
            let Some(_number) = parse_number(input) else {
                return true;  // 预期的失败情况
            };
            false  // 不应该到达这里
        }
        assert!(test_parse_failure());

        // 测试嵌套的 let-else
        fn get_user_by_id(id: u32) -> Option<&'static str> {
            match id {
                1 => Some("Alice"),
                2 => Some("Bob"),
                _ => None,
            }
        }

        fn get_user_age(user: &str) -> Option<u32> {
            match user {
                "Alice" => Some(25),
                "Bob" => Some(30),
                _ => None,
            }
        }

        fn test_nested_let_else() -> bool {
            let user_id = 1;
            let Some(user) = get_user_by_id(user_id) else {
                return false;
            };

            let Some(age) = get_user_age(user) else {
                return false;
            };

            age == 25
        }
        assert!(test_nested_let_else());

        // 测试 Result 类型的 let-else
        fn validate_config(config: &str) -> Result<&'static str, String> {
            if config.is_empty() {
                Err("配置不能为空".to_string())
            } else if config.len() > 100 {
                Err("配置过长".to_string())
            } else {
                Ok("配置有效")
            }
        }

        let config = "valid";
        let Ok(msg) = validate_config(config) else {
            panic!("配置验证应该成功");
        };
        assert_eq!(msg, "配置有效");
    }

    #[test]
    fn test_let_else_with_enums() {
        #[derive(Debug)]
        enum HttpResponse {
            Success { data: String, status: u16 },
            Error { message: String, code: u32 },
        }

        fn make_api_request(endpoint: &str) -> HttpResponse {
            match endpoint {
                "/api/users" => HttpResponse::Success {
                    data: "[{\"id\": 1, \"name\": \"Alice\"}]".to_string(),
                    status: 200,
                },
                "/api/error" => HttpResponse::Error {
                    message: "Internal Server Error".to_string(),
                    code: 500,
                },
                _ => HttpResponse::Error {
                    message: "Not Found".to_string(),
                    code: 404,
                },
            }
        }

        // 测试成功的 API 响应
        let endpoint = "/api/users";
        let HttpResponse::Success { data, status } = make_api_request(endpoint) else {
            panic!("API 请求应该成功");
        };
        assert_eq!(status, 200);
        assert!(data.contains("Alice"));

        // 测试失败的 API 响应
        fn test_failed_api_request() -> bool {
            let endpoint = "/api/nonexistent";
            let HttpResponse::Success { .. } = make_api_request(endpoint) else {
                return true;  // 预期的失败情况
            };
            false
        }
        assert!(test_failed_api_request());
    }
}

// 辅助函数
fn calculate(a: i32, b: i32, operation: &str) -> Result<i32, String> {
    match operation {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b != 0 {
                Ok(a / b)
            } else {
                Err("除数不能为零".to_string())
            }
        }
        _ => Err("未知操作".to_string()),
    }
}

fn validate_age(age: i32) -> Result<(), String> {
    if age < 0 {
        return Err("年龄不能为负数".to_string());
    }
    if age > 150 {
        return Err("年龄过大".to_string());
    }
    Ok(())
}