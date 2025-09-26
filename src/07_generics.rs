// Rust 泛型系统
// 深入讲解 Rust 泛型的各个方面，包括泛型函数、结构体、枚举、约束、实现和高级模式

// ===========================================
// 1. 泛型函数 (Generic Functions)
// ===========================================

// 泛型函数是 Rust 泛型系统的基础，它允许我们编写适用于多种类型的代码
// 泛型的主要目标是减少代码重复，同时保持类型安全和性能

fn generic_functions() {
    println!("=== 泛型函数 ===");

    // 基本泛型函数：类型参数 T 在函数签名中声明
    // <T: PartialOrd> 表示 T 必须实现 PartialOrd trait（可比较排序）
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // 泛型函数的优势：同一个函数可以用于多种类型
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    println!("数字列表: {:?}", numbers);
    println!("最大数字: {}", largest(&numbers));

    println!("字符列表: {:?}", chars);
    println!("最大字符: {}", largest(&chars));

    // 多个泛型参数：函数可以有多个类型参数
    // 这使得函数可以处理不同类型的组合
    fn pick<T, U>(t: T, u: U) -> T {
        t  // 返回第一个参数的类型
    }

    let number = pick(10, "hello");         // T 是 i32，U 是 &str
    let string = pick("world", 3.14);        // T 是 &str，U 是 f64
    println!("选择结果: {} {}", number, string);

    // 泛型函数的约束：通过 trait bounds 限制类型参数
    // T: std::fmt::Debug 表示 T 必须可以被调试打印
    fn print_debug<T: std::fmt::Debug>(item: T) {
        println!("调试打印: {:?}", item);
    }

    print_debug(42);                          // i32 实现了 Debug
    print_debug("hello");                     // &str 实现了 Debug
    print_debug(vec![1, 2, 3]);             // Vec<i32> 实现了 Debug

    // 泛型函数的设计考虑：
    // 1. 类型参数应该具有语义意义
    // 2. 约束应该尽可能宽松（但必要）
    // 3. 函数名应该清楚表达其泛型性质
    // 4. 考虑使用具体的类型替代泛型，如果适用的话

    // 泛型与宏的区别：
    // - 泛型是类型安全的，编译时检查
    // - 泛型支持 IDE 补全和类型推断
    // - 宏是语法层面的，更灵活但安全性较低
    // - 泛型通常更易读和维护

    println!();
}

// ===========================================
// 2. 泛型结构体 (Generic Structs)
// ===========================================

// 泛型结构体允许我们定义可以存储任意类型的数据结构
// 这是 Rust 中代码复用和类型抽象的重要机制

fn generic_structs() {
    println!("=== 泛型结构体 ===");

    // 基本泛型结构体：单个类型参数
    // Point<T> 可以存储任意类型的 x 和 y 坐标
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // 同一个结构体可以用于不同的类型
    let integer = Point { x: 5, y: 10 };      // Point<i32>
    let float = Point { x: 1.0, y: 4.0 };      // Point<f64>
    let text = Point { x: "hello", y: "world" }; // Point<&str>

    println!("整数点: {:?}", integer);
    println!("浮点点: {:?}", float);
    println!("文本点: {:?}", text);

    // 多个泛型参数：结构体可以有多个不同的类型参数
    // 这使得结构体更加灵活，可以存储不同类型的数据
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    let pair1 = Pair { first: 1, second: 2.0 };        // Pair<i32, f64>
    let pair2 = Pair { first: "hello", second: 'a' };    // Pair<&str, char>
    let pair3 = Pair { first: vec![1, 2], second: 42 }; // Pair<Vec<i32>, i32>

    println!("混合对1: {:?}", pair1);
    println!("混合对2: {:?}", pair2);
    println!("混合对3: {:?}", pair3);

    // 泛型结构体的方法实现：使用 impl<T> 语法
    // 这些方法适用于所有可能的类型参数 T
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }

    let point = Point::new(5, 10);
    println!("点的 x 坐标: {}", point.x());

    // 特定类型的实现：为特定类型提供专门的方法
    // 这些方法只对特定类型可用
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let float_point = Point { x: 3.0, y: 4.0 };
    println!("距离原点: {}", float_point.distance_from_origin());

    // 注意：distance_from_origin 方法只在 Point<f32> 上可用
    // let integer_point = Point { x: 3, y: 4 };
    // integer_point.distance_from_origin(); // 编译错误

    // 泛型结构体的设计模式：
    // 1. 包装器模式：为现有类型添加功能
    // 2. 容器模式：存储和操作多个值
    // 3. 配置模式：使用泛型表示配置选项
    // 4. 状态模式：使用泛型表示不同的状态

    // 泛型结构体的内存布局：
    // - 泛型不影响内存布局的基本结构
    // - 具体类型替换后，内存布局是确定的
    // - 编译器会为每个具体类型生成专门的代码

    println!();
}

