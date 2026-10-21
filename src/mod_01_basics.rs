// 教程里的类型、字段和方法经常只为演示语法而定义，不一定会被全部读取或调用，所以允许 dead_code；
// 其他 lint（未使用的导入、变量、可变性等）必须保持干净，不要再给模块添加新的 allow。
#![allow(dead_code)]

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

    let a: i8 = -128; // 8位有符号整数：-128 到 127
    let b: u8 = 255; // 8位无符号整数：0 到 255
    let c: i16 = -32768; // 16位有符号整数：-32768 到 32767
    let d: u16 = 65535; // 16位无符号整数：0 到 65535
    let e: i32 = -2147483648; // 32位有符号整数：-2147483648 到 2147483647
    let f: u32 = 4294967295; // 32位无符号整数：0 到 4294967295
    let g: i64 = -9223372036854775808; // 64位有符号整数
    let h: u64 = 18446744073709551615; // 64位无符号整数
    let i: i128 = -170141183460469231731687303715884105728; // 128位有符号整数
    let j: u128 = 340282366920938463463374607431768211455; // 128位无符号整数

    // 架构相关类型：大小取决于目标平台
    let k: isize = -4; // 有符号整数，大小与指针相同
    let l: usize = 4; // 无符号整数，大小与指针相同

    // 整数字面量的多种表示方式
    let decimal = 98_222; // 十进制，下划线提高可读性
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节字面量（仅适用于 u8）

    println!("各种整数类型和表示方式");
    println!("有符号整数的最小值: i8 = {a}，i16 = {c}，i32 = {e}，i64 = {g}，i128 = {i}");
    println!("无符号整数的最大值: u8 = {b}，u16 = {d}，u32 = {f}，u64 = {h}，u128 = {j}");
    println!("与指针同宽的类型: isize = {k}，usize = {l}");
    println!("字面量: 十进制 {decimal}，十六进制 {hex}，八进制 {octal}，二进制 {binary}，字节 {byte}");

    // 浮点类型：IEEE 754 标准的浮点数
    // f32：单精度浮点数，精度约为 6-9 位十进制数
    // f64：双精度浮点数，精度约为 15-17 位十进制数（默认）
    // 注：数学常量请使用 std::f32::consts 等常量，避免手写字面量
    let m: f32 = std::f32::consts::PI; // 单精度
    let n: f64 = std::f64::consts::E; // 双精度

    // 浮点数的特殊值
    let infinity: f64 = f64::INFINITY;
    let negative_infinity: f64 = f64::NEG_INFINITY;
    let nan: f64 = f64::NAN;

    println!("浮点类型示例: {}, {}", m, n);
    println!("浮点数的特殊值: {infinity}，{negative_infinity}，{nan}（NaN 与任何值都不相等，包括它自己）");

    // 布尔类型：逻辑运算的基础
    // 占用 1 字节内存，只能取 true 或 false
    let p = true;
    let q: bool = false; // 显式类型标注

    // 布尔运算
    let r = p && q; // 逻辑与
    let s = p || q; // 逻辑或
    let t = !p; // 逻辑非

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

    println!(
        "字符类型: '{}', 字符串切片: \"{}\"",
        single_char, string_slice
    );

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
    println!(
        "索引访问元组: {}, {}, {}",
        five_hundred, six_point_four, one
    );

    // 单元元组：特殊的空元组
    // 在函数不返回值时隐式使用
    let unit = ();
    println!("单元元组: {:?}", unit);

    // 数组（Array）：固定大小的同类型元素集合
    // 与元组不同，数组的所有元素必须是相同类型
    // 数组在栈上分配，大小在编译时确定
    let a = [1, 2, 3, 4, 5]; // 类型推断为 [i32; 5]
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("一年有 {} 个月，第一个月是 {}", months.len(), months[0]);

    // 显式类型标注
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // 创建包含 5 个 3 的数组
    println!("显式类型标注: {b:?}；重复初始化 [3; 5]: {c:?}");

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
    std::f64::consts::PI
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
    println!(
        "函数调用结果: add_one(5)={}, add_two(5)={}",
        result1, result2
    );

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
    println!("语句里绑定的 y = {y}，表达式绑定的 x = {x}");
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

    // 这是模块级文档注释：用于描述整个模块
    // 通常放在文件的开头，描述模块的用途和功能

    // 注释的最佳实践：
    // 1. 解释"为什么"而不是"是什么"
    // 2. 保持注释与代码同步更新
    // 3. 使用文档注释为公共 API 生成文档
    // 4. 在复杂算法处添加详细注释
    // 5. 避免显而易见的注释

    // 代码示例：良好的注释风格
    let counter = 0; // 计数器，跟踪处理的项目数量

    // 计算平均值，处理可能的除零错误
    let total = 42; // 假设的总数
    let average = if counter > 0 {
        // 使用浮点数除法获得精确结果
        (total as f64) / (counter as f64)
    } else {
        0.0 // 没有项目时返回 0
    };

    println!("平均值: {average}（还没有处理任何项目，所以是 0）");
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
    for number in (1..4).rev() {
        // 反向迭代
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
        return; // 提前返回
    }
    println!("传统方式继续执行");

    // Rust 1.65+ let-else 方式
    let input = "456";
    let Some(number) = parse_number(input) else {
        println!("let-else 解析失败");
        return; // 提前返回
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
        content
            .lines()
            .find(|line| line.starts_with("database_url"))
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
// 8. Rust 1.81 #[expect(lint)] 属性
// ===========================================

// Rust 1.81 引入了 #[expect(lint)] 属性，这是一个重要的编译器改进
// 它允许开发者明确预期某些 lint 警告，从而提供更好的代码维护体验
// 与 #[allow] 不同，#[expect] 在预期的 lint 没有被触发时，会发出 `unfulfilled_lint_expectations` 警告：
// 提醒你这个“预期”已经过期了，可以删掉。它默认是 warn 级别，不是错误；
// 在 CI 里用 `-D warnings` 把警告当错误处理时，才会让构建失败。
//
// 1.81 同时稳定了 lint 的 `reason` 参数：`#[expect(dead_code, reason = "留给下个版本使用")]`，
// 把“为什么要放过这条警告”直接写在属性里，编译器输出提示时也会带上这段理由。

fn compiler_attributes() {
    println!("=== Rust 1.81 #[expect(lint)] 属性 ===");

    // #[expect(lint)] 属性的背景和意义：
    // 1. 显式意图：明确表达对特定 lint 的预期
    // 2. 维护性保障：当代码修改导致预期警告消失时，编译器会提醒
    // 3. 文档作用：告诉其他开发者这个警告是预期的，不是遗漏
    // 4. 渐进式改进：可以逐步清理代码中的警告
    // 5. 团队协作：统一团队对 lint 警告的处理策略

    // 传统方法与 #[expect] 的对比：
    // #[allow(dead_code)] - 静默警告，没有维护保障
    // #[expect(dead_code)] - 预期警告，如果警告消失会得到 unfulfilled_lint_expectations 警告

    // 示例 1: 预期 dead_code 警告
    #[expect(dead_code)]
    fn unused_function() {
        println!("这是一个故意未使用的函数");
    }

    // 示例 2: 预期 unused_variables 警告
    fn function_with_unused_variables() {
        #[expect(unused_variables)]
        let future_feature = "预留变量";

        println!("函数中包含预期的未使用变量");
    }

    // 示例 3: 预期 deprecated 警告
    #[deprecated(since = "1.0.0", note = "使用 new_function 替代")]
    fn old_function() {
        println!("这是一个已弃用的函数");
    }

    fn new_function() {
        println!("这是新的替代函数");
    }

    // 演示调用已弃用函数
    #[expect(deprecated)]
    old_function();

    // 示例 4: 在模块级别使用 #[expect]
    mod example_module {
        #![expect(unused_imports)]

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
        #[expect(unused_variables)]
        let expected_variable = "这应该产生警告";

        println!("#[expect] 提供了更好的维护保障");
    }

    // 示例 6: 多重 lint 预期
    #[expect(
        dead_code,
        unused_variables
    )]
    fn multiple_lints_example() {
        let unused = "多个 lint 预期";
        println!("同时预期多个 lint 警告");
    }

    // 示例 6.5: 用 reason 写明理由（Rust 1.81）
    // reason 对 allow / warn / deny / expect 都适用，让以后读代码的人知道为什么要这么做
    #[expect(dead_code, reason = "接口已经约定好，下个迭代才会有调用方")]
    fn reserved_for_next_iteration() {}

    #[allow(unused_variables, reason = "回调签名固定，这个参数暂时用不上")]
    fn callback_with_unused_arg(event_id: u32) {}

    // 示例 7: 条件性预期警告
    #[cfg(debug_assertions)]
    #[expect(dead_code)]
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
        #[deprecated(since = "2.0.0", note = "使用 new_api() 替代")]
        pub fn old_api() {
            println!("遗留 API 实现");
        }

        pub fn new_api() {
            println!("新 API 实现");
        }

        // 在某些情况下，我们可能需要保留旧的 API 调用
        pub fn transition_code() {
            #[expect(deprecated)]
            old_api(); // 临时调用旧 API
            new_api(); // 同时使用新 API
        }
    }

    // 场景 2: 测试代码中的预期警告
    mod test_utilities {
        #[expect(dead_code)]
        pub fn setup_test_environment() {
            println!("设置测试环境");
        }

        #[expect(dead_code)]
        pub fn generate_test_data() -> Vec<i32> {
            vec![1, 2, 3, 4, 5]
        }

        #[expect(unused_variables)]
        pub fn test_placeholder() {
            let placeholder = "测试占位符";
        }
    }

    // 场景 3: 条件编译的预期警告
    mod experimental_features {
        #![expect(dead_code)]

        pub fn experimental_feature() {
            println!("实验性功能实现");
        }
    }

    // 场景 4: 代码生成器的预期警告
    mod generated_code {
        // 在实际应用中，这可能是由代码生成器产生的代码
        #[expect(dead_code)]
        fn generated_placeholder() {
            // 占位符实现
        }

        #[expect(unused_variables)]
        fn generated_with_placeholders() {
            let future_impl = "预留实现";
        }
    }

    // 场景 5: 性能优化相关的预期警告
    mod performance_code {
        #[expect(clippy::modulo_one)]
        pub fn optimized_calculation() {
            // 某些性能优化可能会触发 lint
            let _result = 100 % 1; // 故意的模1运算
        }
    }

    println!("实际应用示例演示完成");
}

