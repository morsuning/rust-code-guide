// Rust 枚举
// 深入讲解枚举定义、模式匹配、Option、Result 等核心概念
// 枚举是 Rust 中表达类型安全联合体的强大工具，支持数据变体和模式匹配

// ===========================================
// 1. 基础枚举 (Basic Enums)
// ===========================================

// 枚举（Enumeration）是一种类型，可以有多个不同的值
// 每个可能的值都被称为一个变体（variant）
// 枚举是 Rust 中实现代数数据类型（ADT）的基础

fn basic_enums() {
    println!("=== 基础枚举 ===");

    // 定义枚举：创建自定义类型
    // 枚举使用 enum 关键字定义，后跟枚举名称和变体列表
    // 枚举名称采用大驼峰命名法（PascalCase）
    // 变体也采用大驼峰命名法
    #[derive(Debug)]
    enum IpAddrKind {
        V4,    // IPv4 地址类型
        V6,    // IPv6 地址类型
    }

    // 枚举的本质：定义了一个新的类型，该类型的值只能是预定义的变体之一
    // 这种类型安全性确保程序不会出现无效的状态值
    // 编译器会检查所有可能的值都被正确处理

    // 使用枚举：通过 枚举名::变体名 的语法访问变体
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("枚举值: {:?} {:?}", four, six);

    // 枚举作为函数参数：类型安全的参数传递
    // 函数可以明确指定参数必须是枚举类型
    // 这避免了使用魔法数字或字符串的错误
    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("路由 IPv4 地址"),
            IpAddrKind::V6 => println!("路由 IPv6 地址"),
        }
    }

    // 调用函数时，必须传递有效的枚举值
    // 编译器会确保没有传入无效的类型或值
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // 枚举的优势：
    // 1. 类型安全：确保只能使用预定义的值
    // 2. 自文档化：代码清楚地表达了可能的值
    // 3. 编译时检查：编译器会检查所有可能的情况
    // 4. 模式匹配：可以使用 match 表达式处理所有变体
    // 5. 扩展性：可以轻松添加新的变体

    // 枚举与整数的区别：
    // - 枚举提供类型安全，整数没有
    // - 枚举变体有名称，更易读
    // - 枚举可以包含数据，整数不能
    // - 枚举支持模式匹配，整数需要额外的检查

    // 枚举的设计原则：
    // 1. 优先使用枚举而不是魔法数字或字符串
    // 2. 确保变体名称清晰表达其含义
    // 3. 使用模式匹配处理所有可能的情况
    // 4. 考虑为枚举实现 Debug trait 以便于调试

    println!();
}

// ===========================================
// 2. 带数据的枚举 (Enums with Data)
// ===========================================

// Rust 枚举的强大之处在于每个变体可以包含不同类型的数据
// 这使得枚举可以表达复杂的联合类型，同时保持类型安全

fn enums_with_data() {
    println!("=== 带数据的枚举 ===");

    // 定义带数据的枚举：每个变体可以包含不同类型和数量的数据
    // 这种能力使 Rust 枚举成为表达代数数据类型的理想选择
    #[derive(Debug)]
    enum Message {
        Quit,                       // 没有关联数据的变体
        Move { x: i32, y: i32 },    // 包含匿名结构体的变体
        Write(String),              // 包含单个字符串的变体
        ChangeColor(i32, i32, i32), // 包含三个整数的变体
    }

    // 枚举变体的数据类型：
    // 1. 单元变体（Unit-like）：类似单元类型，没有数据
    // 2. 结构体变体（Struct-like）：包含命名字段，类似结构体
    // 3. 元组变体（Tuple-like）：包含位置字段，类似元组
    // 4. 每个变体可以有不同的数据结构和类型

    // 创建带数据的枚举实例：为每个变体提供相应的数据
    let q = Message::Quit;                           // 单元变体
    let m = Message::Move { x: 12, y: 24 };         // 结构体变体
    let w = Message::Write(String::from("hello"));    // 元组变体
    let c = Message::ChangeColor(0, 255, 255);        // 元组变体

    // 为枚举实现方法：使用 impl 块
    // 枚举的方法可以访问和操作枚举实例的数据
    impl Message {
        // 主要方法：根据消息类型执行不同的操作
        fn call(&self) {
            match self {
                Message::Quit => println!("退出消息"),
                Message::Move { x, y } => println!("移动消息: x={}, y={}", x, y),
                Message::Write(text) => println!("写入消息: {}", text),
                Message::ChangeColor(r, g, b) => println!("颜色变更: RGB({}, {}, {})", r, g, b),
            }
        }

        // 辅助方法：获取消息类型
        fn get_message_type(&self) -> &'static str {
            match self {
                Message::Quit => "Quit",
                Message::Move { .. } => "Move",          // 使用 .. 忽略字段
                Message::Write(_) => "Write",            // 使用 _ 忽略数据
                Message::ChangeColor(..) => "ChangeColor", // 使用 .. 忽略多个字段
            }
        }

        // 数据提取方法：从消息中提取相关信息
        fn extract_data(&self) -> Option<String> {
            match self {
                Message::Write(text) => Some(text.clone()),
                Message::Move { x, y } => Some(format!("Move({}, {})", x, y)),
                Message::ChangeColor(r, g, b) => Some(format!("RGB({}, {}, {})", r, g, b)),
                Message::Quit => None,
            }
        }
    }

    // 调用枚举方法
    q.call();
    m.call();
    w.call();
    c.call();

    println!("消息类型: {}", m.get_message_type());

    if let Some(data) = w.extract_data() {
        println!("提取的数据: {}", data);
    }

    // 带数据枚举的优势：
    // 1. 类型安全：每个变体都有特定的数据类型
    // 2. 内存效率：只存储当前变体需要的数据
    // 3. 表达力强：可以表达复杂的数据关系
    // 4. 模式匹配：可以安全地提取和使用数据
    // 5. 扩展性：可以轻松添加新的变体和数据类型

    // 实际应用场景：
    // 1. 消息系统：不同类型的消息携带不同的数据
    // 2. 状态机：每个状态有不同的相关数据
    // 3. 配置系统：不同配置项有不同的数据结构
    // 4. API 响应：不同响应类型有不同的数据格式

    // 内存布局考虑：
    // - 枚举的总大小是最大变体的大小加上标签（discriminant）
    // - 标签用于区分当前是哪个变体
    // - Rust 会自动优化枚举的内存布局

    println!();
}