// ===========================================
// 3. 泛型枚举 (Generic Enums)
// ===========================================

// 泛型枚举是 Rust 中最强大的特性之一
// 它们允许我们定义可以包含任意类型数据的变体

fn generic_enums() {
    println!("=== 泛型枚举 ===");

    // 标准库中的泛型枚举：Option<T>
    // Option<T> 表示值可能存在或不存在的情况
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;

    println!("Option 示例: {:?} {:?} {:?}", some_number, some_char, absent_number);

    // 标准库中的泛型枚举：Result<T, E>
    // Result<T, E> 表示操作可能成功或失败的情况
    let success: Result<i32, String> = Ok(200);
    let error: Result<i32, String> = Err("Something went wrong".to_string());

    println!("Result 示例: {:?} {:?}", success, error);

    // 自定义泛型枚举：根据需求设计
    #[derive(Debug)]
    enum Container<T> {
        Empty,           // 空容器，不包含值
        Value(T),        // 包含一个值
        Pair(T, T),      // 包含两个相同类型的值
    }

    let empty: Container<i32> = Container::Empty;
    let single: Container<f64> = Container::Value(3.14);
    let pair: Container<&str> = Container::Pair("hello", "world");

    println!("自定义枚举: {:?} {:?} {:?}", empty, single, pair);

    // 复杂泛型枚举：多个类型参数
    #[derive(Debug)]
    enum Result<T, E> {
        Ok(T),           // 成功，包含类型 T 的值
        Err(E),          // 失败，包含类型 E 的错误
    }

    let ok_result: Result<String, std::io::Error> = Result::Ok("Success".to_string());
    let err_result: Result<i32, String> = Result::Err("Error occurred".to_string());

    println!("复杂枚举: {:?} {:?}", ok_result, err_result);

    // 泛型枚举的方法实现：为所有类型参数提供通用方法
    impl<T> Container<T> {
        fn is_empty(&self) -> bool {
            matches!(self, Container::Empty)
        }

        fn count(&self) -> usize {
            match self {
                Container::Empty => 0,
                Container::Value(_) => 1,
                Container::Pair(_, _) => 2,
            }
        }
    }

    println!("空容器: {} 个元素", empty.count());
    println!("单值容器: {} 个元素", single.count());
    println!("双值容器: {} 个元素", pair.count());

    // 泛型枚举的优势：
    // 1. 类型安全：编译器确保所有情况都被处理
    // 2. 表达力强：可以表示复杂的数据关系
    // 3. 内存高效：只存储实际使用的变体
    // 4. 模式匹配支持：与 Rust 的模式匹配完美集成

    // 泛型枚举的实际应用：
    // 1. 错误处理：Result<T, E>
    // 2. 可选值：Option<T>
    // 3. 状态机：表示不同的系统状态
    // 4. 解析结果：表示解析的不同结果
    // 5. 配置类型：表示不同的配置选项

    // 泛型枚举的设计原则：
    // 1. 变体应该有清晰的语义
    // 2. 类型参数应该有意义
    // 3. 考虑使用具体类型替代泛型，如果适用
    // 4. 确保所有变体都是必要的

    println!();
}

// ===========================================
// 4. 泛型约束 (Generic Constraints)
// ===========================================

// 泛型约束（trait bounds）限制泛型类型参数必须实现的 trait
// 这是 Rust 泛型系统中保证类型安全和功能完整性的关键机制