// ===========================================
// 9. Rust 1.95-1.97 数值与转换增强
// ===========================================

// Rust 1.95 和 1.97 为数值类型补充了两组非常实用的 API：
// - Rust 1.95: `bool: TryFrom<{integer}>`，把“整数转布尔”变成显式可失败转换
// - Rust 1.97: `highest_one` / `lowest_one` / `isolate_*_one` / `bit_width`，
//   一次取到“最高/最低的置位位置”，省去手写位扫描循环
//
// 对新手来说，这一节想传达两件事：
// 1. 标准库在持续补齐“很常用但过去要自己写”的小工具
// 2. 新增 API 的命名往往直接对应真实场景：`isolate` 表示“把某一位单独拎出来”

fn numeric_and_conversion_enhancements() {
    println!("=== Rust 1.95-1.97 数值与转换增强 ===");

    // Rust 1.95: `bool: TryFrom<{integer}>`
    //
    // 之前想把整数当布尔用，通常写 `x != 0`。
    // 现在标准库提供了一个显式的转换接口，并且是“可失败”的：
    // - 0 转成 false
    // - 1 转成 true
    // - 其他值转换失败，返回错误
    //
    // 它的价值在于把“语义边界”讲清楚：
    // 只有 0 和 1 是合法的布尔编码，其余整数都不该被悄悄当成 true。
    let ok_true: Result<bool, _> = 1u8.try_into();
    let ok_false: Result<bool, _> = 0u16.try_into();
    let bad: Result<bool, _> = 2u8.try_into();
    println!("bool::try_from(1) = {ok_true:?}");
    println!("bool::try_from(0) = {ok_false:?}");
    println!("bool::try_from(2) = {bad:?}");

    // 实际使用中常见的模式是配合 `?` 或 `unwrap_or`：
    // 解析“0/1 标志位”时，非法值直接按默认值处理。
    let flag: bool = "1".parse::<u8>().unwrap().try_into().unwrap_or(false);
    println!("解析标志位（输入 1）得到: {flag}");

    // Rust 1.97: `highest_one` / `lowest_one`
    //
    // 这两个方法回答同一个问题：“这个整数的二进制里，最高/最低的 1 在第几位？”
    // 以 0b1010_0100 为例：
    // - 最低的 1 在第 2 位（从 0 开始数）
    // - 最高的 1 在第 7 位
    //
    // 为什么有用？
    // 1. 判断“下一个更大的 2 的幂”相关计算
    // 2. 位图、位掩码操作里快速定位置位区间
    // 3. 序列化/协议解析中查找定界位
    let bits: u32 = 0b1010_0100;
    println!("0b1010_0100: highest_one = {:?}, lowest_one = {:?}", bits.highest_one(), bits.lowest_one());

    // `isolate_highest_one` / `isolate_lowest_one` 更进一步：
    // 它们直接返回“只有那一位是 1”的掩码，其余位全部清零。
    //
    // 这特别适合：
    // 1. 从位图中提取单个标志后继续做位运算
    // 2. 逐步消费置位位（处理一个，清掉一个）
    let mask_hi = bits.isolate_highest_one();
    let mask_lo = bits.isolate_lowest_one();
    println!("isolate_highest_one = {mask_hi:#010b}, isolate_lowest_one = {mask_lo:#010b}");

    // 对于全零值，highest_one / lowest_one 返回 None，
    // isolate 系列返回 0，所以不存在“越界”问题。
    let zero: u32 = 0;
    println!("0 的 highest_one = {:?}, isolate_lowest_one = {:#010b}", zero.highest_one(), zero.isolate_lowest_one());

    // 有符号整数同样适用：它按“补码位模式”计算，不关心符号语义。
    let neg: i32 = -4; // 二进制是 ...1111_1100
    println!("-4 的 lowest_one = {:?}（补码 ...1111_1100 中，从低到高第一个 1 在第 2 位）", neg.lowest_one());

    // Rust 1.97: `bit_width`
    //
    // 返回“表示这个值最少需要多少位”，等价于 `64 - leading_zeros`，
    // 但写成专门的方法更直白，也免去计算类型的位数（32 位还是 64 位）。
    // 注意：bit_width 只对无符号整数和 NonZero 系列可用。
    println!("255u32 的 bit_width = {}", 255u32.bit_width());
    println!("1u32 的 bit_width = {}", 1u32.bit_width());

    // Rust 1.97: `NonZero` 系列的位操作方法
    //
    // `NonZeroU32` 等类型保证“非零”，因此：
    // - `highest_one` / `lowest_one` 直接返回 usize，不需要 Option
    // - 同样提供 isolate 系列和 bit_width
    //
    // 对新手来说，这里想强调 NonZero 类型的价值：
    // 类型系统直接排除 0，调用方少写一次判断。
    // 注意返回类型和普通整数不同：`NonZeroU32::bit_width()` 返回 `NonZeroU8`（非零数至少需要 1 位，类型里直接排除了 0），
    // 而 `u32::bit_width()` 返回 `u32`。
    use std::num::NonZeroU32;
    let nonzero = NonZeroU32::new(0b1010_0100).unwrap();
    println!("NonZeroU32(0b1010_0100): highest_one = {}, lowest_one = {}, bit_width = {}",
             nonzero.highest_one(), nonzero.lowest_one(), nonzero.bit_width());
    let nz_isolate = nonzero.isolate_lowest_one();
    println!("NonZeroU32 的 isolate_lowest_one = {nz_isolate:#010b}");

    // Rust 1.97: `char::is_control` 可以在 const 上下文中使用
    //
    // 之前 `is_control` 只是普通方法，现在可以在常量求值中使用，
    // 例如在编译期校验字符表、生成控制字符掩码。
    const NEWLINE_IS_CONTROL: bool = '\n'.is_control();
    const LETTER_IS_CONTROL: bool = 'a'.is_control();
    println!("const 求值: '\\n'.is_control() = {NEWLINE_IS_CONTROL}, 'a'.is_control() = {LETTER_IS_CONTROL}");

    // 小结：
    // 1. TryFrom 系列让“0/1 布尔编码”的转换变得显式且安全
    // 2. 位操作方法族把位扫描、掩码提取等高频操作收进标准库
    // 3. NonZero 类型配合新方法，可以用类型系统排除非法输入
    // 4. const 能力扩展让字符判断可以在编译期完成
    println!();
}

