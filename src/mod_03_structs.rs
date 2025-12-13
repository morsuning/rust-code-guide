#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Rust 结构体（Structs）
// 深入讲解结构体定义、方法实现、所有权、泛型等核心概念
// 结构体是 Rust 中创建自定义类型的主要方式，提供了强大的数据组织能力

// ===========================================
// 1. 基础结构体（Basic Structs）
// ===========================================

// 结构体（Struct）是 Rust 中最重要的复合类型之一
// 它允许我们将多个相关的数据项组合成一个有意义的整体
// 结构体是面向对象编程中"类"概念的基础，但更加轻量和灵活

fn basic_structs() {
    println!("=== 基础结构体 ===");

    // 定义结构体：创建自定义类型
    // 结构体使用 struct 关键字定义，后跟结构体名称和花括号内的字段列表
    // 每个字段都有名称和类型，这使得结构体具有自描述性
    // 结构体名称采用大驼峰命名法（PascalCase），字段名采用蛇形命名法（snake_case）
    struct User {
        active: bool,           // 用户是否激活（布尔类型，实现 Copy trait）
        username: String,       // 用户名（字符串类型，拥有堆内存）
        email: String,          // 邮箱地址（字符串类型，拥有堆内存）
        sign_in_count: u64,     // 登录次数（无符号64位整数，实现 Copy trait）
    }

    // 结构体的设计原则：
    // 1. 相关性：所有字段应该在逻辑上相关，表达一个完整的概念
    // 2. 内聚性：结构体应该有明确的职责和用途
    // 3. 可扩展性：设计时考虑未来可能的需求变化
    // 4. 内存效率：考虑字段的大小和对齐

    // 创建结构体实例：实例化结构体类型
    // 创建实例时必须为所有字段提供值，顺序可以任意但需要指定字段名
    // 字段初始化语法：字段名: 值
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 内存布局理解：
    // - 结构体在内存中是连续存储的
    // - 字段按照定义的顺序排列，考虑内存对齐
    // - 整体大小是所有字段大小之和加上可能的对齐填充
    // - 结构体实例通常在栈上分配，但包含堆数据的字段在堆上分配

    println!("用户信息: 活跃={}, 用户名={}, 邮箱={}, 登录次数={}",
             user1.active, user1.username, user1.email, user1.sign_in_count);

    // 可变结构体：使用 mut 关键字允许修改字段
    // Rust 默认变量是不可变的，结构体也不例外
    // 需要修改结构体字段时，必须将实例声明为 mut
    let mut user2 = User {
        active: false,
        username: String::from("anotherusername"),
        email: String::from("another@example.com"),
        sign_in_count: 1,
    };

    // 修改字段：通过点号语法访问和修改字段
    // 只有声明为 mut 的结构体实例才能修改字段
    // 字段修改遵循 Rust 的所有权规则
    user2.email = String::from("anotheremail@example.com");
    println!("修改后的邮箱: {}", user2.email);

    // 结构体更新语法：基于现有实例创建新实例
    // ..语法表示使用另一个实例的剩余字段值
    // 这是一种便捷的语法，避免重复编写相同的字段值
    // 注意：..user2 会移动 user2 中未显式指定的字段
    let user3 = User {
        email: String::from("yetanother@example.com"),
        ..user2 // 使用 user2 的其他字段值（active, username, sign_in_count）
    };

    // 所有权转移说明：
    // - 如果字段类型实现了 Copy trait，则进行复制
    // - 如果字段类型没有实现 Copy trait（如 String），则发生移动
    // - user2 的 username 和 email 字段被移动后，user2 不能再使用这些字段

    println!("更新语法创建的用户: 邮箱={}, 用户名={}",
             user3.email, user3.username);
    // println!("{}", user2.username); // 编译错误：user2.username 已被移动

    // 元组结构体（Tuple Structs）：具名元组类型
    // 元组结构体有名称但没有字段名，字段通过索引访问
    // 适用于当字段名不重要，但类型本身有意义的场景
    struct Color(i32, i32, i32);      // RGB 颜色值
    struct Point(i32, i32, i32);      // 3D 坐标点

    // 元组结构体的应用场景：
    // 1. 包装器：为现有类型提供新的语义
    // 2. 抽象：隐藏具体实现细节
    // 3. 类型安全：防止混淆不同类型但相同结构的数据

    let black = Color(0, 0, 0);          // 黑色
    let origin = Point(0, 0, 0);         // 原点
    println!("颜色: ({}, {}, {})", black.0, black.1, black.2);
    println!("点: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 类单元结构体（Unit-like Structs）：没有字段的空结构体
    // 类单元结构体类似于单元类型 ()，但有自己的类型
    // 适用于实现 trait 但不需要存储数据的场景
    struct AlwaysEqual;                   // 总是相等的类型

    let subject = AlwaysEqual;
    println!("类单元结构体创建成功");

    // 类单元结构体的用途：
    // 1. 标记类型：用于表示某些状态或特性
    // 2. 特征实现：实现 trait 但不需要数据
    // 3. 单例模式：全局唯一的实例类型

    // 结构体选择指南：
    // 1. 常规结构体：当字段名有重要语义时使用
    // 2. 元组结构体：当字段名不重要，但类型区分很重要时使用
    // 3. 类单元结构体：当只需要类型而不需要数据时使用

    println!();
}

// ===========================================
// 2. 结构体方法（Methods）
// ===========================================

// 方法（Methods）是与结构体关联的函数
// 方法的第一个参数总是 self，表示方法被调用的实例
// 方法让结构体不仅能够存储数据，还能定义行为

fn struct_methods() {
    println!("=== 结构体方法 ===");

    // 定义带有方法的结构体
    // 使用 #[derive(Debug)] 属性自动生成调试格式的实现
    #[derive(Debug)]
    struct Rectangle {
        width: u32,       // 宽度
        height: u32,      // 高度
    }

    // impl 块：为结构体实现方法
    // impl 块用于定义结构体的行为，包括方法和关联函数
    // 一个结构体可以有多个 impl 块，便于代码组织
    impl Rectangle {
        // 实例方法：以 &self 为第一个参数
        // &self 表示对实例的不可变借用，方法不会修改实例
        // self 是 &Rectangle 的语法糖
        fn area(&self) -> u32 {
            // 可以通过 self 访问实例的字段和方法
            self.width * self.height
        }

        // 可变方法：以 &mut self 为第一个参数
        // &mut self 表示对实例的可变借用，方法可以修改实例
        // 用于实现修改状态的操作
        fn set_width(&mut self, width: u32) {
            self.width = width;
        }

        // 消费方法：以 self 为第一个参数
        // self 表示获取实例的所有权，方法消耗实例
        // 实例被移动到方法中，之后不能再使用
        // 用于实现转换或清理操作
        fn consume(self) {
            println!("消耗了矩形: {}x{}", self.width, self.height);
            // 方法结束时，self 被丢弃，相关内存被释放
        }

        // 关联函数（Associated Functions）：不以 self 为参数
        // 关联函数类似于其他语言中的静态方法
        // 常用于构造函数或工具函数
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }

        // 带多个参数的方法
        // 方法可以接受任意数量的参数，除了 self
        fn can_hold(&self, other: &Rectangle) -> bool {
            // 检查当前矩形是否能容纳另一个矩形
            self.width > other.width && self.height > other.height
        }

        // 链式调用方法：返回 &mut self 以支持方法链
        fn resize(&mut self, width: u32, height: u32) -> &mut Self {
            self.width = width;
            self.height = height;
            self  // 返回可变引用
        }

        // 只读方法：返回不可变引用
        fn dimensions(&self) -> (u32, u32) {
            (self.width, self.height)
        }
    }

    // 创建结构体实例
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 调用方法：使用点号语法
    // 方法调用会自动处理 self 参数的借用
    println!("矩形: {:?}", rect1);
    println!("面积: {}", rect1.area());

    // 调用可变方法
    rect1.set_width(40);
    println!("修改后的宽度: {}", rect1.width);

    // 调用关联函数：使用 :: 语法
    let rect2 = Rectangle::square(10);
    println!("正方形: {:?}", rect2);

    // 方法调用的实际机制：
    // 1. rect1.area() 实际上是 Rectangle::area(&rect1) 的语法糖
    // 2. rect1.set_width(40) 实际上是 Rectangle::set_width(&mut rect1, 40) 的语法糖
    // 3. rect3.consume() 实际上是 Rectangle::consume(rect3) 的语法糖

    // 带多个参数的方法调用
    println!("rect1 能容纳 rect2 吗? {}", rect1.can_hold(&rect2));

    // 链式调用示例
    let mut rect3 = Rectangle {
        width: 20,
        height: 30,
    };
    rect3.resize(25, 35).set_width(30);
    println!("链式调用后的矩形: {:?}", rect3);

    // 只读方法调用
    let (width, height) = rect1.dimensions();
    println!("矩形尺寸: {}x{}", width, height);

    // 演示 consume 方法
    let rect4 = Rectangle {
        width: 20,
        height: 30,
    };
    rect4.consume();
    // println!("{:?}", rect4); // 编译错误：rect4 已被消耗

    // 方法的设计原则：
    // 1. 单一职责：每个方法应该只做一件事
    // 2. 命名清晰：方法名应该清楚地表达其功能
    // 3. 最小惊讶：方法的行为应该符合用户的预期
    // 4. 错误处理：考虑使用 Result 或 Option 处理可能的错误

    // self 参数的类型：
    // 1. &self：不可变借用，不修改实例
    // 2. &mut self：可变借用，修改实例
    // 3. self：获取所有权，可能消耗实例
    // 4. 无 self：关联函数，不依赖实例状态

    println!();
}