fn generic_constraints() {
    println!("=== 泛型约束 ===");

    // 基本约束：使用 : 语法指定 trait bounds
    // T 必须同时实现 Debug 和 PartialEq traits
    fn compare_and_print<T: std::fmt::Debug + PartialEq>(a: T, b: T) {
        println!("比较: {:?} 和 {:?}", a, b);
        if a == b {
            println!("相等");
        } else {
            println!("不相等");
        }
    }

    compare_and_print(5, 5);        // i32 实现了 Debug 和 PartialEq
    compare_and_print("hello", "world"); // &str 也实现了这些 traits

    // 使用 where 子句：对于复杂的约束，where 子句更清晰
    // where 子句可以提高复杂约束的可读性
    fn advanced_function<T, U>(t: T, u: U)
    where
        T: std::fmt::Debug + Clone,        // T 必须可调试和克隆
        U: std::fmt::Debug + Clone,        // U 也必须可调试和克隆
    {
        println!("高级函数: {:?} {:?}", t, u);
    }

    advanced_function(42, "hello");
    advanced_function(vec![1, 2, 3], vec!['a', 'b']);

    // 多重约束：类型参数可以实现多个 traits
    // 这为泛型代码提供了丰富的功能保证
    fn display_and_clone<T: std::fmt::Display + Clone>(item: T) -> (String, T) {
        let display = item.to_string();     // 使用 Display trait
        let cloned = item.clone();          // 使用 Clone trait
        (display, cloned)
    }

    let (str_val, cloned_num) = display_and_clone(42);
    println!("显示和克隆: {} {}", str_val, cloned_num);

    let (str_val, cloned_str) = display_and_clone("hello");
    println!("显示和克隆: {} {}", str_val, cloned_str);

    // 生命周期约束：确保引用的有效性
    // 泛型类型参数可以与生命周期参数结合使用
    fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: PartialOrd,                      // T 必须可比较
    {
        if x > y {
            x
        } else {
            y
        }
    }

    let num1 = 5;
    let num2 = 10;
    println!("较长数字引用: {}", longest(&num1, &num2));

    // 约束的层次结构：
    // 1. 基础约束：如 Clone, Copy, Debug
    // 2. 功能约束：如 Add, PartialEq, PartialOrd
    // 3. 转换约束：如 From, TryFrom, AsRef
    // 4. 集合约束：如 IntoIterator, Iterator

    // 常用的标准库约束：
    fn common_constraints_example<T>()
    where
        T: std::fmt::Debug +           // 可调试打印
          std::fmt::Display +          // 可显示
          std::clone::Clone +          // 可克隆
          std::cmp::PartialEq +        // 可比较相等性
          std::cmp::PartialOrd +       // 可比较大小
          std::marker::Copy,           // 可按位复制
    {
        println!("这是一个具有多种约束的泛型函数");
    }

    // 约束的最佳实践：
    // 1. 只约束必要的 traits
    // 2. 优先使用标准库 traits
    // 3. 考虑约束的语义意义
    // 4. 使用 where 子句提高可读性

    // 约束与性能：
    // - 约束本身不影响运行时性能
    // - 约束影响编译时间和代码生成
    // - 过度的约束可能限制泛型的使用范围

    println!();
}

// ===========================================
// 5. 泛型实现 (Generic Implementations)
// ===========================================

// 泛型实现允许我们为泛型类型实现 traits 和方法
// 这是 Rust 中代码抽象和复用的核心机制

fn generic_implementations() {
    println!("=== 泛型实现 ===");

    // 为泛型结构体实现 trait：通用实现适用于所有类型参数
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
    }

    // 通用实现：适用于所有类型 T
    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }

        fn get(&self) -> &T {
            &self.value
        }

        fn get_mut(&mut self) -> &mut T {
            &mut self.value
        }

        fn into_inner(self) -> T {
            self.value
        }
    }

    let wrapper = Wrapper::new(42);
    println!("包装器值: {}", wrapper.get());

    let mut wrapper = Wrapper::new(vec![1, 2, 3]);
    wrapper.get_mut().push(4);
    println!("修改后的包装器: {:?}", wrapper.get());

    let inner = wrapper.into_inner();
    println!("内部值: {:?}", inner);

    // 特定类型的实现：为特定类型提供专门的方法
    // 这些方法只在特定类型上可用
    impl Wrapper<i32> {
        fn is_positive(&self) -> bool {
            self.value > 0
        }

        fn abs(self) -> Self {
            Wrapper::new(self.value.abs())
        }
    }

    let int_wrapper = Wrapper::new(-42);
    println!("整数包装器是正数: {}", int_wrapper.is_positive());
    println!("绝对值包装器: {:?}", int_wrapper.abs().get());

    // 条件实现：基于 trait bounds 的实现
    // 只有当类型参数满足特定约束时，实现才可用
    trait DisplayAndDebug {
        fn display_and_debug(&self) -> String;
    }

    // 为所有同时实现 Display 和 Debug 的类型提供实现
    impl<T: std::fmt::Display + std::fmt::Debug> DisplayAndDebug for T {
        fn display_and_debug(&self) -> String {
            format!("Display: {}, Debug: {:?}", self, self)
        }
    }

    let value = 42;
    println!("条件实现: {}", value.display_and_debug());

    let text = "hello";
    println!("条件实现: {}", text.display_and_debug());

    // 为特定约束组合实现 traits
    trait Summary {
        fn summarize(&self) -> String;
    }

    // 为所有实现 Display 的类型提供 Summary 实现
    impl<T: std::fmt::Display> Summary for T {
        fn summarize(&self) -> String {
            format!("摘要: {}", self)
        }
    }

    let number = 42;
    let text = "hello world";
    println!("数字摘要: {}", number.summarize());
    println!("文本摘要: {}", text.summarize());

    // 泛型实现的模式：
    // 1. 通用实现：适用于所有类型
    // 2. 特定实现：适用于特定类型
    // 3. 条件实现：基于 trait bounds
    // 4. 组合实现：基于多个 traits

    // 实现的选择策略：
    // - 优先使用通用实现
    // - 只在必要时使用特定实现
    // - 合理使用条件实现提供额外功能
    // - 保持实现的一致性和可预测性

    // 泛型实现的编译器优化：
    // - 单态化（monomorphization）确保性能
    // - 编译器会为每个具体类型生成专门代码
    // - 泛型代码的性能与手写代码相当

    println!();
}