// ===========================================
// 10. Rust 1.98 数值增强：代数浮点运算与整数缓冲格式化
// ===========================================

// 数值类型新增了三组工具（均在 Rust 1.98 稳定）：
// - 浮点“代数”运算：`algebraic_add / sub / mul / div / rem`，允许编译器重排浮点运算来换取性能
// - 整数缓冲格式化：`NumBuffer` + `format_into`，不分配堆内存就能把整数转成字符串
// - `NonZero<T>::from_str_radix`：按指定进制把字符串直接解析成非零整数
//
// 对新手来说，这一节想传达三件事：
// 1. 浮点数不满足结合律，所以编译器默认“不敢”重排你写的运算顺序
// 2. 性能优化经常是“由你明确表态：这里我愿意牺牲一点精确性”，而不是编译器自作主张
// 3. “少一次堆分配”是标准库持续优化的方向，`format_into` 就是这个思路的产物

fn numeric_enhancements_198() {
    println!("=== Rust 1.98 数值增强 ===");

    // ---------- 一、代数浮点运算 ----------
    //
    // 先看一个事实：浮点加法不满足结合律。
    // 同样三个数，加的顺序不同，结果可能相差最低的一两位。
    let left = (0.1_f64 + 0.2) + 0.3;
    let right = 0.1_f64 + (0.2 + 0.3);
    println!("(0.1 + 0.2) + 0.3 = {left}，0.1 + (0.2 + 0.3) = {right}，相等吗？{}", left == right);

    // 正因为如此，Rust 默认严格按源码顺序求值：`a + b + c + d` 一定是 `((a + b) + c) + d`。
    // 这保证了结果可复现，代价是编译器无法把一个求和循环改成“多路并行累加再合并”，
    // 而那正是 SIMD 向量化加速求和/点积的做法。
    //
    // `algebraic_*` 就是让你明确告诉编译器：“这里把浮点数当成实数来对待，运算顺序随你重排。”
    // 它的效果类似其他语言里的 `-ffast-math`，但只作用于你显式调用的这几个运算，
    // 不会像全局开关那样悄悄影响整个程序。
    let samples = [0.1_f64, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    let strict_sum: f64 = samples.iter().sum();
    let algebraic_sum = samples.iter().fold(0.0_f64, |acc, &x| acc.algebraic_add(x));
    println!("严格顺序求和 = {strict_sum}，代数求和 = {algebraic_sum}");
    // 在没有优化的 debug 构建里，两个结果往往逐位相同：编译器没有去重排。
    // 差异（以及向量化带来的加速）主要出现在 `cargo run --release` 这类开启优化的构建中。

    // 点积是最典型的受益场景：乘加链很长、对最后一位误差不敏感。
    fn dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b)
            .fold(0.0, |acc, (&x, &y)| acc.algebraic_add(x.algebraic_mul(y)))
    }
    println!("点积 = {}", dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]));

    // 使用时必须清楚四点（出自标准库文档）：
    // 1. 结果“非确定”：即使输入相同，同一次运行里两次调用也可能得到不同的结果；
    //    单次运算的精度没有任何保证，只保证得到“某个合法的浮点数”
    // 2. NaN、±Inf、-0.0 这类“非数学”的值可能表现得出乎意料（编译器可以忽略零的符号、
    //    把除法改写成乘以倒数……），处理含有这些值的数据要格外小心
    // 3. 它“不是未定义行为”，但 unsafe 代码不能依赖返回值的任何性质来保证内存安全
    // 4. 依赖精确运算顺序的算法不能用它，例如 Kahan 补偿求和——
    //    那个算法靠的正是“看起来多余的 (t - sum) - y”来抵消舍入误差，
    //    一旦允许按实数代数化简，补偿项会被当成 0 优化掉，算法就失效了
    //
    // 五个方法分别对应 + - * / %，在 f32 和 f64 上都有，而且都是 const fn。
    println!(
        "sub = {}, div = {}, rem = {}",
        5.0_f64.algebraic_sub(2.0),
        9.0_f64.algebraic_div(3.0),
        7.5_f64.algebraic_rem(2.0)
    );

    // ---------- 二、整数缓冲格式化 ----------
    //
    // 把整数变成字符串，最常见的两种写法各有代价：
    // - `n.to_string()`：每次调用都分配一个新的 `String`（堆分配）
    // - `write!(out, "{n}")`：要经过 `fmt::Formatter`，中间有动态分发
    //
    // 1.98 新增的 `format_into` 换了个思路：由调用者提供一块缓冲区 `NumBuffer<T>`，
    // 数字直接写进去，返回一个借用这块缓冲区的 `&str`。
    // 缓冲区放在栈上，大小由类型决定（刚好够放下该类型最长的十进制表示，含负号），
    // 因此不需要堆分配，也绕开了动态分发。
    //
    // 路径说明：1.98 里 `NumBuffer` 只能从 `core::fmt` 引入，
    // 1.99 起 `std::fmt::NumBuffer` 也可以使用（标准库补了一条 re-export）。
    use std::fmt::NumBuffer;

    let mut buf = NumBuffer::<u64>::new();
    let text: &str = 1_234_567_u64.format_into(&mut buf);
    println!("format_into 得到 {text:?}，长度 {}", text.len());

    // 负数同样可以，符号会包含在结果里。
    let mut signed_buf = NumBuffer::<i32>::new();
    println!("负数: {:?}", (-2026_i32).format_into(&mut signed_buf));

    // 同一块缓冲区可以在循环里反复使用，这正是“零分配”的价值所在。
    // 编译器会保证：下一次 `format_into` 之前，上一次返回的 `&str` 必须已经用完，
    // 因为它们借用的是同一块内存（这是借用检查在这里帮我们避免“读到被覆盖的旧数据”）。
    let mut loop_buf = NumBuffer::<u32>::new();
    for n in [7_u32, 1024, 65_535] {
        let s: &str = n.format_into(&mut loop_buf);
        println!("  {n} 的十进制文本是 {s:?}");
    }

    // 所有基本整数类型（i8..i128、u8..u128、isize、usize）都有这个方法。
    // 缓冲区的具体字节数属于实现细节，不要依赖它，只需知道它“够用”。

    // ---------- 三、NonZero::from_str_radix ----------
    //
    // `NonZero<T>` 在类型层面排除了 0。以前想从“带进制的文本”解析出它，
    // 得先 `u32::from_str_radix` 再 `NonZero::new`，要处理两层失败。
    // 现在 `from_str_radix` 直接可用：文本解析失败，或解析结果为 0，都会返回 `ParseIntError`。
    use std::num::NonZeroU32;
    let hex = NonZeroU32::from_str_radix("ff", 16);
    let zero = NonZeroU32::from_str_radix("0", 10);
    println!("\"ff\"(16 进制) 解析为 {hex:?}");
    println!(
        "\"0\" 解析失败，原因是 {:?}（IntErrorKind::Zero 表示“解析出来是 0”）",
        zero.as_ref().err().map(|e| e.kind())
    );

    // 附带一提：1.79 起，`NonZeroU32`、`NonZeroU8` 等都只是 `NonZero<u32>`、`NonZero<u8>` 的类型别名，
    // 泛型写法 `NonZero<T>` 和具体别名可以互换使用。
    let generic_form: std::num::NonZero<u32> = std::num::NonZero::new(7).unwrap();
    let alias_form: NonZeroU32 = generic_form;
    println!("NonZero<u32> 与 NonZeroU32 是同一个类型: {alias_form}");

    // 小结：
    // 1. `algebraic_*`：用确定性换速度，只在能接受舍入误差的数值循环里用
    // 2. `format_into`：热点路径上避免为“整数转文本”反复分配 String
    // 3. `NonZero::from_str_radix`：把“解析”和“非零校验”合成一步
    println!();
}

