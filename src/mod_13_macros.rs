#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Rust 宏系统
// 深入讲解 Rust 的元编程能力，包括声明式宏、过程宏和高级宏技巧
//
// Rust 的宏系统是编译时代码生成的强大工具，它允许开发者：
// - 减少重复代码（DRY 原则）
// - 创建领域特定语言（DSL）
// - 实现编译时计算和验证
// - 生成标准化的代码模式
// - 提供类型安全的抽象
//
// 宏的分类：
// 1. 声明式宏（Declarative Macros）：使用 macro_rules! 定义的模式匹配宏
// 2. 过程宏（Procedural Macros）：使用 Rust 代码生成 Rust 代码的宏
// 3. 派生宏（Derive Macros）：自动为类型实现 trait
// 4. 属性宏（Attribute Macros）：修改或扩展项目属性
// 5. 函数宏（Function-like Macros）：类似函数调用的宏

// ===========================================
// 1. 声明式宏基础 (Declarative Macros Basics)
// ===========================================

// 声明式宏是 Rust 中最基础的宏形式，使用 macro_rules! 宏来定义
// 它们通过模式匹配和代码替换来工作，类似于 C/C++ 的宏但更安全和强大
// 声明式宏在编译时展开，不产生运行时开销
//
// 声明式宏的核心概念：
// - 模式匹配：根据输入的语法结构选择匹配的模式
// - 元变量（Metavariables）：捕获和重用输入的代码片段
// - 代码生成：根据模式生成相应的 Rust 代码
// - 卫生性（Hygiene）：避免变量名冲突和作用域污染
//
// 元变量类型说明：
// - $item：项目（函数、结构体等）
// - $block：代码块
// - $stmt：语句
// - $pat：模式
// - $expr：表达式
// - $ty：类型
// - $ident：标识符
// - $path：路径
// - $tt：标记树（Token Tree）
// - $meta：元数据
// - $lifetime：生命周期
// - $vis：可见性
// - $literal：字面量

fn declarative_macros_basics() {
    println!("=== 声明式宏基础 ===");

    // 基本宏定义：最简单的宏形式
    // 宏定义使用 macro_rules! 关键字，后跟宏名称和匹配规则
    // 宏名称通常使用蛇形命名法（snake_case）
    macro_rules! say_hello {
        // 匹配规则：() 表示匹配空的输入
        () => {
            // 生成的代码：简单的 println! 调用
            println!("Hello, world!");
        };
    }

    say_hello!();

    // 带参数的宏：使用元变量捕获输入
    // $name:expr 表示匹配一个表达式，并将其捕获为名为 name 的元变量
    macro_rules! greet {
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
    }

    greet!("Alice");
    greet!("Bob");

    // 多个参数的宏：可以捕获多个元变量
    // 每个参数都需要有自己的元变量名称
    macro_rules! add {
        ($a:expr, $b:expr) => {
            $a + $b
        };
    }

    let result = add!(5, 3);
    println!("5 + 3 = {}", result);

    // 可变参数的宏：使用重复模式
    // $($arg:expr),* 表示匹配零个或多个由逗号分隔的表达式
    // - $arg：元变量名称
    // - expr：元变量类型（表达式）
    // - ,：分隔符
    // - *：重复器（零次或多次）
    macro_rules! print_all {
        ($($arg:expr),*) => {
            $(
                println!("Item: {}", $arg);
            )*
        };
    }

    print_all!(1, "hello", 3.14, true);

    // 声明式宏的优势和注意事项：
    // 优势：
    // 1. 编译时展开，无运行时开销
    // 2. 可以处理任意类型的参数
    // 3. 支持复杂的模式匹配
    // 4. 提供更好的错误信息
    //
    // 注意事项：
    // 1. 调试困难，因为展开的代码可能很复杂
    // 2. 过度使用会降低代码可读性
    // 3. 需要考虑宏的卫生性和作用域问题
    // 4. 复杂的宏可能影响编译时间

    println!();
}

// ===========================================
// 2. 宏模式匹配 (Macro Pattern Matching)
// ===========================================

// 宏模式匹配是声明式宏的核心特性，它允许宏根据输入的语法结构选择不同的代码生成规则
// 这种模式匹配比函数重载更灵活，可以基于语法模式而非类型进行分发
//
// 模式匹配的规则：
// 1. 自上而下匹配：宏会按照定义的顺序尝试匹配输入
// 2. 最先匹配：第一个成功匹配的模式会被使用
// 3. 完全匹配：模式必须完全匹配输入，不能有剩余未匹配的 token
// 4. 回溯：如果一个模式匹配失败，宏会尝试下一个模式
//
// 模式匹配的优势：
// - 基于语法结构而非类型的分发
// - 支持可变参数和可选参数
// - 可以创建 DSL（领域特定语言）
// - 编译时确定匹配结果，无运行时开销

