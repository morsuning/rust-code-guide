#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_macros, deprecated)]

// Rust 特征系统 (Traits)
// 深入讲解 Rust 中 trait 的定义、实现、高级用法和最佳实践
// Trait 是 Rust 实现抽象和多态的核心机制，也是类型安全的基础



// ===========================================
// Rust 特征 (Traits) 教程的本质与定义 (Trait Essence and Definition)
// ===========================================

// Trait 是 Rust 中定义共享行为的抽象机制
// 它允许我们定义类型必须实现的方法集合，从而实现多态
// Trait 的设计哲学：编译时接口抽象 + 运行时零成本抽象

fn trait_essence_and_definition() {
    println!("=== Trait 的本质与定义 ===");

    // Trait 的核心作用：
    // 1. 定义接口契约：规定类型必须实现的方法
    // 2. 实现多态：允许不同类型以统一方式处理
    // 3. 约束泛型：为泛型类型提供行为约束
    // 4. 扩展功能：为现有类型添加新功能

    // 定义一个简单的 Summary trait
    // 这个 trait 定义了任何可以被摘要的行为
    trait Summary {
        // 必须实现的方法：每个实现 Summary 的类型都必须提供这个方法
        fn summarize(&self) -> String;

        // 可以提供默认实现：类型可以选择重写或使用默认行为
        fn summarize_brief(&self) -> String {
            format!("摘要: {}", self.summarize())
        }
    }

    // 为 Article 结构体实现 Summary trait
    // 这种实现方式是 Rust 的核心特性：为任何类型实现任何 trait
    #[derive(Debug)]
    struct Article {
        title: String,
        author: String,
        content: String,
        published: bool,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            // 使用 format! 宏创建格式化字符串
            // 这是 Rust 中字符串插值的标准方式
            if self.published {
                format!("《{}》- {}", self.title, self.author)
            } else {
                format!("《{}》(草稿)- {}", self.title, self.author)
            }
        }

        // 重写默认实现，提供更简洁的摘要
        fn summarize_brief(&self) -> String {
            format!("{}的简短摘要", self.title)
        }
    }

    // 为 Tweet 结构体实现 Summary trait
    // 演示同一个 trait 可以在不同类型上有不同的实现
    #[derive(Debug)]
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
        likes: u32,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            // Tweet 的摘要逻辑与 Article 完全不同
            // 这体现了 trait 的灵活性：只约束方法签名，不约束实现逻辑
            if self.retweet {
                format!("RT @{}: {}", self.username, self.content)
            } else if self.reply {
                format!("@{} 回复: {}", self.username, self.content)
            } else {
                format!("@{}: {}", self.username, self.content)
            }
        }
    }

    // 创建实例并调用 trait 方法
    let article = Article {
        title: String::from("Rust 内存安全机制深度解析"),
        author: String::from("张三"),
        content: String::from("本文深入探讨 Rust 的所有权系统如何保证内存安全..."),
        published: true,
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("刚学完所有权系统，感觉 Rust 的设计真的很优雅！"),
        reply: false,
        retweet: false,
        likes: 42,
    };

    // 调用 trait 方法
    // 注意：虽然 Article 和 Tweet 是不同的类型，但都实现了 Summary trait
    // 这使得我们可以用相同的方式调用 summarize 方法
    println!("文章摘要: {}", article.summarize());
    println!("文章简短摘要: {}", article.summarize_brief());
    println!("推文摘要: {}", tweet.summarize());
    println!("推文简短摘要: {}", tweet.summarize_brief());

    // Trait 设计的最佳实践：
    // 1. 保持 trait 专注：每个 trait 应该只定义一组相关的行为
    // 2. 提供合理的默认实现：减少实现者的工作量
    // 3. 考虑向后兼容：添加新方法时最好提供默认实现
    // 4. 文档清晰：详细说明每个方法的预期行为和使用场景

    println!();
}

// ===========================================
// 2. 默认实现与方法重写 (Default Implementation and Method Override)
// ===========================================

// Trait 的默认实现是 Rust 的重要特性，它允许我们：
// 1. 为方法提供默认行为，实现者可以选择是否重写
// 2. 向后兼容地扩展 trait，而不破坏现有实现
// 3. 减少样板代码，提高开发效率