// ===========================================
// 11. 整数运算补充：整数平方根、无符号减有符号与严格运算（Rust 1.84 / 1.90 / 1.91）
// ===========================================

// 这一节的几组 API 都在回答同一个问题：整数运算遇到意外情况（溢出、负数开方）时，怎样处理才最清楚？
// - `isqrt` / `checked_isqrt`（1.84）：整数平方根，不再绕道浮点数
// - `checked_sub_signed` 等四个方法（1.90）：无符号数减有符号数
// - `strict_add` / `strict_sub` / `strict_mul` 等一整族（1.91）：不论 debug 还是 release，溢出一律 panic
// - 旧的整数模块（`std::i32::MAX` 这种写法）在 1.99 起全面弃用，请改用 `i32::MAX`

fn integer_arithmetic_updates() {
    println!("=== 整数运算补充：isqrt、无符号减有符号、严格运算（Rust 1.84 / 1.90 / 1.91） ===");

    // ---------- 整数平方根 isqrt（Rust 1.84） ----------
    //
    // 它求“不超过平方根的最大整数”（向下取整）。过去的常见写法是 `(n as f64).sqrt() as u64`，有两个隐患：
    // 1. f64 只有 53 位有效数字，大整数转成浮点数时就已经丢了精度，结果可能差 1
    // 2. 来回转换，意图不清晰
    // `isqrt` 全程使用整数运算，结果总是精确的。
    let big = u64::MAX;
    println!("u64::MAX 经过浮点数求平方根: {}（差了 1）", (big as f64).sqrt() as u64);
    println!("u64::MAX 的 isqrt: {}", big.isqrt());

    // 典型场景：判断素数。试除只需要检查到 √n，也就是 `n.isqrt()`。
    fn is_prime(n: u64) -> bool {
        n >= 2 && (2..=n.isqrt()).all(|divisor| !n.is_multiple_of(divisor))
    }
    println!("97 是素数吗？{}；91 呢？{}（91 = 7 × 13）", is_prime(97), is_prime(91));

    // 有符号数没有负数的实数平方根：`isqrt` 遇到负数会 panic，`checked_isqrt` 返回 None。
    println!(
        "(-4).checked_isqrt() = {:?}，16.checked_isqrt() = {:?}",
        (-4_i32).checked_isqrt(),
        16_i32.checked_isqrt()
    );

    // ---------- 无符号数减有符号数（Rust 1.90） ----------
    //
    // 场景：库存、余额、下标这类量用无符号类型，而变动量可能带符号。
    // “加有符号数”这半边早在 1.66 就有了（`checked_add_signed` 等）；1.90 补齐了减法这半边，
    // 一共四种溢出策略，和其他整数运算一致：
    // - `checked_sub_signed`：溢出返回 None
    // - `saturating_sub_signed`：卡在 0 或类型最大值上
    // - `wrapping_sub_signed`：回绕
    // - `overflowing_sub_signed`：返回（回绕后的结果，是否溢出）
    // 以前要先判断变动量的正负，再分别走 `-` 或 `+`，而且 `i32::MIN` 取反本身就会溢出，边界很容易写错。
    let stock: u8 = 5;
    println!("库存 5 减 3: {:?}", stock.checked_sub_signed(3));
    println!("库存 5 减 -3（减去负数等于加上它的绝对值）: {:?}", stock.checked_sub_signed(-3));
    println!(
        "库存 5 减 10 会溢出: checked = {:?}，saturating = {}，wrapping = {}，overflowing = {:?}",
        stock.checked_sub_signed(10),
        stock.saturating_sub_signed(10),
        stock.wrapping_sub_signed(10),
        stock.overflowing_sub_signed(10)
    );

    // ---------- 严格运算 strict_*（Rust 1.91） ----------
    //
    // 整数溢出的默认行为取决于构建模式：debug 构建会 panic，release 构建则悄悄回绕（wrap around）。
    // 这是出了名的陷阱——测试（debug）里一切正常的代码，上线（release）后可能静默算错。
    // 想要固定的行为，过去有三种选择：`wrapping_*`（永远回绕）、`checked_*`（返回 Option，由你处理）、
    // `saturating_*`（卡在边界）。唯独缺一种：“溢出就是 bug，请在任何构建里立刻 panic”。
    // 1.91 的 `strict_*` 补上了它：`strict_add`、`strict_sub`、`strict_mul`、`strict_neg`、`strict_pow`、
    // `strict_div`、`strict_shl` 等，在 debug 和 release 里行为完全一致——溢出就 panic。
    // 它们都是 const fn：在常量里溢出，就是编译错误（E0080）。
    // 典型场景：金额、长度、下标这类计算，溢出意味着数据已经坏了，继续运行只会扩大破坏。
    let price: u32 = 1999;
    let quantity: u32 = 3;
    println!("总价 = {}（任何构建模式下，溢出都会 panic，而不是静默回绕）", price.strict_mul(quantity));
    const SAFE_SUM: u8 = 100u8.strict_add(100); // 200，没有溢出，编译通过
    println!("常量里的严格加法: {SAFE_SUM}");
    // 还有混合符号的版本：`strict_add_signed`（无符号加有符号）、`strict_sub_unsigned`（有符号减无符号）等。
    println!("10_u32.strict_add_signed(-4) = {}", 10_u32.strict_add_signed(-4));

    // ---------- 旧整数模块全面弃用（Rust 1.99） ----------
    //
    // 1.43 之前，整数的最大最小值只能写成 `std::i32::MAX` 这样的“模块常量”；
    // 1.43 引入了关联常量 `i32::MAX`，直接写在类型上，更短也更直观。
    // 1.99 起，旧模块里的常量（`std::i32::MAX`、`std::u8::MAX` 等）全面弃用，使用它们会收到警告：
    //     use of deprecated constant `std::i32::MAX`: replaced by the `MAX` associated constant on this type
    // 新代码一律写 `i32::MAX` / `u8::MIN` / `f64::EPSILON`。
    println!("i32::MAX = {}，u8::MIN = {}", i32::MAX, u8::MIN);

    // 小结：
    // 1. 求整数的平方根用 `isqrt`，不要 `as f64` 绕一圈
    // 2. 无符号数加减有符号数，用 `*_add_signed` / `*_sub_signed`，不要手写正负分支
    // 3. 溢出就是 bug 的地方，用 `strict_*` 让它在任何构建里都立刻暴露
    println!();
}