// ===========================================
// 3. 结构体和所有权（Structs and Ownership）
// ===========================================

// 结构体与所有权系统密切相关
// 理解结构体中的所有权语义对于写出安全的 Rust 代码至关重要

fn structs_and_ownership() {
    println!("=== 结构体和所有权 ===");

    // 定义包含不同所有权类型的结构体
    #[derive(Debug)]
    struct User {
        username: String,      // 拥有字符串数据（堆分配）
        email: String,        // 拥有字符串数据（堆分配）
        active: bool,          // 布尔值（栈分配，实现 Copy trait）
        sign_in_count: u64,    // 无符号整数（栈分配，实现 Copy trait）
    }

    // 字段简写语法：当变量名和字段名相同时可以省略
    // 这是 Rust 1.0 引入的语法糖，使代码更简洁
    fn build_user(username: String, email: String) -> User {
        User {
            username,     // 等价于 username: username
            email,        // 等价于 email: email
            active: true,
            sign_in_count: 1,
        }
    }

    // 构建用户实例
    let user1 = build_user(
        String::from("testuser"),
        String::from("test@example.com"),
    );

    println!("构建的用户: {:?}", user1);

    // 所有权转移示例
    // 当结构体包含非 Copy 类型时，移动结构体会移动其字段
    let username = String::from("newuser");
    let email = String::from("newuser@example.com");
    let user2 = build_user(username, email);
    // println!("{}", username); // 编译错误：username 已被移动到 user2 中

    // 借用结构体：使用引用避免所有权转移
    // 借用允许访问结构体数据而不获取所有权
    fn display_user(user: &User) {
        println!("用户信息: {} ({}) - 登录次数: {}",
                 user.username, user.email, user.sign_in_count);
    }

    // 借用结构体进行读取操作
    display_user(&user2);
    println!("用户仍然可用: {:?}", user2);

    // 可变借用：修改结构体数据
    fn increment_sign_in_count(user: &mut User) {
        user.sign_in_count += 1;
        println!("登录次数增加: {}", user.sign_in_count);
    }

    // 需要可变借用时要确保没有其他借用
    let mut user3 = User {
        username: String::from("mutable_user"),
        email: String::from("mutable@example.com"),
        active: true,
        sign_in_count: 1,
    };

    increment_sign_in_count(&mut user3);
    println!("修改后的用户: {:?}", user3);

    // 结构体中的引用：生命周期注解
    // 当结构体包含引用时，必须使用生命周期注解确保引用有效
    #[derive(Debug)]
    struct UserRef<'a> {
        username: &'a str,     // 字符串切片引用
        email: &'a str,       // 字符串切片引用
        active: bool,
    }

    let name = String::from("ref_user");
    let email_addr = String::from("ref@example.com");

    let user_ref = UserRef {
        username: &name,
        email: &email_addr,
        active: true,
    };

    println!("引用用户: {:?}", user_ref);
    // name 和 email_addr 必须在 user_ref 之前保持有效

    // Copy trait 的实现
    // 如果结构体的所有字段都实现了 Copy trait，结构体也会自动实现 Copy
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // 复制而不是移动
    println!("复制后的点: {:?} {:?}", p1, p2);

    // Clone trait 的实现
    // Clone trait 允许显式复制结构体
    #[derive(Debug, Clone)]
    struct UserClone {
        username: String,
        email: String,
    }

    let uc1 = UserClone {
        username: String::from("clone_user"),
        email: String::from("clone@example.com"),
    };

    let uc2 = uc1.clone();  // 深度复制
    println!("克隆的用户: {:?} {:?}", uc1, uc2);

    // 结构体所有权的最佳实践：
    // 1. 优先使用所有权而非借用，除非有性能需求
    // 2. 在需要共享访问时使用不可变借用
    // 3. 在需要修改时使用可变借用
    // 4. 在需要引用时使用生命周期注解
    // 5. 考虑实现 Clone trait 以支持复制操作

    // 内存布局考虑：
    // - 结构体在栈上分配，但可能包含堆分配的数据
    // - 字段的顺序影响内存布局和对齐
    // - 考虑内存访问模式进行字段排序优化

    println!();
}

