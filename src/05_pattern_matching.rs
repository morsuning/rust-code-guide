// Rust 模式匹配
// 深入讲解 Rust 模式匹配系统的各个方面，包括 match 表达式、模式语法、解构、守卫等高级特性

// ===========================================
// 1. match 表达式基础 (Match Expression Basics)
// ===========================================

// match 表达式是 Rust 中最强大的控制流工具
// 它不仅仅是 C 语言中的 switch 语句的增强版本，而是一个完整的模式匹配系统
// match 的核心特性：穷尽性检查、模式匹配、表达式返回值、编译时优化

fn match_basics() {
    println!("=== match 表达式基础 ===");

    // 基本 match 表达式
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"), // 通配符模式，确保匹配所有可能情况
    }

    // match 作为表达式的强大特性
    // match 可以返回值，这使得它比传统的 if-else 链更加优雅
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("Match 表达式结果: {}", result);

    // 多模式匹配：使用 | 运算符匹配多个模式
    let x = 1;
    match x {
        1 | 2 => println!("One or two"), // 或模式：匹配 1 或 2
        3 => println!("Three"),
        _ => println!("Other"),
    }

    // match 的穷尽性检查：
    // Rust 编译器会确保 match 覆盖所有可能的情况
    // 这是 Rust 安全性的重要体现，防止运行时的未定义行为
    // 如果移除 _ 分支，编译会失败，因为没有覆盖所有可能的整数值

    println!();
}

// ===========================================
// 2. 模式语法 (Pattern Syntax)
// ===========================================

// 模式是 Rust 中用于匹配数据结构的语法元素
// 理解模式语法对于掌握 Rust 的模式匹配系统至关重要

fn pattern_syntax() {
    println!("=== 模式语法 ===");

    // 字面量模式：最简单的模式形式
    let x = 1;
    match x {
        1 => println!("数字 1"),
        2 => println!("数字 2"),
        _ => println!("其他数字"),
    }

    // 命名变量模式：将值绑定到变量
    // 这种模式在处理 Option 和 Result 类型时特别有用
    let x = Some(5);
    match x {
        Some(i) => println!("Some( {} )", i), // 将 Some 中的值绑定到变量 i
        None => println!("None"),
    }

    // 多重模式：使用 | 组合多个模式
    let x = 5;
    match x {
        1 | 2 => println!("1 或 2"),
        3 | 4 | 5 => println!("3, 4 或 5"),
        _ => println!("其他"),
    }

    // 范围模式：使用 ..= 运算符匹配数值范围
    // 范围模式只适用于实现了 PartialOrd 特征的类型
    let x = 7;
    match x {
        1..=5 => println!("1 到 5 之间"),      // 包含边界：1, 2, 3, 4, 5
        6..=10 => println!("6 到 10 之间"),    // 包含边界：6, 7, 8, 9, 10
        _ => println!("其他"),
    }

    // 范围模式的注意事项：
    // - 范围必须是有效的（start <= end）
    // - 只适用于实现了 PartialOrd 的类型
    // - 对于整数类型，编译器会检查范围的有效性

    println!();
}

// ===========================================
// 3. 解构模式 (Destructuring Patterns)
// ===========================================

// 解构是 Rust 模式匹配的核心特性之一
// 它允许我们在匹配的同时提取复杂数据结构中的值