fn default_implementation_and_override() {
    println!("=== 默认实现与方法重写 ===");

    // 定义一个带有默认实现的 trait
    // 这个设计展示了如何提供灵活而实用的默认行为
    trait Processor {
        // 必须实现的方法：定义核心处理逻辑
        fn process(&self, data: &str) -> String;

        // 默认实现的方法：提供通用的预处理逻辑
        fn preprocess(&self, data: &str) -> String {
            // 默认的预处理：去除前后空白字符
            data.trim().to_string()
        }

        // 默认实现的方法：提供通用的后处理逻辑
        fn postprocess(&self, result: &str) -> String {
            // 默认的后处理：添加处理完成标记
            format!("[PROCESSED] {}", result)
        }

        // 使用默认实现的方法：提供完整的处理流程
        fn full_process(&self, data: &str) -> String {
            let preprocessed = self.preprocess(data);
            let processed = self.process(&preprocessed);
            self.postprocess(&processed)
        }
    }

    // 文本处理器：完全使用默认实现
    // 这种方式适合简单场景，快速实现基本功能
    struct SimpleTextProcessor;

    impl Processor for SimpleTextProcessor {
        fn process(&self, data: &str) -> String {
            // 简单的处理：转换为大写
            data.to_uppercase()
        }
    }

    // 高级文本处理器：重写部分默认实现
    // 这种方式提供了更多的自定义能力
    struct AdvancedTextProcessor {
        remove_numbers: bool,
    }

    impl Processor for AdvancedTextProcessor {
        fn process(&self, data: &str) -> String {
            // 高级处理：根据配置决定是否移除数字
            if self.remove_numbers {
                data.chars()
                    .filter(|&c| !c.is_numeric())
                    .collect()
            } else {
                data.to_string()
            }
        }

        // 重写预处理方法，添加更多清理逻辑
        fn preprocess(&self, data: &str) -> String {
            // 先调用默认的预处理（trim）
            let cleaned = data.trim().to_string();
            // 额外的清理：移除多余的空格
            cleaned.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        }

        // 重写后处理方法，添加时间戳
        fn postprocess(&self, result: &str) -> String {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            format!("[PROCESSED@{}] {}", timestamp, result)
        }
    }

    // 测试不同的处理器
    let simple_processor = SimpleTextProcessor;
    let advanced_processor = AdvancedTextProcessor { remove_numbers: true };

    let test_data = "  Hello World 123  ";

    println!("原始数据: '{}'", test_data);
    println!("简单处理器结果: '{}'", simple_processor.full_process(test_data));
    println!("高级处理器结果: '{}'", advanced_processor.full_process(test_data));

    // 默认实现的优势：
    // 1. 代码复用：避免在每个实现中重复相同的逻辑
    // 2. 灵活性：实现者可以选择使用默认实现或自定义实现
    // 3. 向后兼容：添加新方法不会破坏现有代码
    // 4. 渐进式改进：可以从简单实现开始，逐步添加自定义逻辑

    // 注意事项：
    // 1. 默认实现不能访问实现类型的私有字段
    // 2. 如果默认实现依赖于其他方法，确保这些方法有合理的默认行为
    // 3. 考虑默认实现的性能影响，避免不必要的计算

    println!();
}

// ===========================================
// 3. Trait Bound 与泛型约束 (Trait Bounds and Generic Constraints)
// ===========================================

// Trait Bound 是 Rust 泛型编程的核心，它允许我们：
// 1. 约束泛型类型必须实现特定的 trait
// 2. 在编译时保证类型安全
// 3. 实现编译时多态，零运行时开销

fn trait_bounds_and_generic_constraints() {
    println!("=== Trait Bound 与泛型约束 ===");

    // 定义一些基本的 trait 用于演示
    trait Displayable {
        fn display(&self) -> String;
    }

    trait Comparable {
        fn compare(&self, other: &Self) -> i32;
    }

    trait Processable {
        fn process(&mut self);
    }

    // Trait Bound 的不同语法形式
    // 形式 1：impl Trait 语法（简洁，适用于函数参数）
    fn notify_displayable(item: &impl Displayable) {
        println!("显示项目: {}", item.display());
    }

    // 形式 2：完整的泛型语法（更灵活，适用于复杂约束）
    fn notify_and_compare<T: Displayable + Comparable>(item1: &T, item2: &T) {
        println!("项目1: {}", item1.display());
        println!("项目2: {}", item2.display());
        let comparison = item1.compare(item2);
        match comparison {
            0 => println!("项目相等"),
            1 => println!("项目1大于项目2"),
            -1 => println!("项目1小于项目2"),
            _ => println!("比较结果: {}", comparison),
        }
    }

    // 形式 3：where 子句（最清晰，适用于复杂约束）
    fn process_and_display<T, U>(processor: &T, data: &mut U)
    where
        T: Displayable,
        U: Processable + Displayable,
    {
        println!("处理器: {}", processor.display());
        data.process();
        println!("处理后的数据: {}", data.display());
    }

    // 实现具体的类型
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Displayable for Point {
        fn display(&self) -> String {
            format!("Point({}, {})", self.x, self.y)
        }
    }

    impl Comparable for Point {
        fn compare(&self, other: &Self) -> i32 {
            // 比较两个点的距离原点的距离
            let dist1 = (self.x * self.x + self.y * self.y) as f64;
            let dist2 = (other.x * other.x + other.y * other.y) as f64;
            if dist1 > dist2 { 1 }
            else if dist1 < dist2 { -1 }
            else { 0 }
        }
    }

    #[derive(Debug)]
    struct DataProcessor {
        name: String,
    }

    impl Displayable for DataProcessor {
        fn display(&self) -> String {
            format!("处理器 '{}'", self.name)
        }
    }

    #[derive(Debug)]
    struct DataBuffer {
        content: String,
        processed: bool,
    }

    impl Displayable for DataBuffer {
        fn display(&self) -> String {
            if self.processed {
                format!("数据(已处理): {}", self.content)
            } else {
                format!("数据(未处理): {}", self.content)
            }
        }
    }

    impl Processable for DataBuffer {
        fn process(&mut self) {
            self.content = self.content.to_uppercase();
            self.processed = true;
        }
    }

    // 测试各种 trait bound
    let point1 = Point { x: 3, y: 4 };
    let point2 = Point { x: 1, y: 2 };

    let processor = DataProcessor {
        name: "文本处理器".to_string(),
    };

    let mut buffer = DataBuffer {
        content: "Hello World".to_string(),
        processed: false,
    };

    notify_displayable(&point1);
    notify_and_compare(&point1, &point2);
    process_and_display(&processor, &mut buffer);

    // 高级 trait bound 用法
    // 1. 生命周期约束
    fn longest_with_display<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: Displayable + Comparable,
    {
        if x.compare(y) >= 0 { x } else { y }
    }

    // 2. 多个泛型参数的约束
    fn transfer_data<T, U>(source: &T, destination: &mut U)
    where
        T: Displayable,
        U: Processable + Displayable,
    {
        println!("从 '{}' 传输数据", source.display());
        destination.process();
        println!("传输完成到 '{}'", destination.display());
    }

    // 3. trait bound 的组合使用
    fn process_collection<T>(items: &mut [T])
    where
        T: Processable + Displayable,
    {
        for item in items.iter_mut() {
            item.process();
        }
        println!("处理完成后的集合:");
        for item in items.iter() {
            println!("  {}", item.display());
        }
    }

    // Trait Bound 的最佳实践：
    // 1. 优先使用 impl Trait 语法，更简洁直观
    // 2. 对于复杂约束，使用 where 子句提高可读性
    // 3. 只约束必要的 trait，避免过度约束
    // 4. 考虑使用 trait 别名简化复杂的约束组合

    println!();
}