// ===========================================
// 4. 结构体模式匹配（Struct Pattern Matching）
// ===========================================

// 模式匹配是 Rust 中处理结构体的强大工具
// 它允许同时解构数据和控制程序流程

fn struct_pattern_matching() {
    println!("=== 结构体模式匹配 ===");

    // 定义用于模式匹配的结构体
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };

    // 解构结构体：提取字段到变量
    // let 绑定中的模式匹配允许同时解构和绑定
    let Point { x, y } = point;
    println!("解构的坐标: x={}, y={}", x, y);

    // 解构并重命名：使用字段名: 变量名 语法
    // 当变量名与字段名不同时，可以使用重命名
    let Point { x: a, y: b } = point;
    println!("重命名的坐标: a={}, b={}", a, b);

    // match 表达式中的结构体模式
    // match 提供了穷尽性检查，确保所有情况都被处理
    match point {
        Point { x: 0, y } => println!("在 y 轴上: y={}", y),
        Point { x, y: 0 } => println!("在 x 轴上: x={}", x),
        Point { x, y } => println!("在其他位置: x={}, y={}", x, y),
    }

    // 使用 .. 忽略部分字段
    // 当只关心部分字段时，可以使用 .. 忽略其他字段
    match point {
        Point { x: 0, .. } => println!("x 坐标为 0"),
        Point { y: 0, .. } => println!("y 坐标为 0"),
        Point { .. } => println!("x 和 y 都不为 0"),
    }

    // 函数参数中的模式匹配
    // 函数参数也可以使用模式匹配来解构参数
    fn print_point(Point { x, y }: Point) {
        println!("函数参数解构: x={}, y={}", x, y);
    }

    print_point(point);

    // 复杂结构的模式匹配
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 10, y: 0 },
    };

    // 嵌套模式匹配
    match rect {
        Rectangle {
            top_left: Point { x: 0, y },
            bottom_right: Point { x: bottom_x, y: 0 }
        } => {
            println!("矩形从 (0, {}) 到 ({}, 0)", y, bottom_x);
        }
        Rectangle { top_left, bottom_right } => {
            println!("其他矩形: {:?} 到 {:?}", top_left, bottom_right);
        }
    }

    // 模式匹配中的守卫条件
    fn describe_point(point: Point) -> &'static str {
        match point {
            Point { x, y } if x == y => "在 y=x 线上",
            Point { x, y } if x > y => "在 y=x 线下方",
            Point { x, y } if x < y => "在 y=x 线上方",
            _ => "其他位置", // 虽然不会到达，但保持完整性
        }
    }

    println!("点 (1,1): {}", describe_point(Point { x: 1, y: 1 }));
    println!("点 (2,1): {}", describe_point(Point { x: 2, y: 1 }));

    // if let 简化模式匹配
    // 当只关心一种模式时，可以使用 if let 简化
    fn get_point_x(point: Point) -> Option<i32> {
        match point {
            Point { x: 0, .. } => None,
            Point { x, .. } => Some(x),
        }
    }

    if let Some(x) = get_point_x(point) {
        println!("点的 x 坐标: {}", x);
    }

    // 模式匹配的最佳实践：
    // 1. 优先使用 match 而不是多个 if-else
    // 2. 利用模式的穷尽性检查确保完整性
    // 3. 使用有意义的变量名增强可读性
    // 4. 适当使用 .. 忽略不关心的字段
    // 5. 在简单情况下使用 if let 或 while let

    println!();
}