// ===========================================
// 3. Option 枚举 (Option Enum)
// ===========================================

// Option 是 Rust 标准库中最重要的枚举之一
// 它用于表达可能存在或可能不存在的值，是 Rust 处理空值的类型安全方式

fn option_enum() {
    println!("=== Option 枚举 ===");

    // Option 枚举的定义（预定义在标准库中）
    // Option<T> 是一个泛型枚举，可以包装任何类型 T
    // enum Option<T> {
    //     Some(T),   // 存在值
    //     None,      // 不存在值
    // }

    // 使用 Option 包装不同的数据类型
    let some_number = Some(5);                      // Option<i32>
    let some_string = Some("a string");              // Option<&str>
    let absent_number: Option<i32> = None;           // 明确指定类型的 None

    println!("Option 值: {:?} {:?} {:?}", some_number, some_string, absent_number);

    // Option 的核心价值：类型安全地处理可能为空的值
    // 相比其他语言中的空指针，Option 在编译时就能防止空引用错误

    // 使用 match 处理 Option：穷尽所有可能的情况
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,                           // 没有值，返回 None
            Some(i) => Some(i + 1),                 // 有值，进行加法运算
        }
    }

    // 处理不同的情况
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Option 运算: {:?} {:?}", six, none);

    // 使用 if let 简化单分支的 Option 处理
    // if let 是 match 的语法糖，用于只关心一种情况
    if let Some(x) = some_number {
        println!("if let: Some({})", x);
    }

    // Option 的常用方法：
    // 1. unwrap()：获取值，如果是 None 则 panic
    // 2. expect()：获取值，如果是 None 则 panic 并显示自定义消息
    // 3. unwrap_or()：获取值，如果是 None 则返回默认值
    // 4. unwrap_or_else()：获取值，如果是 None 则调用闭包生成默认值
    // 5. map()：对值应用函数，如果是 None 则返回 None
    // 6. and_then()：链式处理 Option

    // 演示 Option 方法的使用
    let value = some_number.unwrap();                    // 5
    let default_value = absent_number.unwrap_or(0);       // 0
    let mapped_value = some_number.map(|x| x * 2);        // Some(10)

    println!("unwrap: {}", value);
    println!("unwrap_or: {}", default_value);
    println!("map: {:?}", mapped_value);

    // Option 与错误处理的结合
    fn safe_divide(numerator: i32, denominator: i32) -> Option<i32> {
        if denominator == 0 {
            None                                    // 除零错误
        } else {
            Some(numerator / denominator)            // 成功结果
        }
    }

    match safe_divide(10, 2) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("不能除以零"),
    }

    // Option 的设计哲学：
    // 1. 显式处理：强制程序员考虑空值的情况
    // 2. 类型安全：编译时防止空引用错误
    // 3. 组合性强：可以链式调用多个操作
    // 4. 零成本抽象：编译时优化，运行时没有额外开销

    // 最佳实践：
    // 1. 优先使用 Option 而不是空指针或魔法值
    // 2. 在函数签名中明确表示可能返回空值
    // 3. 使用 match 或 if let 处理所有情况
    // 4. 谨慎使用 unwrap，只在确定值存在时使用
    // 5. 使用 map 和 and_then 进行链式操作

    println!();
}

// ===========================================
// 4. Result 枚举 (Result Enum)
// ===========================================

// Result 是 Rust 中处理错误的基石
// 它提供了类型安全的方式来表达操作可能成功或失败的结果

fn result_enum() {
    println!("=== Result 枚举 ===");

    // Result 枚举的定义（预定义在标准库中）
    // Result<T, E> 是一个泛型枚举，用于表示可能失败的操作
    // enum Result<T, E> {
    //     Ok(T),     // 成功，包含类型 T 的值
    //     Err(E),    // 失败，包含类型 E 的错误
    // }

    // 使用 Result 表达可能失败的操作
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Cannot divide by zero"))    // 错误情况
        } else {
            Ok(numerator / denominator)                   // 成功情况
        }
    }

    // 执行除法操作
    let result1 = divide(10.0, 2.0);     // 成功
    let result2 = divide(10.0, 0.0);     // 失败

    // 使用 match 处理 Result：处理所有可能的结果
    match result1 {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(error) => println!("错误: {}", error),
    }

    match result2 {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(error) => println!("错误: {}", error),
    }

    // Result 的便利方法：
    // 1. unwrap()：获取成功值，如果是 Err 则 panic
    // 2. expect()：获取成功值，如果是 Err 则 panic 并显示自定义消息
    // 3. unwrap_or()：获取成功值，如果是 Err 则返回默认值
    // 4. unwrap_or_else()：获取成功值，如果是 Err 则调用闭包生成默认值
    // 5. map()：对成功值应用函数，如果是 Err 则保持不变
    // 6. map_err()：对错误值应用函数，如果是 Ok 则保持不变
    // 7. and_then()：链式处理 Result

    // 使用 unwrap 和 expect
    let value = divide(10.0, 2.0).unwrap();           // 5.0
    println!("unwrap 结果: {}", value);

    let value = divide(10.0, 2.0).expect("除法失败");  // 5.0
    println!("expect 结果: {}", value);

    // 演示 unwrap_or 的使用
    let default_value = divide(10.0, 0.0).unwrap_or(0.0);
    println!("unwrap_or 结果: {}", default_value);

    // 错误传播：使用 ? 操作符简化错误处理
    // ? 操作符是 Rust 中最常用的错误处理语法糖
    fn process_number(num: f64) -> Result<String, String> {
        let doubled = num * 2.0;
        let result = divide(doubled, 2.0)?;    // 如果失败，立即返回错误
        Ok(format!("处理结果: {}", result))
    }

    match process_number(5.0) {
        Ok(result) => println!("处理成功: {}", result),
        Err(error) => println!("处理失败: {}", error),
    }

    // Result 的组合操作
    fn validate_age(age: i32) -> Result<i32, String> {
        if age < 0 {
            Err("年龄不能为负数".to_string())
        } else if age > 150 {
            Err("年龄过大".to_string())
        } else {
            Ok(age)
        }
    }

    fn get_adult_category(age: i32) -> Result<String, String> {
        let valid_age = validate_age(age)?;          // 验证年龄
        let category = if valid_age >= 18 {
            "成年人"
        } else {
            "未成年人"
        };
        Ok(category.to_string())
    }

    match get_adult_category(25) {
        Ok(category) => println!("年龄分类: {}", category),
        Err(error) => println!("错误: {}", error),
    }

    // Result 与 Option 的区别：
    // - Option<T> 表示值可能存在或不存在
    // - Result<T, E> 表示操作可能成功或失败，失败时包含错误信息
    // - Result 通常用于可能失败的 I/O 操作、解析操作等
    // - Option 通常用于可能为空的查找操作

    // Result 的最佳实践：
    // 1. 在函数签名中明确表达可能失败的操作
    // 2. 使用具体的错误类型，提供有用的错误信息
    // 3. 优先使用 ? 操作符进行错误传播
    // 4. 在应用程序边界处处理错误
    // 5. 谨慎使用 unwrap，只在确定成功时使用

    println!();
}