fn destructuring_patterns() {
    println!("=== 解构模式 ===");

    // 解构结构体：提取结构体字段的值
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("在 x 轴上: {}", x),       // 匹配 y 坐标为 0 的点
        Point { x: 0, y } => println!("在 y 轴上: {}", y),       // 匹配 x 坐标为 0 的点
        Point { x, y } => println!("在其他位置: ({}, {})", x, y), // 匹配其他所有情况
    }

    // 解构枚举：处理枚举的不同变体
    // 枚举解构是 Rust 中处理代数数据类型（ADT）的重要方式
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("退出消息"),                              // 无数据的枚举变体
        Message::Move { x, y } => println!("移动消息: ({}, {})", x, y),      // 结构体风格的枚举变体
        Message::Write(text) => println!("写入消息: {}", text),              // 包含单个值的枚举变体
        Message::ChangeColor(r, g, b) => println!("颜色消息: RGB({}, {}, {})", r, g, b), // 元组风格的枚举变体
    }

    // 解构嵌套结构：处理复杂的数据结构
    #[derive(Debug)]
    struct Color(i32, i32, i32); // 元组结构体
    let point = (1, 2, Color(0, 160, 255));
    match point {
        (_, _, Color(r, g, b)) => println!("颜色: RGB({}, {}, {})", r, g, b),
    }

    // 解构模式的强大之处：
    // 1. 可以一次性提取多个值
    // 2. 可以在匹配的同时进行数据验证
    // 3. 可以处理任意深度的嵌套结构
    // 4. 与所有权系统完美集成

    println!();
}

// ===========================================
// 4. 忽略模式 (Ignoring Patterns)
// ===========================================

// 有时我们需要匹配某些结构但只关心部分值
// Rust 提供了多种忽略值的方式，每种都有其特定的用途

fn ignoring_patterns() {
    println!("=== 忽略模式 ===");

    // 使用 _ 忽略整个值：当我们不关心具体的值时
    // _ 模式不会绑定变量，因此不会触发未使用变量的警告
    let some_value = Some(3);
    match some_value {
        Some(_) => println!("Some 值被忽略"), // 只关心是 Some 而不是 None
        None => println!("None 值"),
    }

    // 使用 _ 忽略部分值：从元组或结构体中选择性地提取值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("一些数字: {}, {}, {}", first, third, fifth);
        }
    }

    // 使用 .. 忽略剩余值：当我们只关心前面或后面的值时
    // .. 模式可以匹配任意数量的剩余元素
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("第一个: {}, 最后一个: {}", first, last);
        }
    }

    // 使用 _ 忽略未使用的变量：避免编译器警告
    // 这是一种良好的编程实践，表明我们故意不使用某个变量
    let _x = 5;
    let y = 10;
    println!("只使用 y: {}", y);

    // 忽略模式的选择策略：
    // - _：当我们完全不关心值时
    // - _variable_name：当我们想表示有意忽略但不希望有警告时
    // - ..：当我们想忽略多个连续的值时
    // - 变量名前加 _：当我们想忽略但保留名字以提高可读性时

    println!();
}

// ===========================================
// 5. 匹配守卫 (Match Guards)
// ===========================================

// 匹配守卫是 match 分支中额外的 if 条件
// 它们提供了比单独的模式更复杂的匹配逻辑

fn match_guards() {
    println!("=== 匹配守卫 ===");

    // 基本匹配守卫：在模式匹配后添加额外的条件
    // 守卫让我们能够表达模式本身无法表达的复杂条件
    let x = Some(5);
    match x {
        Some(n) if n < 5 => println!("小于 5: {}", n),       // 守卫：n < 5
        Some(n) if n == 5 => println!("等于 5: {}", n),      // 守卫：n == 5
        Some(n) => println!("大于 5: {}", n),               // 没有守卫的默认情况
        None => println!("None"),
    }

    // 多重匹配守卫：将守卫应用于多个模式
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("4, 5 或 6 且 y 为 true"),  // 守卫应用于整个模式组
        4 | 5 | 6 => println!("4, 5 或 6 但 y 为 false"),     // 没有守卫的同一组模式
        _ => println!("其他"),
    }

    // 复杂匹配守卫：实现复杂的业务逻辑
    let temperature = 25;
    match temperature {
        temp if temp < 0 => println!("冰冻: {}°C", temp),
        temp if temp < 10 => println!("寒冷: {}°C", temp),
        temp if temp < 20 => println!("凉爽: {}°C", temp),
        temp if temp < 30 => println!("温暖: {}°C", temp),
        _ => println!("炎热: {}°C", temperature),
    }

    // 匹配守卫的优势：
    // 1. 可以使用任意布尔表达式
    // 2. 可以访问模式中绑定的变量
    // 3. 可以调用函数进行复杂的验证
    // 4. 提供了比纯模式更灵活的匹配能力

    // 匹配守卫的注意事项：
    // - 守卫中的代码会在每次匹配时执行
    // - 守卫可能会失败（panic），需要谨慎处理
    // - 守卫不能移动模式中绑定的值

    println!();
}