fn macro_pattern_matching() {
    println!("=== 宏模式匹配 ===");

    // 多重分发：根据参数数量选择不同的实现
    // 这是函数重载的宏版本，可以根据参数数量生成不同的代码
    macro_rules! create_point {
        // 匹配两个参数，创建 2D 点
        ($x:expr, $y:expr) => {
            Point { x: $x, y: $y }
        };
        // 匹配三个参数，创建 3D 点
        ($x:expr, $y:expr, $z:expr) => {
            Point3D {
                x: $x,
                y: $y,
                z: $z,
            }
        };
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let p2 = create_point!(10, 20); // 匹配第一个模式
    let p3 = create_point!(10, 20, 30); // 匹配第二个模式
    println!("2D点: {:?}", p2);
    println!("3D点: {:?}", p3);

    // 基于语法的模式匹配：创建迷你 DSL
    // 这个宏展示了如何基于语法结构而非类型进行分发
    macro_rules! operation {
        // 匹配 "add" 关键字后跟两个表达式
        (add $a:expr, $b:expr) => {
            $a + $b
        };
        // 匹配 "sub" 关键字后跟两个表达式
        (sub $a:expr, $b:expr) => {
            $a - $b
        };
        // 匹配 "mul" 关键字后跟两个表达式
        (mul $a:expr, $b:expr) => {
            $a * $b
        };
        // 匹配 "div" 关键字后跟两个表达式
        (div $a:expr, $b:expr) => {
            $a / $b
        };
    }

    println!("操作结果:");
    println!("  add: {}", operation!(add 10, 5));
    println!("  sub: {}", operation!(sub 10, 5));
    println!("  mul: {}", operation!(mul 10, 5));
    println!("  div: {}", operation!(div 10, 5));

    // 复杂模式匹配示例：条件编译宏
    // 注意：这个宏在表达式中使用#[cfg]属性会导致编译错误，暂时注释掉
    /*
    macro_rules! conditional_operation {
        (debug $op:ident($($args:expr),*)) => {
            #[cfg(debug_assertions)]
            {
                println!("Debug: 执行 {}", stringify!($op));
                $op($($args),*)
            }
            #[cfg(not(debug_assertions))]
            {
                $op($($args),*)
            }
        };
        (release $op:ident($($args:expr),*)) => {
            #[cfg(not(debug_assertions))]
            {
                $op($($args),*)
            }
            #[cfg(debug_assertions)]
            {
                println!("Release: 跳过 {}", stringify!($op));
                0
            }
        };
    }
    */

    fn calculate_sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // 注释掉使用条件编译宏的代码，因为使用了不稳定的特性
    /*
    let debug_result = conditional_operation!(debug calculate_sum(5, 3));
    let release_result = conditional_operation!(release calculate_sum(5, 3));
    println!("调试结果: {}", debug_result);
    println!("发布结果: {}", release_result);
    */

    // 提供一个简单的替代实现
    let simple_result = calculate_sum(5, 3);
    println!("简单调用结果: {}", simple_result);

    // 模式匹配的最佳实践：
    // 1. 从具体到一般：将更具体的模式放在前面
    // 2. 避免歧义：确保模式之间不会意外重叠
    // 3. 提供默认模式：考虑提供一个兜底的匹配规则
    // 4. 错误处理：在无法匹配时提供清晰的错误信息
    // 5. 性能考虑：复杂模式匹配可能影响编译时间

    println!();
}

// ===========================================
// 3. 重复模式 (Repetition Patterns)
// ===========================================

// 重复模式是 Rust 宏中最强大的特性之一，它允许宏处理可变数量的参数
// 通过重复模式，可以创建类似 vec!、println! 这样的标准库宏
//
// 重复模式的语法：
// - $(...)*：匹配零次或多次重复
// - $(...)+：匹配一次或多次重复
// - $(...)?：匹配零次或一次重复（可选）
// - $(...),*：由逗号分隔的重复
// - $(...);*：由分号分隔的重复
//
// 重复模式的组成部分：
// 1. 匹配内容：要重复的模式
// 2. 分隔符：可选的分隔符（如逗号、分号）
// 3. 重复器：指定重复次数（*、+、?）
// 4. 元变量：在重复中捕获的变量

fn repetition_patterns() {
    println!("=== 重复模式 ===");

    // 基本重复模式：创建类似 vec! 的宏
    // 这个宏展示了如何使用重复模式创建向量
    macro_rules! create_vector {
        ($($x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                // $() 中的代码会对每个匹配的元素重复执行
                $(
                    temp_vec.push($x);  // 对每个表达式调用 push
                )*
                temp_vec
            }
        };
    }

    let v = create_vector!(1, 2, 3, 4, 5);
    println!("创建的向量: {:?}", v);

    // 复杂重复模式：函数多次调用
    // 这个宏接受一个函数表达式、调用次数和可变参数
    macro_rules! call_multiple_times {
        ($func:expr, $count:expr, $($arg:expr),*) => {
            for _ in 0..$count {
                // 将可变参数传递给函数
                $func($($arg),*);
            }
        };
    }

    fn print_sum(a: i32, b: i32) {
        println!("Sum: {}", a + b);
    }

    call_multiple_times!(print_sum, 3, 10, 20);

    // 嵌套重复模式：创建矩阵
    // 这个宏展示了如何使用嵌套重复模式创建二维结构
    macro_rules! create_matrix {
        // 外层重复：匹配由分号分隔的行
        ($($($x:expr),*);*) => {
            {
                let mut matrix = Vec::new();
                $(
                    // 内层重复：匹配由逗号分隔的列
                    let mut row = Vec::new();
                    $(
                        row.push($x);
                    )*
                    matrix.push(row);
                )*
                matrix
            }
        };
    }

    let matrix = create_matrix!(1, 2, 3; 4, 5, 6; 7, 8, 9);
    println!("创建的矩阵: {:?}", matrix);

    // 高级重复模式：带有条件的重复
    macro_rules! debug_print_all {
        ($($arg:expr),*) => {
            $(
                #[cfg(debug_assertions)]
                println!("Debug: {} = {:?}", stringify!($arg), $arg);
            )*
        };
    }

    debug_print_all!(42, "hello", vec![1, 2, 3]);

    // 重复模式与标识符生成
    macro_rules! create_fields {
        ($($field:ident: $type:ty),*) => {
            struct Data {
                $(
                    $field: $type,
                )*
            }
        };
    }

    create_fields!(name: String, age: u32, email: String);
    // 注意：这个宏会创建一个名为 Data 的结构体

    // 重复模式的最佳实践：
    // 1. 明确分隔符：使用清晰的分隔符避免歧义
    // 2. 限制复杂度：过度嵌套的重复模式难以维护
    // 3. 错误处理：考虑参数验证和错误消息
    // 4. 性能考虑：大量重复可能影响编译时间
    // 5. 可读性：为复杂的重复模式添加注释

    // 重复模式的常见用例：
    // 1. 数据结构创建（vec!、HashMap!）
    // 2. 函数调用链（println!、format!）
    // 3. 代码生成（impl blocks、trait implementations）
    // 4. 条件编译（#[cfg]属性的重复应用）
    // 5. 测试生成（生成多个测试用例）

    println!();
}

// ===========================================
// 4. 宏卫生 (Macro Hygiene)
// ===========================================

// 宏卫生（Macro Hygiene）是 Rust 宏系统的重要特性，它确保宏不会意外地与外部代码产生命名冲突
// 这与 C/C++ 的宏形成鲜明对比，后者的文本替换可能导致各种命名冲突问题
//
// 宏卫生的核心原则：
// 1. 作用域隔离：宏内部定义的变量不会污染外部作用域
// 2. 标识符唯一性：宏生成的标识符具有唯一的语法上下文
// 3. 局部性：宏参数和局部变量只在宏展开范围内有效
// 4. 透明性：宏的使用者不需要关心内部实现细节
//
// Rust 实现宏卫生的方式：
// - 语法上下文（Syntax Contexts）：每个标识符都有其来源信息
// - 局部变量作用域：宏内部变量不会影响外部同名变量
// - 宏参数隔离：宏参数在展开时保持其卫生性

fn macro_hygiene() {
    println!("=== 宏卫生 ===");

    // 演示1：宏内部变量不会污染外部作用域
    // 即使宏内部定义了与外部同名的变量，也不会发生冲突
    macro_rules! scoped_var {
        () => {
            // 这个 x 是宏内部的局部变量，与外部的 x 无关
            let x = 42;
            println!("宏内 x = {}", x);
        };
    }

    let x = 100;
    println!("宏外 x = {}", x);
    scoped_var!();
    println!("宏后 x = {}", x); // 外部的 x 保持不变，不受宏影响

    // 演示2：宏参数的卫生性
    // 宏参数在展开时会保持其卫生性，不会与外部变量冲突
    macro_rules! print_double {
        ($x:expr) => {
            // 这个 x 是新的局部变量，不会影响外部的 x
            let x = $x * 2;
            println!("Double: {}", x);
        };
    }

    let x = 5;
    print_double!(x);
    println!("Original x = {}", x); // 外部的 x 保持不变

    // 演示3：宏卫生与块表达式
    // 宏展开的代码在语法上等同于手写的代码
    macro_rules! safe_increment {
        ($x:expr) => {{
            let temp = $x;
            temp + 1
        }};
    }

    let value = 10;
    let result = safe_increment!(value);
    println!("安全递增: {} -> {}", value, result);

    // 演示4：卫生性与标识符生成
    // 使用 $crate 来引用宏定义所在的 crate，确保路径的卫生性
    // 注意：这个宏在表达式中定义结构体会导致编译错误，暂时注释掉
    /*
    macro_rules! create_logger {
        () => {
            // 使用 $crate 确保引用正确的模块
            struct Logger {
                name: String,
            }

            impl Logger {
                fn new(name: &str) -> Self {
                    Logger {
                        name: name.to_string(),
                    }
                }

                fn log(&self, message: &str) {
                    println!("[{}] {}", self.name, message);
                }
            }

            Logger::new("DefaultLogger")
        };
    }

    let logger = create_logger!();
    logger.log("这是一条日志消息");
    */

    // 提供一个简单的替代实现
    struct Logger {
        name: String,
    }

    impl Logger {
        fn new(name: &str) -> Self {
            Logger {
                name: name.to_string(),
            }
        }

        fn log(&self, message: &str) {
            println!("[{}] {}", self.name, message);
        }
    }

    let logger = Logger::new("DefaultLogger");
    logger.log("这是一条日志消息");

    // 演示5：卫生性的局限性
    // 某些情况下，卫生性可能不够，需要额外的注意
    macro_rules! risky_macro {
        () => {
            // 这个宏可能影响外部的同名变量
            // 虽然在卫生性上是安全的，但可能在语义上造成混淆
            let counter = 100;
            println!("宏内创建的 counter: {}", counter);
        };
    }

    let counter = 50;
    risky_macro!();
    println!("外部的 counter: {}", counter);

    // 宏卫生的最佳实践：
    // 1. 避免使用通用的变量名：使用具有描述性的名称
    // 2. 使用块表达式：将宏展开的代码包装在块中
    // 3. 谨慎使用标识符：避免可能与外部冲突的标识符
    // 4. 考虑 $crate：在跨 crate 使用时使用 $crate
    // 5. 文档化行为：清楚说明宏的副作用和卫生性保证

    // 与 C/C++ 宏的对比：
    // C/C++ 宏：简单的文本替换，容易产生命名冲突
    // Rust 宏：语法级别的转换，具有卫生性保证
    // 这使得 Rust 宏更加安全和可预测

    println!();
}

// ===========================================
// 5. 过程宏基础 (Procedural Macros Basics)
// ===========================================

// 过程宏（Procedural Macros）是 Rust 中更高级的宏形式
// 它们允许在编译时以编程方式生成代码，而不是简单的模式匹配和替换
// 过程宏是 Rust 元编程能力的核心，使得编译时代码生成成为可能

fn procedural_macros_basics() {
    println!("=== 过程宏基础 ===");

    // 过程宏的类型和用途
    // 过程宏分为三种主要类型，每种都有其特定的应用场景：
    // 1. 派生宏（Derive Macros）：#[derive(...)] 形式，为结构体和枚举自动实现 trait
    // 2. 属性宏（Attribute Macros）：#[attr(...)] 形式，修改或增强项的行为
    // 3. 函数宏（Function-like Macros）：macro!(...) 形式，类似函数调用的宏

    // 内置派生宏：Rust 标准库提供的常用派生宏
    // 这些宏为类型提供了常用的功能实现，是 Rust 生态的基础
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Debug：自动生成格式化输出的实现，用于调试和打印
    // Clone：创建值的深拷贝，对于包含堆数据的类型很重要
    // PartialEq：提供相等性比较的能力
    let person_clone = person.clone();
    println!("原始: {:?}", person);
    println!("克隆: {:?}", person_clone);
    println!("相等: {}", person == person_clone);

    // 派生宏的工作原理：
    // 1. 编译器在编译时遇到 #[derive(...)]
    // 2. 调用相应的过程宏函数，传入被修饰项的语法树（TokenStream）
    // 3. 过程宏分析语法树，生成新的代码（也是 TokenStream）
    // 4. 编译器将生成的代码插入到程序中进行编译

    // 属性宏的示例：编译器内置的属性宏
    // 属性宏可以修改代码的行为或提供元信息
    #[allow(unused_variables)] // 允许未使用的变量，避免编译器警告
    let x = 42;

    // 函数宏的示例：编译器内置的函数宏
    // 函数宏在编译时执行，可以生成代码或执行计算
    let s = stringify!(hello world); // 将输入转换为字符串字面量
    println!("字符串化: {}", s);

    // 编译时信息宏：提供编译时的元信息
    // 这些宏在编译时展开，提供源码位置信息
    let file = file!(); // 当前文件名
    let line = line!(); // 当前行号
    let column = column!(); // 当前列号
    println!("文件: {}, 行: {}", file, line);

    // 编译时环境宏：访问编译时环境
    // 这些宏提供编译时的上下文信息
    println!("当前模块: {}", module_path!()); // 当前模块路径
    println!("类型名: {}", std::any::type_name::<i32>()); // 类型名称

    // 过程宏的编译时特性：
    // 1. 编译时执行：过程宏在编译时运行，不会增加运行时开销
    // 2. 语法分析：可以访问和解析完整的语法树
    // 3. 代码生成：可以生成任意的 Rust 代码
    // 4. 错误报告：可以在编译时生成自定义的错误信息

    // 与声明式宏的对比：
    // 声明式宏：基于模式匹配，适用于简单的代码替换
    // 过程宏：基于语法树操作，适用于复杂的代码生成和元编程

    // 过程宏的应用场景：
    // 1. 序列化/反序列化：自动实现数据转换
    // 2. 依赖注入：自动生成依赖关系代码
    // 3. 代码生成：根据数据结构生成相关代码
    // 4. 领域特定语言（DSL）：创建特定领域的语言

    println!();
}

// ===========================================
// 6. 自定义派生宏 (Custom Derive Macros)
// ===========================================

// 自定义派生宏是过程宏最常见和最有用的形式之一
// 它允许为自定义类型自动实现 trait，减少样板代码，提高开发效率
// 许多流行的 Rust 库都使用自定义派生宏来简化 API 使用

fn custom_derive_macros() {
    println!("=== 自定义派生宏 ===");

    // 自定义派生宏的实际应用场景
    // 自定义派生宏广泛应用于以下领域：
    // 1. 序列化框架（如 serde）：自动实现序列化和反序列化
    // 2. 数据库 ORM：自动生成数据库操作代码
    // 3. Web 框架：自动生成路由和处理器代码
    // 4. 验证库：自动生成数据验证逻辑
    // 5. 测试框架：自动生成测试代码

    // 定义一个自定义 trait 来演示派生宏的功能
    // 这个 trait 代表可以生成摘要的类型
    trait Summary {
        fn summarize(&self) -> String;
    }

    // 手动实现 trait 的方式（传统方法）
    // 虽然可行，但对于大量类型来说很繁琐
    #[derive(Debug)]
    struct Article {
        title: String,
        author: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            // 手动编写摘要逻辑
            format!("{} by {}", self.title, self.author)
        }
    }

    let article = Article {
        title: "Rust Programming".to_string(),
        author: "Alice".to_string(),
    };

    println!("文章摘要: {}", article.summarize());

    // 模拟自定义派生宏的功能
    // 在实际的项目中，我们需要创建一个过程宏 crate
    // 这里用声明式宏来模拟派生宏的行为
    macro_rules! impl_summary {
        ($struct_name:ident) => {
            // 自动为指定类型实现 Summary trait
            impl Summary for $struct_name {
                fn summarize(&self) -> String {
                    format!("自定义摘要: {:?}", self)
                }
            }
        };
    }

    // 使用模拟的派生宏
    #[derive(Debug)]
    struct Book {
        title: String,
        pages: u32,
    }

    // 调用宏来自动实现 trait
    impl_summary!(Book);

    let book = Book {
        title: "The Rust Book".to_string(),
        pages: 500,
    };

    println!("图书摘要: {}", book.summarize());

    // 真实的自定义派生宏结构
    // 一个真实的派生宏项目通常包含：
    // 1. 过程宏 crate（名称以 -proc-macro 结尾）
    // 2. 宏定义文件（lib.rs）
    // 3. 主要逻辑函数
    /*
    // 以下是真实派生宏的代码结构：
    // 在 proc-macro crate 中：
    use proc_macro::TokenStream;
    use quote::quote;
    use syn;

    #[proc_macro_derive(Summary)]
    pub fn summary_derive(input: TokenStream) -> TokenStream {
        // 解析输入的语法树
        let ast = syn::parse(input).unwrap();

        // 生成实现代码
        impl_summary(&ast)
    }
    */

    // 派生宏的高级特性
    // 真实的派生宏可以：
    // 1. 访问结构体的字段信息
    // 2. 根据字段类型生成不同的代码
    // 3. 处理泛型和生命周期参数
    // 4. 生成复杂的实现逻辑
    // 5. 提供配置选项（通过属性）

    // 派生宏的最佳实践：
    // 1. 提供合理的默认行为
    // 2. 允许通过属性进行自定义
    // 3. 处理错误情况并提供有用的错误信息
    // 4. 编写良好的文档和示例
    // 5. 考虑性能影响（编译时开销）

    // 派生宏的实际案例：
    // 1. serde 的 Serialize/Deserialize：自动处理复杂数据结构的序列化
    // 2. diesel 的 Associations：自动处理数据库关联关系
    // 3. actix-web 的 FromRequest：自动处理 HTTP 请求解析
    // 4. thiserror 的 Error：自动实现错误类型

    println!();
}