// ===========================================
// 5. 结构体示例程序（Example Program）
// ===========================================

// 通过完整的图书管理系统示例，展示结构体在实际应用中的使用
// 这个示例涵盖了结构体的核心概念和最佳实践

fn struct_example_program() {
    println!("=== 结构体示例程序 ===");

    // 图书结构体：定义图书的属性和行为
    #[derive(Debug)]
    struct Book {
        title: String,        // 书名
        author: String,       // 作者
        pages: u32,           // 页数
        available: bool,      // 是否可借阅
    }

    // 实现图书的行为
    impl Book {
        // 构造函数：创建新的图书实例
        // 关联函数，用于实例化结构体
        fn new(title: String, author: String, pages: u32) -> Book {
            Book {
                title,
                author,
                pages,
                available: true,  // 新书默认可借阅
            }
        }

        // 借书操作：改变图书状态
        // 可变方法，修改图书的可借阅状态
        fn borrow(&mut self) {
            if self.available {
                self.available = false;
                println!("《{}》已被借出", self.title);
            } else {
                println!("《{}》当前不可借", self.title);
            }
        }

        // 还书操作：恢复图书可借阅状态
        // 可变方法，恢复图书的可借阅状态
        fn return_book(&mut self) {
            if !self.available {
                self.available = true;
                println!("《{}》已归还", self.title);
            } else {
                println!("《{}》未被借出", self.title);
            }
        }

        // 查询状态：检查图书是否可借阅
        // 不可变方法，查询图书状态
        fn is_available(&self) -> bool {
            self.available
        }

        // 生成摘要：返回图书的基本信息
        // 不可变方法，格式化输出图书信息
        fn summary(&self) -> String {
            format!("《{}》 by {} ({} pages) - {}",
                    self.title, self.author, self.pages,
                    if self.available { "Available" } else { "Checked out" })
        }

        // 获取详细信息：返回图书的所有信息
        fn details(&self) -> String {
            format!("Title: {}\nAuthor: {}\nPages: {}\nStatus: {}",
                    self.title, self.author, self.pages,
                    if self.available { "Available" } else { "Checked out" })
        }
    }

    // 图书管理系统：创建和管理图书集合
    let mut book1 = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        519,
    );

    let mut book2 = Book::new(
        String::from("Programming Rust"),
        String::from("Jim Blandy, Jason Orendorff, and Leonora Tindall"),
        544,
    );

    // 显示初始状态
    println!("=== 图书列表 ===");
    println!("{}", book1.summary());
    println!("{}", book2.summary());

    // 执行借书操作
    println!("\n=== 借书操作 ===");
    book1.borrow();
    book2.borrow();
    book1.borrow(); // 尝试再次借书1（应该失败）

    // 显示借书后状态
    println!("\n=== 借书后状态 ===");
    println!("{}", book1.summary());
    println!("{}", book2.summary());

    // 执行还书操作
    println!("\n=== 还书操作 ===");
    book1.return_book();
    book2.return_book(); // 尝试归还未借出的书（应该失败）

    // 显示还书后状态
    println!("\n=== 还书后状态 ===");
    println!("{}", book1.summary());
    println!("{}", book2.summary());

    // 演示详细信息方法
    println!("\n=== 详细信息 ===");
    println!("Book 1 Details:\n{}", book1.details());

    // 图书馆管理系统扩展
    #[derive(Debug)]
    struct Library {
        name: String,
        books: Vec<Book>,
    }

    impl Library {
        fn new(name: String) -> Library {
            Library {
                name,
                books: Vec::new(),
            }
        }

        fn add_book(&mut self, book: Book) {
            self.books.push(book);
            println!("已添加图书到图书馆: {}", self.name);
        }

        fn list_available_books(&self) -> Vec<&Book> {
            self.books.iter()
                .filter(|book| book.is_available())
                .collect()
        }

        fn find_by_author(&self, author: &str) -> Vec<&Book> {
            self.books.iter()
                .filter(|book| book.author.contains(author))
                .collect()
        }
    }

    // 创建图书馆并管理图书
    let mut library = Library::new(String::from("中心图书馆"));
    library.add_book(Book::new(
        String::from("Rust by Example"),
        String::from("The Rust Community"),
        300,
    ));

    println!("\n=== 图书馆管理 ===");
    let available_books = library.list_available_books();
    println!("可借阅的图书数量: {}", available_books.len());

    let rust_books = library.find_by_author("Rust");
    println!("包含 'Rust' 的图书数量: {}", rust_books.len());

    // 这个示例展示了结构体在实际应用中的关键概念：
    // 1. 数据封装：将相关数据和行为组织在一起
    // 2. 状态管理：通过方法管理对象的状态
    // 3. 错误处理：处理各种边界情况
    // 4. 代码重用：通过方法提供可重用的功能
    // 5. 扩展性：可以轻松添加新的功能和属性

    println!();
}