// ===========================================
// 6. @ 绑定 (@ Binding)
// ===========================================

// @ 绑定允许我们在使用复杂模式的同时，将值绑定到变量
// 这在需要同时进行模式匹配和值绑定时非常有用

fn at_binding() {
    println!("=== @ 绑定 ===");

    // 基本 @ 绑定：在范围匹配的同时绑定值
    // @ 让我们既能匹配特定的模式，又能保留完整的值
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            // id_variable 被绑定到实际的 id 值，但只在 id 在 3..=7 范围内时匹配
            println!("找到 id 在范围内: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            // 只匹配，不绑定到变量
            println!("找到 id 在 10 到 12 之间");
        }
        Message::Hello { id } => {
            // 默认情况，绑定到变量
            println!("找到其他 id: {}", id);
        }
    }

    // @ 绑定与守卫的结合：实现更复杂的匹配逻辑
    let x = 5;
    match x {
        x @ 1..=5 if x % 2 == 0 => println!("偶数在 1-5 范围内: {}", x),
        x @ 1..=5 => println!("奇数在 1-5 范围内: {}", x),
        _ => println!("其他数字"),
    }

    // @ 绑定的实际应用场景：
    // 1. 验证数据格式的同时保留原始数据
    // 2. 在匹配特定模式的同时进行数据转换
    // 3. 实现复杂的验证和提取逻辑
    // 4. 保持代码的可读性和表达力

    // @ 绑定的语法规则：
    // - variable_name @ pattern：将匹配的值绑定到 variable_name
    // - 可以与任何模式类型结合使用
    // - 变量名的作用域仅限于匹配分支

    println!();
}

// ===========================================
// 7. if let 和 while let
// ===========================================

// if let 和 while let 是 match 表达式的简化形式
// 它们提供了更简洁的语法来处理常见的匹配模式

fn if_let_while_let() {
    println!("=== if let 和 while let ===");

    // if let 简化单分支匹配：处理 Option 类型的常见模式
    // if let 更适合只关心一种匹配情况，而不关心其他情况
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用最喜欢的颜色: {}", color);
    } else if is_tuesday {
        println!("星期二使用绿色");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色");
        } else {
            println!("使用橙色");
        }
    } else {
        println!("使用蓝色");
    }

    // while let 解构：处理迭代和数据结构的逐步处理
    // while let 特别适合处理栈、队列等数据结构
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // if let 和 while let 的选择指南：
    // - if let：当只关心一种匹配模式时
    // - match：当需要处理所有可能的匹配情况时
    // - while let：当需要重复处理某种模式直到失败时
    // - for 循环：当需要迭代所有元素时

    // 使用 if let 的最佳实践：
    // 1. 用于简单的 Option 和 Result 处理
    // 2. 当错误处理可以统一处理时
    // 3. 当代码可读性比完整性更重要时

    println!();
}

// ===========================================
// 8. let-else 语句
// ===========================================

// let-else 是 Rust 1.65 引入的特性，用于简化错误处理
// 它提供了一种优雅的方式来处理必须成功的操作