// ===========================================
// 7. 属性宏 (Attribute Macros)
// ===========================================

// 属性宏是过程宏的一种，可以修改或增强项的行为
// 属性宏可以应用于函数、结构体、枚举、模块等各种项
// 它们是 Rust 元编程的重要组成部分，提供了强大的代码修改能力

fn attribute_macros() {
    println!("=== 属性宏 ===");

    // 属性宏的类型和应用场景
    // 属性宏分为内置属性宏和自定义属性宏：
    // 1. 内置属性宏：编译器提供的，如 #[derive]、#[allow]、#[inline] 等
    // 2. 自定义属性宏：用户自定义的，如 #[route]、#[test] 等

    // 内置属性宏：调试相关
    // Debug 和 Clone 是最常用的派生属性
    #[derive(Debug, Clone)]
    struct Data {
        value: i32,
    }

    // 内置属性宏：性能优化相关
    // 这些属性宏帮助编译器优化代码性能
    #[inline] // 建议编译器内联此函数，减少函数调用开销
    fn always_inline(x: i32) -> i32 {
        x + 1
    }

    #[cold] // 告诉编译器此路径很少执行，优化分支预测
    fn unlikely_path() {
        println!("这条路径很少执行");
    }

    // 内置属性宏：测试相关
    // 测试框架使用属性宏来标识测试函数
    #[cfg(test)] // 只在测试模式下编译此模块
    mod tests {
        #[test] // 标识这是一个测试函数
        fn test_example() {
            assert!(true);
        }
    }

    // 内置属性宏：条件编译
    // 条件编译允许根据不同的编译条件包含或排除代码
    #[cfg(debug_assertions)] // 只在调试模式下编译
    const DEBUG_MODE: bool = true;

    #[cfg(not(debug_assertions))] // 只在非调试模式下编译
    const DEBUG_MODE: bool = false;

    println!("调试模式: {}", DEBUG_MODE);

    // 内置属性宏：文档生成
    // 文档注释会被转换为属性，用于生成 HTML 文档
    /// 这是一个示例函数
    /// # 参数
    /// * `x` - 输入值
    /// # 返回值
    /// 返回输入值的两倍
    /// # 示例
    /// ```
    /// fn example_function(x: i32) -> i32 {
    ///     x * 2
    /// }
    /// let result = example_function(5);
    /// assert_eq!(result, 10);
    /// ```
    fn example_function(x: i32) -> i32 {
        x * 2
    }

    let result = example_function(5);
    println!("示例函数结果: {}", result);

    // 属性宏的语法和工作原理：
    // 1. 语法：#[attr] 或 #[attr(arg)] 或 #[attr(key = value)]
    // 2. 应用：可以应用到各种语法项上
    // 3. 处理：在编译时由编译器或过程宏处理
    // 4. 效果：修改代码行为或提供元信息

    // 自定义属性宏的实际应用：
    // 1. Web 框架：#[route("/api/users")] 自动生成路由处理
    // 2. 序列化框架：#[serde(rename = "username")] 控制序列化名称
    // 3. 数据库 ORM：#[primary_key] 标识主键字段
    // 4. 异步运行时：#[tokio::main] 自动生成异步主函数
    // 5. 测试框架：#[tokio::test] 自动生成异步测试

    // 属性宏的高级特性：
    // 1. 可以接受参数：#[attr(arg1, arg2, key = value)]
    // 2. 可以嵌套：#[outer_attr(inner_attr(...))]
    // 3. 可以有条件地应用：#[cfg(target_os = "linux")]
    // 4. 可以修改语法树：添加、删除、修改代码元素

    // 属性宏的最佳实践：
    // 1. 提供清晰的文档说明
    // 2. 使用合理的默认值
    // 3. 处理错误情况并提供有用的错误信息
    // 4. 避免过度使用，保持代码可读性
    // 5. 考虑编译时性能影响

    // 常用的内置属性宏分类：
    // 1. 条件编译：#[cfg(...)], #[cfg_attr(...)]
    // 2. 调试：#[derive(Debug)], #[allow(...)]
    // 3. 优化：#[inline], #[cold], #[no_mangle]
    // 4. 测试：#[test], #[should_panic]
    // 5. 文档：#[doc(...)], #[doc(alias = "...")]
    // 6. 导出：#[export_name], #[no_link]

    println!();
}