// ===========================================
// 4. Trait 对象与动态分发 (Trait Objects and Dynamic Dispatch)
// ===========================================

// Trait 对象是 Rust 实现运行时多态的机制
// 它允许我们在运行时处理不同类型的实例，只要它们实现了相同的 trait

fn trait_objects_and_dynamic_dispatch() {
    println!("=== Trait 对象与动态分发 ===");

    // 定义一个可绘制的 trait
    // 注意：trait 对象要求 trait 是对象安全的（object-safe）
    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64;
        fn description(&self) -> String {
            format!("面积: {:.2}", self.area())
        }
        fn clone_box(&self) -> Box<dyn Drawable>;
    }

    // 实现不同的图形类型
    #[derive(Debug, Clone)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
        color: String,
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("🔵 绘制圆形: 位置({:.1}, {:.1}), 半径{:.1}, 颜色{}",
                     self.x, self.y, self.radius, self.color);
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn description(&self) -> String {
            format!("{}圆形，面积: {:.2}", self.color, self.area())
        }

        fn clone_box(&self) -> Box<dyn Drawable> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug, Clone)]
    struct Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: String,
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("🔲 绘制矩形: 位置({:.1}, {:.1}), 尺寸{:.1}x{:.1}, 颜色{}",
                     self.x, self.y, self.width, self.height, self.color);
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn description(&self) -> String {
            format!("{}矩形，面积: {:.2}", self.color, self.area())
        }

        fn clone_box(&self) -> Box<dyn Drawable> {
            Box::new(self.clone())
        }
    }

    // 图形管理器：使用 trait 对象集合
    struct ShapeManager {
        shapes: Vec<Box<dyn Drawable>>,
        total_area: f64,
    }

    impl std::fmt::Debug for ShapeManager {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "ShapeManager {{ shapes: [{} shapes], total_area: {:.2} }}",
                   self.shapes.len(), self.total_area)
        }
    }

    impl ShapeManager {
        fn new() -> Self {
            ShapeManager {
                shapes: Vec::new(),
                total_area: 0.0,
            }
        }

        // 添加图形：接受任何实现了 Drawable 的类型
        fn add_shape(&mut self, shape: Box<dyn Drawable>) {
            self.total_area += shape.area();
            self.shapes.push(shape);
        }

        // 绘制所有图形：使用 trait 对象进行动态分发
        fn draw_all(&self) {
            println!("=== 绘制所有图形 ===");
            for shape in &self.shapes {
                shape.draw();
            }
        }

        // 计算总面积
        fn get_total_area(&self) -> f64 {
            self.total_area
        }

        // 获取图形描述
        fn get_descriptions(&self) -> Vec<String> {
            self.shapes.iter()
                .map(|shape| shape.description())
                .collect()
        }

        // 查找最大图形
        fn find_largest(&self) -> Option<&dyn Drawable> {
            self.shapes.iter()
                .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap_or(std::cmp::Ordering::Equal))
                .map(|boxed| boxed.as_ref())
        }
    }

    // 创建不同类型的图形
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle {
            x: 0.0, y: 0.0, radius: 5.0, color: "红色".to_string(),
        }),
        Box::new(Rectangle {
            x: 10.0, y: 10.0, width: 8.0, height: 6.0, color: "蓝色".to_string(),
        }),
        Box::new(Circle {
            x: 20.0, y: 20.0, radius: 3.0, color: "绿色".to_string(),
        }),
    ];

    // 使用图形管理器
    let mut manager = ShapeManager::new();
    for shape in shapes {
        manager.add_shape(shape);
    }

    manager.draw_all();
    println!("总面积: {:.2}", manager.get_total_area());

    println!("\n图形描述:");
    for desc in manager.get_descriptions() {
        println!("  {}", desc);
    }

    if let Some(largest) = manager.find_largest() {
        println!("\n最大图形的面积: {:.2}", largest.area());
    }

    // Trait 对象的特点和注意事项：
    // 1. 动态分发：在运行时确定调用哪个实现
    // 2. 对象安全：trait 必须满足特定条件才能作为 trait 对象
    // 3. 内存布局：trait 对象是胖指针，包含数据和虚表指针
    // 4. 性能考虑：静态分发（泛型）通常比动态分发更高效

    // 对象安全的条件：
    // - trait 不能有泛型类型参数
    // - trait 不能包含 Self 类型（除了方法的第一个参数）
    // - trait 的方法不能使用 Self 语法糖
    // - trait 的所有方法必须符合对象安全的要求

    println!();
}

// ===========================================
// 5. 关联类型与泛型关联类型 (Associated Types and Generic Associated Types)
// ===========================================

// 关联类型是 trait 中的类型占位符，它让 trait 更加灵活和类型安全