fn let_else_statement() {
    println!("=== let-else 语句 ===");

    // let-else 语句：简化的错误处理模式
    // let-else 的语义：如果模式匹配成功，继续执行；否则执行 else 块
    fn get_user_name() -> Option<String> {
        Some("Alice".to_string())
    }

    let Some(name) = get_user_name() else {
        println!("未找到用户名");
        return;
    };

    println!("用户名: {}", name);

    // let-else 与错误处理：处理 Result 类型
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse().map_err(|e| format!("解析错误: {}", e))
    }

    let Ok(number) = parse_number("42") else {
        println!("数字解析失败");
        return;
    };

    println!("解析的数字: {}", number);

    // let-else 的使用场景：
    // 1. 在函数开始处进行参数验证
    // 2. 处理必须成功的操作
    // 3. 提前退出错误情况
    // 4. 减少嵌套的 if-else 结构

    // let-else 的优势：
    // 1. 减少代码嵌套，提高可读性
    // 2. 明确表达"必须成功"的意图
    // 3. 与所有权系统良好集成
    // 4. 提供了统一的错误处理模式

    println!();
}

// ===========================================
// 9. 模式匹配和所有权
// ===========================================

// 模式匹配与 Rust 的所有权系统紧密集成
// 理解匹配如何影响所有权对于编写安全的 Rust 代码至关重要

fn pattern_matching_ownership() {
    println!("=== 模式匹配和所有权 ===");

    // 移动语义：match 会移动匹配的值
    // 这与 Rust 的所有权规则一致：值只能有一个所有者
    let s = String::from("hello");
    match s {
        value => println!("字符串: {}", value), // s 被移动到 value
    }
    // println!("{}", s); // 编译错误：s 已被移动，不再有效

    // 借用模式：使用引用避免所有权转移
    // 通过借用，我们可以在匹配后仍然使用原始值
    let s = String::from("hello");
    match &s {
        value => println!("借用字符串: {}", value), // s 被借用，而不是移动
    }
    println!("原字符串仍然可用: {}", s);

    // 可变借用模式：在匹配的同时修改值
    // 可变借用允许我们在匹配过程中修改数据
    let mut s = String::from("hello");
    match &mut s {
        value => {
            value.push_str(", world");
            println!("修改后的字符串: {}", value);
        }
    }
    println!("最终字符串: {}", s);

    // 匹配与所有权的最佳实践：
    // 1. 如果不需要修改数据，优先使用不可变借用
    // 2. 如果需要修改数据，使用可变借用
    // 3. 如果确实需要转移所有权，确保这是设计意图
    // 4. 在复杂匹配中，注意生命周期的影响

    // 避免所有权问题的策略：
    // 1. 使用 & 和 &mut 进行借用
    // 2. 对于 Copy 类型，不用担心移动问题
    // 3. 使用 clone() 当确实需要复制数据时
    // 4. 合理设计数据结构以支持借用模式

    println!();
}

// ===========================================
// 10. 模式匹配高级示例
// ===========================================

// 高级模式匹配技术展示了 Rust 模式系统的真正威力
// 这些技术在实际项目中非常有用

fn advanced_pattern_matching() {
    println!("=== 高级模式匹配示例 ===");

    // 复杂结构解构：处理嵌套的数据结构
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        address: Address,
    }

    #[derive(Debug)]
    struct Address {
        street: String,
        city: String,
        zip: String,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        address: Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            zip: "10001".to_string(),
        },
    };

    match person {
        Person {
            name,
            age: age @ 25..=35,      // @ 绑定年龄范围
            address: Address { city, .. }, // 只关心城市，忽略其他字段
        } => {
            println!("匹配到 {} 岁，住在 {} 的 {}", age, city, name);
        }
        _ => println!("其他年龄或城市"),
    }

    // 切片模式：处理数组和切片的特定模式
    // 切片模式在处理固定大小的数据时非常有用
    let slice = &[1, 2, 3, 4, 5];
    match slice {
        [first, .., last] => println!("切片: 首元素 {}, 尾元素 {}", first, last),
        [] => println!("空切片"),
    }

    // Box 模式：解构智能指针
    // 这展示了 Rust 模式系统如何与智能指针集成
    let boxed_value = Box::new(5);
    match boxed_value {
        Box(value) => println!("Box 中的值: {}", value),
    }

    // 高级模式的实际应用：
    // 1. 数据验证和转换
    // 2. 状态机的实现
    // 3. 解析器的设计
    // 4. 复杂业务逻辑的处理

    // 模式匹配的性能考虑：
    // 1. 编译器会优化常见的模式匹配
    // 2. 复杂的模式可能会产生额外的运行时开销
    // 3. 在性能关键路径上，考虑使用更简单的匹配方式
    // 4. 利用编译器的优化和内联

    println!();
}