// ===========================================
// 5. 枚举模式匹配 (Enum Pattern Matching)
// ===========================================

// 模式匹配是 Rust 中处理枚举的主要方式
// match 表达式提供了穷尽性检查，确保所有情况都被处理

fn enum_pattern_matching() {
    println!("=== 枚举模式匹配 ===");

    // 定义复杂的枚举类型
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        Arizona,
        Arkansas,
        California,
        // 更多州...
    }

    #[derive(Debug)]
    enum Coin {
        Penny,                      // 便士
        Nickel,                     // 镍币
        Dime,                        // 一角硬币
        Quarter(UsState),            // 25美分硬币（包含州信息）
    }

    // 模式匹配：根据硬币类型返回其价值
    // match 表达式会检查所有可能的变体
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("幸运便士！");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("州季度: {:?}", state);
                25
            }
        }
    }

    // 创建不同的硬币实例
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alaska);

    // 演示模式匹配
    println!("便士价值: {}", value_in_cents(penny));
    println!("季度价值: {}", value_in_cents(quarter));

    // 模式匹配的高级特性：
    // 1. 字段解构：提取枚举变体中的数据
    // 2. 多重匹配：使用 | 匹配多个变体
    // 3. 范围匹配：使用 .. 匹配值范围
    // 4. 守卫条件：使用 if 添加额外的条件
    // 5. 绑定：使用 @ 将值绑定到变量

    // 更复杂的模式匹配示例
    #[derive(Debug)]
    enum Shape {
        Circle(f64),                    // 圆形（半径）
        Rectangle(f64, f64),            // 矩形（宽高）
        Triangle(f64, f64, f64),        // 三角形（三边）
        Point,                          // 点
    }

    fn describe_shape(shape: Shape) -> String {
        match shape {
            Shape::Circle(radius) => format!("圆形，半径: {}", radius),
            Shape::Rectangle(width, height) => format!("矩形，宽: {}, 高: {}", width, height),
            Shape::Triangle(a, b, c) => format!("三角形，边长: {}, {}, {}", a, b, c),
            Shape::Point => "点".to_string(),
        }
    }

    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(10.0, 20.0);
    let point = Shape::Point;

    println!("{}", describe_shape(circle));
    println!("{}", describe_shape(rectangle));
    println!("{}", describe_shape(point));

    // 模式匹配的穷尽性检查：
    // - Rust 编译器会检查 match 是否覆盖所有可能的情况
    // - 如果遗漏某些情况，编译会失败
    // - 这确保了代码的完整性和安全性

    // 模式匹配的优势：
    // 1. 类型安全：编译时检查所有情况
    // 2. 数据提取：可以在匹配的同时提取数据
    // 3. 可读性强：清晰地表达处理逻辑
    // 4. 性能优化：编译器可以优化模式匹配

    println!();
}

// ===========================================
// 6. 通配符模式 (Wildcard Patterns)
// ===========================================

// 通配符模式用于匹配不关心的值
// 这在处理复杂模式或只关心部分值时非常有用

fn wildcard_patterns() {
    println!("=== 通配符模式 ===");

    let dice_roll = 9;

    // 使用 _ 通配符匹配所有其他情况
    // _ 是一个通配符，匹配任何值但不绑定到变量
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),           // 匹配除了 3 和 7 之外的所有值
    }

    // 使用 .. 忽略元组中的剩余值
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {    // 只关心第一个和最后一个值
            println!("第一个: {}, 最后一个: {}", first, last);
        }
    }

    // 在结构体中使用通配符
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let point = Point { x: 1, y: 2, z: 3 };

    match point {
        Point { x, .. } => {     // 只关心 x 字段
            println!("点的 x 坐标: {}", x);
        }
    }

    // 在枚举中使用通配符
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let message = Message::Write(String::from("Hello"));

    match message {
        Message::Write(text) => println!("写入消息: {}", text),
        _ => println!("其他类型的消息"),  // 匹配所有其他消息类型
    }

    // 通配符的最佳实践：
    // 1. 使用 _ 明确表示不关心的值
    // 2. 使用 .. 忽略多个连续的值
    // 3. 在模式匹配的最后使用 _ 作为默认情况
    // 4. 避免过多的通配符，保持代码清晰

    println!();
}

fn add_fancy_hat() {
    println!("添加了华丽的帽子！");
}

fn remove_fancy_hat() {
    println!("移除了华丽的帽子！");
}

fn reroll() {
    println!("重新投掷骰子...");
}

// ===========================================
// 7. 匹配守卫 (Match Guards)
// ===========================================

// 匹配守卫是 match 分支中的额外条件
// 它们允许在模式匹配的基础上添加更复杂的逻辑