fn associated_types_and_gat() {
    println!("=== 关联类型与泛型关联类型 ===");

    // 标准库的 Iterator trait 的简化版本
    // 关联类型 Item 让 Iterator trait 能够为每个实现指定具体的迭代项类型
    trait Iterator {
        type Item; // 关联类型：指定迭代器产生的元素类型

        fn next(&mut self) -> Option<Self::Item>;

        // 其他方法可以使用 Item 类型
        fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized,
        {
            // 这是一个简化版本，实际的 collect 更复杂
            unimplemented!()
        }
    }

    // 实现一个简单的计数器迭代器
    #[derive(Debug)]
    struct Counter {
        current: u32,
        max: u32,
    }

    impl Iterator for Counter {
        type Item = u32; // 指定迭代项类型为 u32

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let result = self.current;
                self.current += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    // 自定义容器 trait
    // 关联类型让容器可以指定它们存储的元素类型
    trait Container {
        type Item; // 关联类型：容器存储的元素类型

        fn get(&self, index: usize) -> Option<&Self::Item>;
        fn put(&mut self, item: Self::Item);
        fn len(&self) -> usize;

        // 使用关联类型的方法
        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn iter(&self) -> std::vec::IntoIter<&Self::Item> {
            // 简化实现：返回向量元素的迭代器
            let mut items = Vec::new();
            for i in 0..self.len() {
                if let Some(item) = self.get(i) {
                    items.push(item);
                }
            }
            items.into_iter()
        }
    }

    // 简单的数组容器实现
    #[derive(Debug)]
    struct ArrayContainer<T, const N: usize> {
        data: [Option<T>; N],
        size: usize,
    }

    impl<T, const N: usize> Container for ArrayContainer<T, N> {
        type Item = T;

        fn get(&self, index: usize) -> Option<&Self::Item> {
            if index < self.size {
                self.data[index].as_ref()
            } else {
                None
            }
        }

        fn put(&mut self, item: Self::Item) {
            if self.size < N {
                self.data[self.size] = Some(item);
                self.size += 1;
            }
        }

        fn len(&self) -> usize {
            self.size
        }
    }

    
    // 测试关联类型的使用
    let mut counter = Counter { current: 0, max: 5 };
    println!("计数器迭代:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }

    let mut container = ArrayContainer::<i32, 5> {
        data: [None, None, None, None, None],
        size: 0,
    };

    container.put(10);
    container.put(20);
    container.put(30);

    println!("\n容器内容:");
    for i in 0..container.len() {
        if let Some(item) = container.get(i) {
            println!("  {}: {:?}", i, item);
        }
    }

    println!("\n使用迭代器遍历容器:");
    for item in container.iter() {
        println!("  {:?}", item);
    }

    // 关联类型的优势：
    // 1. 类型安全：编译器保证类型一致性
    // 2. 语义清晰：明确表达 trait 与类型的关系
    // 3. 灵活性：一个 trait 可以有不同的类型参数
    // 4. 减少泛型参数：避免过多的泛型类型参数

    // 与泛型参数的区别：
    // - 泛型参数：每个具体类型产生不同的实现
    // - 关联类型：每个类型只能有一个实现，但可以指定不同的关联类型

    println!();
}

// ===========================================
// 6. 运算符重载与默认泛型参数 (Operator Overloading and Default Generic Parameters)
// ===========================================

// Rust 允许我们通过实现特定的 trait 来重载运算符
// 这让自定义类型可以像内置类型一样使用运算符

fn operator_overloading_and_default_generics() {
    println!("=== 运算符重载与默认泛型参数 ===");

    use std::ops::Add;

    // 二维向量类型
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Vector2D {
        x: f64,
        y: f64,
    }

    // 实现向量与向量的加法
    impl Add for Vector2D {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Vector2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    // 实现向量与标量的加法
    impl Add<f64> for Vector2D {
        type Output = Self;

        fn add(self, scalar: f64) -> Self::Output {
            Vector2D {
                x: self.x + scalar,
                y: self.y + scalar,
            }
        }
    }

    // 复数类型
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl Add for Complex {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Complex {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }

    // 其他运算符的实现
    use std::ops::{Sub, Mul, Div, Neg};

    impl Sub for Vector2D {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Vector2D {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul<f64> for Vector2D {
        type Output = Self;

        fn mul(self, scalar: f64) -> Self::Output {
            Vector2D {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    impl Neg for Vector2D {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vector2D {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    // 测试运算符重载
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };

    println!("向量运算示例:");
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v1 + v2 = {:?}", v1 + v2);
    println!("v1 - v2 = {:?}", v1 - v2);
    println!("v1 * 2.5 = {:?}", v1 * 2.5);
    println!("-v1 = {:?}", -v1);
    println!("v1 + 1.5 = {:?}", v1 + 1.5);

    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };

    println!("\n复数运算示例:");
    println!("c1 = {:?}", c1);
    println!("c2 = {:?}", c2);
    println!("c1 + c2 = {:?}", c1 + c2);

    // 运算符重载的最佳实践：
    // 1. 语义一致性：运算符的行为应该符合数学直觉
    // 2. 性能考虑：运算符通常被频繁调用，要确保高效
    // 3. 错误处理：考虑运算失败的情况（如除零）
    // 4. 文档清晰：说明运算符的具体行为

    // 默认泛型参数的优势：
    // 1. 向后兼容：可以添加新的泛型参数而不破坏现有代码
    // 2. 简化常见用例：最常见的情况使用默认值
    // 3. 灵活性：需要时可以指定不同的类型参数
    // 4. 逐步改进：可以从简单实现开始，逐步添加功能

    println!();
}

// ===========================================
// 7. 完全限定语法与消歧义 (Fully Qualified Syntax and Disambiguation)
// ===========================================

// 当多个 trait 定义了相同的方法时，需要使用完全限定语法来明确调用

fn fully_qualified_syntax_and_disambiguation() {
    println!("=== 完全限定语法与消歧歧义 ===");

    trait Pilot {
        fn fly(&self);
        fn take_off(&self);
    }

    trait Wizard {
        fn fly(&self);
        fn cast_spell(&self);
    }

    // Human 类型实现了两个 trait，都有 fly 方法
    struct Human {
        name: String,
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("{}(飞行员): 准备起飞，检查仪表...", self.name);
        }

        fn take_off(&self) {
            println!("{}(飞行员): 起飞！", self.name);
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("{}(巫师): 念咒语，悬浮起来！", self.name);
        }

        fn cast_spell(&self) {
            println!("{}(巫师): 施展魔法！", self.name);
        }
    }

    impl Human {
        // Human 本身也有一个 fly 方法
        fn fly(&self) {
            println!("{}(普通人): 挥动手臂，想象自己在飞...", self.name);
        }

        // 自身特有的方法
        fn walk(&self) {
            println!("{}: 正常走路", self.name);
        }
    }

    let person = Human {
        name: "张三".to_string(),
    };

    // 默认调用：调用类型自身的方法
    person.fly();
    person.walk();

    // 使用完全限定语法调用特定 trait 的方法
    // 语法：<Type as Trait>::method(&instance)
    Pilot::fly(&person);
    Wizard::fly(&person);

    Pilot::take_off(&person);
    Wizard::cast_spell(&person);

    // 关联函数的冲突解决
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            "小狗".to_string()
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            "幼犬".to_string()
        }
    }

    println!("\n关联函数消歧义:");
    println!("Dog::baby_name() = {}", Dog::baby_name());
    println!("<Dog as Animal>::baby_name() = {}", <Dog as Animal>::baby_name());

    // 更复杂的场景：泛型函数中的 trait 消歧义
    trait Display {
        fn display(&self) -> String;
    }

    trait Debug {
        fn debug(&self) -> String;
    }

    struct MyStruct(i32);

    impl Display for MyStruct {
        fn display(&self) -> String {
            format!("Display: {}", self.0)
        }
    }

    impl Debug for MyStruct {
        fn debug(&self) -> String {
            format!("Debug: MyStruct({})", self.0)
        }
    }

    // 同时需要两个 trait 的函数
    fn show_both<T: Display + Debug>(item: &T) {
        println!("Display: {}", item.display());
        println!("Debug: {}", item.debug());
    }

    let item = MyStruct(42);
    show_both(&item);

    // 完全限定语法的使用场景：
    // 1. 方法名冲突时明确调用特定的实现
    // 2. 在泛型代码中指定确切的 trait 实现
    // 3. 调用关联函数时需要区分不同 trait
    // 4. 在文档和注释中明确指出使用哪个实现

    println!();
}

// ===========================================
// 8. 父 trait 与 trait 层次 (Supertraits and Trait Hierarchy)
// ===========================================

// 父 trait 允许我们构建 trait 的层次结构，一个 trait 可以依赖于另一个 trait

fn supertraits_and_trait_hierarchy() {
    println!("=== 父 trait 与 trait 层次 ===");

    use std::fmt::Display;

    // 定义一个需要 Display 的父 trait
    // OutlinePrint 只能为实现了 Display 的类型实现
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string(); // 可以调用 Display 的方法
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // 定义一个更复杂的 trait 层次
    trait Shape: Display {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    trait ColoredShape: Shape {
        fn color(&self) -> &str;
        fn colored_description(&self) -> String {
            format!("{} {}", self.color(), self)
        }
    }

    // 实现具体的形状
    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
        color: String,
    }

    // 必须先实现 Display
    impl Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "圆形(中心({:.1},{:.1}), 半径{:.1})", self.x, self.y, self.radius)
        }
    }

    // 然后实现 Shape
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    // 实现 OutlinePrint
    impl OutlinePrint for Circle {}

    // 最后实现 ColoredShape
    impl ColoredShape for Circle {
        fn color(&self) -> &str {
            &self.color
        }
    }

    // 测试 trait 层次
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
        color: "红色".to_string(),
    };

    println!("使用父 trait:");
    circle.outline_print();
    println!("面积: {:.2}", circle.area());
    println!("周长: {:.2}", circle.perimeter());
    println!("着色描述: {}", circle.colored_description());

    // 多层次的父 trait
    trait Serialize: Display {
        fn serialize(&self) -> String;
    }

    trait JsonSerializable: Serialize {
        fn to_json(&self) -> String {
            format!("{{\"value\": \"{}\"}}", self.serialize())
        }
    }

    #[derive(Debug)]
    struct DataPoint {
        name: String,
        value: i32,
    }

    impl Display for DataPoint {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}: {}", self.name, self.value)
        }
    }

    impl Serialize for DataPoint {
        fn serialize(&self) -> String {
            format!("{}={}", self.name, self.value)
        }
    }

    impl JsonSerializable for DataPoint {}

    let data = DataPoint {
        name: "温度".to_string(),
        value: 25,
    };

    println!("\n多层次父 trait:");
    println!("序列化: {}", data.serialize());
    println!("JSON: {}", data.to_json());

    // 父 trait 的设计原则：
    // 1. 功能相关性：父 trait 应该提供必要的功能
    // 2. 最小化约束：只要求真正需要的方法
    // 3. 组合优于继承：通过组合多个 trait 构建复杂功能
    // 4. 考虑实现成本：父 trait 的方法应该容易实现

    println!();
}