// ===========================================
// 11. matches! 宏 (Rust 1.42)
// ===========================================

// matches! 宏是 Rust 1.42 版本引入的重要特性
// 它提供了一种简洁的方式来检查值是否匹配特定模式
// matches! 是模式匹配的语法糖，特别适合用于条件判断

fn matches_macro() {
    println!("=== matches! 宏 ===");

    // matches! 宏的基本语法：matches!(value, pattern)
    // 它返回一个布尔值，表示值是否匹配给定的模式
    let value = Some(5);

    // 使用 matches! 检查 Option 类型
    assert!(matches!(value, Some(_)));
    assert!(!matches!(value, None));

    // matches! 与复杂模式的结合
    // 它支持所有模式匹配语法，包括守卫和范围
    let number = 7;

    // 检查数字是否在特定范围内
    assert!(matches!(number, 1..=10));
    assert!(!matches!(number, 11..=20));

    // 使用守卫进行更复杂的检查
    assert!(matches!(number, x if x > 5 && x < 10));
    assert!(!matches!(number, x if x > 10));

    // matches! 与枚举类型的完美结合
    // 这是 matches! 最常见的使用场景之一
    #[derive(Debug)]
    enum Status {
        Success,
        Error(String),
        Pending,
        Timeout,
    }

    let status = Status::Error("网络连接失败".to_string());

    // 检查特定状态变体
    assert!(matches!(status, Status::Error(_)));
    assert!(!matches!(status, Status::Success));

    // 带有模式解构的 matches!
    let error_status = Status::Error("文件不存在".to_string());
    let has_network_error = matches!(error_status, Status::Error(msg) if msg.contains("网络"));
    let has_file_error = matches!(error_status, Status::Error(msg) if msg.contains("文件"));

    println!("网络错误: {}", has_network_error);  // false
    println!("文件错误: {}", has_file_error);      // true

    // matches! 在实际编程中的应用场景
    // 场景 1：输入验证
    fn validate_age(age: u8) -> bool {
        matches!(age, 18..=120)
    }

    println!("年龄 25 有效: {}", validate_age(25));   // true
    println!("年龄 15 有效: {}", validate_age(15));   // false
    println!("年龄 150 有效: {}", validate_age(150)); // false

    // 场景 2：状态检查
    fn is_terminal_status(status: &Status) -> bool {
        matches!(status, Status::Success | Status::Error(_))
    }

    let terminal_status = Status::Success;
    let non_terminal_status = Status::Pending;

    println!("成功状态是终端状态: {}", is_terminal_status(&terminal_status));       // true
    println!("等待状态是终端状态: {}", is_terminal_status(&non_terminal_status));    // false

    // 场景 3：数据类型检查
    fn is_string_value(value: &serde_json::Value) -> bool {
        matches!(value, serde_json::Value::String(_))
    }

    // 这里使用模拟的 JSON 值检查
    #[derive(Debug)]
    enum JsonValue {
        String(String),
        Number(i64),
        Boolean(bool),
        Null,
    }

    let string_value = JsonValue::String("hello".to_string());
    let number_value = JsonValue::Number(42);

    println!("字符串值: {}", matches!(string_value, JsonValue::String(_)));    // true
    println!("数字值: {}", matches!(number_value, JsonValue::String(_)));     // false

    // matches! 与 if 表达式的对比
    // 传统的 if let 方式需要更多的代码
    let value = Some(5);

    // 使用 matches! 的简洁方式
    let is_some_positive = matches!(value, Some(x) if x > 0);

    // 传统的 if let 方式（更冗长）
    let is_some_positive_traditional = if let Some(x) = value {
        x > 0
    } else {
        false
    };

    println!("matches! 方式: {}", is_some_positive);
    println!("传统方式: {}", is_some_positive_traditional);

    // matches! 的优势：
    // 1. 语法简洁，减少代码量
    // 2. 可读性强，意图明确
    // 3. 支持所有模式匹配语法
    // 4. 编译器优化，性能良好
    // 5. 在条件判断中特别有用

    // matches! 的使用建议：
    // 1. 在简单的条件判断中优先使用 matches!
    // 2. 对于复杂的匹配逻辑，仍然使用完整的 match 表达式
    // 3. 在函数返回条件判断中使用 matches! 提高可读性
    // 4. 结合守卫使用时，注意逻辑的清晰性

    println!();
}