// ===========================================
// 12. 解构赋值与 Unicode 标识符（Rust 1.59 / 1.53）
// ===========================================

// - 解构赋值（1.59）：`=` 左边也可以写模式，把右边的值拆开，赋给**已经存在**的变量
// - Unicode 标识符（1.53）：变量名、函数名、类型名可以直接用中文等非 ASCII 字符

fn destructuring_assignment_and_unicode_identifiers() {
    println!("=== 解构赋值与 Unicode 标识符（Rust 1.59 / 1.53） ===");

    // ---------- 解构赋值（Rust 1.59） ----------
    //
    // `let (a, b) = ...` 是**声明**新变量；解构赋值则是给**已有**的变量重新赋值，
    // 左边的形状（元组、数组、结构体）要和右边对应。
    // 最典型的场景：一次更新多个相互依赖的状态变量。下面的斐波那契迭代里，
    // `(a, b) = (b, a + b)` 先把右边整体算完，再同时赋给 a 和 b；
    // 如果拆成 `a = b; b = a + b;` 两条语句，第二条用到的就是已经被改写的 a，结果是错的。
    let (mut a, mut b) = (0_u64, 1_u64);
    for _ in 0..10 {
        (a, b) = (b, a + b);
    }
    println!("迭代 10 次后的斐波那契数对: ({a}, {b})");

    // 不借助临时变量交换两个值
    let (mut left, mut right) = ("左", "右");
    (left, right) = (right, left);
    println!("交换之后: left = {left}，right = {right}");

    // 左边也可以是结构体或数组；用 `_` 忽略不需要的部分，用 `..` 跳过中间的元素
    struct Point {
        x: i32,
        y: i32,
    }
    let (px, py);
    Point { x: px, y: py } = Point { x: 3, y: 4 }; // 结构体：字段名: 目标变量
    let (first, last);
    (first, .., last) = (10, 20, 30, 40);
    let second;
    [_, second, _] = [7, 8, 9];
    println!("结构体: ({px}, {py})，元组首尾: ({first}, {last})，数组中间: {second}");

    // ---------- Unicode 标识符（Rust 1.53） ----------
    //
    // 标识符可以包含 Unicode 字符（规则出自 Unicode 标准的 UAX #31），变量、函数、类型都可以直接用中文命名。
    // 在教学、领域建模（比如财务、法律术语）和个人项目里，这很直观：
    fn 计算面积(长: f64, 宽: f64) -> f64 {
        长 * 宽
    }
    let 房间面积 = 计算面积(5.0, 4.0);
    println!("房间面积 = {房间面积} 平方米");
    // 但团队协作和开源项目的最佳实践依旧是：公共 API 使用 ASCII 英文名——
    // 便于检索和输入，也避免外观相似的字符造成混淆。编译器提供了几条 lint 帮你把关：
    // - `non_ascii_idents`（默认 allow）：想在项目里整体禁止非 ASCII 标识符，就把它设成 deny
    // - `confusable_idents`（默认 warn）：两个标识符长得几乎一样（比如拉丁字母 a 与西里尔字母 а）时警告
    // - `mixed_script_confusables`、`uncommon_codepoints`（默认 warn）：混用多种文字或用到罕见字符时警告

    // 小结：
    // 1. 同时更新多个互相依赖的变量，用解构赋值，右边先整体求值，不会读到更新到一半的值
    // 2. 中文标识符适合教学和领域建模，对外公开的接口还是用英文
    println!();
}