// ===========================================
// 8. 函数宏 (Function-like Macros)
// ===========================================

// 函数宏（Function-like Macros）是过程宏的一种，其调用形式类似于函数调用
// 它们提供了比声明式宏更强大的能力，可以进行复杂的语法分析和代码生成
// 函数宏在 Rust 生态中有着广泛的应用

fn function_like_macros() {
    println!("=== 函数宏 ===");

    // 函数宏的特点和优势
    // 函数宏相比声明式宏具有以下优势：
    // 1. 更强大的语法分析能力：可以解析完整的语法树
    // 2. 更好的错误处理：可以提供有意义的编译时错误信息
    // 3. 更灵活的参数处理：可以处理复杂的参数结构
    // 4. 更好的 IDE 支持：IDE 可以提供更好的代码补全和检查

    // 内置函数宏：标准库提供的常用函数宏
    // 这些宏在日常编程中经常使用
    let vec = vec![1, 2, 3]; // 创建并初始化向量
    println!("向量宏: {:?}", vec);

    let format_str = format!("Hello {}!", "world"); // 格式化字符串
    println!("格式化宏: {}", format_str);

    // 条件编译宏：控制代码的编译
    // 使用声明式宏实现条件编译功能
    macro_rules! conditional_compile {
        ($($item:item)*) => {
            $(
                #[cfg(debug_assertions)] // 只在调试模式下编译这些项
                $item
            )*
        };
    }

    conditional_compile! {
        fn debug_function() {
            println!("只在调试模式下编译");
        }
    }

    // 编译时字符串操作宏
    // 这些宏在编译时执行字符串操作
    let concatenated = concat!("Hello", " ", "World", "!"); // 编译时字符串拼接
    println!("拼接字符串: {}", concatenated);

    // 环境变量访问宏
    // 安全地访问编译时环境变量
    if let Some(home) = option_env!("HOME") {
        // 返回 Option<&str>
        println!("HOME 目录: {}", home);
    } else {
        println!("未找到 HOME 环境变量");
    }

    // 模块路径宏：获取编译时信息
    // 这些宏提供源码位置和路径信息
    println!("当前模块: {}", module_path!()); // 当前模块路径
    println!("当前文件: {}", file!()); // 当前文件名
    println!("当前行号: {}", line!()); // 当前行号
    println!("当前列号: {}", column!()); // 当前列号

    // 函数宏的语法和调用形式：
    // 1. 基本语法：macro_name!(token_stream)
    // 2. 可以接受任意数量的参数
    // 3. 可以有复杂的参数结构
    // 4. 在编译时展开并执行

    // 函数宏的应用场景：
    // 1. 字符串格式化：format!, write!, println!
    // 2. 数据结构创建：vec!, hashmap!, BTreeMap!
    // 3. 编译时信息获取：file!, line!, module_path!
    // 4. 环境变量访问：env!, option_env!
    // 5. 条件编译：cfg!, include!, concat!

    // 自定义函数宏的优势：
    // 1. 类型安全：可以提供类型检查
    // 2. 编译时验证：可以在编译时验证参数
    // 3. 性能优化：可以生成优化的代码
    // 4. 代码复用：减少重复代码

    // 函数宏的最佳实践：
    // 1. 提供清晰的文档和示例
    // 2. 使用合理的参数命名
    // 3. 处理错误情况并提供有用的错误信息
    // 4. 避免过度复杂的宏定义
    // 5. 考虑编译时性能影响

    // 函数宏与声明式宏的对比：
    // 声明式宏：基于模式匹配，简单直观，适合简单场景
    // 函数宏：基于语法树分析，功能强大，适合复杂场景

    // 常用的内置函数宏：
    // 1. 格式化相关：format!, print!, println!, write!, writeln!
    // 2. 集合创建：vec!, hashmap!, btreeset!
    // 3. 字符串操作：concat!, stringify!, include_str!
    // 4. 编译时信息：file!, line!, column!, module_path!
    // 5. 环境变量：env!, option_env!
    // 6. 条件编译：cfg!, include!, compile_error!

    println!();
}