// ===========================================
// 12. 模式匹配最佳实践
// ===========================================

// 掌握模式匹配的最佳实践对于编写高质量 Rust 代码至关重要

fn pattern_matching_best_practices() {
    println!("=== 模式匹配最佳实践 ===");

    // 使用 match 确保穷尽性：让编译器帮助我们发现遗漏的情况
    // 穷尽性检查是 Rust 安全性的重要体现
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let direction = Direction::North;
    match direction {
        Direction::North => println!("向北"),
        Direction::South => println!("向南"),
        Direction::East => println!("向东"),
        Direction::West => println!("向西"),
    }

    // 使用 if let 简化单分支匹配：提高代码可读性
    // if let 适合处理简单的 Option 和 Result 操作
    let some_value = Some(5);
    if let Some(value) = some_value {
        println!("找到值: {}", value);
    }

    // 使用 while let 处理迭代：简化循环逻辑
    // while let 特别适合处理需要逐步处理的数据结构
    let mut counter = 0;
    while let Some(_) = some_value {
        counter += 1;
        if counter > 2 {
            break;
        }
    }

    // 使用 match 作为表达式：利用 Rust 的表达式特性
    // match 作为表达式可以替代复杂的 if-else 链
    let number = 3;
    let description = match number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };
    println!("{} is {}", number, description);

    // 最佳实践总结：
    // 1. 优先使用 match 进行穷尽性检查
    // 2. 使用 if let 简化简单的匹配逻辑
    // 3. 利用 match 作为表达式提高代码简洁性
    // 4. 在适当的时候使用匹配守卫和 @ 绑定
    // 5. 注意模式匹配对所有权的影响
    // 6. 保持代码的可读性和维护性

    // 常见陷阱和解决方案：
    // 1. 遗忘 _ 分支 → 编译器会提醒
    // 2. 不必要的移动 → 使用借用
    // 3. 过度复杂的模式 → 考虑拆分逻辑
    // 4. 性能问题 → 使用基准测试验证

    println!();
}

// ===========================================
// 12. 模式匹配示例程序
// ===========================================

// 通过实际的示例程序来展示模式匹配的强大功能
// 这些示例展示了模式匹配在实际项目中的应用