// ===========================================
// 9. Newtype 模式与类型安全 (Newtype Pattern and Type Safety)
// ===========================================

// Newtype 模式是 Rust 中实现类型安全和抽象的重要模式

fn newtype_pattern_and_type_safety() {
    println!("=== Newtype 模式与类型安全 ===");

    // 基本的 Newtype 模式：包装现有类型
    struct Meters(u32);
    struct Centimeters(u32);
    struct Kilometers(u32);

    // 为 Newtype 实现特定行为
    impl Meters {
        fn new(value: u32) -> Self {
            Meters(value)
        }

        fn value(&self) -> u32 {
            self.0
        }

        fn to_centimeters(&self) -> Centimeters {
            Centimeters(self.0 * 100)
        }

        fn to_kilometers(&self) -> Kilometers {
            Kilometers(self.0 / 1000)
        }
    }

    impl Centimeters {
        fn to_meters(&self) -> Meters {
            Meters(self.0 / 100)
        }
    }

    impl Kilometers {
        fn to_meters(&self) -> Meters {
            Meters(self.0 * 1000)
        }
    }

    // 为 Newtype 实现运算符和 Clone
    use std::ops::{Add, Sub, Mul};

    impl Clone for Meters {
        fn clone(&self) -> Self {
            Meters(self.0)
        }
    }

    impl Add for Meters {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Meters(self.0 + other.0)
        }
    }

    impl Sub for Meters {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Meters(self.0 - other.0)
        }
    }

    impl Mul<u32> for Meters {
        type Output = Self;

        fn mul(self, scalar: u32) -> Self::Output {
            Meters(self.0 * scalar)
        }
    }

    // 为 Newtype 实现 Display
    use std::fmt::Display;

    impl Display for Meters {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}米", self.0)
        }
    }

    impl Display for Centimeters {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}厘米", self.0)
        }
    }

    impl Display for Kilometers {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}千米", self.0)
        }
    }

    // 使用 Newtype 的类型安全函数
    fn calculate_distance(dist1: Meters, dist2: Meters) -> Meters {
        // 这函数只接受 Meters 类型，防止了单位混淆
        if dist1.0 > dist2.0 {
            dist1 - dist2
        } else {
            dist2 - dist1
        }
    }

    // 测试 Newtype 模式
    let distance1 = Meters::new(1000);
    let distance2 = Meters::new(500);

    println!("Newtype 模式示例:");
    println!("距离1: {}", distance1);
    println!("距离2: {}", distance2);
    println!("距离差: {}", calculate_distance(distance1.clone(), distance2));

    let centimeters = distance1.to_centimeters();
    println!("转换为厘米: {}", centimeters);

    let kilometers = Kilometers(2);
    println!("从千米转换: {}", kilometers.to_meters());

    // Newtype 模式的高级用法
    // 1. 为外部类型实现外部 trait
    struct Wrapper(Vec<String>);

    impl Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // 2. 验证和约束的 Newtype
    struct NonEmptyString(String);

    impl NonEmptyString {
        fn new(s: String) -> Result<Self, String> {
            if s.trim().is_empty() {
                Err("字符串不能为空".to_string())
            } else {
                Ok(NonEmptyString(s.trim().to_string()))
            }
        }

        fn value(&self) -> &str {
            &self.0
        }
    }

    // 3. 状态机的 Newtype
    struct UnvalidatedInput(String);
    struct ValidatedInput(String);

    impl UnvalidatedInput {
        fn validate(self) -> Result<ValidatedInput, String> {
            if self.0.len() > 3 && self.0.chars().all(|c| c.is_alphanumeric()) {
                Ok(ValidatedInput(self.0))
            } else {
                Err("输入无效".to_string())
            }
        }
    }

    let wrapper = Wrapper(vec![
        "Hello".to_string(),
        "World".to_string(),
        "Rust".to_string(),
    ]);
    println!("\n包装外部类型: {}", wrapper);

    match NonEmptyString::new("  有效内容  ".to_string()) {
        Ok(valid) => println!("有效字符串: '{}'", valid.value()),
        Err(e) => println!("错误: {}", e),
    }

    let input = UnvalidatedInput("abc123".to_string());
    match input.validate() {
        Ok(validated) => println!("验证通过: {}", validated.0),
        Err(e) => println!("验证失败: {}", e),
    }

    // Newtype 模式的优势：
    // 1. 类型安全：防止单位混淆和错误使用
    // 2. 封装：隐藏实现细节，提供清晰的接口
    // 3. 扩展性：为现有类型添加新功能
    // 4. 零成本：编译时优化，无运行时开销
    // 5. 文档性：类型名称本身就传达了语义

    println!();
}