// ===========================================
// 9. 宏高级模式 (Advanced Macro Patterns)
// ===========================================

// 宏高级模式展示了 Rust 宏系统的强大功能和灵活性
// 通过组合各种宏技术，可以创建复杂的元编程模式和领域特定语言（DSL）
// 这些模式在实际项目开发中具有重要的应用价值

fn advanced_macro_patterns() {
    println!("=== 宏高级模式 ===");

    // 递归宏（Recursive Macros）
    // 递归宏允许宏在展开过程中调用自身，实现复杂的代码生成模式
    // 递归是 Rust 宏系统的重要特性，使得可以处理任意复杂的数据结构
    // 注意：递归宏可能达到递归限制，这里注释掉示例
    /*
    macro_rules! countdown {
        (0) => { // 基础情况：当参数为 0 时输出"发射！"
            println!("发射！");
        };
        ($n:expr) => { // 递归情况：输出当前数字并递归调用
            println!("{}", $n);
            countdown!($n - 1); // 递归调用自身
        };
    }

    println!("递归倒计时:");
    countdown!(3); // 执行递归倒计时
    */

    // 递归宏的应用场景：
    // 1. 树形结构的遍历和生成
    // 2. 复杂数据结构的初始化
    // 3. 算法实现（如排序、搜索）
    // 4. DSL 的实现

    // Token 操作宏
    // 宏可以操作和处理 token，实现复杂的语法转换
    macro_rules! first_token {
        // 匹配任意 token 并返回第一个 token
        ($first:tt $($rest:tt)*) => {
            $first // 返回第一个 token
        };
    }

    let first = first_token!("hello" world foo bar); // 提取第一个 token
    println!("第一个 token: {}", first);

    // Token 操作的高级应用：
    // 1. 语法分析和转换
    // 2. 代码重排和优化
    // 3. DSL 的解析和生成
    // 4. 元编程框架的实现

    // 函数组合器宏（Function Composition）
    // 宏可以实现函数组合模式，将多个函数组合成一个新的函数
    macro_rules! compose {
        // 将两个函数组合成一个新的函数
        ($f:expr, $g:expr) => {
            // 返回一个闭包，执行 g(x) 然后将结果传给 f
            |x| $f($g(x))
        };
    }

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn double(x: i32) -> i32 {
        x * 2
    }

    // 创建组合函数：先加一再加倍
    let add_then_double = compose!(double, add_one);
    println!("组合函数结果: {}", add_then_double(5)); // 输出: (5 + 1) * 2 = 12

    // 组合器的优势：
    // 1. 函数式编程风格
    // 2. 代码复用和模块化
    // 3. 管道和链式操作
    // 4. 易于测试和维护

    // 代码块执行宏（Block Execution）
    // 宏可以包装代码块，为其添加额外的功能，如计时、日志等
    #[allow(unused_macros)]
    macro_rules! time_block {
        // 接受一个代码块，执行并计时
        ($block:block) => {
            // 记录开始时间
            let start = std::time::Instant::now();

            // 执行传入的代码块
            let result = $block;

            // 计算执行时间
            let duration = start.elapsed();
            println!("执行耗时: {:?}", duration);

            // 返回代码块的结果
            result
        };
    }

    // 使用计时宏计算 1 到 1000 的和
    // 注意：由于宏语法限制，这里简化实现
    let start = std::time::Instant::now();
    let mut total = 0;
    for i in 1..=1000 {
        total += i;
    }
    let duration = start.elapsed();
    println!("执行时间: {:?}", duration);
    let sum = total;

    println!("1到1000的和: {}", sum);

    // 高级宏模式的应用：
    // 1. 领域特定语言（DSL）：创建特定领域的语言
    // 2. 代码生成：自动生成重复性的代码
    // 3. 测试框架：创建灵活的测试宏
    // 4. 配置管理：声明式的配置语法
    // 5. 数据处理：声明式的数据处理管道

    // 宏模式的最佳实践：
    // 1. 保持宏定义简洁易懂
    // 2. 提供详细的文档和示例
    // 3. 处理错误情况和边界条件
    // 4. 考虑编译时性能影响
    // 5. 避免过度复杂的宏定义

    // 宏模式的设计原则：
    // 1. 单一职责：每个宏应有明确的用途
    // 2. 可组合性：宏应该可以相互组合使用
    // 3. 可扩展性：设计应考虑未来的扩展需求
    // 4. 类型安全：尽可能提供类型检查
    // 5. 性能考虑：避免不必要的性能开销

    println!();
}