// ===========================================
// 6. 结构体属性（Struct Attributes）
// ===========================================

// 属性（Attributes）是 Rust 中的元数据
// 它们为编译器提供额外的信息，控制代码的生成和行为

fn struct_attributes() {
    println!("=== 结构体属性 ===");

    // Debug 属性：自动生成调试格式实现
    // Debug trait 允许使用 {:?} 和 {:#?} 格式化输出
    #[derive(Debug)]
    struct DebugStruct {
        name: String,
        value: i32,
    }

    let debug_struct = DebugStruct {
        name: String::from("test"),
        value: 42,
    };

    println!("Debug 格式: {:?}", debug_struct);
    println!("Pretty Debug 格式: {:#?}", debug_struct);

    // Clone 属性：实现深拷贝功能
    // Clone trait 提供显式的复制方法 clone()
    #[derive(Debug, Clone)]
    struct CloneStruct {
        name: String,
        value: i32,
    }

    let clone1 = CloneStruct {
        name: String::from("clone_test"),
        value: 100,
    };

    let clone2 = clone1.clone();  // 深度复制
    println!("克隆测试: {:?} {:?}", clone1, clone2);

    // PartialEq 属性：实现相等性比较
    // PartialEq trait 提供 == 和 != 操作符
    #[derive(Debug, Clone, PartialEq)]
    struct PartialEqStruct {
        name: String,
        value: i32,
    }

    let eq1 = PartialEqStruct {
        name: String::from("equal"),
        value: 50,
    };

    let eq2 = PartialEqStruct {
        name: String::from("equal"),
        value: 50,
    };

    let eq3 = PartialEqStruct {
        name: String::from("different"),
        value: 50,
    };

    println!("相等测试: eq1 == eq2 = {}", eq1 == eq2);
    println!("不等测试: eq1 == eq3 = {}", eq1 == eq3);

    // Eq 属性：实现完全相等性
    // Eq trait 用于表示自反性（a == a）的类型
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct EqStruct {
        id: u32,
    }

    let eq_a = EqStruct { id: 1 };
    let eq_b = EqStruct { id: 1 };
    println!("Eq 测试: eq_a == eq_b = {}", eq_a == eq_b);

    // PartialOrd 和 Ord 属性：实现排序功能
    // PartialOrd 提供部分排序，Ord 提供完全排序
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct OrdStruct {
        priority: u32,
        name: String,
    }

    let ord1 = OrdStruct {
        priority: 1,
        name: String::from("High"),
    };

    let ord2 = OrdStruct {
        priority: 2,
        name: String::from("Low"),
    };

    println!("排序测试: ord1 < ord2 = {}", ord1 < ord2);

    // Hash 属性：实现哈希功能
    // Hash trait 允许结构体作为 HashMap 的键
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct HashStruct {
        id: u32,
        category: String,
    }

    use std::collections::HashMap;
    let mut map = HashMap::new();
    let hash_key = HashStruct {
        id: 1,
        category: String::from("test"),
    };

    map.insert(hash_key, "Some value");
    println!("HashMap 测试成功");

    // Default 属性：实现默认值
    // Default trait 提供默认的构造方法
    #[derive(Debug, Clone, Default)]
    struct DefaultStruct {
        count: u32,
        enabled: bool,
        name: String,
    }

    let default_val = DefaultStruct::default();
    println!("默认值: {:?}", default_val);

    // 自定义属性：控制结构体的行为
    #[repr(C)]  // 指定内存布局为 C 兼容
    #[repr(packed)]  // 紧凑内存布局，无填充
    struct CLayoutStruct {
        a: u8,
        b: u32,
    }

    println!("C 布局结构体大小: {}", std::mem::size_of::<CLayoutStruct>());

    // 属性的选择指南：
    // 1. Debug：几乎所有结构体都应该实现
    // 2. Clone/PartialEq：根据需要选择
    // 3. Eq/Ord/Hash：当需要相应功能时实现
    // 4. Default：当有合理的默认值时实现
    // 5. repr：当需要特定内存布局时使用

    println!();
}