// ===========================================
// 10. 综合示例：图形处理系统 (Comprehensive Example: Graphics Processing System)
// ===========================================

// 展示如何综合运用各种 trait 特性构建一个完整的系统

fn comprehensive_graphics_system() {
    println!("=== 综合示例：图形处理系统 ===");

    use std::fmt::Debug;

    // 核心图形 trait 层次
    trait Renderable: Debug {
        fn render(&self) -> String;
        fn bounds(&self) -> (f64, f64, f64, f64); // x, y, width, height
    }

    trait Transformable: Renderable {
        fn translate(&mut self, dx: f64, dy: f64);
        fn scale(&mut self, factor: f64);
        fn rotate(&mut self, angle: f64);
    }

    trait Interactive: Transformable {
        fn contains_point(&self, x: f64, y: f64) -> bool;
        fn on_click(&self) -> String;
        fn on_hover(&self) -> String;
    }

    // 具体图形实现
    #[derive(Debug, Clone)]
    struct Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: String,
        rotation: f64,
    }

    impl Renderable for Rectangle {
        fn render(&self) -> String {
            format!("矩形({:.1},{:.1}) {:.1}x{:.1} {} 旋转{:.1}°",
                   self.x, self.y, self.width, self.height, self.color, self.rotation)
        }

        fn bounds(&self) -> (f64, f64, f64, f64) {
            (self.x, self.y, self.width, self.height)
        }
    }

    impl Transformable for Rectangle {
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }

        fn scale(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }

        fn rotate(&mut self, angle: f64) {
            self.rotation += angle;
        }
    }

    impl Interactive for Rectangle {
        fn contains_point(&self, x: f64, y: f64) -> bool {
            x >= self.x && x <= self.x + self.width &&
            y >= self.y && y <= self.y + self.height
        }

        fn on_click(&self) -> String {
            format!("点击了{}矩形", self.color)
        }

        fn on_hover(&self) -> String {
            format!("悬停在{}矩形上", self.color)
        }
    }

    // 圆形实现
    #[derive(Debug, Clone)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
        color: String,
    }

    impl Renderable for Circle {
        fn render(&self) -> String {
            format!("圆形({:.1},{:.1}) r={:.1} {}", self.x, self.y, self.radius, self.color)
        }

        fn bounds(&self) -> (f64, f64, f64, f64) {
            (self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0)
        }
    }

    impl Transformable for Circle {
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }

        fn scale(&mut self, factor: f64) {
            self.radius *= factor;
        }

        fn rotate(&mut self, _angle: f64) {
            // 圆形旋转无视觉效果
        }
    }

    impl Interactive for Circle {
        fn contains_point(&self, x: f64, y: f64) -> bool {
            let dx = x - self.x;
            let dy = y - self.y;
            dx * dx + dy * dy <= self.radius * self.radius
        }

        fn on_click(&self) -> String {
            format!("点击了{}圆形", self.color)
        }

        fn on_hover(&self) -> String {
            format!("悬停在{}圆形上", self.color)
        }
    }

    // 图形管理器
    struct GraphicsManager {
        shapes: Vec<Box<dyn Interactive>>,
        selected_shape: Option<usize>,
    }

    impl GraphicsManager {
        fn new() -> Self {
            GraphicsManager {
                shapes: Vec::new(),
                selected_shape: None,
            }
        }

        fn add_shape(&mut self, shape: Box<dyn Interactive>) {
            self.shapes.push(shape);
        }

        fn render_all(&self) -> Vec<String> {
            self.shapes.iter()
                .map(|shape| shape.render())
                .collect()
        }

        fn handle_click(&mut self, x: f64, y: f64) -> Option<String> {
            for (i, shape) in self.shapes.iter().enumerate() {
                if shape.contains_point(x, y) {
                    self.selected_shape = Some(i);
                    return Some(shape.on_click());
                }
            }
            self.selected_shape = None;
            None
        }

        fn handle_hover(&self, x: f64, y: f64) -> Option<String> {
            for shape in self.shapes.iter() {
                if shape.contains_point(x, y) {
                    return Some(shape.on_hover());
                }
            }
            None
        }

        fn transform_selected(&mut self, dx: f64, dy: f64, scale: f64, rotation: f64) {
            if let Some(index) = self.selected_shape {
                if let Some(shape) = self.shapes.get_mut(index) {
                    shape.translate(dx, dy);
                    shape.scale(scale);
                    shape.rotate(rotation);
                }
            }
        }
    }

    // 使用图形管理器
    let mut manager = GraphicsManager::new();

    // 添加不同的图形
    manager.add_shape(Box::new(Rectangle {
        x: 10.0, y: 10.0, width: 100.0, height: 50.0,
        color: "红色".to_string(), rotation: 0.0,
    }));

    manager.add_shape(Box::new(Circle {
        x: 200.0, y: 100.0, radius: 30.0,
        color: "蓝色".to_string(),
    }));

    manager.add_shape(Box::new(Rectangle {
        x: 50.0, y: 200.0, width: 80.0, height: 80.0,
        color: "绿色".to_string(), rotation: 45.0,
    }));

    println!("渲染所有图形:");
    for rendering in manager.render_all() {
        println!("  {}", rendering);
    }

    // 模拟交互
    println!("\n模拟用户交互:");

    // 点击红色矩形
    if let Some(response) = manager.handle_click(60.0, 35.0) {
        println!("点击响应: {}", response);
    }

    // 悬停在蓝色圆形上
    if let Some(response) = manager.handle_hover(200.0, 100.0) {
        println!("悬停响应: {}", response);
    }

    // 变换选中的图形
    manager.transform_selected(10.0, 5.0, 1.1, 15.0);

    println!("\n变换后的图形:");
    for rendering in manager.render_all() {
        println!("  {}", rendering);
    }

    // 这个综合示例展示了：
    // 1. trait 层次结构的设计
    // 2. 动态分发与 trait 对象的使用
    // 3. 复杂的交互逻辑
    // 4. 类型安全的多态行为
    // 5. 实际应用场景的实现

    println!();
}



// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 特征系统深度演示");
    println!("=====================");

    trait_essence_and_definition();
    default_implementation_and_override();
    trait_bounds_and_generic_constraints();
    trait_objects_and_dynamic_dispatch();
    associated_types_and_gat();
    operator_overloading_and_default_generics();
    fully_qualified_syntax_and_disambiguation();
    supertraits_and_trait_hierarchy();
    newtype_pattern_and_type_safety();
    comprehensive_graphics_system();

    println!("特征系统演示完成！");
    println!("\n关键要点总结:");
    println!("1. Trait 是 Rust 中实现抽象和多态的核心机制");
    println!("2. Trait Bound 提供编译时类型安全和零成本抽象");
    println!("3. Trait 对象支持运行时多态，但有一定性能开销");
    println!("4. 关联类型增强了 trait 的表达能力和类型安全");
    println!("5. 父 trait 允许构建有意义的 trait 层次结构");
    println!("6. Newtype 模式提供了强大的类型安全和抽象能力");
    println!("7. 运算符重载让自定义类型更加自然和直观");
    println!("8. 完全限定语法解决了方法名冲突问题");
    println!("9. 合理的 trait 设计是高质量 Rust 代码的关键");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_bounds() {
        #[derive(Debug)]
        struct TestPoint {
            x: i32,
            y: i32,
        }

        trait TestDisplay {
            fn test_display(&self) -> String;
        }

        impl TestDisplay for TestPoint {
            fn test_display(&self) -> String {
                format!("({}, {})", self.x, self.y)
            }
        }

        fn display_test<T: TestDisplay>(item: &T) -> String {
            item.test_display()
        }

        let point = TestPoint { x: 1, y: 2 };
        assert_eq!(display_test(&point), "(1, 2)");
    }

    #[test]
    fn test_associated_types() {
        trait Container {
            type Item;
            fn get(&self) -> Self::Item;
        }

        struct StringContainer {
            value: String,
        }

        impl Container for StringContainer {
            type Item = String;
            fn get(&self) -> Self::Item {
                self.value.clone()
            }
        }

        let container = StringContainer {
            value: "test".to_string(),
        };
        assert_eq!(container.get(), "test");
    }

    #[test]
    fn test_operator_overloading() {
        use std::ops::Add;

        #[derive(Debug, PartialEq)]
        struct Vector {
            x: i32,
            y: i32,
        }

        impl Add for Vector {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Vector {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        let v1 = Vector { x: 1, y: 2 };
        let v2 = Vector { x: 3, y: 4 };
        let result = v1 + v2;
        assert_eq!(result, Vector { x: 4, y: 6 });
    }

    #[test]
    fn test_newtype_pattern() {
        struct Meters(u32);

        impl Meters {
            fn new(value: u32) -> Self {
                Meters(value)
            }

            fn value(&self) -> u32 {
                self.0
            }
        }

        impl std::ops::Add for Meters {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Meters(self.0 + other.0)
            }
        }

        let m1 = Meters::new(100);
        let m2 = Meters::new(200);
        let result = m1 + m2;
        assert_eq!(result.value(), 300);
    }

    #[test]
    fn test_trait_objects() {
        trait Animal {
            fn make_sound(&self) -> &'static str;
        }

        struct Dog;
        impl Animal for Dog {
            fn make_sound(&self) -> &'static str {
                "汪汪"
            }
        }

        struct Cat;
        impl Animal for Cat {
            fn make_sound(&self) -> &'static str {
                "喵喵"
            }
        }

        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog),
            Box::new(Cat),
        ];

        assert_eq!(animals[0].make_sound(), "汪汪");
        assert_eq!(animals[1].make_sound(), "喵喵");
    }

    #[test]
    fn test_fully_qualified_syntax() {
        trait Pilot {
            fn fly(&self) -> &'static str;
        }

        trait Wizard {
            fn fly(&self) -> &'static str;
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) -> &'static str {
                "飞行员飞行"
            }
        }

        impl Wizard for Human {
            fn fly(&self) -> &'static str {
                "巫师飞行"
            }
        }

        let person = Human;
        assert_eq!(Pilot::fly(&person), "飞行员飞行");
        assert_eq!(Wizard::fly(&person), "巫师飞行");
    }

    #[test]
    fn test_supertraits() {
        use std::fmt::Display;

        trait OutlinePrint: Display {
            fn outline_print(&self);
        }

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl OutlinePrint for Point {
            fn outline_print(&self) {
                let output = format!("{}", self);
                println!("**{}**", output);
            }
        }

        let p = Point { x: 1, y: 2 };
        // 测试 OutlinePrint 可以被调用
        p.outline_print();
    }
}