// ===========================================
// 13. 带标签的代码块：break '标签 值（Rust 1.65）
// ===========================================

// `loop` 可以用 `break 值` 返回结果，带标签的循环 `'outer: loop` 还能一次跳出多层。
// 但有时只是想“按顺序做几项检查，一旦命中就提前给出结果”，并不需要循环，
// 写成 `loop { ..; break; }` 很别扭，拆成独立函数又显得小题大做。
// 1.65 起，任何带标签的代码块都可以用 `break '标签 值` 提前跳出，并把这个值作为整个块的结果：
//     let 结果 = '标签: { .. break '标签 值; .. 默认值 };
// 块里最后一个表达式，是“没有任何检查命中时”的默认结果，类型必须和每个 break 带出的值一致。
// 它和提前 `return` 的区别：`return` 退出的是整个函数；带标签的块只是让这一小段表达式提前得出值，
// 函数后面的代码照常执行。

fn labeled_block_break() {
    println!("=== 带标签的代码块：break '标签 值（Rust 1.65） ===");

    // 典型场景：给输入分类，规则按优先级排列，命中就停；分类之后函数还要继续做别的事
    fn describe(input: &str) -> String {
        let kind = 'classify: {
            if input.is_empty() {
                break 'classify "空";
            }
            if input.chars().all(|c| c.is_ascii_digit()) {
                break 'classify "数字";
            }
            if input.chars().all(char::is_alphabetic) {
                break 'classify "字母或汉字";
            }
            "混合内容" // 没有任何规则命中时的默认结果
        };
        format!("{input:?} 属于「{kind}」")
    }
    for input in ["", "2026", "rust", "你好", "a1!"] {
        println!("{}", describe(input));
    }

    // 另一个典型场景：在嵌套循环里找到第一个满足条件的位置，并一次跳出所有循环、同时带出结果
    let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let position = 'search: {
        for (row, cells) in grid.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell == 6 {
                    break 'search Some((row, col));
                }
            }
        }
        None // 找遍了也没有
    };
    println!("数字 6 在二维数组中的位置（行，列）: {position:?}");

    // 最佳实践：
    // 1. 一连串“命中即停”的判断，用带标签的块比层层嵌套的 if-else 更扁平
    // 2. 如果这些判断本身就是一个独立的概念，仍然优先拆成函数并使用 `return`
    println!();
}