// ===========================================
// 6. 泛型在标准库中的使用
// ===========================================

// Rust 标准库广泛使用泛型，了解这些用法对于掌握 Rust 至关重要

fn generics_in_std_lib() {
    println!("=== 标准库中的泛型 ===");

    // Vec<T>：动态大小的数组，可以存储任意类型
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let strings: Vec<&str> = vec!["hello", "world", "rust"];
    let any_type: Vec<Box<dyn std::fmt::Debug>> = vec![
        Box::new(42),
        Box::new("hello"),
        Box::new(vec![1, 2, 3]),
    ];

    println!("数字向量: {:?}", numbers);
    println!("字符串向量: {:?}", strings);
    println!("混合类型向量: {:?}", any_type);

    // HashMap<K, V>：键值对映射，键和值可以是不同类型
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("HashMap: {:?}", scores);

    // Iterator trait：迭代器模式的核心，支持链式操作
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    let even_numbers: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    let sum: i32 = numbers.iter().sum();

    println!("原始向量: {:?}", numbers);
    println!("加倍后: {:?}", doubled);
    println!("偶数: {:?}", even_numbers);
    println!("总和: {}", sum);

    // Box<T>：智能指针，用于堆分配
    let boxed_int: Box<i32> = Box::new(42);
    let boxed_str: Box<str> = Box::from("hello");

    println!("Boxed int: {}", boxed_int);
    println!("Boxed str: {}", boxed_str);

    // Option<T> 和 Result<T, E>：错误处理的核心
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    let success: Result<i32, String> = Ok(200);
    let failure: Result<i32, String> = Err("Error".to_string());

    println!("Option: {:?} {:?}", some_value, none_value);
    println!("Result: {:?} {:?}", success, failure);

    // Rc<T>：引用计数智能指针，用于共享所有权
    use std::rc::Rc;
    let shared_data = Rc::new(String::from("Hello"));
    let shared1 = Rc::clone(&shared_data);
    let shared2 = Rc::clone(&shared_data);

    println!("Rc 共享数据: {}", shared1);
    println!("引用计数: {}", Rc::strong_count(&shared_data));

    // Arc<T>：原子引用计数，用于线程间共享
    use std::sync::Arc;
    let thread_safe_data = Arc::new(vec![1, 2, 3]);

    // Cell<T> 和 RefCell<T>：内部可变性
    use std::cell::RefCell;
    let mutable_data = RefCell::new(42);
    *mutable_data.borrow_mut() = 100;

    println!("RefCell 数据: {}", mutable_data.borrow());

    // 标准库泛型的设计模式：
    // 1. 容器类型：Vec<T>, HashMap<K, V>, HashSet<T>
    // 2. 智能指针：Box<T>, Rc<T>, Arc<T>, Cell<T>
    // 3. 错误处理：Option<T>, Result<T, E>
    // 4. 迭代器：Iterator<Item = T>
    // 5. 字符串处理：String, &str, Cow<str>

    // 泛型在标准库中的优势：
    // - 代码复用：同一个实现适用于多种类型
    // - 类型安全：编译时检查确保类型正确
    // - 性能优化：单态化确保零开销抽象
    // - 表达力强：可以表示复杂的类型关系

    // 使用标准库泛型的最佳实践：
    // 1. 了解常用泛型类型的特性和限制
    // 2. 选择合适的泛型类型解决特定问题
    // 3. 理解所有权和借用规则对泛型的影响
    // 4. 合理使用 trait bounds 约束泛型类型

    println!();
}

// ===========================================
// 7. 泛型性能
// ===========================================

// Rust 泛型的一个重要特性是零开销抽象
// 理解泛型的性能机制对于编写高性能代码至关重要