fn pattern_matching_example_program() {
    println!("=== 模式匹配示例程序 ===");

    // 示例 1：简单的计算器
    // 展示如何使用模式匹配实现基本的计算逻辑
    fn calculate(op: char, a: i32, b: i32) -> Result<i32, String> {
        match op {
            '+' => Ok(a + b),
            '-' => Ok(a - b),
            '*' => Ok(a * b),
            '/' => {
                if b == 0 {
                    Err("除零错误".to_string())
                } else {
                    Ok(a / b)
                }
            }
            _ => Err("未知运算符".to_string()),
        }
    }

    let operations = vec![('+', 10, 5), ('-', 10, 5), ('*', 10, 5), ('/', 10, 2), ('/', 10, 0)];

    for (op, a, b) in operations {
        match calculate(op, a, b) {
            Ok(result) => println!("{} {} {} = {}", a, op, b, result),
            Err(error) => println!("错误: {} {} {} - {}", a, op, b, error),
        }
    }

    println!();

    // 示例 2：文本处理程序
    // 展示如何使用模式匹配处理字符串和文本数据
    fn process_text(text: &str) -> Vec<&str> {
        text.split_whitespace().collect()
    }

    let texts = vec![
        "Hello World",
        "Rust Programming",
        "Pattern Matching",
        "",
        "Multiple   Spaces",
    ];

    for text in texts {
        match process_text(text) {
            words if words.is_empty() => println!("空文本"),
            words => println!("文本: '{}' -> {:?} 个单词", text, words.len()),
        }
    }

    // 示例 3：配置文件解析
    // 展示如何使用模式匹配处理配置数据
    #[derive(Debug)]
    enum ConfigValue {
        String(String),
        Number(i64),
        Boolean(bool),
        List(Vec<ConfigValue>),
    }

    fn parse_config_value(key: &str, value: ConfigValue) -> String {
        match (key, value) {
            ("host", ConfigValue::String(host)) => format!("主机: {}", host),
            ("port", ConfigValue::Number(port)) => format!("端口: {}", port),
            ("debug", ConfigValue::Boolean(debug)) => format!("调试模式: {}", debug),
            ("features", ConfigValue::List(features)) => {
                let feature_names: Vec<String> = features.iter().map(|f| match f {
                    ConfigValue::String(name) => name.clone(),
                    _ => "unknown".to_string(),
                }).collect();
                format!("特性: {}", feature_names.join(", "))
            }
            (key, value) => format!("未知配置 {}: {:?}", key, value),
        }
    }

    let config = vec![
        ("host", ConfigValue::String("localhost".to_string())),
        ("port", ConfigValue::Number(8080)),
        ("debug", ConfigValue::Boolean(true)),
        ("features", ConfigValue::List(vec![
            ConfigValue::String("auth".to_string()),
            ConfigValue::String("logging".to_string()),
        ])),
    ];

    for (key, value) in config {
        println!("{}", parse_config_value(key, value));
    }

    // 实际应用中的模式匹配：
    // 1. 解析器和编译器的实现
    // 2. 网络协议的处理
    // 3. 用户界面的状态管理
    // 4. 游戏逻辑的实现
    // 5. 数据库查询的处理

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 模式匹配演示");
    println!("=================");

    match_basics();
    pattern_syntax();
    destructuring_patterns();
    ignoring_patterns();
    match_guards();
    at_binding();
    if_let_while_let();
    let_else_statement();
    pattern_matching_ownership();
    advanced_pattern_matching();
    matches_macro();
    pattern_matching_best_practices();
    pattern_matching_example_program();

    println!("模式匹配演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_basics() {
        let number = 3;
        let result = match number {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "other",
        };
        assert_eq!(result, "three");
    }

    #[test]
    fn test_destructuring() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };
        let result = match p {
            Point { x: 0, y } => format!("在 y 轴上: {}", y),
            Point { x, y: 0 } => format!("在 x 轴上: {}", x),
            Point { x, y } => format!("在其他位置: ({}, {})", x, y),
        };
        assert_eq!(result, "在 y 轴上: 7");
    }

    #[test]
    fn test_match_guards() {
        let x = Some(5);
        let result = match x {
            Some(n) if n < 5 => "小于 5",
            Some(n) if n == 5 => "等于 5",
            Some(n) => "大于 5",
            None => "none",
        };
        assert_eq!(result, "等于 5");
    }

    #[test]
    fn test_at_binding() {
        let x = 5;
        let result = match x {
            x @ 1..=5 if x % 2 == 0 => "偶数",
            x @ 1..=5 => "奇数",
            _ => "其他",
        };
        assert_eq!(result, "奇数");
    }

    #[test]
    fn test_if_let() {
        let some_value = Some(5);
        let mut result = String::new();

        if let Some(value) = some_value {
            result = format!("找到 {}", value);
        } else {
            result = "未找到".to_string();
        }

        assert_eq!(result, "找到 5");
    }

    #[test]
    fn test_calculator() {
        fn calculate(op: char, a: i32, b: i32) -> Result<i32, String> {
            match op {
                '+' => Ok(a + b),
                '-' => Ok(a - b),
                '*' => Ok(a * b),
                '/' => {
                    if b == 0 {
                        Err("除零错误".to_string())
                    } else {
                        Ok(a / b)
                    }
                }
                _ => Err("未知运算符".to_string()),
            }
        }

        assert_eq!(calculate('+', 10, 5), Ok(15));
        assert_eq!(calculate('-', 10, 5), Ok(5));
        assert_eq!(calculate('*', 10, 5), Ok(50));
        assert_eq!(calculate('/', 10, 2), Ok(5));
        assert_eq!(calculate('/', 10, 0), Err("除零错误".to_string()));
        assert_eq!(calculate('%', 10, 5), Err("未知运算符".to_string()));
    }

    #[test]
    fn test_pattern_ownership() {
        let s = String::from("hello");
        let result = match &s {
            value => value.len(),
        };
        assert_eq!(result, 5);
        assert_eq!(s, "hello"); // s 仍然可用，因为我们只借用了它
    }

    #[test]
    fn test_complex_destructuring() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u32,
        }

        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };

        let result = match person {
            Person { name, age: age @ 25..=35 } => format!("{}: {}", name, age),
            Person { name, age } => format!("{}: {} (其他年龄)", name, age),
        };
        assert_eq!(result, "Alice: 30");
    }

    #[test]
    fn test_slice_patterns() {
        let slice = &[1, 2, 3, 4, 5];
        let result = match slice {
            [first, .., last] => format!("{}..{}", first, last),
            [] => "empty".to_string(),
        };
        assert_eq!(result, "1..5");
    }

    #[test]
    fn test_matches_macro() {
        // 测试基本的 matches! 功能
        let value = Some(5);
        assert!(matches!(value, Some(_)));
        assert!(!matches!(value, None));

        // 测试范围匹配
        let number = 7;
        assert!(matches!(number, 1..=10));
        assert!(!matches!(number, 11..=20));

        // 测试守卫
        assert!(matches!(number, x if x > 5 && x < 10));
        assert!(!matches!(number, x if x > 10));

        // 测试枚举匹配
        #[derive(Debug)]
        enum Status {
            Success,
            Error(String),
            Pending,
        }

        let status = Status::Error("test error".to_string());
        assert!(matches!(status, Status::Error(_)));
        assert!(!matches!(status, Status::Success));

        // 测试带解构的 matches!
        let error_status = Status::Error("file error".to_string());
        assert!(matches!(error_status, Status::Error(msg) if msg.contains("file")));
        assert!(!matches!(error_status, Status::Error(msg) if msg.contains("network")));

        // 测试年龄验证函数
        fn validate_age(age: u8) -> bool {
            matches!(age, 18..=120)
        }

        assert!(validate_age(25));
        assert!(!validate_age(15));
        assert!(!validate_age(150));

        // 测试多重模式
        assert!(matches!(number, 5 | 6 | 7 | 8));
        assert!(!matches!(number, 1 | 2 | 3));
    }

    #[test]
    fn test_matches_vs_traditional() {
        let value = Some(5);

        // 使用 matches!
        let is_some_positive_matches = matches!(value, Some(x) if x > 0);

        // 使用传统方式
        let is_some_positive_traditional = if let Some(x) = value {
            x > 0
        } else {
            false
        };

        assert_eq!(is_some_positive_matches, is_some_positive_traditional);
        assert!(is_some_positive_matches);
    }
}