/*
// ===========================================
// 7. Rust 1.92: Type Alias Impl Trait (TAIT)
// ===========================================

// Rust 1.92 完全稳定了 Type Alias Impl Trait (TAIT)
// 允许在类型别名中使用 impl Trait，常用于隐藏具体类型或简化复杂类型（如 Future）

pub fn tait_demo() {
    println!("=== Rust 1.92: Type Alias Impl Trait (TAIT) ===");

    // 定义一个不透明类型别名
    type MyIterator = impl Iterator<Item = i32>;

    fn make_iter(n: i32) -> MyIterator {
        (0..n).map(|x| x * 2)
    }

    let iter = make_iter(5);
    for x in iter {
        print!("{} ", x);
    }
    println!();

    // 在 Trait 中使用
    trait Container {
        type Item;
        type Iter: Iterator<Item = Self::Item>;
        fn items(&self) -> Self::Iter;
    }
    
    struct MyVec(Vec<i32>);
    
    impl Container for MyVec {
        type Item = i32;
        // 使用 TAIT 简化返回类型
        type Iter = impl Iterator<Item = i32>;
        
        fn items(&self) -> Self::Iter {
            self.0.clone().into_iter()
        }
    }
    
    println!("TAIT 使得隐藏具体实现细节变得非常简单，特别是在涉及闭包或 Future 时。");
    println!();
}
*/