fn generic_performance() {
    println!("=== 泛型性能 ===");

    // 单态化（Monomorphization）：Rust 泛型性能的核心机制
    // 编译器会为每个具体类型生成专门的代码版本
    fn generic_max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }

    // 编译时会为具体类型生成专门版本
    let int_result = generic_max(5, 10);        // 生成 generic_max_i32 版本
    let float_result = generic_max(3.14, 2.71); // 生成 generic_max_f64 版本
    let string_result = generic_max("apple", "banana"); // 生成 generic_max_str 版本

    println!("整数最大值: {}", int_result);
    println!("浮点最大值: {}", float_result);
    println!("字符串最大值: {}", string_result);

    // 泛型 vs 具体类型：性能对比
    // 由于单态化，泛型代码的性能与手写代码相同
    fn specific_max_i32(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }

    // 性能测试：比较泛型版本和具体版本
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        let _ = generic_max(5, 10);
    }
    let generic_duration = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        let _ = specific_max_i32(5, 10);
    }
    let specific_duration = start.elapsed();

    println!("泛型版本耗时: {:?}", generic_duration);
    println!("具体版本耗时: {:?}", specific_duration);

    // 内存布局：泛型不影响内存布局
    #[derive(Debug)]
    struct GenericPair<T> {
        first: T,
        second: T,
    }

    println!("GenericPair<i32> 大小: {}", std::mem::size_of::<GenericPair<i32>>());
    println!("GenericPair<f64> 大小: {}", std::mem::size_of::<GenericPair<f64>>());

    // 编译时优化：泛型代码可以享受所有的编译器优化
    fn generic_sum<T: std::ops::Add<Output = T> + Copy>(values: &[T]) -> T {
        let mut sum = values[0];
        for &value in &values[1..] {
            sum = sum + value;
        }
        sum
    }

    let numbers = [1, 2, 3, 4, 5];
    let floats = [1.0, 2.0, 3.0, 4.0, 5.0];

    println!("整数和: {}", generic_sum(&numbers));
    println!("浮点和: {}", generic_sum(&floats));

    // 虚函数调用 vs 单态化：
    // - 单态化消除了虚函数调用的开销
    // - 泛型方法调用是静态分发的
    // - 编译器可以内联泛型函数调用

    // 泛型的编译时开销：
    // - 增加编译时间（需要生成多个代码版本）
    // - 增加二进制文件大小
    // - 但不影响运行时性能

    // 优化泛型代码的策略：
    // 1. 合理使用泛型，避免过度抽象
    // 2. 在性能关键路径考虑使用具体类型
    // 3. 使用 trait bounds 而不是运行时类型检查
    // 4. 理解单态化的代码生成机制

    // 零开销抽象的原则：
    // 1. 不使用的泛型代码不会产生运行时开销
    // 2. 泛型代码的性能与手写代码相同
    // 3. 编译时优化不依赖于运行时信息
    // 4. 类型检查在编译时完成

    println!();
}

// ===========================================
// 8. 泛型高级模式
// ===========================================

// Rust 泛型系统支持多种高级模式，这些模式在复杂的项目中非常有用