// ===========================================
// 14. char 的关联常量与方法（Rust 1.52）
// ===========================================

// 和整数一样，`char` 的常量和转换函数以前是放在 `std::char` 模块里的独立项（`std::char::MAX`、`std::char::from_u32(..)`），
// 1.52 起它们成了 `char` 类型自己的关联项，直接写 `char::MAX`、`char::from_u32(..)`，更好找也更好读：
// - `char::MAX`：最大的合法字符 U+10FFFF；`char::REPLACEMENT_CHARACTER`：替换字符 U+FFFD（�）
// - `char::UNICODE_VERSION`：标准库实现所依据的 Unicode 版本
// - `char::from_u32(码点)`：数字转字符，不是合法码点（比如 UTF-16 代理区）时返回 `None`
// - `char::from_digit(数字, 进制)`：数字转数字字符；`char::decode_utf16(码元序列)`：解码 UTF-16
// - `len_utf8()` / `len_utf16()`：这个字符编码成 UTF-8 / UTF-16 需要几个字节 / 码元

fn char_constants_and_methods() {
    println!("=== char 的关联常量与方法（Rust 1.52） ===");

    println!(
        "char::MAX = U+{:X}，替换字符 = {:?}，Unicode 版本 = {:?}",
        char::MAX as u32,
        char::REPLACEMENT_CHARACTER,
        char::UNICODE_VERSION
    );

    // from_u32：数字 → 字符。0xD800~0xDFFF 是 UTF-16 的代理区，不是合法的字符
    println!("from_u32(0x4E2D) = {:?}，from_u32(0xD800) = {:?}", char::from_u32(0x4E2D), char::from_u32(0xD800));

    // from_digit：数字 → 数字字符，常用来手写进制转换
    println!("from_digit(7, 10) = {:?}，from_digit(11, 16) = {:?}", char::from_digit(7, 10), char::from_digit(11, 16));

    // 同一个字符，在不同编码里占的空间不一样
    for ch in ['a', '中', '😀'] {
        println!("{ch:?} 占 UTF-8 {} 字节、UTF-16 {} 个码元", ch.len_utf8(), ch.len_utf16());
    }

    // 典型场景：解码 UTF-16 数据（Windows API、JavaScript 字符串），遇到非法数据用替换字符代替，而不是整体失败
    let units = [0x4F60, 0x597D, 0xD800, 0x0021]; // “你好”，一个落单的代理码元，“!”
    let decoded: String = char::decode_utf16(units).map(|unit| unit.unwrap_or(char::REPLACEMENT_CHARACTER)).collect();
    println!("解码 UTF-16: {decoded:?}");

    // 最佳实践：
    // 1. 处理外部来的数字 → 字符转换，用 `char::from_u32` 并处理 None，不要用 `as char`（它只接受 u8）
    // 2. 计算文本占用空间，用 `len_utf8()`，不要假设一个字符就是一个字节
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
    numeric_and_conversion_enhancements();
    numeric_enhancements_198();
    integer_arithmetic_updates();
    destructuring_assignment_and_unicode_identifiers();
    labeled_block_break();
    char_constants_and_methods();

    println!("基础语法演示完成！");
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
        assert_eq!(y, 5);
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
                return true; // 预期的失败情况
            };
            false // 不应该到达这里
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
                return true; // 预期的失败情况
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

#[test]
fn test_numeric_and_conversion_enhancements() {
    // bool: TryFrom（Rust 1.95）
    let ok_true: Result<bool, _> = 1u8.try_into();
    let ok_false: Result<bool, _> = 0u8.try_into();
    let bad: Result<bool, _> = 2u8.try_into();
    assert_eq!(ok_true, Ok(true));
    assert_eq!(ok_false, Ok(false));
    assert!(bad.is_err());

    // 整数位操作（Rust 1.97）
    let bits: u32 = 0b1010_0100;
    assert_eq!(bits.highest_one(), Some(7));
    assert_eq!(bits.lowest_one(), Some(2));
    assert_eq!(bits.isolate_highest_one(), 0b1000_0000);
    assert_eq!(bits.isolate_lowest_one(), 0b0000_0100);
    assert_eq!(255u32.bit_width(), 8);
    assert_eq!(0u32.highest_one(), None);

    // NonZero 版本直接返回 usize，bit_width 返回 NonZeroU8
    let nonzero = std::num::NonZeroU32::new(bits).unwrap();
    assert_eq!(nonzero.lowest_one(), 2);
    assert_eq!(nonzero.bit_width().get(), 8);

    // const char::is_control（Rust 1.97）
    const NEWLINE_IS_CONTROL: bool = '\n'.is_control();
    assert!(NEWLINE_IS_CONTROL);
}