fn match_guards() {
    println!("=== 匹配守卫 ===");

    let num = Some(4);

    // 使用 if 作为匹配守卫
    // 匹配守卫允许在模式匹配的基础上添加额外的条件
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("大于或等于 5: {}", x),
        None => println!("没有值"),
    }

    let x = Some(5);
    let y = 10;

    // 多重条件匹配守卫
    match x {
        Some(50) => println!("得到 50"),
        Some(n) if n == y => println!("匹配 y: {}", n),
        Some(n) if n + 4 == y => println!("n + 4 == y: {}", n),
        _ => println!("默认情况"),
    }

    // 匹配守卫的实际应用
    #[derive(Debug)]
    enum Event {
        KeyPress(char),
        MouseClick { x: i32, y: i32 },
        Resize(u32, u32),
        Quit,
    }

    fn handle_event(event: Event) {
        match event {
            Event::KeyPress('q') | Event::KeyPress('Q') => {
                println!("退出请求");
            }
            Event::KeyPress(c) if c.is_ascii() => {
                println!("按键: {}", c);
            }
            Event::MouseClick { x, y } if x > 0 && y > 0 => {
                println!("鼠标点击在正象限: ({}, {})", x, y);
            }
            Event::Resize(width, height) if width > 800 && height > 600 => {
                println!("窗口较大: {}x{}", width, height);
            }
            Event::Quit => {
                println!("退出事件");
            }
            _ => {
                println!("其他事件");
            }
        }
    }

    // 演示事件处理
    handle_event(Event::KeyPress('a'));
    handle_event(Event::KeyPress('q'));
    handle_event(Event::MouseClick { x: 100, y: 200 });
    handle_event(Event::Resize(1024, 768));

    // 匹配守卫的优势：
    // 1. 增强表达能力：可以添加复杂的条件逻辑
    // 2. 保持清晰性：条件与模式放在一起，便于理解
    // 3. 避免嵌套：减少不必要的嵌套 match 或 if
    // 4. 灵活性：可以使用变量和复杂的表达式

    // 匹配守卫的最佳实践：
    // 1. 保持守卫条件简单清晰
    // 2. 优先使用模式匹配而不是守卫
    // 3. 在需要时使用守卫补充模式匹配
    // 4. 避免过于复杂的守卫条件

    println!();
}

// ===========================================
// 8. @ 绑定 (@ Binding)
// ===========================================

// @ 绑定允许在匹配模式的同时将值绑定到变量
// 这在需要同时匹配和使用值的情况下非常有用

fn at_binding() {
    println!("=== @ 绑定 ===");

    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // 使用 @ 绑定：在匹配范围的同时绑定值到变量
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            // id_variable 被绑定到实际的 id 值
            println!("找到范围内的 id: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("找到 id 在 10 到 12 之间");
        }
        Message::Hello { id } => {
            println!("找到其他 id: {}", id);
        }
    }

    // @ 绑定的实际应用
    fn process_number(num: i32) {
        match num {
            // 匹配偶数并同时绑定值
            even_num @ 2 | even_num @ 4 | even_num @ 6 | even_num @ 8 => {
                println!("小偶数: {}", even_num);
            }
            // 匹配正数范围
            positive @ 1..=100 => {
                println!("正数: {}", positive);
            }
            // 匹配负数范围
            negative @ -100..=-1 => {
                println!("负数: {}", negative);
            }
            // 匹配零
            0 => {
                println!("零");
            }
            // 默认情况
            _ => {
                println!("超出范围");
            }
        }
    }

    // 演示 @ 绑定的使用
    process_number(4);
    process_number(50);
    process_number(-25);
    process_number(0);
    process_number(200);

    // 在解构中使用 @ 绑定
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };

    match point {
        Point { x: x_val @ 5..=15, y: y_val @ 15..=25 } => {
            println!("点在目标区域内: ({}, {})", x_val, y_val);
        }
        Point { x, y } => {
            println!("点在目标区域外: ({}, {})", x, y);
        }
    }

    // @ 绑定的优势：
    // 1. 同时匹配和使用值
    // 2. 减少代码重复
    // 3. 提高代码可读性
    // 4. 支持复杂的模式匹配逻辑

    // @ 绑定的最佳实践：
    // 1. 使用有意义的变量名
    // 2. 在需要同时匹配和使用值时使用
    // 3. 保持绑定条件的清晰性
    // 4. 避免过度复杂的绑定表达式

    println!();
}

// ===========================================
// 9. 枚举和所有权 (Enums and Ownership)
// ===========================================

// 枚举与所有权系统密切相关
// 理解枚举的所有权语义对于写出安全的 Rust 代码至关重要

fn enums_and_ownership() {
    println!("=== 枚举和所有权 ===");

    #[derive(Debug)]
    enum Data {
        Text(String),      // 拥有字符串数据
        Number(i32),       // 拥有整数数据
        Boolean(bool),     // 拥有布尔数据
    }

    // 枚举变体拥有数据
    // 当创建枚举实例时，数据被移动到枚举变体中
    let text_data = Data::Text(String::from("Hello, World!"));
    let number_data = Data::Number(42);
    let _boolean_data = Data::Boolean(true);

    // 移动语义：函数获取枚举的所有权
    // 当枚举被传递给函数时，其所有权也会被转移
    fn process_data(data: Data) {
        match data {
            Data::Text(text) => println!("文本数据: {}", text),
            Data::Number(num) => println!("数字数据: {}", num),
            Data::Boolean(b) => println!("布尔数据: {}", b),
        }
    }

    process_data(text_data);
    // process_data(text_data); // 编译错误：text_data 已被移动

    // 借用枚举：使用引用而不是所有权
    // 使用引用可以访问枚举而不获取所有权
    fn display_data(data: &Data) {
        match data {
            Data::Text(text) => println!("借用文本数据: {}", text),
            Data::Number(num) => println!("借用数字数据: {}", num),
            Data::Boolean(b) => println!("借用布尔数据: {}", b),
        }
    }

    display_data(&number_data);
    display_data(&number_data); // 可以多次借用

    // 可变借用：修改枚举中的数据
    fn modify_data(data: &mut Data) {
        match data {
            Data::Text(text) => text.push_str(" (modified)"),
            Data::Number(num) => *num += 1,
            Data::Boolean(b) => *b = !*b,
        }
    }

    let mut mutable_data = Data::Text(String::from("Original"));
    display_data(&mutable_data);
    modify_data(&mut mutable_data);
    display_data(&mutable_data);

    // 枚举的所有权考虑：
    // 1. 枚举变体拥有其数据
    // 2. 移动枚举会移动其数据
    // 3. 借用枚举允许访问而不获取所有权
    // 4. 可变借用允许修改枚举的数据

    // 实际应用中的所有权模式：
    // 1. 优先使用不可变引用进行读取操作
    // 2. 只在必要时使用可变引用进行修改
    // 3. 在需要转移所有权时谨慎使用移动语义
    // 4. 考虑使用 Clone trait 支持复制操作

    println!();
}