fn advanced_generic_patterns() {
    println!("=== 高级泛型模式 ===");

    // 关联类型：trait 中的类型占位符
    // 关联类型使得 trait 更加灵活和表达力强
    trait Container {
        type Item;                          // 关联类型
        fn get(&self) -> Self::Item;
        fn set(&mut self, item: Self::Item);
    }

    struct NumberContainer<T> {
        value: T,
    }

    impl<T> Container for NumberContainer<T> {
        type Item = T;                       // 指定关联类型

        fn get(&self) -> Self::Item {
            self.value.clone()
        }

        fn set(&mut self, item: Self::Item) {
            self.value = item;
        }
    }

    let mut container = NumberContainer { value: 42 };
    println!("容器值: {}", container.get());
    container.set(100);
    println!("修改后的值: {}", container.get());

    // 泛型默认参数：为类型参数提供默认值
    // 这使得泛型类型更加易用
    struct Pair<T = i32> {                   // T 默认为 i32
        first: T,
        second: T,
    }

    let default_pair = Pair { first: 5, second: 10 };  // 使用默认类型 i32
    let string_pair = Pair { first: "hello", second: "world" }; // 显式指定 &str

    println!("默认参数对: {:?} {:?}", default_pair, string_pair);

    // const 泛型：在编译时使用常量值作为泛型参数
    // 这使得编译时计算和类型安全更加灵活
    fn create_array<T, const N: usize>() -> [T; N]
    where
        T: Default,
    {
        [T::default(); N]
    }

    let array: [i32; 5] = create_array();
    let string_array: [String; 3] = create_array();

    println!("const 泛型数组: {:?}", array);
    println!("const 泛型字符串数组: {:?}", string_array);

    // impl Trait：简化返回类型表示
    // 允许函数返回实现了特定 trait 的类型
    fn returns_iterator() -> impl Iterator<Item = i32> {
        vec![1, 2, 3].into_iter()
    }

    fn returns_display() -> impl std::fmt::Display {
        42
    }

    let iter = returns_iterator();
    let sum: i32 = iter.sum();
    println!("impl Trait 迭代器求和: {}", sum);

    let display = returns_display();
    println!("impl Trait 显示值: {}", display);

    // 泛型关联类型（GATs）：更复杂的关联类型
    trait Factory {
        type Output;
        fn create(&self) -> Self::Output;
    }

    struct IntFactory;
    impl Factory for IntFactory {
        type Output = i32;
        fn create(&self) -> Self::Output {
            42
        }
    }

    let factory = IntFactory;
    println!("工厂创建: {}", factory.create());

    // 高级 trait 约束：
    trait Process<T> {
        fn process(&self, input: T) -> T;
    }

    impl<T, U> Process<T> for U
    where
        T: std::ops::Add<Output = T> + Copy,
        U: Fn(T) -> T,
    {
        fn process(&self, input: T) -> T {
            self(input)
        }
    }

    let adder = |x| x + 1;
    println!("处理结果: {}", adder.process(5));

    // 存在性类型（existential types）：
    // 使用 dyn Trait 表示动态分发的对象
    let trait_objects: Vec<Box<dyn std::fmt::Debug>> = vec![
        Box::new(42),
        Box::new("hello"),
        Box::new(vec![1, 2, 3]),
    ];

    for obj in trait_objects {
        println!("Trait 对象: {:?}", obj);
    }

    // 高级泛型模式的应用场景：
    // 1. 关联类型：设计灵活的 trait 接口
    // 2. 默认参数：提供常用的类型默认值
    // 3. const 泛型：编译时计算和类型安全
    // 4. impl Trait：简化复杂的返回类型
    // 5. GATs：表示复杂的类型关系

    // 选择合适的高级模式：
    // - 考虑代码的可读性和维护性
    // - 评估对编译时间和性能的影响
    // - 确保类型系统的正确使用
    // - 保持与 Rust 生态系统的兼容性

    println!();
}

// ===========================================
// 9. 泛型示例程序
// ===========================================

// 通过完整的示例程序展示泛型的实际应用