// ===========================================
// 7. 结构体生命周期（Struct Lifetimes）
// ===========================================

// 生命周期确保结构体中的引用不会悬垂
// 这是 Rust 内存安全系统的核心组成部分

fn struct_lifetimes() {
    println!("=== 结构体生命周期 ===");

    // 带生命周期的结构体：确保引用的有效性
    // 生命周期注解 'a 表示引用的有效期
    #[derive(Debug)]
    struct Book<'a> {
        title: &'a str,      // 书名引用，生命周期为 'a
        author: &'a str,     // 作者引用，生命周期为 'a
    }

    // 生命周期的作用：
    // 1. 确保引用不会比被引用的数据活得更长
    // 2. 防止悬垂指针和内存安全问题
    // 3. 在编译时检查引用的有效性

    let title = String::from("Rust Programming");
    let author = String::from("Rust Team");

    // 创建结构体实例，引用绑定到局部变量
    let book = Book {
        title: &title,
        author: &author,
    };

    println!("带生命周期的书: {:?}", book);
    // book 不能比 title 和 author 活得更长

    // 生命周期省略规则：
    // 1. 每个引用参数都有自己的生命周期
    // 2. 如果只有一个输入生命周期，它被赋予所有输出生命周期
    // 3. 如果有多个输入生命周期，其中一个是 &self，则 self 的生命周期被赋予输出

    // 静态生命周期：'static 表示整个程序的生命周期
    #[derive(Debug)]
    struct StaticString {
        text: &'static str,  // 静态生命周期引用
    }

    let static_string = StaticString {
        text: "This is a static string",  // 字符串字面量是 'static 的
    };

    println!("静态字符串: {:?}", static_string);

    // 多个生命周期的结构体
    #[derive(Debug)]
    struct PairOfStrings<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }

    let s1 = String::from("First");
    let s2 = String::from("Second");

    let pair = PairOfStrings {
        first: &s1,
        second: &s2,
    };

    println!("多生命周期对: {:?}", pair);

    // 生命周期与泛型的结合
    #[derive(Debug)]
    struct Container<'a, T> {
        item: &'a T,  // 引用类型 T，生命周期为 'a
    }

    let number = 42;
    let container = Container {
        item: &number,
    };

    println!("泛型容器: {:?}", container);

    // 生命周期的高级用法
    #[derive(Debug)]
    struct Excerpt<'a> {
        part: &'a str,
    }

    fn announce_and_return_part<'a>(announcement: &str, excerpt: &'a str) -> &'a str {
        println!("公告: {}", announcement);
        excerpt  // 返回的生命周期与 excerpt 相同
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };

    println!("摘录: {:?}", excerpt);

    // 生命周期的最佳实践：
    // 1. 优先让编译器推断生命周期
    // 2. 只在必要时显式指定生命周期
    // 3. 使用有意义的生命周期名称
    // 4. 避免过长的生命周期
    // 5. 考虑使用拥有数据的结构体而非引用

    // 生命周期与内存安全：
    // - 生命周期是编译时检查，没有运行时开销
    // - 防止使用已释放的内存
    // - 确保引用的有效性
    // - 是 Rust 内存安全保证的重要组成部分

    println!();
}