#[test]
fn test_numeric_enhancements_198() {
    use std::fmt::NumBuffer;
    use std::num::{IntErrorKind, NonZeroU32};

    // 浮点加法不满足结合律，这是 algebraic_* 存在的前提
    assert_ne!((0.1_f64 + 0.2) + 0.3, 0.1_f64 + (0.2 + 0.3));

    // 代数运算的结果不保证逐位一致，只能在容差内比较
    let samples = [0.1_f64, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    let strict: f64 = samples.iter().sum();
    let algebraic = samples.iter().fold(0.0_f64, |acc, &x| acc.algebraic_add(x));
    assert!((strict - algebraic).abs() < 1e-12);
    assert!((6.0_f64.algebraic_mul(7.0) - 42.0).abs() < 1e-12);
    assert!((7.5_f64.algebraic_rem(2.0) - 1.5).abs() < 1e-12);
    // const fn：可以在常量表达式里调用
    const DOUBLED: f32 = 1.5_f32.algebraic_mul(2.0);
    assert!((DOUBLED - 3.0).abs() < 1e-6);

    // format_into：同一块缓冲区反复使用，结果与 to_string 一致
    let mut buf = NumBuffer::<u64>::new();
    assert_eq!(1_234_567_u64.format_into(&mut buf), "1234567");
    assert_eq!(u64::MAX.format_into(&mut buf), u64::MAX.to_string());
    let mut signed = NumBuffer::<i32>::new();
    assert_eq!((-2026_i32).format_into(&mut signed), "-2026");
    assert_eq!(i32::MIN.format_into(&mut signed), i32::MIN.to_string());

    // NonZero::from_str_radix：解析成功与“解析为 0”两种结果
    assert_eq!(NonZeroU32::from_str_radix("ff", 16).map(NonZeroU32::get), Ok(255));
    assert_eq!(
        NonZeroU32::from_str_radix("0", 10).unwrap_err().kind(),
        &IntErrorKind::Zero
    );
}

#[test]
fn test_integer_arithmetic_updates() {
    use std::panic::catch_unwind;

    // isqrt 是精确的；经过浮点数的写法在 u64::MAX 上差了 1
    assert_eq!(u64::MAX.isqrt(), 4_294_967_295);
    assert_eq!((u64::MAX as f64).sqrt() as u64, 4_294_967_296);
    assert_eq!([0_u32, 15, 16, 17].map(u32::isqrt), [0, 3, 4, 4]);
    assert_eq!((-1_i32).checked_isqrt(), None);
    assert!(catch_unwind(|| (-1_i32).isqrt()).is_err());

    // 无符号减有符号：四种溢出策略
    assert_eq!(5_u8.checked_sub_signed(3), Some(2));
    assert_eq!(5_u8.checked_sub_signed(-3), Some(8));
    assert_eq!(5_u8.checked_sub_signed(10), None);
    assert_eq!(5_u8.saturating_sub_signed(10), 0);
    assert_eq!(5_u8.wrapping_sub_signed(10), 251);
    assert_eq!(5_u8.overflowing_sub_signed(10), (251, true));

    // strict_*：没有溢出时与普通运算一致，溢出时 panic（与构建模式无关）
    assert_eq!(1999_u32.strict_mul(3), 5997);
    assert_eq!(10_u32.strict_add_signed(-4), 6);
    assert!(catch_unwind(|| 250_u8.strict_add(10)).is_err());
    assert!(catch_unwind(|| 0_u32.strict_sub(1)).is_err());
}

#[test]
fn test_destructuring_assignment_and_unicode_identifiers() {
    // 解构赋值：右边先整体求值，所以 (a, b) = (b, a + b) 与手写两条语句的结果不同
    let (mut a, mut b) = (0_u64, 1_u64);
    for _ in 0..10 {
        (a, b) = (b, a + b);
    }
    assert_eq!((a, b), (55, 89));

    let (mut x, mut y) = (1, 2);
    (x, y) = (y, x);
    assert_eq!((x, y), (2, 1));

    let (first, last);
    (first, .., last) = (10, 20, 30, 40);
    assert_eq!((first, last), (10, 40));

    // Unicode 标识符
    let 房间面积 = 5.0_f64 * 4.0;
    assert_eq!(房间面积, 20.0);
}

#[test]
fn test_labeled_block_break_and_char_constants() {
    // 带标签的块：规则按顺序检查，命中即停；没有命中时取块末尾的默认值
    let classify = |input: &str| {
        'classify: {
            if input.is_empty() {
                break 'classify "空";
            }
            if input.chars().all(|c| c.is_ascii_digit()) {
                break 'classify "数字";
            }
            "其他"
        }
    };
    assert_eq!([classify(""), classify("2026"), classify("a1")], ["空", "数字", "其他"]);

    // 从嵌套循环里带值跳出
    let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let find = |target: i32| {
        'search: {
            for (row, cells) in grid.iter().enumerate() {
                for (col, &cell) in cells.iter().enumerate() {
                    if cell == target {
                        break 'search Some((row, col));
                    }
                }
            }
            None
        }
    };
    assert_eq!(find(6), Some((1, 2)));
    assert_eq!(find(10), None);

    // char 的关联常量与方法
    assert_eq!(char::MAX as u32, 0x10FFFF);
    assert_eq!(char::REPLACEMENT_CHARACTER, '\u{FFFD}');
    assert_eq!(char::from_u32(0x4E2D), Some('中'));
    assert_eq!(char::from_u32(0xD800), None); // 代理区不是合法字符
    assert_eq!(char::from_digit(11, 16), Some('b'));
    assert_eq!(('a'.len_utf8(), '中'.len_utf8(), '😀'.len_utf8()), (1, 3, 4));
    assert_eq!(('a'.len_utf16(), '😀'.len_utf16()), (1, 2));
    let decoded: String = char::decode_utf16([0x4F60, 0x597D, 0xD800, 0x21])
        .map(|unit| unit.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect();
    assert_eq!(decoded, "你好\u{FFFD}!");

    // NonZero<T> 与 NonZeroU32 是同一个类型
    let generic_form: std::num::NonZero<u32> = std::num::NonZero::new(7).unwrap();
    let alias_form: std::num::NonZeroU32 = generic_form;
    assert_eq!(alias_form.get(), 7);
}