fn generic_example_program() {
    println!("=== 泛型示例程序 ===");

    // 示例 1：泛型栈实现
    // 展示如何使用泛型实现通用的数据结构
    #[derive(Debug)]
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }

        fn peek(&self) -> Option<&T> {
            self.items.last()
        }

        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn clear(&mut self) {
            self.items.clear();
        }

        // 迭代器支持
        fn iter(&self) -> std::slice::Iter<T> {
            self.items.iter()
        }
    }

    // 使用泛型栈
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    println!("整数栈: {:?}", int_stack);
    println!("栈顶元素: {:?}", int_stack.peek());
    println!("弹出元素: {:?}", int_stack.pop());
    println!("弹出后的栈: {:?}", int_stack);

    let mut string_stack = Stack::new();
    string_stack.push("hello".to_string());
    string_stack.push("world".to_string());

    println!("字符串栈: {:?}", string_stack);

    // 示例 2：泛型缓存实现
    // 展示如何使用泛型实现通用的缓存机制
    #[derive(Debug)]
    struct Cache<K, V> {
        data: std::collections::HashMap<K, V>,
        capacity: usize,
    }

    impl<K, V> Cache<K, V>
    where
        K: std::hash::Hash + Eq + Clone,    // 键类型必须可哈希、相等和克隆
        V: Clone,                            // 值类型必须可克隆
    {
        fn new(capacity: usize) -> Self {
            Cache {
                data: std::collections::HashMap::new(),
                capacity,
            }
        }

        fn get(&self, key: &K) -> Option<V> {
            self.data.get(key).cloned()
        }

        fn set(&mut self, key: K, value: V) {
            if self.data.len() >= self.capacity {
                self.data.clear();  // 简单的清理策略
            }
            self.data.insert(key, value);
        }

        fn contains_key(&self, key: &K) -> bool {
            self.data.contains_key(key)
        }

        fn size(&self) -> usize {
            self.data.len()
        }

        fn clear(&mut self) {
            self.data.clear();
        }
    }

    // 使用泛型缓存
    let mut cache = Cache::new(3);
    cache.set("key1".to_string(), 100);
    cache.set("key2".to_string(), 200);
    cache.set("key3".to_string(), 300);

    println!("缓存大小: {}", cache.size());
    println!("获取 key1: {:?}", cache.get(&"key1".to_string()));
    println!("包含 key2: {}", cache.contains_key(&"key2".to_string()));

    // 测试容量限制
    cache.set("key4".to_string(), 400);
    println!("添加 key4 后的缓存大小: {}", cache.size());
    println!("key1 是否还在缓存中: {}", cache.contains_key(&"key1".to_string()));

    // 示例 3：泛型事件处理器
    // 展示如何使用泛型实现事件处理系统
    #[derive(Debug, Clone)]
    enum Event<T> {
        Data(T),
        Control(String),
    }

    trait EventHandler<T> {
        fn handle(&self, event: Event<T>);
    }

    struct Logger;
    impl<T: std::fmt::Debug> EventHandler<T> for Logger {
        fn handle(&self, event: Event<T>) {
            match event {
                Event::Data(data) => println!("记录数据事件: {:?}", data),
                Event::Control(msg) => println!("记录控制事件: {}", msg),
            }
        }
    }

    struct Processor<T> {
        handlers: Vec<Box<dyn EventHandler<T>>>,
    }

    impl<T> Processor<T> {
        fn new() -> Self {
            Processor { handlers: Vec::new() }
        }

        fn add_handler<H: EventHandler<T> + 'static>(&mut self, handler: H) {
            self.handlers.push(Box::new(handler));
        }

        fn process_event(&self, event: Event<T>) {
            for handler in &self.handlers {
                handler.handle(event.clone());
            }
        }
    }

    // 使用事件处理器
    let mut processor = Processor::new();
    processor.add_handler(Logger);

    processor.process_event(Event::Data(42));
    processor.process_event(Event::Control("start".to_string()));

    // 示例 4：泛型配置系统
    // 展示如何使用泛型实现类型安全的配置
    #[derive(Debug, Clone)]
    struct Config<T> {
        name: String,
        value: T,
        description: String,
    }

    impl<T> Config<T> {
        fn new(name: String, value: T, description: String) -> Self {
            Config { name, value, description }
        }

        fn get(&self) -> &T {
            &self.value
        }

        fn set(&mut self, value: T) {
            self.value = value;
        }
    }

    // 配置管理器
    struct ConfigManager {
        configs: Vec<Box<dyn std::any::Any>>,  // 使用 Any 存储不同类型的配置
    }

    impl ConfigManager {
        fn new() -> Self {
            ConfigManager { configs: Vec::new() }
        }

        fn add_config<T: 'static>(&mut self, config: Config<T>) {
            self.configs.push(Box::new(config));
        }

        fn get_config<T: 'static>(&self, name: &str) -> Option<&Config<T>> {
            self.configs.iter()
                .filter_map(|config| config.downcast_ref::<Config<T>>())
                .find(|config| config.name == name)
        }
    }

    // 使用配置系统
    let mut manager = ConfigManager::new();
    manager.add_config(Config::new("timeout".to_string(), 30, "连接超时时间".to_string()));
    manager.add_config(Config::new("max_connections".to_string(), 100, "最大连接数".to_string()));
    manager.add_config(Config::new("debug_mode".to_string(), true, "调试模式".to_string()));

    if let Some(timeout_config) = manager.get_config::<i32>("timeout") {
        println!("超时配置: {} = {}", timeout_config.name, timeout_config.get());
    }

    if let Some(debug_config) = manager.get_config::<bool>("debug_mode") {
        println!("调试配置: {} = {}", debug_config.name, debug_config.get());
    }

    // 泛型示例程序的要点：
    // 1. 数据结构抽象：使用泛型实现通用的数据结构
    // 2. 算法抽象：使用泛型实现类型无关的算法
    // 3. 系统架构：使用泛型设计灵活的系统组件
    // 4. 类型安全：在编译时确保类型正确性
    // 5. 性能优化：通过单态化获得零开销抽象

    // 泛型在实际项目中的应用：
    // - 数据库抽象层
    // - 网络协议处理
    // - 游戏引擎组件
    // - 用户界面框架
    // - 序列化/反序列化系统

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 泛型演示");
    println!("=============");

    generic_functions();
    generic_structs();
    generic_enums();
    generic_constraints();
    generic_implementations();
    generics_in_std_lib();
    generic_performance();
    advanced_generic_patterns();
    generic_example_program();

    println!("泛型演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_function() {
        fn largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let numbers = vec![1, 5, 2, 8, 3];
        assert_eq!(*largest(&numbers), 8);

        let chars = vec!['a', 'z', 'm'];
        assert_eq!(*largest(&chars), 'z');
    }

    #[test]
    fn test_point_struct() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn new(x: T, y: T) -> Self {
                Point { x, y }
            }
        }

        let point = Point::new(5, 10);
        assert_eq!(point.x, 5);
        assert_eq!(point.y, 10);
    }

    #[test]
    fn test_container_enum() {
        #[derive(Debug)]
        enum Container<T> {
            Empty,
            Value(T),
            Pair(T, T),
        }

        impl<T> Container<T> {
            fn count(&self) -> usize {
                match self {
                    Container::Empty => 0,
                    Container::Value(_) => 1,
                    Container::Pair(_, _) => 2,
                }
            }
        }

        let empty: Container<i32> = Container::Empty;
        let single: Container<i32> = Container::Value(42);
        let pair: Container<i32> = Container::Pair(1, 2);

        assert_eq!(empty.count(), 0);
        assert_eq!(single.count(), 1);
        assert_eq!(pair.count(), 2);
    }

    #[test]
    fn test_wrapper_struct() {
        struct Wrapper<T> {
            value: T,
        }

        impl<T> Wrapper<T> {
            fn new(value: T) -> Self {
                Wrapper { value }
            }

            fn get(&self) -> &T {
                &self.value
            }
        }

        let wrapper = Wrapper::new(42);
        assert_eq!(*wrapper.get(), 42);

        let string_wrapper = Wrapper::new("hello".to_string());
        assert_eq!(string_wrapper.get(), "hello");
    }

    #[test]
    fn test_stack_implementation() {
        #[derive(Debug)]
        struct Stack<T> {
            items: Vec<T>,
        }

        impl<T> Stack<T> {
            fn new() -> Self {
                Stack { items: Vec::new() }
            }

            fn push(&mut self, item: T) {
                self.items.push(item);
            }

            fn pop(&mut self) -> Option<T> {
                self.items.pop()
            }

            fn is_empty(&self) -> bool {
                self.items.is_empty()
            }

            fn len(&self) -> usize {
                self.items.len()
            }
        }

        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);

        stack.push(1);
        stack.push(2);
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_generic_constraints() {
        fn compare_and_print<T: std::fmt::Debug + PartialEq>(a: T, b: T) -> bool {
            a == b
        }

        assert!(compare_and_print(5, 5));
        assert!(!compare_and_print(5, 10));
        assert!(compare_and_print("hello", "hello"));
    }

    #[test]
    fn test_cache_implementation() {
        #[derive(Debug)]
        struct Cache<K, V> {
            data: std::collections::HashMap<K, V>,
            capacity: usize,
        }

        impl<K, V> Cache<K, V>
        where
            K: std::hash::Hash + Eq + Clone,
            V: Clone,
        {
            fn new(capacity: usize) -> Self {
                Cache {
                    data: std::collections::HashMap::new(),
                    capacity,
                }
            }

            fn get(&self, key: &K) -> Option<V> {
                self.data.get(key).cloned()
            }

            fn set(&mut self, key: K, value: V) {
                if self.data.len() >= self.capacity {
                    self.data.clear();
                }
                self.data.insert(key, value);
            }

            fn contains_key(&self, key: &K) -> bool {
                self.data.contains_key(key)
            }

            fn size(&self) -> usize {
                self.data.len()
            }
        }

        let mut cache = Cache::new(2);
        cache.set("key1".to_string(), 100);
        cache.set("key2".to_string(), 200);

        assert_eq!(cache.size(), 2);
        assert_eq!(cache.get(&"key1".to_string()), Some(100));
        assert!(cache.contains_key(&"key2".to_string()));

        // 测试容量限制
        cache.set("key3".to_string(), 300);
        assert_eq!(cache.size(), 1); // 应该被清空
    }

    #[test]
    fn test_associated_types() {
        trait Container {
            type Item;
            fn get(&self) -> Self::Item;
        }

        struct NumberContainer<T> {
            value: T,
        }

        impl<T> Container for NumberContainer<T> {
            type Item = T;

            fn get(&self) -> Self::Item {
                self.value.clone()
            }
        }

        let container = NumberContainer { value: 42 };
        assert_eq!(container.get(), 42);
    }

    #[test]
    fn test_const_generics() {
        fn create_array<T, const N: usize>() -> [T; N]
        where
            T: Default,
        {
            [T::default(); N]
        }

        let array: [i32; 3] = create_array();
        assert_eq!(array, [0, 0, 0]);

        let string_array: [String; 2] = create_array();
        assert_eq!(string_array, ["".to_string(), "".to_string()]);
    }
}