// ===========================================
// 8. 结构体和泛型（Structs and Generics）
// ===========================================

// 泛型结构体可以操作多种类型的数据
// 这提供了代码重用和类型安全的平衡

fn structs_and_generics() {
    println!("=== 结构体和泛型 ===");

    // 单个泛型参数的结构体
    #[derive(Debug)]
    struct Pair<T> {
        first: T,
        second: T,
    }

    // 为泛型结构体实现方法
    impl<T> Pair<T> {
        // 构造函数：创建新的 Pair
        fn new(first: T, second: T) -> Self {
            Pair { first, second }
        }

        // 获取第一个元素的引用
        fn get_first(&self) -> &T {
            &self.first
        }

        // 获取第二个元素的引用
        fn get_second(&self) -> &T {
            &self.second
        }

        // 检查两个元素是否相等（需要 PartialEq 约束）
        fn are_equal(&self) -> bool
        where
            T: PartialEq,
        {
            self.first == self.second
        }
    }

    // 为特定类型实现额外方法
    impl Pair<i32> {
        // 特定于 i32 的方法
        fn sum(&self) -> i32 {
            self.first + self.second
        }
    }

    // 使用泛型结构体
    let int_pair = Pair::new(1, 2);
    let string_pair = Pair::new(String::from("hello"), String::from("world"));

    println!("整数对: {:?}", int_pair);
    println!("字符串对: {:?}", string_pair);

    // 调用泛型方法
    println!("第一个元素: {}", int_pair.get_first());
    println!("相等性检查: {}", int_pair.are_equal());

    // 调用特定类型的方法
    println!("整数对的和: {}", int_pair.sum());

    // 多个泛型参数的结构体
    #[derive(Debug)]
    struct KeyValue<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> KeyValue<K, V> {
        fn new(key: K, value: V) -> Self {
            KeyValue { key, value }
        }

        fn get_key(&self) -> &K {
            &self.key
        }

        fn get_value(&self) -> &V {
            &self.value
        }
    }

    let kv1 = KeyValue {
        key: String::from("name"),
        value: String::from("Alice"),
    };

    let kv2 = KeyValue {
        key: 1,
        value: 42.0,
    };

    println!("键值对1: {:?}", kv1);
    println!("键值对2: {:?}", kv2);

    // 带约束的泛型：限制泛型类型的范围
    #[derive(Debug)]
    struct Container<T: std::fmt::Debug> {
        item: T,
    }

    impl<T: std::fmt::Debug> Container<T> {
        fn display(&self) {
            println!("容器内容: {:?}", self.item);
        }

        fn into_inner(self) -> T {
            self.item
        }
    }

    let container = Container {
        item: vec![1, 2, 3],
    };

    container.display();

    // 复杂的泛型约束
    #[derive(Debug)]
    struct ProcessableItem<T>
    where
        T: std::fmt::Debug + Clone,
    {
        item: T,
        processed: bool,
    }

    impl<T> ProcessableItem<T>
    where
        T: std::fmt::Debug + Clone,
    {
        fn new(item: T) -> Self {
            ProcessableItem {
                item,
                processed: false,
            }
        }

        fn process(&mut self) {
            self.processed = true;
            println!("处理完成: {:?}", self.item);
        }

        fn get_item(&self) -> T {
            self.item.clone()
        }
    }

    let mut processable = ProcessableItem::new(String::from("Hello"));
    processable.process();
    println!("可处理项目: {:?}", processable);

    // 泛型结构体的实际应用
    #[derive(Debug)]
    struct Result<T, E> {
        success: bool,
        data: Option<T>,
        error: Option<E>,
    }

    impl<T, E> Result<T, E> {
        fn success(data: T) -> Self {
            Result {
                success: true,
                data: Some(data),
                error: None,
            }
        }

        fn failure(error: E) -> Self {
            Result {
                success: false,
                data: None,
                error: Some(error),
            }
        }

        fn is_success(&self) -> bool {
            self.success
        }
    }

    let success_result: Result<i32, String> = Result::success(42);
    let failure_result: Result<i32, String> = Result::failure(String::from("Error"));

    println!("成功结果: {:?}", success_result);
    println!("失败结果: {:?}", failure_result);

    // 泛型结构体的最佳实践：
    // 1. 使用有意义的类型参数名称（T, U, V 或更具描述性的名称）
    // 2. 添加必要的约束以支持所需操作
    // 3. 考虑为特定类型提供优化实现
    // 4. 保持接口简单和一致
    // 5. 在文档中清晰地说明泛型参数的要求

    // 泛型的优势：
    // 1. 代码重用：一个定义适用于多种类型
    // 2. 类型安全：编译时检查类型错误
    // 3. 性能：单态化（monomorphization）提供零成本抽象
    // 4. 灵活性：可以处理各种数据类型

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 结构体演示");
    println!("===============");

    basic_structs();
    struct_methods();
    structs_and_ownership();
    struct_pattern_matching();
    struct_example_program();
    struct_attributes();
    struct_lifetimes();
    structs_and_generics();

    println!("结构体演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    // 测试矩形面积计算
    #[test]
    fn test_rectangle_area() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        let rect = Rectangle::square(5);
        assert_eq!(rect.area(), 25);
    }

    // 测试图书借阅系统
    #[test]
    fn test_book_borrow_return() {
        #[derive(Debug)]
        struct Book {
            title: String,
            available: bool,
        }

        impl Book {
            fn new(title: String) -> Book {
                Book {
                    title,
                    available: true,
                }
            }

            fn borrow(&mut self) {
                if self.available {
                    self.available = false;
                }
            }

            fn return_book(&mut self) {
                if !self.available {
                    self.available = true;
                }
            }

            fn is_available(&self) -> bool {
                self.available
            }
        }

        let mut book = Book::new(String::from("Test Book"));
        assert!(book.is_available());

        book.borrow();
        assert!(!book.is_available());

        book.return_book();
        assert!(book.is_available());
    }

    // 测试泛型对结构体
    #[test]
    fn test_pair_struct() {
        #[derive(Debug, PartialEq)]
        struct Pair<T> {
            first: T,
            second: T,
        }

        impl<T> Pair<T> {
            fn new(first: T, second: T) -> Self {
                Pair { first, second }
            }
        }

        let pair1 = Pair::new(1, 2);
        let pair2 = Pair::new(1, 2);
        assert_eq!(pair1, pair2);
    }

    // 测试结构体模式匹配
    #[test]
    fn test_struct_pattern_matching() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 0, y: 5 };

        let result = match point {
            Point { x: 0, y } => format!("On y-axis at {}", y),
            Point { x, y: 0 } => format!("On x-axis at {}", x),
            Point { x, y } => format!("At position ({}, {})", x, y),
        };

        assert_eq!(result, "On y-axis at 5");
    }

    // 测试结构体所有权
    #[test]
    fn test_struct_ownership() {
        #[derive(Debug, Clone)]
        struct User {
            username: String,
            email: String,
        }

        impl User {
            fn new(username: String, email: String) -> User {
                User { username, email }
            }
        }

        let user1 = User::new(
            String::from("test"),
            String::from("test@example.com"),
        );

        let user2 = user1.clone();
        assert_eq!(user1.username, user2.username);
        assert_eq!(user1.email, user2.email);
    }

    // 测试结构体方法链
    #[test]
    fn test_method_chaining() {
        #[derive(Debug)]
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Self {
                Counter { count: 0 }
            }

            fn increment(&mut self) -> &mut Self {
                self.count += 1;
                self
            }

            fn multiply(&mut self, factor: u32) -> &mut Self {
                self.count *= factor;
                self
            }

            fn get_count(&self) -> u32 {
                self.count
            }
        }

        let mut counter = Counter::new();
        let result = counter.increment().increment().multiply(3).get_count();
        assert_eq!(result, 6);
    }
}