// ===========================================
// 10. 枚举实现 (Enum Implementation)
// ===========================================

// 枚举可以实现方法，这为枚举添加了行为
// 实现方法使枚举成为真正的面向对象数据类型

fn enum_implementation() {
    println!("=== 枚举实现 ===");

    #[derive(Debug, Clone)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    // 为枚举实现方法
    impl TrafficLight {
        // 构造函数：创建新的红绿灯实例
        fn new() -> Self {
            TrafficLight::Red
        }

        // 状态转换：获取下一个状态
        fn next(&self) -> Self {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green => TrafficLight::Yellow,
            }
        }

        // 获取持续时间：返回当前状态的持续时间（秒）
        fn duration(&self) -> u32 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 25,
            }
        }

        // 检查是否为停止状态
        fn is_stop(&self) -> bool {
            matches!(self, TrafficLight::Red) // 使用 matches! 宏
        }

        // 获取描述信息
        fn description(&self) -> &'static str {
            match self {
                TrafficLight::Red => "停止",
                TrafficLight::Yellow => "准备",
                TrafficLight::Green => "通行",
            }
        }

        // 获取状态颜色
        fn color(&self) -> &'static str {
            match self {
                TrafficLight::Red => "红色",
                TrafficLight::Yellow => "黄色",
                TrafficLight::Green => "绿色",
            }
        }

        // 检查是否可以通行
        fn can_pass(&self) -> bool {
            matches!(self, TrafficLight::Green)
        }

        // 模拟运行一段时间后的状态
        fn after_duration(&self, seconds: u32) -> Self {
            let mut current = self.clone();
            let mut remaining = seconds;

            while remaining > 0 {
                let duration = current.duration();
                if remaining >= duration {
                    current = current.next();
                    remaining -= duration;
                } else {
                    break;
                }
            }

            current
        }
    }

    // 演示红绿灯系统
    let mut light = TrafficLight::new();
    println!("当前信号灯: {:?} - {} - 颜色: {} - 停止: {} - 可通行: {} - 持续时间: {}秒",
             light, light.description(), light.color(), light.is_stop(), light.can_pass(), light.duration());

    light = light.next();
    println!("下一个信号灯: {:?} - {} - 颜色: {} - 停止: {} - 可通行: {} - 持续时间: {}秒",
             light, light.description(), light.color(), light.is_stop(), light.can_pass(), light.duration());

    light = light.next();
    println!("下一个信号灯: {:?} - {} - 颜色: {} - 停止: {} - 可通行: {} - 持续时间: {}秒",
             light, light.description(), light.color(), light.is_stop(), light.can_pass(), light.duration());

    // 演示时间模拟
    let future_light = TrafficLight::new().after_duration(40);
    println!("40秒后的信号灯: {:?} - {}", future_light, future_light.description());

    // 枚举实现的优势：
    // 1. 封装性：将数据和操作封装在一起
    // 2. 可扩展性：可以轻松添加新的方法和功能
    // 3. 类型安全：编译时检查方法的正确性
    // 4. 可读性：代码清晰表达意图

    // 枚举实现的最佳实践：
    // 1. 保持方法的单一职责
    // 2. 使用有意义的命名
    // 3. 提供完整的 API
    // 4. 考虑实现标准的 trait（如 Debug、Clone、PartialEq）

    println!();
}

// ===========================================
// 11. 枚举和泛型 (Enums and Generics)
// ===========================================

// 泛型枚举是 Rust 中最重要的特性之一
// 它们允许创建可以操作多种类型的通用数据结构

fn enums_and_generics() {
    println!("=== 枚举和泛型 ===");

    // 自定义泛型枚举
    #[derive(Debug)]
    enum MyOption<T> {
        Some(T),
        None,
    }

    #[derive(Debug)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // 自定义泛型枚举
    #[derive(Debug)]
    enum Container<T, E> {
        Item(T),        // 包含任意类型 T 的项
        Error(E),       // 包含任意错误类型 E 的错误
        Empty,          // 空容器
    }

    // 使用泛型枚举
    let item: Container<i32, String> = Container::Item(42);
    let error: Container<i32, String> = Container::Error(String::from("Something went wrong"));
    let empty: Container<i32, String> = Container::Empty;

    println!("泛型容器: {:?} {:?} {:?}", item, error, empty);

    // 为泛型枚举实现方法
    impl<T, E> Container<T, E> {
        // 检查是否包含项
        fn is_item(&self) -> bool {
            matches!(self, Container::Item(_))
        }

        // 检查是否包含错误
        fn is_error(&self) -> bool {
            matches!(self, Container::Error(_))
        }

        // 检查是否为空
        fn is_empty(&self) -> bool {
            matches!(self, Container::Empty)
        }
    }

    println!("Item? {}", item.is_item());
    println!("Error? {}", error.is_error());
    println!("Empty? {}", empty.is_empty());

    // 带约束的泛型枚举：限制泛型类型的范围
    #[derive(Debug)]
    enum Displayable<T: std::fmt::Display> {
        Value(T),
    }

    let displayable = Displayable::Value(42);
    println!("可显示的值: {:?}", displayable);

    // 带约束的泛型枚举实现
    impl<T: std::fmt::Display> Displayable<T> {
        fn display_value(&self) -> String {
            match self {
                Displayable::Value(value) => value.to_string(),
            }
        }
    }

    println!("显示值: {}", displayable.display_value());

    // 更复杂的泛型枚举示例
    #[derive(Debug)]
    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    impl<L, R> Either<L, R> {
        // 检查是否为 Left
        fn is_left(&self) -> bool {
            matches!(self, Either::Left(_))
        }

        // 检查是否为 Right
        fn is_right(&self) -> bool {
            matches!(self, Either::Right(_))
        }

        // 提取 Left 值
        fn left(self) -> MyOption<L> {
            match self {
                Either::Left(value) => MyOption::Some(value),
                Either::Right(_) => MyOption::None,
            }
        }

        // 提取 Right 值
        fn right(self) -> MyOption<R> {
            match self {
                Either::Right(value) => MyOption::Some(value),
                Either::Left(_) => MyOption::None,
            }
        }
    }

    let left_value: Either<String, i32> = Either::Left(String::from("Left value"));
    let right_value: Either<String, i32> = Either::Right(42);

    println!("Left? {} Right? {}", left_value.is_left(), left_value.is_right());
    println!("Left value: {:?}", left_value.left());
    println!("Right value: {:?}", right_value.right());

    // 泛型枚举的优势：
    // 1. 代码重用：一个枚举定义适用于多种类型
    // 2. 类型安全：编译时检查类型错误
    // 3. 灵活性：可以处理各种数据类型
    // 4. 组合性：可以创建复杂的数据结构

    // 泛型枚举的最佳实践：
    // 1. 使用有意义的类型参数名称
    // 2. 添加必要的泛型约束
    // 3. 提供完整的 API
    // 4. 考虑实现标准的 trait

    println!();
}