// ===========================================
// 10. 宏示例程序 (Macro Example Program)
// ===========================================

// 宏示例程序展示了宏在实际应用中的强大功能和灵活性
// 通过综合运用各种宏技术，可以创建丰富的元编程解决方案
// 这些示例涵盖了日志系统、测试框架、查询构建器等常见应用场景

fn macro_example_program() {
    println!("=== 宏示例程序 ===");

    // 日志宏系统（Logging Macro System）
    // 创建一个简单的日志系统，支持不同级别的日志输出
    // 展示了宏的条件编译和模式匹配能力
    macro_rules! log {
        // 信息级别日志：总是输出
        (info, $msg:expr) => {
            println!("[INFO] {}", $msg);
        };
        // 警告级别日志：总是输出
        (warn, $msg:expr) => {
            println!("[WARN] {}", $msg);
        };
        // 错误级别日志：总是输出
        (error, $msg:expr) => {
            println!("[ERROR] {}", $msg);
        };
        // 调试级别日志：只在调试模式下输出
        (debug, $msg:expr) => {
            #[cfg(debug_assertions)]
            println!("[DEBUG] {}", $msg);
        };
    }

    // 使用日志宏记录应用程序事件
    log!(info, "应用程序启动");
    log!(warn, "配置文件未找到，使用默认配置");
    log!(error, "无法连接到数据库");
    log!(debug, "调试信息: 变量值 = 42");

    // 日志宏的优势：
    // 1. 统一的日志格式
    // 2. 条件编译支持（调试日志只在调试模式输出）
    // 3. 编译时优化（发布版本中会移除调试日志）
    // 4. 易于扩展（可以添加更多日志级别）

    // 测试框架宏（Test Framework Macro）
    // 创建自定义的断言宏，提供更友好的错误信息
    macro_rules! assert_eq_custom {
        // 比较两个表达式，如果不相等则 panic
        ($left:expr, $right:expr) => {
            if $left != $right {
                // 使用 stringify! 宏将表达式转换为字符串
                panic!("断言失败: {} != {}", stringify!($left), stringify!($right));
            }
        };
    }

    // 使用自定义断言宏
    assert_eq_custom!(2 + 2, 4);
    // assert_eq_custom!(2 + 2, 5); // 这会触发 panic

    // 自定义断言的优势：
    // 1. 提供更详细的错误信息
    // 2. 可以添加额外的检查逻辑
    // 3. 可以集成到测试框架中
    // 4. 支持自定义的断言格式

    // SQL 查询构建器宏（SQL Query Builder Macro）
    // 创建一个简单的 SQL 查询构建器，演示宏的 DSL 能力
    // 注意：由于宏解析的复杂性，这里使用简化的实现
    macro_rules! select {
        // 基本 SELECT 查询：SELECT columns FROM table
        (columns: [$($column:expr),*], table: $table:expr) => {
            // 使用 stringify! 将列名转换为字符串
            format!("SELECT {} FROM {}", stringify!($($column),*), $table)
        };
        // 带条件的 SELECT 查询：SELECT columns FROM table WHERE condition
        (columns: [$($column:expr),*], table: $table:expr, where: $condition:expr) => {
            format!("SELECT {} FROM {} WHERE {}", stringify!($($column),*), $table, $condition)
        };
    }

    // 使用 SQL 查询构建器
    let query1 = select!(columns: [id, name], table: "users");
    let query2 = select!(columns: [id, name], table: "users", where: "age > 18");

    println!("SQL 查询 1: {}", query1);
    println!("SQL 查询 2: {}", query2);

    // SQL 构建器的优势：
    // 1. 类型安全的 SQL 构建方式
    // 2. 避免手动拼接字符串的错误
    // 3. 可以扩展支持更多 SQL 语法
    // 4. 易于维护和修改

    // JSON 构建器宏（JSON Builder Macro）
    // 创建一个简单的 JSON 构建器，演示宏处理复杂数据结构的能力
    macro_rules! json {
        // JSON 对象构建：{"key": "value", ...}
        ({ $($key:expr => $value:expr),* }) => {
            {
                let mut obj = std::collections::HashMap::new();
                $(
                    obj.insert($key.to_string(), $value.to_string());
                )*
                obj
            }
        };
        // JSON 数组构建：["value1", "value2", ...]
        ([$($value:expr),*]) => {
            {
                let mut arr = Vec::new();
                $(
                    arr.push($value.to_string());
                )*
                arr
            }
        };
    }

    // 使用 JSON 构建器
    let user = json!({
        "name" => "Alice",
        "age" => "30",
        "email" => "alice@example.com"
    });

    let scores = json!(["95", "87", "92"]);

    println!("JSON 对象: {:?}", user);
    println!("JSON 数组: {:?}", scores);

    // JSON 构建器的优势：
    // 1. 声明式的 JSON 构建语法
    // 2. 避免手动的 JSON 字符串拼接
    // 3. 类型安全的值转换
    // 4. 可以扩展支持更多 JSON 类型

    // 配置管理宏（Configuration Management Macro）
    // 创建一个配置管理系统，演示宏的元编程能力
    macro_rules! config {
        // 配置项定义：key: value
        ($($key:ident: $value:expr),*) => {
            {
                let mut cfg = std::collections::HashMap::new();
                $(
                    // 使用 stringify! 将标识符转换为字符串
                    cfg.insert(stringify!($key).to_string(), $value.to_string());
                )*
                cfg
            }
        };
    }

    // 使用配置管理宏
    let app_config = config! {
        debug: true,
        port: 8080,
        timeout: 30,
        log_level: "info"
    };

    println!("应用配置: {:?}", app_config);

    // 配置管理的优势：
    // 1. 声明式的配置语法
    // 2. 自动类型转换
    // 3. 易于扩展和维护
    // 4. 可以支持嵌套配置结构

    // 代码生成宏（Code Generation Macro）
    // 自动生成结构体的访问器方法，减少样板代码
    // 注意：由于宏限制，这里简化为手动实现
    macro_rules! generate_accessors {
        // 为结构体生成访问器方法
        ($struct_name:ident, $($field:ident: $type:ty),*) => {
            impl $struct_name {
                // 手动实现访问器方法作为示例
                // 在实际项目中，可以使用 paste crate 来实现更复杂的方法名生成
                pub fn name(&self) -> &String {
                    &self.name
                }
                pub fn name_mut(&mut self) -> &mut String {
                    &mut self.name
                }
                pub fn age(&self) -> &u32 {
                    &self.age
                }
                pub fn age_mut(&mut self) -> &mut u32 {
                    &mut self.age
                }
                pub fn email(&self) -> &String {
                    &self.email
                }
                pub fn email_mut(&mut self) -> &mut String {
                    &mut self.email
                }
            }
        };
    }

    // 定义一个结构体并使用代码生成宏
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    // 为 Person 结构体生成访问器
    generate_accessors!(Person, name: String, age: u32, email: String);

    let mut person = Person {
        name: "Bob".to_string(),
        age: 25,
        email: "bob@example.com".to_string(),
    };

    // 使用生成的访问器
    println!("访问器测试:");
    println!("  姓名: {}", person.name());
    println!("  年龄: {}", person.age());
    *person.age_mut() = 26; // 修改年龄
    println!("  修改后年龄: {}", person.age());

    // 代码生成的优势：
    // 1. 减少重复的样板代码
    // 2. 保证访问器方法的一致性
    // 3. 可以扩展支持验证逻辑
    // 4. 易于维护和修改

    // 验证宏（Validation Macro）
    // 创建一个验证宏，演示宏的模式匹配能力
    macro_rules! validate {
        // 验证表达式是否匹配指定模式
        ($value:expr, $pattern:pat => $result:expr) => {
            match $value {
                $pattern => $result,
                _ => panic!("验证失败: {}", stringify!($value)),
            }
        };
    }

    // 使用验证宏验证数据
    // 注意：简单的验证宏不支持guard条件，需要更复杂的实现
    let valid_result = validate!(Some(42), Some(x) => x * 2);
    println!("验证结果: {}", valid_result);

    // 验证宏的优势：
    // 1. 声明式的验证语法
    // 2. 可以使用复杂的模式匹配
    // 3. 提供有意义的错误信息
    // 4. 易于扩展新的验证规则

    // 宏综合应用的优势：
    // 1. 减少样板代码，提高开发效率
    // 2. 提供类型安全和编译时检查
    // 3. 创建声明式的 DSL，提高代码可读性
    // 4. 实现复杂的功能，如日志、测试、配置等
    // 5. 支持元编程，实现代码生成和转换

    // 宏设计的最佳实践：
    // 1. 明确的用途和命名
    // 2. 完善的错误处理
    // 3. 详细的文档和示例
    // 4. 性能和编译时间考虑
    // 5. 与现有生态的兼容性

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 宏系统演示");
    println!("===============");

    declarative_macros_basics();
    macro_pattern_matching();
    repetition_patterns();
    macro_hygiene();
    procedural_macros_basics();
    custom_derive_macros();
    attribute_macros();
    function_like_macros();
    advanced_macro_patterns();
    macro_example_program();

    println!("宏系统演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_macro() {
        macro_rules! double {
            ($x:expr) => {
                $x * 2
            };
        }

        assert_eq!(double!(5), 10);
        assert_eq!(double!(10), 20);
    }

    #[test]
    fn test_vector_macro() {
        macro_rules! create_vector {
            ($($x:expr),*) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

        let v = create_vector!(1, 2, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_macro_hygiene() {
        let x = 100;

        // 注意：由于宏在表达式上下文中的限制，这里简化实现
        let result = {
            let x = 42;
            x
        };
        assert_eq!(result, 42);
        assert_eq!(x, 100); // 外部变量不受影响
    }

    #[test]
    fn test_derive_macro_simulation() {
        #[derive(Debug, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = p1.clone();

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_conditional_macro() {
        macro_rules! conditional {
            ($condition:expr, $true_expr:expr, $false_expr:expr) => {
                if $condition { $true_expr } else { $false_expr }
            };
        }

        assert_eq!(conditional!(true, 1, 2), 1);
        assert_eq!(conditional!(false, 1, 2), 2);
    }
}