// ===========================================
// 12. 枚举示例程序 (Enum Example Program)
// ===========================================

// 通过一个完整的 HTTP 状态码处理示例，展示枚举的实际应用
// 这个示例展示了枚举在现实世界编程中的各种用法

fn enum_example_program() {
    println!("=== 枚举示例程序 ===");

    // 定义 HTTP 状态码枚举
    #[derive(Debug, Clone, PartialEq)]
    enum HttpStatus {
        Ok,                         // 200 OK
        NotFound,                   // 404 Not Found
        BadRequest,                 // 400 Bad Request
        InternalServerError,        // 500 Internal Server Error
    }

    // 为 HTTP 状态码实现方法
    impl HttpStatus {
        // 获取状态码
        fn code(&self) -> u16 {
            match self {
                HttpStatus::Ok => 200,
                HttpStatus::BadRequest => 400,
                HttpStatus::NotFound => 404,
                HttpStatus::InternalServerError => 500,
            }
        }

        // 获取状态消息
        fn message(&self) -> &'static str {
            match self {
                HttpStatus::Ok => "OK",
                HttpStatus::BadRequest => "Bad Request",
                HttpStatus::NotFound => "Not Found",
                HttpStatus::InternalServerError => "Internal Server Error",
            }
        }

        // 检查是否为成功状态
        fn is_success(&self) -> bool {
            matches!(self, HttpStatus::Ok)
        }

        // 检查是否为客户端错误
        fn is_client_error(&self) -> bool {
            matches!(self, HttpStatus::BadRequest | HttpStatus::NotFound)
        }

        // 检查是否为服务器错误
        fn is_server_error(&self) -> bool {
            matches!(self, HttpStatus::InternalServerError)
        }

        // 获取状态类别
        fn category(&self) -> &'static str {
            match self {
                HttpStatus::Ok => "Success",
                HttpStatus::BadRequest | HttpStatus::NotFound => "Client Error",
                HttpStatus::InternalServerError => "Server Error",
            }
        }

        // 检查是否应该重试请求
        fn should_retry(&self) -> bool {
            matches!(self, HttpStatus::InternalServerError)
        }

        // 获取建议的用户消息
        fn user_message(&self) -> &'static str {
            match self {
                HttpStatus::Ok => "请求成功",
                HttpStatus::BadRequest => "请求格式错误",
                HttpStatus::NotFound => "资源未找到",
                HttpStatus::InternalServerError => "服务器内部错误",
            }
        }
    }

    // HTTP 响应处理
    let responses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::BadRequest,
        HttpStatus::InternalServerError,
    ];

    // 处理所有响应
    for response in responses {
        println!("HTTP {}: {} - 类别: {} - 成功: {} - 客户端错误: {} - 服务器错误: {} - 应重试: {} - 用户消息: {}",
                 response.code(), response.message(), response.category(),
                 response.is_success(), response.is_client_error(), response.is_server_error(),
                 response.should_retry(), response.user_message());
    }

    // 模拟 HTTP 请求处理
    fn simulate_http_request(url: &str) -> HttpStatus {
        match url {
            "/api/users" => HttpStatus::Ok,
            "/api/users/999" => HttpStatus::NotFound,
            "/api/users/invalid" => HttpStatus::BadRequest,
            "/api/crash" => HttpStatus::InternalServerError,
            _ => HttpStatus::NotFound,
        }
    }

    // 模拟多个请求
    let urls = vec!["/api/users", "/api/users/999", "/api/crash", "/unknown"];

    println!("\n=== 模拟 HTTP 请求 ===");
    for url in urls {
        let status = simulate_http_request(url);
        println!("请求 '{}' -> HTTP {} ({})", url, status.code(), status.message());

        // 根据状态码采取不同的处理逻辑
        match status {
            HttpStatus::Ok => println!("  ✓ 请求成功"),
            HttpStatus::BadRequest => println!("  ⚠ 请求格式错误，请检查输入"),
            HttpStatus::NotFound => println!("  ⚠ 资源不存在"),
            HttpStatus::InternalServerError => {
                println!("  ⚠ 服务器错误，应该重试");
                if status.should_retry() {
                    println!("  → 准备重试请求");
                }
            }
        }
    }

    // 这个示例展示了枚举在实际应用中的关键概念：
    // 1. 类型安全：确保只能使用预定义的状态码
    // 2. 数据封装：状态码和相关功能封装在一起
    // 3. 模式匹配：根据不同的状态码执行不同的逻辑
    // 4. 扩展性：可以轻松添加新的状态码
    // 5. 可读性：代码清晰表达了 HTTP 处理的逻辑

    println!();
}

// ===========================================
// 12. Rust 1.62+ 枚举增强特性（Enhanced Enum Features）
// ===========================================

// Rust 1.62 引入了重要的枚举增强特性，特别是 `#[default]` 属性
// 这些改进使枚举在模式匹配和默认值处理方面更加强大和便捷
// 这些特性大大提升了代码的可读性和维护性

// Rust 1.62+ 枚举增强的核心价值：
// 1. 简化默认值处理：为枚举变体添加默认标记
// 2. 提升代码可读性：使意图更加明确
// 3. 减少样板代码：自动生成默认实现
// 4. 增强类型安全：编译时保证默认值的有效性

fn enhanced_enum_features() {
    println!("=== Rust 1.62+ 枚举增强特性 ===");

    // 1. #[default] 属性（Rust 1.62 新特性）
    // #[default] 属性允许为枚举的某个变体标记为默认值
    // 这在使用 `..` 模式或 `Default` trait 时特别有用

    #[derive(Debug, PartialEq, Default, Copy, Clone)]
    #[repr(u16)]  // 确保底层类型一致，便于默认值处理
    enum HttpStatus {
        #[default]  // 标记为默认变体
        Ok = 200,
        BadRequest = 400,
        NotFound = 404,
        InternalServerError = 500,
    }

    // 使用 #[default] 的好处：
    // - 明确表达设计意图：哪个变体被认为是"默认"状态
    // - 简化模式匹配：可以使用 `..` 通配符匹配默认情况
    // - 自动实现 Default trait：无需手动实现

    // 实现了 Default trait 的枚举可以直接使用 default()
    let default_status = HttpStatus::default();
    println!("默认 HTTP 状态：{:?}", default_status);
    assert_eq!(default_status, HttpStatus::Ok);

    // 2. 模式匹配中的默认值处理
    // #[default] 使模式匹配更加简洁和直观

    fn handle_status(status: HttpStatus) -> &'static str {
        match status {
            HttpStatus::BadRequest => "请求格式错误",
            HttpStatus::NotFound => "资源不存在",
            HttpStatus::InternalServerError => "服务器内部错误",
            _ => "请求成功",  // 匹配剩余情况，包括 #[default] 标记的变体
        }
    }

    // 使用 #[default] 可以更明确地表达意图
    fn handle_status_explicit(status: HttpStatus) -> &'static str {
        match status {
            HttpStatus::BadRequest => "请求格式错误",
            HttpStatus::NotFound => "资源不存在",
            HttpStatus::InternalServerError => "服务器内部错误",
            HttpStatus::Ok => "请求成功",  // 明确匹配默认变体
        }
    }

    let test_statuses = vec![
        HttpStatus::Ok,
        HttpStatus::BadRequest,
        HttpStatus::NotFound,
        HttpStatus::InternalServerError,
    ];

    for status in test_statuses {
        println!("状态 {:?} -> {}", status, handle_status(status));
    }

    // 3. 在结构体中使用带有默认值的枚举
    #[derive(Debug)]
    struct ApiResponse {
        status: HttpStatus,  // 使用带有默认值的枚举
        message: String,
    }

    // 可以利用枚举的默认值来简化结构体创建
    impl Default for ApiResponse {
        fn default() -> Self {
            ApiResponse {
                status: HttpStatus::default(),  // 使用枚举的默认值
                message: String::from("OK"),
            }
        }
    }

    let default_response = ApiResponse::default();
    println!("默认 API 响应：{:?}", default_response);

    // 4. 配置管理中的默认值应用
    #[derive(Debug, PartialEq, Default)]
    enum LogLevel {
        #[default]  // 默认日志级别
        Info,
        Debug,
        Warn,
        Error,
    }

    #[derive(Debug)]
    struct AppConfig {
        log_level: LogLevel,
        max_connections: u32,
    }

    impl Default for AppConfig {
        fn default() -> Self {
            AppConfig {
                log_level: LogLevel::default(),  // 使用默认日志级别
                max_connections: 100,
            }
        }
    }

    // 从环境变量或配置文件中读取配置，如果未设置则使用默认值
    let config = AppConfig::default();
    println!("默认配置：{:?}", config);

    // 5. 状态机中的默认状态
    #[derive(Debug, PartialEq, Default)]
    enum ConnectionState {
        #[default]  // 默认状态
        Disconnected,
        Connecting,
        Connected,
        Disconnecting,
        Error,
    }

    #[derive(Debug)]
    struct NetworkManager {
        state: ConnectionState,
        retry_count: u32,
    }

    impl Default for NetworkManager {
        fn default() -> Self {
            NetworkManager {
                state: ConnectionState::default(),  // 从默认状态开始
                retry_count: 0,
            }
        }
    }

    let manager = NetworkManager::default();
    println!("网络管理器初始状态：{:?}", manager);

    // 6. 在 API 版本管理中的应用
    #[derive(Debug, PartialEq, Default)]
    enum ApiVersion {
        #[default]  // 默认使用最新版本
        V2,
        V1,  // 旧版本
    }

    #[derive(Debug)]
    struct ApiRequest {
        version: ApiVersion,
        endpoint: String,
    }

    impl Default for ApiRequest {
        fn default() -> Self {
            ApiRequest {
                version: ApiVersion::default(),  // 默认使用最新 API 版本
                endpoint: String::from("/"),
            }
        }
    }

    let request = ApiRequest::default();
    println!("默认 API 请求：{:?}", request);

    // #[default] 属性的最佳实践：
    // 1. 明确语义：只在变体确实代表"默认"或"正常"状态时使用
    // 2. 一致性：在整个项目中保持默认值语义的一致性
    // 3. 文档化：在注释中说明为什么选择某个变体作为默认值
    // 4. 兼容性：考虑添加新变体时对默认值选择的影响
    // 5. 测试：确保默认值的行为符合预期

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 枚举演示");
    println!("==============");

    basic_enums();
    enums_with_data();
    option_enum();
    result_enum();
    enum_pattern_matching();
    wildcard_patterns();
    match_guards();
    at_binding();
    enums_and_ownership();
    enum_implementation();
    enums_and_generics();
    enum_example_program();
    enhanced_enum_features();

    println!("枚举演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 Option 处理
    #[test]
    fn test_option_handling() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        assert_eq!(plus_one(Some(5)), Some(6));
        assert_eq!(plus_one(None), None);
    }

    // 测试 Result 处理
    #[test]
    fn test_result_handling() {
        fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
            if denominator == 0.0 {
                Err(String::from("Cannot divide by zero"))
            } else {
                Ok(numerator / denominator)
            }
        }

        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(10.0, 0.0), Err("Cannot divide by zero".to_string()));
    }

    // 测试交通信号灯系统
    #[test]
    fn test_traffic_light() {
        #[derive(Debug, Clone)]
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }

        impl TrafficLight {
            fn next(&self) -> Self {
                match self {
                    TrafficLight::Red => TrafficLight::Green,
                    TrafficLight::Yellow => TrafficLight::Red,
                    TrafficLight::Green => TrafficLight::Yellow,
                }
            }

            fn is_stop(&self) -> bool {
                matches!(self, TrafficLight::Red)
            }
        }

        let red = TrafficLight::Red;
        let green = red.next();
        let yellow = green.next();
        let red_again = yellow.next();

        assert!(red.is_stop());
        assert!(!green.is_stop());
        assert!(!yellow.is_stop());
        assert!(red_again.is_stop());
    }

    // 测试 HTTP 状态码
    #[test]
    fn test_http_status() {
        #[derive(Debug, PartialEq)]
        enum HttpStatus {
            Ok,
            NotFound,
            BadRequest,
            InternalServerError,
        }

        impl HttpStatus {
            fn code(&self) -> u16 {
                match self {
                    HttpStatus::Ok => 200,
                    HttpStatus::BadRequest => 400,
                    HttpStatus::NotFound => 404,
                    HttpStatus::InternalServerError => 500,
                }
            }

            fn is_success(&self) -> bool {
                matches!(self, HttpStatus::Ok)
            }
        }

        assert_eq!(HttpStatus::Ok.code(), 200);
        assert!(HttpStatus::Ok.is_success());
        assert!(!HttpStatus::NotFound.is_success());
    }

    // 测试枚举模式匹配
    #[test]
    fn test_enum_pattern_matching() {
        #[derive(Debug)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Quarter), 25);
    }

    // 测试带数据的枚举
    #[test]
    fn test_data_enums() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }

        impl Message {
            fn is_move(&self) -> bool {
                matches!(self, Message::Move { .. })
            }
        }

        let move_msg = Message::Move { x: 10, y: 20 };
        let quit_msg = Message::Quit;

        assert!(move_msg.is_move());
        assert!(!quit_msg.is_move());
    }

    // 测试 @ 绑定
    #[test]
    fn test_at_binding() {
        #[derive(Debug)]
        enum Value {
            Number(i32),
        }

        let value = Value::Number(5);

        let result = match value {
            Value::Number(num @ 1..=10) => format!("小数字: {}", num),
            Value::Number(num @ 11..=100) => format!("中数字: {}", num),
            Value::Number(num) => format!("大数字: {}", num),
        };

        assert_eq!(result, "小数字: 5");
    }

    // 测试匹配守卫
    #[test]
    fn test_match_guards() {
        let some_value = Some(5);

        let result = match some_value {
            Some(x) if x > 3 => "大于3",
            Some(x) if x > 0 => "大于0",
            Some(_) => "其他Some值",
            None => "None",
        };

        assert_eq!(result, "大于3");
    }

    // 测试泛型枚举
    #[test]
    fn test_generic_enums() {
        #[derive(Debug, PartialEq)]
        enum Container<T> {
            Some(T),
            None,
        }

        let container = Container::Some(42);
        assert!(matches!(container, Container::Some(_)));
    }

    #[test]
    fn test_rust_162_default_attribute() {
        #[derive(Debug, PartialEq, Default)]
        #[repr(u16)]
        enum HttpStatus {
            #[default]
            Ok = 200,
            BadRequest = 400,
            NotFound = 404,
            InternalServerError = 500,
        }

        // 测试默认值实现
        let default_status = HttpStatus::default();
        assert_eq!(default_status, HttpStatus::Ok);

        // 测试模式匹配中的默认行为
        fn handle_status(status: HttpStatus) -> &'static str {
            match status {
                HttpStatus::BadRequest => "请求格式错误",
                HttpStatus::NotFound => "资源不存在",
                HttpStatus::InternalServerError => "服务器内部错误",
                _ => "请求成功",
            }
        }

        assert_eq!(handle_status(HttpStatus::Ok), "请求成功");
        assert_eq!(handle_status(HttpStatus::BadRequest), "请求格式错误");
    }

    #[test]
    fn test_default_enum_in_structs() {
        #[derive(Debug, PartialEq, Default)]
        enum LogLevel {
            #[default]
            Info,
            Debug,
            Warn,
            Error,
        }

        #[derive(Debug, PartialEq)]
        struct AppConfig {
            log_level: LogLevel,
            max_connections: u32,
        }

        impl Default for AppConfig {
            fn default() -> Self {
                AppConfig {
                    log_level: LogLevel::default(),
                    max_connections: 100,
                }
            }
        }

        let config = AppConfig::default();
        assert_eq!(config.log_level, LogLevel::Info);
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_state_machine_with_default() {
        #[derive(Debug, PartialEq, Default)]
        enum ConnectionState {
            #[default]
            Disconnected,
            Connecting,
            Connected,
            Disconnecting,
            Error,
        }

        #[derive(Debug)]
        struct NetworkManager {
            state: ConnectionState,
            retry_count: u32,
        }

        impl Default for NetworkManager {
            fn default() -> Self {
                NetworkManager {
                    state: ConnectionState::default(),
                    retry_count: 0,
                }
            }
        }

        let manager = NetworkManager::default();
        assert_eq!(manager.state, ConnectionState::Disconnected);
        assert_eq!(manager.retry_count, 0);
    }
}