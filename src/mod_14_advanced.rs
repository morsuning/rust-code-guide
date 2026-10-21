// 教程里的类型、字段和方法经常只为演示语法而定义，不一定会被全部读取或调用，所以允许 dead_code；
// 其他 lint（未使用的导入、变量、可变性等）必须保持干净，不要再给模块添加新的 allow。
#![allow(dead_code)]

// Rust 高级特性（Advanced Rust Features）
// 包含 unsafe Rust、高级生命周期、高级 trait 等深度特性的详细讲解
// （内联汇编、内存布局等更偏底层、更低频的内容，集中在第 19 章“进阶参考”里）
// 这些特性展示了 Rust 在系统编程、高性能计算和底层开发中的强大能力

// Rust 高级特性的核心价值：
// 1. 系统编程能力：提供接近硬件的控制能力
// 2. 零成本抽象：高级特性不会带来运行时开销
// 3. 类型安全：即使在 unsafe 代码中也尽可能保持类型安全
// 4. 性能优化：提供细粒度的性能控制能力
// 5. 互操作性：与其他语言和系统的无缝集成

// ===========================================
// 1. Unsafe Rust（不安全 Rust）
// ===========================================

// Unsafe Rust 是 Rust 类型系统中的一个重要机制
// 它允许程序员在需要时绕过 Rust 的安全检查，直接进行底层操作
// 这不是 Rust 的缺陷，而是系统编程的必要工具，用于处理 Rust 安全机制无法覆盖的场景

// Unsafe Rust 的核心原则：
// 1. 最小化原则：只在绝对必要时使用 unsafe
// 2. 封装原则：将 unsafe 代码封装在安全的接口后面
// 3. 文档化原则：详细说明 unsafe 代码的安全要求和约束
// 4. 隔离原则：将 unsafe 代码限制在尽可能小的范围内

fn unsafe_rust_basics() {
    println!("=== Unsafe Rust 基础 ===");

    // Unsafe Rust 的主要能力：
    // 1. 解引用裸指针（Raw Pointer Dereference）
    // 2. 调用 unsafe 函数和 trait 方法
    // 3. 访问或修改可变静态变量
    // 4. 实现 unsafe trait
    // 5. 访问 union 字段
    // 6. 编写内联汇编

    // 裸指针的创建和使用
    // 裸指针是不受生命周期约束的指针，类似于 C/C++ 中的指针
    // 它们可以指向任意内存地址，但解引用操作必须在 unsafe 块中进行
    let mut num = 5;

    // 创建不可变裸指针：*const T
    // 这种指针只能读取数据，不能修改数据
    let r1 = &num as *const i32;

    // 创建可变裸指针：*mut T
    // 这种指针可以读取和修改数据
    let r2 = &mut num as *mut i32;

    println!("不可变裸指针: {:p}", r1);
    println!("可变裸指针: {:p}", r2);

    // 裸指针与引用的区别：
    // 1. 引用：有生命周期保证，编译器确保安全性
    // 2. 裸指针：无生命周期保证，需要程序员手动确保安全性
    // 3. 引用：不能为空，不能有别名（在某些情况下）
    // 4. 裸指针：可以为空，可以有多个别名

    // 在 unsafe 块中解引用裸指针
    // unsafe 块是程序员对编译器的承诺："我保证这里的操作是安全的"
    unsafe {
        println!("解引用不可变裸指针: {}", *r1);
        println!("解引用可变裸指针: {}", *r2);

        // 通过可变裸指针修改原始数据
        // 这展示了 unsafe 的强大能力：绕过 Rust 的借用检查器
        *r2 = 10;
        println!("修改后的值: {}", *r1);

        // 裸指针的危险性演示
        // 裸指针可以悬垂（dangling），导致未定义行为
        let _dangling_ptr: *const i32;
        {
            let local_var = 42;
            _dangling_ptr = &local_var as *const i32;
            // local_var 在这里离开作用域，dangling_ptr 变成悬垂指针
        }
        // println!("悬垂指针: {}", unsafe { *dangling_ptr }); // 未定义行为！
    }

    // 在 unsafe 块外不能解引用裸指针
    // 编译器会强制执行这个限制，确保 unsafe 操作的明确性
    // println!("{}", *r1); // 编译错误：解引用裸指针需要 unsafe 块

    // 裸指针的常见用途：
    // 1. 与 C 语言库交互
    // 2. 实现自定义数据结构（如链表、树等）
    // 3. 性能关键的代码路径
    // 4. 操作系统开发
    // 5. 嵌入式系统编程

    // 裸指针的安全使用原则：
    // 1. 确保指针有效性：保证指针指向有效的内存地址
    // 2. 避免数据竞争：在并发环境中正确处理共享数据
    // 3. 遵循对齐要求：确保访问内存时满足硬件对齐要求
    // 4. 处理生命周期：避免悬垂指针和过早释放

    println!();
}

// ===========================================
// 2. Unsafe 函数（Unsafe Functions）
// ===========================================

// Unsafe 函数是包含不安全操作的函数
// 它们通过 unsafe 关键字明确告知调用者："此函数可能包含不安全操作，调用者需要确保调用条件"

// Unsafe 函数的设计原则：
// 1. 明确性：通过 unsafe 关键字明确标识不安全操作
// 2. 文档化：详细说明函数的前提条件和安全要求
// 3. 最小化：只在必要时使用 unsafe
// 4. 封装：尽可能提供安全的包装接口

fn unsafe_functions() {
    println!("=== Unsafe 函数 ===");

    // 定义 unsafe 函数
    // unsafe 关键字是函数契约的一部分，告诉编译器此函数可能执行不安全操作
    unsafe fn dangerous() {
        // 这个函数被标记为 unsafe，因为它可能执行危险操作
        // 在实际应用中，这里可能是内存操作、系统调用等
        println!("执行危险操作");
    }

    // 调用 unsafe 函数必须在 unsafe 块中
    // 这个要求确保调用者明确知道他们正在调用不安全函数
    unsafe {
        dangerous();
    }

    // 调用 unsafe 函数的安全要求：
    // 1. 理解函数的安全前提条件
    // 2. 确保调用时满足这些条件
    // 3. 对可能的后果负责

    // 包含外部函数调用的 unsafe 函数
    // 这类函数通常用于与 C 语言库或其他外部代码交互
    unsafe fn extern_call() {
        println!("调用外部函数");
        // 这里可以调用 C 函数等外部代码，例如：
        // external_c_function();
        // 因为外部函数不受 Rust 安全规则约束，所以需要 unsafe
    }

    unsafe {
        extern_call();
    }

    // 函数指针与 unsafe 的结合
    // unsafe 函数指针可以指向 unsafe 函数
    unsafe fn unsafe_operation(x: *mut i32) {
        // 通过裸指针修改数据，这是不安全操作
        unsafe {
            *x += 1;
        }
    }

    // 定义 unsafe 函数指针
    // 函数指针类型也要标记为 unsafe
    let unsafe_fn_ptr: unsafe fn(*mut i32) = unsafe_operation;

    let mut value = 42;
    let ptr = &mut value as *mut i32;

    unsafe {
        // 通过 unsafe 函数指针调用函数
        unsafe_fn_ptr(ptr);
        println!("调用 unsafe 函数后的值: {}", value);
    }

    // unsafe 函数的常见用途：
    // 1. 系统调用：与操作系统内核交互
    // 2. 硬件访问：直接操作硬件寄存器
    // 3. 内存管理：手动内存分配和释放
    // 4. FFI（外部函数接口）：调用 C/C++/Assembly 代码
    // 5. 低级优化：编译器无法自动优化的操作

    // unsafe 函数的最佳实践：
    // 1. 提供安全包装：为常用的 unsafe 函数提供安全接口
    // 2. 详细文档：说明函数的前提条件和安全要求
    // 3. 参数验证：在函数内部验证参数的有效性
    // 4. 错误处理：提供明确的错误处理机制
    // 5. 测试覆盖：编写充分的测试确保安全性

    // unsafe 函数的示例：
    // 示例：安全的内存拷贝函数
    use std::ptr;

    // 安全地拷贝内存块
    // # 安全性要求
    // - src 和 dst 必须有效且不重叠
    // - count 必须正确反映内存块大小
    pub unsafe fn copy_memory(src: *const u8, dst: *mut u8, count: usize) {
        unsafe {
            ptr::copy(src, dst, count);
        }
    }

    println!();
}

// ===========================================
// 3. 可变静态变量（Mutable Static Variables）
// ===========================================

// 静态变量（Static Variables）是在程序整个生命周期中存在的变量
// 它们存储在程序的静态存储区，在程序启动时初始化，在程序结束时销毁

// 可变静态变量的风险：
// 1. 数据竞争：多线程同时访问可能导致数据不一致
// 2. 全局状态：使程序的行为难以预测和测试
// 3. 内存安全：可能导致内存安全问题
// 4. 重入问题：在递归或回调中可能导致意外行为

fn mutable_static_variables() {
    println!("=== 可变静态变量 ===");

    // 普通静态变量：不可变的全局数据
    // 普通静态变量是安全的，因为它们在编译时初始化且不可变
    static COUNTER: i32 = 0;
    println!("静态变量 COUNTER: {}", COUNTER);

    // 静态变量的特点：
    // 1. 全局唯一：整个程序中只有一个实例
    // 2. 编译时初始化：在编译时确定初始值
    // 3. 永久存在：程序运行期间一直存在
    // 4. 线程安全：不可变性保证了线程安全

    // 可变静态变量：需要特殊处理的全局可变数据
    // 因为可变静态变量可能导致数据竞争，所以需要 unsafe 来访问
    static mut MUTABLE_COUNTER: i32 = 0;

    // 访问可变静态变量必须在 unsafe 块中
    // 这提醒程序员他们正在处理可能导致数据竞争的操作
    unsafe {
        println!(
            "可变静态变量 MUTABLE_COUNTER: {}",
            std::ptr::addr_of!(MUTABLE_COUNTER).read()
        );

        // 修改可变静态变量
        // 这种操作在多线程环境中是不安全的
        MUTABLE_COUNTER += 1;
        println!(
            "修改后的 MUTABLE_COUNTER: {}",
            std::ptr::addr_of!(MUTABLE_COUNTER).read()
        );

        MUTABLE_COUNTER += 1;
        println!(
            "再次修改后的 MUTABLE_COUNTER: {}",
            std::ptr::addr_of!(MUTABLE_COUNTER).read()
        );
    }

    // 可变静态变量的风险：
    // 1. 数据竞争：多个线程同时修改可能导致未定义行为
    // 2. 全局状态：破坏了程序的纯函数性
    // 3. 测试困难：全局状态使得单元测试复杂化
    // 4. 重入问题：在信号处理或异常处理中可能导致问题

    // 线程安全的可变静态变量：使用原子类型
    // 原子类型提供了线程安全的操作，无需 unsafe
    use std::sync::atomic::{AtomicUsize, Ordering};

    static THREAD_SAFE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    // 原子操作是安全的，不需要 unsafe 块
    // 原子操作保证了操作的原子性和可见性
    THREAD_SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!(
        "线程安全的计数器: {}",
        THREAD_SAFE_COUNTER.load(Ordering::SeqCst)
    );

    THREAD_SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!(
        "线程安全的计数器: {}",
        THREAD_SAFE_COUNTER.load(Ordering::SeqCst)
    );

    // 内存排序（Memory Ordering）的解释：
    // Ordering::SeqCst：顺序一致性，最强的保证
    // Ordering::Relaxed：宽松排序，只保证原子性
    // Ordering::Acquire：获取操作，防止重排
    // Ordering::Release：释放操作，防止重排

    // 静态变量的替代方案：
    // 1. 全局可变状态：直接写 `static STATE: Mutex<T> = Mutex::new(..)`（`Mutex::new` 自 1.63 起是 const fn），
    //    不需要 unsafe，也不需要 Arc——`static` 本身就是全局唯一、生命周期 'static 的共享位置
    //    （`Arc<Mutex<T>>` 适合“在多个线程之间共享所有权”，而不是做全局变量）
    // 2. 延迟初始化的全局值：`std::sync::OnceLock`（1.70）和 `std::sync::LazyLock`（1.80）
    //    （注意不是 `OnceCell`：`static` 要求类型是 Sync，而 `OnceCell` 不是 Sync，只能用在单线程里）
    // 3. 旧代码里常见的 `lazy_static!` 宏，基本都可以用标准库的 `LazyLock` 取代，新代码不必再引入它
    // 4. 使用函数参数：能把状态作为参数传进去，就别用全局状态

    // 静态变量的实际应用：
    // 1. 配置管理：全局配置信息
    // 2. 缓存系统：全局缓存
    // 3. 日志系统：全局日志记录器
    // 4. 性能计数器：全局统计信息
    // 5. 资源管理：全局资源池

    println!();
}

// ===========================================
// 4. Union 类型（Union Types）
// ===========================================

// Union 类型是 Rust 中的一种特殊数据结构
// 它允许多个字段共享相同的内存位置，这与 C 语言中的 union 类似
// Union 的主要用途是进行类型转换、内存优化和与 C 语言代码的互操作

// Union 类型的风险：
// 1. 类型混淆：不同类型的数据可能被错误地解释
// 2. 内存安全：可能导致未定义行为
// 3. 平台依赖：内存布局可能因平台而异
// 4. 调试困难：错误可能难以发现和调试

fn union_types() {
    println!("=== Union 类型 ===");

    // 基本 Union 定义和使用
    // Union 的所有字段共享相同的内存位置
    #[repr(C)] // 使用 C 兼容的内存布局
    union IntOrFloat {
        i: i32, // 整数字段
        f: f32, // 浮点数字段
    }

    let mut u = IntOrFloat { i: 42 };

    // 访问 union 字段必须在 unsafe 块中
    // 因为编译器无法确定当前存储的是哪种类型的数据
    unsafe {
        // 作为整数访问
        println!("作为整数: {}", u.i);

        // 修改为浮点数
        // 这会覆盖内存中的整数值
        u.f = 2.5;
        println!("作为浮点数: {}", u.f);

        // 现在作为整数读取会得到未定义的行为
        // 因为内存中的位模式现在是浮点数格式
        println!("修改后作为整数: {}", u.i); // 未定义行为！
    }

    // Union 的内存布局：
    // Union 的大小是其最大字段的大小
    // 所有字段的起始地址相同
    // 写入一个字段会影响其他字段的值

    // 更安全的 Union 使用模式：类型双关
    // 使用 Union 进行安全的类型转换
    #[repr(C)]
    union SafeUnion {
        value: i32,
        bytes: [u8; 4], // 字节表示
    }

    let safe_union = SafeUnion { value: 0x12345678 };

    unsafe {
        // 通过不同的视角查看相同的数据
        println!("值: 0x{:08x}", safe_union.value);
        println!("字节: {:?}", safe_union.bytes);

        // 这展示了整数在内存中的字节表示
        // 可以用于字节序转换、数据序列化等场景
    }

    // Union 的常见用途：
    // 1. 类型转换：安全地在不同类型之间转换
    // 2. 内存优化：节省内存空间
    // 3. C 互操作：与 C 语言的 union 类型对应
    // 4. 硬件访问：访问硬件寄存器的不同位域
    // 5. 数据解析：解析网络协议或文件格式

    // 使用 Union 的最佳实践：
    // 1. 标记当前类型：使用枚举跟踪当前存储的类型
    // 2. 提供安全接口：封装 unsafe 操作
    // 3. 文档化行为：清楚说明内存布局和访问规则
    // 4. 避免混合类型：避免同时访问不同字段
    // 5. 考虑对齐：确保满足硬件对齐要求

    // Union 与 Enum 的对比：
    // Union: 多个字段共享内存，不安全但节省空间
    // Enum: 每个变体都有自己的数据，安全但占用更多空间

    // 更复杂的 Union 示例：
    /*
    #[repr(C)]
    union NetworkPacket {
        header: PacketHeader,
        raw_bytes: [u8; 64],
    }

    struct PacketHeader {
        version: u8,
        flags: u8,
        length: u16,
    }
    */

    println!();
}

// ===========================================
// 5. 高级生命周期注解（Advanced Lifetime Annotations）
// ===========================================

// 生命周期是 Rust 类型系统的重要组成部分
// 它们确保引用始终有效，防止悬垂指针和内存安全问题
// 高级生命周期注解提供了更精细的控制和更强大的表达能力

// 生命周期的核心概念：
// 1. 生命周期标注：描述引用的有效范围
// 2. 生命周期省略：编译器自动推断生命周期
// 3. 生命周期约束：定义生命周期之间的关系
// 4. 生命周期子类型：描述生命周期的包含关系
// 5. 静态生命周期：程序运行期间始终有效的引用

fn advanced_lifetime_annotations() {
    println!("=== 高级生命周期注解 ===");

    // 生命周期省略规则（Lifetime Elision Rules）
    // 编译器使用这些规则自动推断生命周期，减少标注的负担
    // 规则 1：每个引用参数都有自己的生命周期参数
    // 规则 2：如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数
    // 规则 3：如果方法有 &self 或 &mut self 参数，self 的生命周期被赋予所有输出参数
    fn first_word(s: &str) -> &str {
        // 完整的形式应该是：fn first_word<'a>(s: &'a str) -> &'a str
        // 根据省略规则 2，可以省略生命周期标注
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let text = "hello world";
    let word = first_word(text);
    println!("第一个单词: {}", word);

    // 显式生命周期注解：多参数的情况
    // 当有多个引用参数时，需要明确它们之间的生命周期关系
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // 'a 表示 x 和 y 以及返回值的生命周期必须相同
        // 这确保返回的引用不会悬垂
        if x.len() > y.len() { x } else { y }
    }

    let s1 = "long string";
    let s2 = "short";
    let result = longest(s1, s2);
    println!("较长的字符串: {}", result);

    // 结构体中的生命周期注解
    // 当结构体包含引用时，必须为引用添加生命周期注解
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str, // part 字段的引用生命周期不能超过 'a
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence, // first_sentence 引用自 novel
    };
    println!("重要摘录: {:?}", i);

    // 结构体生命周期的含义：
    // ImportantExcerpt<'a> 实例的生命周期不能超过 'a
    // 这确保了结构体中的引用始终有效

    // 生命周期子类型（Lifetime Subtyping）
    // 描述生命周期之间的包含关系：'b: 'a 表示 'b 的生命周期包含 'a
    fn get_first<'a, 'b: 'a>(x: &'a str, _y: &'b str) -> &'a str {
        // 'b: 'a 表示 'b 的生命周期必须比 'a 长或相等
        // 这允许函数在必要时选择较长的生命周期
        x // 返回 'a 生命周期的引用
    }

    let long_lived = "this is a long lived string";
    {
        let short_lived = "short";
        let result = get_first(long_lived, short_lived);
        println!("生命周期子类型结果: {}", result);
    } // short_lived 离开作用域，但 result 仍然有效

    // 生命周期子类型的应用场景：
    // 1. 灵活的 API 设计：允许不同的生命周期选择
    // 2. 库的设计：提供更灵活的接口
    // 3. 复杂数据结构：处理嵌套的引用关系

    // 'static 生命周期：静态生命周期
    // 'static 表示引用在整个程序运行期间都有效
    let static_str: &'static str = "这是一个静态生命周期的字符串";
    println!("静态生命周期字符串: {}", static_str);

    // 'static 的特点：
    // 1. 字符串字面量具有 'static 生命周期
    // 2. 全局静态变量具有 'static 生命周期
    // 3. 泄漏的 Box<T> 可能获得 'static 生命周期
    // 4. 'static 是最长的生命周期，其他生命周期都可以转换为 'static

    // 生命周期的高级特性：
    // 1. 生命周期约束：可以定义复杂的生命周期关系
    // 2. HRTB（Higher-Ranked Trait Bounds）：高阶 trait 界限
    // 3. 生命周期对象：将生命周期作为类型的一部分
    // 4. 生命周期投影：在复杂类型中提取生命周期

    // 高级生命周期示例：
    /*
    // HRTB 示例
    trait Factory {
        type Output;
        fn create(&self) -> Self::Output;
    }

    fn with_factory<F, T>(factory: &F) -> T
    where
        F: for<'a> Factory<Output = &'a T>,
    {
        factory.create()
    }
    */

    // 生命周期的最佳实践：
    // 1. 优先使用生命周期省略：让编译器自动推断
    // 2. 明确表达意图：在必要时添加显式标注
    // 3. 避免过度约束：使用生命周期子类型提供灵活性
    // 4. 谨慎使用 'static：避免不必要的静态生命周期
    // 5. 文档化：清楚说明复杂生命周期关系的含义

    println!();
}

// ===========================================
// 6. 高级 trait 特性（Advanced Trait Features）
// ===========================================

// 高级 trait 特性是 Rust 类型系统中最强大的部分之一
// 它们提供了代码复用、抽象和灵活性的强大机制
// 通过关联类型、默认参数、完全限定语法等特性，Rust 能够构建复杂而优雅的类型系统

// 高级 trait 的核心价值：
// 1. 代码复用：通过 trait 实现跨类型的代码共享
// 2. 抽象能力：定义通用的行为接口
// 3. 灵活性：支持多种编程模式和设计模式
// 4. 类型安全：在编译时保证类型一致性
// 5. 扩展性：为现有类型添加新功能

fn advanced_trait_features() {
    println!("=== 高级 trait 特性 ===");

    // 关联类型（Associated Types）
    // 关联类型是 trait 中的一种类型占位符，由实现者决定具体类型
    // 这比泛型参数更灵活，适用于 trait 内部需要固定类型关联的场景
    trait Container {
        type Item; // 关联类型，由实现者指定具体类型

        // 创建新的容器实例
        fn new() -> Self;

        // 向容器添加项目
        fn add(&mut self, item: Self::Item);

        // 获取指定位置的项目的引用
        fn get(&self, index: usize) -> Option<&Self::Item>;
    }

    // 为自定义 Vec 类型实现 Container trait
    #[derive(Debug)]
    struct MyVec<T> {
        items: Vec<T>, // 内部使用标准库的 Vec 存储
    }

    // 为所有类型 T 实现 Container trait
    impl<T> Container for MyVec<T> {
        type Item = T; // 指定关联类型为 T

        fn new() -> Self {
            MyVec { items: Vec::new() }
        }

        fn add(&mut self, item: Self::Item) {
            self.items.push(item);
        }

        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
    }

    // 使用关联类型的容器
    let mut container = MyVec::new();
    container.add(1);
    container.add(2);
    container.add(3);

    println!("容器: {:?}", container);
    println!("容器[0]: {:?}", container.get(0));

    // 关联类型 vs 泛型参数：
    // 关联类型：每个实现只能有一个具体类型，适用于内部类型固定的场景
    // 泛型参数：可以为不同类型多次实现，灵活性更高

    // 默认泛型参数和运算符重载（Default Generic Parameters & Operator Overloading）
    // 运算符重载让自定义类型可以像内置类型一样使用运算符
    // 默认泛型参数让 trait 更加灵活和易于使用
    use std::ops::Add;

    // 定义两种不同的单位类型
    #[derive(Debug, PartialEq)]
    struct Millimeters(u32); // 毫米
    #[derive(Debug, PartialEq)]
    struct Meters(u32); // 米

    // 实现 Millimeters + Meters，返回 Millimeters
    // 这展示了不同类型之间的运算符重载
    impl Add<Meters> for Millimeters {
        type Output = Millimeters; // 指定返回类型

        fn add(self, other: Meters) -> Self::Output {
            // 1米 = 1000毫米，进行单位转换
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // 使用重载的 + 运算符
    let distance = Millimeters(500) + Meters(2);
    println!("距离: {:?} 毫米", distance);
    assert_eq!(distance, Millimeters(2500));

    // 运算符重载的设计原则：
    // 1. 直观性：运算符的行为应该符合用户的直觉预期
    // 2. 一致性：与内置类型的运算符行为保持一致
    // 3. 性能：避免不必要的性能开销
    // 4. 安全性：确保运算的安全性约束

    // 完全限定语法（Fully Qualified Syntax）
    // 当一个类型实现了多个具有相同方法名的 trait 时，需要使用完全限定语法来消除歧义
    trait Pilot {
        fn fly(&self); // 飞行能力
    }

    trait Wizard {
        fn fly(&self); // 魔法飞行能力
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("飞行员在驾驶飞机");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("巫师在骑扫帚");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("人类在挥舞手臂");
        }
    }

    let person = Human;
    person.fly(); // 默认调用类型本身的实现

    // 使用完全限定语法调用特定的 trait 实现
    // 语法：<Type as Trait>::function(&instance)
    Pilot::fly(&person); // 调用 Pilot trait 的实现
    Wizard::fly(&person); // 调用 Wizard trait 的实现

    // 完全限定语法的应用场景：
    // 1. 方法名冲突时消除歧义
    // 2. 在泛型代码中明确指定 trait
    // 3. 调用父 trait 的方法
    // 4. 在 trait 对象上调用方法

    // 父 trait（Supertraits）
    // 父 trait 定义了 trait 之间的继承关系，子 trait 必须先实现父 trait
    trait Vehicle {
        fn drive(&self); // 车辆必须能够驾驶
    }

    // Car trait 继承自 Vehicle trait
    // 这意味着任何实现 Car 的类型必须先实现 Vehicle
    trait Car: Vehicle {
        fn honk(&self); // 汽车特有的鸣笛功能
    }

    struct Sedan;

    // 必须先实现父 trait Vehicle
    impl Vehicle for Sedan {
        fn drive(&self) {
            println!("轿车在行驶");
        }
    }

    // 然后才能实现子 trait Car
    impl Car for Sedan {
        fn honk(&self) {
            println!("轿车在鸣笛");
        }
    }

    let car = Sedan;
    car.drive(); // 可以调用父 trait 的方法
    car.honk(); // 也可以调用子 trait 的方法

    // 父 trait 的应用场景：
    // 1. 依赖关系：表达 trait 之间的逻辑依赖
    // 2. 功能分层：构建层次化的 trait 体系
    // 3. 约束传递：自动传递父 trait 的约束条件
    // 4. 代码组织：更好地组织相关的功能

    // trait 的其他高级特性：
    // 1. 标记 trait（Marker Traits）：如 Send、Sync、Copy
    // 2. trait 对象：动态分发机制
    // 3. trait 别名：为复杂的 trait 界限创建别名
    // 4. trait 继承：多重继承和钻石问题处理

    println!();
}

// ===========================================
// 7. 高级类型特性（Advanced Type Features）
// ===========================================

// 高级类型特性展示了 Rust 类型系统的强大和灵活性
// 这些特性在类型安全、抽象能力和代码组织方面提供了丰富的工具
// 通过合理使用这些特性，可以构建更加清晰、安全和可维护的代码

// 高级类型特性的核心价值：
// 1. 类型安全：在编译时捕获更多错误
// 2. 抽象能力：创建更高层次的抽象
// 3. 代码组织：更好地组织和管理代码
// 4. 性能优化：零成本的抽象和优化
// 5. 互操作性：与其他系统和语言的集成

fn advanced_type_features() {
    println!("=== 高级类型特性 ===");

    // Newtype 模式（Newtype Pattern）
    // Newtype 模式通过创建包装类型来提供类型安全和封装
    // 这是 Rust 中实现类型安全和零成本抽象的重要模式
    struct Years(i64); // 包装 i64 类型，表示年份
    struct Days(i64); // 包装 i64 类型，表示天数

    // 为 Years 类型实现方法
    impl Years {
        fn to_days(&self) -> Days {
            // 将年份转换为天数（假设 1 年 = 365 天）
            Days(self.0 * 365)
        }
    }

    // 使用 Newtype 类型
    let years = Years(2);
    let days = years.to_days();
    println!("2 年 = {} 天", days.0);

    // Newtype 模式的优势：
    // 1. 类型安全：防止单位混淆（如将天数当作年份使用）
    // 2. 封装：隐藏实现细节，提供清晰的接口
    // 3. 扩展性：可以为特定类型添加专门的方法
    // 4. 性能：零成本抽象，编译时会优化掉
    // 5. 文档化：类型名称本身就是文档

    // Newtype 的实际应用场景：
    // 1. 单位系统：长度、重量、时间等物理单位
    // 2. 验证类型：包装类型并在构造时验证
    // 3. 特化行为：为特定类型提供特殊实现
    // 4. 抽象边界：隐藏内部实现，暴露抽象接口

    // 类型别名（Type Aliases）
    // 类型别名为现有类型创建新名称，但不会创建新类型
    // 它主要用于提高代码可读性和简化复杂类型
    type Kilometers = i32; // 为 i32 创建别名，表示公里数

    // 使用类型别名
    let distance: Kilometers = 5;
    println!("距离: {} 公里", distance);

    // 类型别名 vs Newtype：
    // 类型别名：完全等价于原类型，可以隐式转换
    // Newtype：创建新类型，需要显式转换

    // 类型别名的常见用途：
    // 1. 文档化：让类型名称更具描述性
    // 2. 简化：简化复杂的泛型类型
    // 3. 平台抽象：为不同平台提供统一的类型名称
    // 4. 代码维护：便于后续更改底层类型

    // 函数指针（Function Pointers）
    // 函数指针允许将函数作为值来传递和存储
    // 这是函数式编程和高阶函数的基础
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // 创建函数指针
    // fn 是函数指针类型的关键字
    let f: fn(i32) -> i32 = add_one;
    println!("函数指针调用: {}", f(5));

    // 函数指针的注意事项：
    // 1. 函数指针只能指向函数，不能指向闭包
    // 2. 函数指针可以复制、比较和存储
    // 3. 函数指针是 Sized 类型，大小已知
    // 4. 函数指针支持泛型和生命周期

    // 闭包作为函数参数（Closures as Function Parameters）
    // 闭包比函数指针更灵活，可以捕获环境变量
    // 使用 trait 界限来约束闭包的行为
    fn call_with_twice<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32, // Fn trait 表示闭包可以获取不可变引用
    {
        f(2) * 2
    }

    let result = call_with_twice(|x| x + 1);
    println!("闭包作为参数: {}", result);

    // 闭包 trait 的三种形式：
    // Fn：获取不可变引用，不修改环境
    // FnMut：获取可变引用，可以修改环境
    // FnOnce：获取所有权，只能调用一次

    // 从不返回类型（Never Type）
    // ! 类型表示永不返回，用于表示程序终止或无限循环
    // 这在类型系统中提供了更强的保证
    fn never_returns() -> ! {
        panic!("这个函数永远不会返回");
        // 或者无限循环：loop {}
    }

    // Never 类型的特点：
    // 1. 可以转换为任何类型（空类型）
    // 2. 在控制流分析中很有用
    // 3. 用于表示不可能的情况
    // 4. 在错误处理中标记致命错误

    // 动态大小类型和 Sized trait（Dynamically Sized Types & Sized Trait）
    // 动态大小类型是编译时大小未知的类型，如 [T] 和 str
    // Sized trait 标记编译时大小已知的类型
    fn generic_function<T: Sized>(t: T) -> T {
        // T: Sized 约束要求 T 是编译时大小已知的类型
        // 这是泛型的默认约束，通常可以省略
        t
    }

    let result = generic_function(42);
    println!("泛型函数结果: {}", result);

    // 动态大小类型的处理方式：
    // 1. 使用指针：&str、Box<[T]>、&[T]
    // 2. 使用 trait 对象：dyn Trait
    // 3. 使用固定大小的数组：[T; N]

    // 使用 Box 处理动态大小类型
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // 将 Circle 转换为 trait 对象
    // dyn Shape 表示动态分发的大小不确定的类型
    let circle = Circle { radius: 1.0 };
    let boxed_circle: Box<dyn Shape> = Box::new(circle);
    println!("圆的面积: {}", boxed_circle.area());

    // Trait 对象的特点：
    // 1. 动态分发：运行时确定具体实现
    // 2. 类型擦除：隐藏具体类型信息
    // 3. 大小不确定：需要通过指针使用
    // 4. 灵活性：可以统一处理不同类型

    // 动态分发 vs 静态分发：
    // 动态分发：运行时查找，灵活性高，有性能开销
    // 静态分发：编译时确定，性能好，但灵活性较低

    println!();
}

// ===========================================
// 8. 高级函数和闭包（Advanced Functions and Closures）
// ===========================================

// 高级函数和闭包是 Rust 函数式编程特性的核心
// 它们提供了强大的抽象能力和代码复用机制
// 通过高阶函数、闭包、函数组合等特性，Rust 支持多种编程范式

// 高级函数和闭包的核心价值：
// 1. 函数式编程：支持函数作为一等公民
// 2. 抽象能力：通过函数组合构建复杂逻辑
// 3. 代码复用：避免重复代码，提高复用性
// 4. 灵活性：动态生成和传递函数
// 5. 性能：闭包可以优化为静态调用

fn advanced_functions_and_closures() {
    println!("=== 高级函数和闭包 ===");

    // 函数指针（Function Pointers）
    // 函数指针是指向函数的指针，可以作为参数传递或存储
    // 它们是实现高阶函数和回调机制的基础
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // 函数指针作为参数
    // 这是高阶函数的典型用法：接受函数作为参数
    fn math_operation<F>(a: i32, b: i32, operation: F) -> i32
    where
        F: Fn(i32, i32) -> i32, // 约束 F 必须是接受两个 i32 并返回 i32 的函数
    {
        operation(a, b) // 调用传入的函数
    }

    // 将函数指针作为参数传递
    let result1 = math_operation(5, 3, add);
    let result2 = math_operation(5, 3, subtract);
    println!("函数指针操作: {} {}", result1, result2);

    // 函数指针的特点：
    // 1. 编译时已知：函数签名在编译时确定
    // 2. 零成本：函数调用可以直接优化为静态调用
    // 3. 类型安全：编译时检查类型匹配
    // 4. 可序列化：函数指针可以存储和传递

    // 返回闭包（Returning Closures）
    // 闭包可以捕获环境变量，这使得它们比函数指针更灵活
    // 返回闭包时需要注意生命周期和所有权问题
    fn create_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
        // 使用 Box 包装闭包，因为闭包大小在编译时未知
        // move 关键字强制闭包获取 factor 的所有权
        Box::new(move |x| x * factor)
    }

    // 使用返回的闭包
    let multiply_by_3 = create_multiplier(3);
    let result = multiply_by_3(5);
    println!("闭包返回值: {}", result);

    // 返回闭包的注意事项：
    // 1. 所有权：使用 move 或明确处理生命周期
    // 2. 动态分发：返回的闭包通常使用 trait 对象
    // 3. 内存管理：考虑堆分配和内存使用
    // 4. 性能：权衡动态分发的开销

    // 高阶函数（Higher-Order Functions）
    // 高阶函数是指接受函数作为参数或返回函数的函数
    // 它们是函数式编程的核心概念
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32, // F 可以是函数指针或闭包
    {
        // 将函数 f 应用到 x，然后将结果再次应用 f
        f(f(x))
    }

    let result = apply_twice(|x| x + 1, 5);
    // 计算：f(f(5)) = f(5 + 1) = f(6) = 6 + 1 = 7
    println!("高阶函数: {}", result);

    // 高阶函数的应用场景：
    // 1. 数据转换：map、filter、reduce 等
    // 2. 函数组合：将多个小函数组合成复杂函数
    // 3. 策略模式：通过函数实现不同的算法策略
    // 4. 回调机制：异步编程和事件处理

    // 函数组合（Function Composition）
    // 函数组合是将多个函数链接起来，形成新的函数
    // 这是数学中函数组合概念的编程实现
    fn compose<F, G>(f: F, g: G) -> Box<dyn Fn(i32) -> i32>
    where
        F: Fn(i32) -> i32 + 'static, // + 'static 表示函数不借用外部变量
        G: Fn(i32) -> i32 + 'static,
    {
        // 返回一个新的闭包，该闭包依次调用 g 和 f
        // 数学上表示为：f ∘ g (x) = f(g(x))
        Box::new(move |x| f(g(x)))
    }

    // 定义两个基础函数
    let add_one = |x| x + 1;
    let double = |x| x * 2;

    // 组合函数：先加 1，然后乘以 2
    let add_then_double = compose(double, add_one);

    let result = add_then_double(5);
    // 计算：double(add_one(5)) = double(5 + 1) = double(6) = 6 * 2 = 12
    println!("函数组合: {}", result);

    // 函数组合的优势：
    // 1. 可读性：表达复杂的计算流程
    // 2. 模块化：将复杂逻辑分解为简单函数
    // 3. 重用性：小函数可以在不同组合中重用
    // 4. 测试性：小函数更容易单独测试

    // 闭包的高级特性：
    // 1. 捕获环境：闭包可以访问外部变量
    // 2. 三种 trait：Fn、FnMut、FnOnce 对应不同的访问权限
    // 3. 优化：编译器可能将闭包优化为静态调用
    // 4. 迭代器适配器：map、filter、fold 等使用闭包

    // 函数式编程的实际应用：
    // 1. 数据处理：链式处理数据流
    // 2. 并发编程：函数式的并发模式
    // 3. 事件驱动：响应式编程模式
    // 4. 算法实现：清晰的算法表达

    // 闭包的生命周期考虑：
    // 1. 借用检查：确保引用的有效性
    // 2. 移动语义：处理所有权转移
    // 3. 静态生命周期：'static 约束的使用
    // 4. 内存管理：避免内存泄漏和悬垂引用

    println!();
}

// ===========================================
// 9. 高级错误处理（Advanced Error Handling）
// ===========================================

// 高级错误处理是 Rust 错误管理体系的深入部分
// 它提供了创建自定义错误类型、错误链、上下文信息等强大功能
// 通过合理的错误处理，可以构建更加健壮和用户友好的应用程序

// 高级错误处理的核心价值：
// 1. 信息丰富：提供详细的错误信息和上下文
// 2. 错误追踪：保留错误发生的完整路径
// 3. 用户友好：为不同层次的用户提供适当的错误信息
// 4. 调试支持：帮助开发者快速定位问题
// 5. 恢复能力：支持错误的优雅处理和恢复

fn advanced_error_handling() {
    println!("=== 高级错误处理 ===");

    // 自定义错误类型（Custom Error Types）
    // 自定义错误类型让应用程序能够表达特定领域的错误
    // 它提供了比标准库错误类型更丰富的信息
    #[derive(Debug)] // 支持调试输出
    enum MyError {
        Io(String),      // IO 操作错误，包含错误信息
        Parse(String),   // 解析错误，包含解析失败的详细信息
        Network(String), // 网络错误，包含网络相关的错误信息
    }

    // 实现 Display trait 以支持用户友好的错误信息显示
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                MyError::Io(msg) => write!(f, "IO 错误: {}", msg),
                MyError::Parse(msg) => write!(f, "解析错误: {}", msg),
                MyError::Network(msg) => write!(f, "网络错误: {}", msg),
            }
        }
    }

    // 实现 Error trait 以支持错误链和标准错误处理
    impl std::error::Error for MyError {}

    // 使用自定义错误类型
    // 自定义错误类型可以包含更丰富的上下文信息
    fn read_file() -> Result<String, MyError> {
        // 模拟文件读取错误
        Err(MyError::Io("无法打开文件".to_string()))
    }

    match read_file() {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("错误: {}", e),
    }

    // 自定义错误类型的设计原则：
    // 1. 领域特定：错误类型应该反映应用领域的特点
    // 2. 信息完整：包含足够的信息用于调试和恢复
    // 3. 层次清晰：错误类型应该有清晰的层次结构
    // 4. 转换容易：支持与其他错误类型的转换

    // 错误链（Error Chaining）
    // 错误链允许将一个错误包装在另一个错误中，保留原始错误信息
    // 这对于调试和错误追踪非常重要
    use std::error::Error;
    use std::fmt;

    // 定义包装错误类型
    #[derive(Debug)]
    struct MyWrappedError {
        source: Box<dyn Error + Send + Sync>, // 存储原始错误
    }

    // 实现 Display trait
    impl fmt::Display for MyWrappedError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "包装的错误: {}", self.source)
        }
    }

    // 实现 Error trait
    impl Error for MyWrappedError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&*self.source) // 返回原始错误的引用
        }
    }

    // 错误链的优势：
    // 1. 保留上下文：不丢失原始错误信息
    // 2. 层次化：可以构建多层次的错误结构
    // 3. 调试友好：提供了完整的错误追踪路径
    // 4. 用户友好：可以为不同用户提供适当的抽象级别

    // ? 操作符的早期返回（Early Return with ? Operator）
    // ? 操作符是 Rust 错误处理的语法糖，简化了错误传播
    // 它使得错误处理代码更加简洁和易读
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>() // 使用标准库的解析函数
    }

    fn process_number(s: &str) -> Result<i32, Box<dyn Error>> {
        // ? 操作符会在错误时自动返回 Err
        let num = parse_number(s)?; // 如果解析失败，立即返回错误

        // 添加额外的验证逻辑
        if num < 0 {
            return Err("数字不能为负数".into()); // 使用 into() 转换为错误对象
        }
        Ok(num * 2)
    }

    // 测试正常情况
    match process_number("42") {
        Ok(result) => println!("处理结果: {}", result),
        Err(e) => println!("处理错误: {}", e),
    }

    // 测试错误情况（负数）
    match process_number("-5") {
        Ok(result) => println!("处理结果: {}", result),
        Err(e) => println!("处理错误: {}", e),
    }

    // ? 操作符的注意事项：
    // 1. 类型转换：错误类型必须能够相互转换
    // 2. 早期返回：遇到第一个错误就立即返回
    // 3. 性能考虑：避免过度使用导致性能问题
    // 4. 错误上下文：可能丢失一些上下文信息

    // 错误处理的最佳实践：
    // 1. 使用 Result：明确区分成功和失败的情况
    // 2. 自定义错误类型：为特定场景创建合适的错误类型
    // 3. 错误链：保留原始错误信息，便于调试
    // 4. 错误上下文：添加足够的上下文信息
    // 5. 错误恢复：在可能的情况下提供恢复机制
    // 6. 日志记录：记录错误信息以便后续分析
    // 7. 用户友好：为终端用户提供清晰的错误信息

    // 错误处理的模式：
    // 1. 快速失败：遇到错误立即返回（fail-fast）
    // 2. 错误聚合：收集多个错误后统一处理
    // 3. 错误恢复：尝试从错误中恢复
    // 4. 错误降级：在错误情况下提供降级服务

    // 错误处理的工具和库：
    // 1. thiserror：用 derive 简化自定义错误类型的创建（库代码首选）
    // 2. anyhow：提供简化的错误处理和上下文添加（应用代码首选）
    // 3. eyre（配合 color-eyre）：可定制的错误报告，支持彩色输出
    // 4. snafu：另一种基于 derive 的错误库，擅长为每一层错误附加上下文
    // （早期的 error-chain、failure 已被 thiserror + anyhow 这套组合取代，新项目不建议再使用）

    println!();
}

// ===========================================
// 10. const fn 改进 (Rust 1.46)
// ===========================================

// const fn 是 Rust 中可以在编译时计算的函数
// Rust 1.46 版本大幅扩展了 const fn 的功能，允许在编译时进行更多操作
// 这为编译时计算、零成本抽象和类型安全提供了强大的工具

// const fn 的核心价值：
// 1. 编译时计算：将运行时计算移到编译时，提高运行时性能
// 2. 常量泛型：支持在类型系统中使用编译时计算的值
// 3. 零成本抽象：const fn 在编译时会被优化掉，没有运行时开销
// 4. 类型安全：在编译时进行更多验证，减少运行时错误
// 5. 元编程：提供有限的编译时编程能力

fn const_fn_improvements() {
    println!("=== const fn 改进 ===");

    // const fn 基础：编译时计算
    // const fn 可以在编译时执行，生成常量值
    const fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    const SUM: i32 = add(10, 20); // 在编译时计算
    println!("编译时计算: 10 + 20 = {}", SUM);

    // 浮点运算：在 const fn 里做浮点计算要到 Rust 1.82 才稳定，在那之前这个函数编译不过。
    // （Rust 1.46 带来的是下面要演示的 if / match / loop 控制流，而不是浮点运算。）
    const fn calculate_circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }

    const AREA: f64 = calculate_circle_area(5.0);
    println!("圆的面积 (r=5): {}", AREA);

    // 条件表达式在 const fn 中的支持
    const fn abs(x: i32) -> i32 {
        if x >= 0 { x } else { -x }
    }

    const ABS_VALUE: i32 = abs(-42);
    println!("绝对值: {}", ABS_VALUE);

    // 循环在 const fn 中的支持
    const fn factorial(n: u32) -> u32 {
        let mut result = 1;
        let mut i = 2;
        while i <= n {
            result *= i;
            i += 1;
        }
        result
    }

    const FACT_5: u32 = factorial(5);
    println!("5! = {}", FACT_5);

    // match 表达式在 const fn 中的支持
    const fn describe_number(n: i32) -> &'static str {
        match n {
            0 => "零",
            1 => "一",
            2 => "二",
            3..=10 => "小数字",
            _ => "大数字",
        }
    }

    const DESCRIPTION: &'static str = describe_number(7);
    println!("数字描述: {}", DESCRIPTION);

    // 数组和切片操作在 const fn 中的支持
    const fn array_sum(arr: &[i32; 5]) -> i32 {
        let mut sum = 0;
        let mut i = 0;
        while i < arr.len() {
            sum += arr[i];
            i += 1;
        }
        sum
    }

    const NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];
    const ARRAY_SUM: i32 = array_sum(&NUMBERS);
    println!("数组求和: {}", ARRAY_SUM);

    // 闭包和函数指针的限制
    // 从 Rust 1.61 起，函数指针可以在 const fn 里被创建、转换和传递；
    // 但**调用**函数指针（以及调用闭包）仍然不被允许——
    // 编译器无法在编译期确定它指向的是不是一个 const 函数。
    // 下面这个函数会报错 "function pointer calls are not allowed in constant functions"，
    // 所以以注释形式保留：
    // const fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    //     op(x, y) // 编译错误：不能在 const fn 中调用函数指针
    // }

    // 正确的方式：使用 const 函数作为操作
    const fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }

    const PRODUCT: i32 = multiply(6, 7);
    println!("乘法结果: {}", PRODUCT);

    // const fn 的实际应用场景
    // 场景 1：编译时的数学计算
    const fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                let mut i = 2;
                while i <= n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                    i += 1;
                }
                b
            }
        }
    }

    const FIB_10: u32 = fibonacci(10);
    println!("斐波那契数列第10项: {}", FIB_10);

    // 场景 2：编译时字符串处理
    const fn string_length(s: &str) -> usize {
        s.len() // 直接使用 len() 方法，它在 const 上下文中可用
    }

    const STR_LEN: usize = string_length("Hello");
    println!("字符串长度: {}", STR_LEN);

    // 场景 3：编译时类型验证
    const fn is_power_of_two(n: u32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }

    const IS_POW2: bool = is_power_of_two(16);
    println!("16 是 2 的幂: {}", IS_POW2);

    // 场景 4：编译时的位操作
    const fn count_ones(n: u32) -> u32 {
        let mut count = 0;
        let mut x = n;
        while x != 0 {
            count += x & 1;
            x >>= 1;
        }
        count
    }

    const ONES: u32 = count_ones(0b10101010);
    println!("0b10101010 中 1 的个数: {}", ONES);

    // const fn 的限制和注意事项
    // 1. 不能进行堆分配（`Box::new`、`vec!` 等在编译期都不可用）
    // 2. 不能调用 trait 方法（对基本类型的运算符，如整数加法，由编译器内置支持，不在此限）；
    //    允许 const fn 调用 trait 方法的 `const trait` 目前仍是不稳定特性
    // 3. 不能调用闭包和函数指针
    // 4. 不能进行 I/O 等有副作用的操作
    // 5. `panic!` / `assert!` 可以使用（Rust 1.57 起）：
    //    在编译期求值时触发，就会变成编译错误，这正是“编译期校验”的常用手法
    // 6. 递归是允许的，只受编译器对求值步数的 lint 约束（Rust 1.72 起不再有硬性上限）

    // const fn 的最佳实践
    // 1. 纯函数：const fn 应该是纯函数，没有副作用
    // 2. 简单逻辑：保持 const fn 的逻辑简单明了
    // 3. 性能考虑：只在确实需要编译时计算时使用
    // 4. 测试覆盖：为 const fn 编写测试，确保编译时和运行时行为一致
    // 5. 文档化：清楚说明 const fn 的约束和使用条件

    // const fn 的发展历程（已对照官方发布说明核实）
    // 1.31：const fn 稳定，但只支持很小的子集（基本算术、调用其他 const fn）
    // 1.33：允许 let 绑定、赋值、不可反驳的模式解构，以及调用 unsafe const fn
    // 1.46：允许 if / match / loop（含 while）
    // 1.56：允许读取 union 字段
    // 1.57：允许 panic! / assert!
    // 1.58：允许解引用 `*const T`
    // 1.61：允许泛型约束、`impl Trait`、创建和传递函数指针
    // 1.82：允许浮点运算
    // 1.83：允许 `&mut` / `*mut` / `&Cell` 等（见后面的第 17 节）
    // （const 泛型参数是另一条线：Rust 1.51 稳定，见下一节。）

    // 目前仍在 nightly 上推进、尚未稳定的 const 方向（以 nightly 为准，stable 上不可用）：
    // 1. `const_trait_impl`：让 trait 方法也能在 const fn 里调用（标准库内部已在使用）
    // 2. `const_heap`：允许在编译期做堆分配
    // 3. `generic_const_exprs`：允许对泛型参数做运算后再当常量用，如 `[T; N * 2]`
    // 4. 标准库中越来越多的函数被逐版本标记为 const（每个版本的发布说明里都有清单）

    println!();
}

// ===========================================
// 11. const 泛型参数 (Rust 1.51)
// ===========================================

// const 泛型参数是 Rust 1.51 版本引入的革命性特性
// 它允许在泛型类型和函数中使用编译时常量作为参数
// 这为数组长度、位宽度、内存对齐等编译时已知的概念提供了类型安全的抽象

// const 泛型的核心价值：
// 1. 类型安全：在编译时验证常量约束，避免运行时错误
// 2. 零成本抽象：编译时优化，无运行时开销
// 3. 表达能力：能够表达更复杂的类型关系和约束
// 4. 泛型编程：扩展了泛型编程的应用场景
// 5. 编程体验：减少了代码重复，提高了代码复用性

fn const_generic_parameters() {
    println!("=== const 泛型参数 ===");

    // const 泛型基础：数组大小的抽象
    // const 泛型最常见的用途是处理编译时已知的数组大小
    fn process_array<const N: usize>(arr: [i32; N]) -> [i32; N] {
        let mut result = [0; N];
        for i in 0..N {
            result[i] = arr[i] * 2; // 将每个元素乘以 2
        }
        result
    }

    // 使用 const 泛型函数处理不同大小的数组
    let arr3 = [1, 2, 3];
    let arr5 = [1, 2, 3, 4, 5];

    let result3 = process_array(arr3);
    let result5 = process_array(arr5);

    println!("3 元素数组处理结果: {:?}", result3);
    println!("5 元素数组处理结果: {:?}", result5);

    // const 泛型的类型检查
    // 编译器会在编译时验证常量参数的有效性
    fn create_buffer<const SIZE: usize>() -> [u8; SIZE] {
        [0; SIZE] // 创建指定大小的零初始化数组
    }

    // 创建不同大小的缓冲区
    let buffer_64 = create_buffer::<64>(); // 64 字节缓冲区
    let buffer_128 = create_buffer::<128>(); // 128 字节缓冲区

    println!("64 字节缓冲区长度: {}", buffer_64.len());
    println!("128 字节缓冲区长度: {}", buffer_128.len());

    // 标准库里的数组 trait：任意长度都能用（Rust 1.47）
    // Rust 1.47 之前，标准库只为长度不超过 32 的数组实现 Debug、Clone、PartialEq 等 trait，
    // 上面 64 字节的缓冲区连 `{:?}` 打印、`==` 比较都做不到——这是 const 泛型出现之前的历史局限。
    // 1.47 起这些 trait 改用 const 泛型来实现，对**任意长度**的数组都成立：
    println!(
        "64 字节缓冲区的 Debug 文本长度 = {}，128 字节缓冲区克隆后相等: {}",
        format!("{buffer_64:?}").len(),
        buffer_128 == buffer_128.clone()
    );
    // 例外：`Default` 仍然只对长度不超过 32 的数组实现（为了保持兼容）。
    // 所以 `let a: [u8; 33] = Default::default();` 无法编译，需要写成 `[0; 33]` 这样的数组表达式。

    // const 泛型与数值运算
    // 可以在 const 泛型参数上进行编译时数值运算
    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        fn new() -> Self {
            Matrix {
                data: [[T::default(); COLS]; ROWS],
            }
        }

        fn get(&self, row: usize, col: usize) -> &T {
            &self.data[row][col]
        }

        fn set(&mut self, row: usize, col: usize, value: T) {
            self.data[row][col] = value;
        }

        fn rows(&self) -> usize {
            ROWS
        }

        fn cols(&self) -> usize {
            COLS
        }
    }

    // 使用 const 泛型创建固定大小的矩阵
    let mut matrix_3x3 = Matrix::<i32, 3, 3>::new();
    matrix_3x3.set(0, 0, 1);
    matrix_3x3.set(1, 1, 2);
    matrix_3x3.set(2, 2, 3);

    println!("3x3 矩阵: {}x{}", matrix_3x3.rows(), matrix_3x3.cols());
    println!(
        "对角线元素: {}, {}, {}",
        matrix_3x3.get(0, 0),
        matrix_3x3.get(1, 1),
        matrix_3x3.get(2, 2)
    );

    // const 泛型约束：对常量参数的限制
    // 可以使用 where 子句对 const 泛型参数添加约束
    // 注意：这是不稳定的特性（generic_const_exprs），stable 上无法使用。
    // 在 nightly 工具链上，需要在 crate 根部写 `#![feature(generic_const_exprs)]` 才能启用。
    // （Cargo 的 `[unstable] build-std` 是“从源码重新编译标准库”的选项，和语言特性开关无关。）
    /*
    fn power_of_two_buffer<const N: usize>() -> [u8; N]
    where
        [(); N - 1]:,  // 确保 N > 0
        [(); { N & (N - 1) }]:, // 确保 N 是 2 的幂
    {
        [0; N] // 创建 2 的幂大小的缓冲区
    }
    */

    // 由于泛型参数在常量操作中的限制，这里使用具体的数值展示
    fn power_of_two_buffer_256() -> [u8; 256] {
        [0; 256] // 创建 256 字节的缓冲区
    }

    // 创建符合约束的缓冲区
    let buffer_256 = power_of_two_buffer_256(); // 256 是 2 的幂
    println!("256 字节缓冲区长度: {}", buffer_256.len());

    // const 泛型与位操作：编译时的位宽度抽象
    // 注意方法名不能叫 `max_value`：整数类型自带同名的固有方法（已被弃用，建议改用 `MAX` 常量），
    // 而固有方法的优先级高于 trait 方法，`u8::max_value()` 会悄悄调用标准库那一个，
    // 根本不会走到我们自己的 trait 实现，编译器也只会给出一条容易被忽略的弃用警告。
    trait BitWidth<const WIDTH: u32> {
        fn width_max() -> Self;
        fn mask() -> Self;
    }

    impl BitWidth<8> for u8 {
        fn width_max() -> Self {
            u8::MAX
        }
        fn mask() -> Self {
            0xFF
        }
    }

    impl BitWidth<16> for u16 {
        fn width_max() -> Self {
            u16::MAX
        }
        fn mask() -> Self {
            0xFFFF
        }
    }

    impl BitWidth<32> for u32 {
        fn width_max() -> Self {
            u32::MAX
        }
        fn mask() -> Self {
            0xFFFFFFFF
        }
    }

    println!("8位最大值: {}", <u8 as BitWidth<8>>::width_max());
    println!("16位最大值: {}", <u16 as BitWidth<16>>::width_max());
    println!("32位最大值: {}", <u32 as BitWidth<32>>::width_max());

    // const 泛型的实际应用场景
    // 场景 1：固定大小的数据结构
    struct FixedSizeBuffer<const SIZE: usize> {
        data: [u8; SIZE],
        position: usize,
    }

    impl<const SIZE: usize> FixedSizeBuffer<SIZE> {
        fn new() -> Self {
            FixedSizeBuffer {
                data: [0; SIZE],
                position: 0,
            }
        }

        fn write(&mut self, byte: u8) -> Result<(), &'static str> {
            if self.position < SIZE {
                self.data[self.position] = byte;
                self.position += 1;
                Ok(())
            } else {
                Err("缓冲区已满")
            }
        }

        fn capacity(&self) -> usize {
            SIZE
        }

        fn len(&self) -> usize {
            self.position
        }
    }

    let mut buffer = FixedSizeBuffer::<16>::new();
    for i in 0..10 {
        buffer.write(i as u8).unwrap();
    }
    println!("固定大小缓冲区: {}/{}", buffer.len(), buffer.capacity());

    // 场景 2：编译时的位域抽象
    // 注意：下面带 `[(); ...]:` 约束的写法依赖不稳定的 generic_const_exprs，需要 nightly
    struct BitField<const WIDTH: u32> {
        value: u32,
    }

    // 注意：这个实现需要 generic_const_exprs 特性，暂时注释掉
    /*
    impl<const WIDTH: u32> BitField<WIDTH>
    where
        [(); WIDTH as usize]:,  // 需要 generic_const_exprs 特性
    {
        fn new(value: u32) -> Self {
            let mask = (1 << WIDTH) - 1;
            BitField {
                value: value & mask,
            }
        }

        fn get(&self) -> u32 {
            let mask = (1 << WIDTH) - 1;
            self.value & mask
        }

        fn set(&mut self, new_value: u32) {
            let mask = (1 << WIDTH) - 1;
            self.value = new_value & mask;
        }
    }
    */

    // 由于泛型常量表达式的限制，使用具体的位数展示
    impl BitField<5> {
        fn new(value: u32) -> Self {
            let mask = (1 << 5) - 1;
            BitField {
                value: value & mask,
            }
        }

        fn get(&self) -> u32 {
            self.value
        }

        fn set(&mut self, new_value: u32) {
            let mask = (1 << 5) - 1;
            self.value = new_value & mask;
        }
    }

    let mut bit_field_5 = BitField::<5>::new(0x1F); // 5位字段
    println!("5位字段值: 0x{:X}", bit_field_5.get());

    bit_field_5.set(0x20); // 超出 5 位范围的值会被截断
    println!("设置后 5 位字段值: 0x{:X}", bit_field_5.get());

    // 场景 3：内存对齐的抽象
    #[repr(C)]
    struct AlignedBuffer<const ALIGNMENT: usize> {
        data: [u8; 64],
        _align: std::marker::PhantomData<[(); ALIGNMENT]>, // 使用 PhantomData 代替
    }

    impl<const ALIGNMENT: usize> AlignedBuffer<ALIGNMENT> {
        fn new() -> Self {
            AlignedBuffer {
                data: [0; 64],
                _align: std::marker::PhantomData,
            }
        }

        fn alignment(&self) -> usize {
            ALIGNMENT
        }
    }

    let aligned_4 = AlignedBuffer::<4>::new();
    let aligned_16 = AlignedBuffer::<16>::new();

    println!("4 字节对齐缓冲区对齐: {}", aligned_4.alignment());
    println!("16 字节对齐缓冲区对齐: {}", aligned_16.alignment());

    // const 泛型的限制和注意事项
    // 1. 参数类型限制：const 泛型参数只能是某些基本类型
    // 2. 表达式限制：不能在 const 泛型参数中使用任意表达式
    // 3. 实现复杂性：复杂的 const 泛型约束可能难以理解和实现
    // 4. 编译时间：复杂的 const 泛型可能增加编译时间
    // 5. 错误信息：const 泛型的错误信息可能比较复杂

    // const 泛型的最佳实践
    // 1. 简单约束：保持 const 泛型约束的简单性
    // 2. 明确文档：清楚说明 const 泛型参数的含义和约束
    // 3. 类型安全：优先使用类型安全的方式表达约束
    // 4. 性能考虑：const 泛型应该提供真正的价值
    // 5. 测试覆盖：为不同的 const 泛型参数组合编写测试

    // const 泛型与 const fn 的结合
    // const 泛型与 const fn 可以很好地结合使用
    const fn array_size<T, const N: usize>(_arr: &[T; N]) -> usize {
        N
    }

    const fn is_even<const N: usize>() -> bool {
        N % 2 == 0
    }

    let test_array = [1, 2, 3, 4, 5];
    // 注意：const 不能使用运行时计算的值
    // 这是编译时限制的示例
    let size = array_size(&test_array);
    // const 泛型参数必须是编译时常量
    let is_even = size % 2 == 0;

    println!("数组大小: {}", size);
    println!("数组大小是偶数: {}", is_even);

    // const 泛型的未来发展
    // 1. 更多类型支持：支持更多的 const 泛型参数类型
    // 2. 更强大的约束：支持更复杂的编译时约束
    // 3. 更好的错误信息：提供更清晰的编译时错误
    // 4. 与其他特性的集成：与 trait、impl block 等特性的深度集成
    // 5. 标准库支持：在标准库中更广泛的应用

    println!();
}

// ===========================================
// 12. Rust 2021 Edition 新特性（Rust 2021 Edition Features）
// ===========================================

// Edition（版本纪元）是 Rust 用来引入“不向后兼容、但可以自动迁移”的语言改动的机制：
// 每隔几年发布一个新 edition，各个 crate 在 Cargo.toml 里用 `edition = "2021"` 选择自己使用哪一个，
// 不同 edition 的 crate 可以互相依赖、混合编译，彼此不受影响。
// Rust 2021 是第三个 edition（之前是 2015 和 2018），随 Rust 1.56 在 2021 年 10 月发布。
//
// 要分清两件事：edition 只负责“必须靠不兼容改动才能实现的变化”；
// 绝大多数新特性（格式化字符串捕获、let-else、GAT……）与 edition 无关，所有 edition 都能用。
//
// Rust 2021 的主要变化（出自官方 Edition Guide）：
// 1. 闭包按字段分别捕获（disjoint capture）
// 2. 预导入项新增了 `TryFrom`、`TryInto`、`FromIterator`
// 3. 数组的 `IntoIterator`：`array.into_iter()` 与 `for x in array` 按值迭代
// 4. `panic!` / `assert!` 的单参数形式也一律是格式化字符串
// 5. 宏模式 `$p:pat` 可以匹配顶层的或模式（`A | B`）
// 6. 为将来的前缀字面量（如 `f"..."`、`k#ident`）保留语法
// 7. Cargo 默认使用 feature 解析器 v2
// 8. 部分 lint 升级为错误：裸 trait 对象（`Box<Trait>` 必须写成 `Box<dyn Trait>`）、
//    已废弃的 `...` 范围模式
// 本项目使用的是 2024 edition，它延续并包含了上面这些变化。

fn rust_2021_edition_features() {
    println!("=== Rust 2021 Edition 新特性 ===");

    // ---------- 1. 闭包按字段分别捕获 ----------
    // 2018 edition：闭包只要用到结构体的任何一个字段，就会把整个结构体捕获进去。
    // 2021 起：闭包只捕获它真正用到的那几个字段。
    // 区别体现在借用冲突上：下面的闭包只可变借用了 `cfg.retries`，
    // 所以闭包存活期间仍然可以读取 `cfg.name`；
    // 在 2018 edition 里，闭包会把整个 `cfg` 可变借用起来，读取 `cfg.name` 就会编译失败。
    struct Config {
        name: String,
        retries: u32,
    }

    let mut cfg = Config {
        name: "svc".to_string(),
        retries: 3,
    };
    let mut bump = || cfg.retries += 1; // 只可变借用 cfg.retries
    println!("闭包存活期间读取另一个字段: {}", cfg.name);
    bump();
    println!("调用闭包之后 retries = {}", cfg.retries);

    // `move` 闭包同理：只把用到的字段移进闭包，其余字段仍留在原结构体里。
    let owner = Config {
        name: "owner".to_string(),
        retries: 0,
    };
    let take_name = move || owner.name; // 只移走 owner.name
    println!("没有被移走的字段仍可使用: retries = {}", owner.retries);
    println!("闭包取走的字段: {}", take_name());
    // 注意：捕获粒度变细，也会改变变量被 drop 的时机。
    // 依赖 drop 顺序的代码在升级 edition 时要重点检查，`cargo fix --edition` 会提示这类风险。

    // ---------- 2. 预导入项新增 TryFrom / TryInto / FromIterator ----------
    // 这三个 trait 从 2021 起自动在作用域内，不再需要 `use std::convert::{TryFrom, TryInto}`。
    // （很多人的印象正好相反：2021 是“新增”而不是“移除”。）
    // 注意 `FromStr` 不在任何 edition 的预导入项里，使用 `from_str` 仍要手动 use。
    let big_number: i64 = 1000;
    let small_number: i32 = big_number.try_into().unwrap_or(0); // 没有 use TryInto
    println!("TryInto 转换：{} -> {}", big_number, small_number);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl TryFrom<(i32, i32)> for Point {
        // 没有 use TryFrom
        type Error = &'static str;

        fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
            if value.0 >= 0 && value.1 >= 0 {
                Ok(Point {
                    x: value.0,
                    y: value.1,
                })
            } else {
                Err("坐标不能为负数")
            }
        }
    }

    println!("TryFrom 转换：{:?}", Point::try_from((10, 20)));
    println!("TryFrom 失败：{:?}", Point::try_from((-1, 5)));

    let squares = Vec::from_iter((1..=4).map(|n| n * n)); // FromIterator 也在预导入项里
    println!("FromIterator：{:?}", squares);

    // FromStr 需要手动引入
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    impl FromStr for Color {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "red" => Ok(Color::Red),
                "green" => Ok(Color::Green),
                "blue" => Ok(Color::Blue),
                _ => Err("无效的颜色"),
            }
        }
    }

    let color: Color = "red".parse().unwrap_or(Color::Red);
    println!("FromStr 解析：{:?}", color);

    // 升级 edition 时的副作用：如果你自己的 trait 里也有名为 `try_into` / `try_from` / `from_iter`
    // 的方法，预导入新增之后调用可能出现歧义（lint `rust_2021_prelude_collisions`），
    // `cargo fix --edition` 会自动把它们改写成完全限定语法。

    // ---------- 3. 数组按值迭代 ----------
    // 数组在 Rust 1.53 实现了按值的 `IntoIterator`，但为了不破坏旧代码，
    // 2018 edition 里 `array.into_iter()` 仍然得到元素的引用；
    // 2021 edition 起，`array.into_iter()` 和 `for x in array` 都按值迭代。
    let words = ["alpha".to_string(), "beta".to_string()];
    let owned: Vec<String> = words.into_iter().collect(); // 拿到 String 的所有权，而不是 &String
    println!("按值迭代数组: {owned:?}");

    // ---------- 4. panic! / assert! 的格式化一致 ----------
    // 2018 edition：`panic!("{}")` 里的 `{}` 被当作普通字符串原样输出，`panic!(123)` 可以抛出任意类型的负载。
    // 2021 起：`panic!` 的第一个参数永远是格式化字符串，和 `println!` / `format!` 保持一致：
    //     panic!("{}");     // 编译错误：格式串里有 {} 但没有提供参数
    //     panic!("{{}}");   // 想输出字面的 {} 需要转义
    //     panic!(123);      // 编译错误：第一个参数必须是字符串字面量
    // 需要抛出任意类型的负载时，改用 `std::panic::panic_any(123)`。

    // ---------- 5. 宏模式 $p:pat 支持顶层或模式 ----------
    // 2018 edition 里 `$p:pat` 不能匹配 `'a' | 'e'` 这样的顶层或模式（需要用 `pat_param`），2021 起可以。
    macro_rules! is_vowel {
        ($c:expr, $p:pat) => {
            matches!($c, $p)
        };
    }

    println!("'e' 是元音吗: {}", is_vowel!('e', 'a' | 'e' | 'i' | 'o' | 'u'));
    println!("'z' 是元音吗: {}", is_vowel!('z', 'a' | 'e' | 'i' | 'o' | 'u'));

    // ---------- 与 edition 无关的补充：格式化字符串捕获（Rust 1.58） ----------
    // 这是 Rust 1.58 的特性，所有 edition 都能用，不属于 2021 edition 的内容，
    // 放在这里是因为它和 `println!` 一起最常被拿来对照。
    // 它允许在格式化宏的字符串里直接使用同名变量：
    let user = "Bob";
    let score = 95;
    let level = 3;

    // 传统的格式化方式
    println!("传统方式：用户={}, 分数={}, 等级={}", user, score, level);
    // 格式化字符串捕获
    println!("捕获方式：用户={user}, 分数={score}, 等级={level}");

    // 捕获只能用于“单个标识符”，复杂表达式需要先算出来或使用命名参数
    let complex_expr = 10 + 20;
    println!("复杂表达式先算出来：{complex_expr}");
    println!("或者使用命名参数：{sum}", sum = 10 + 20);

    // 可以和格式说明符组合
    let memory_usage = 1024 * 1024 * 512; // 512MB
    println!("内存使用={memory_usage} bytes ({memory_usage:#x} hex, {memory_usage:#b} bin)");

    // 格式化捕获的最佳实践：
    // 1. 优先使用捕获语法，更简洁，也避免参数顺序写错
    // 2. 复杂表达式先用 let 绑定出来，或者使用命名参数
    // 3. 日志和调试输出里尤其适合用它提升可读性

    // 迁移到新 edition 的建议：
    // 1. 先运行 `cargo fix --edition`，让 Cargo 自动改写不兼容的写法
    // 2. 然后把 Cargo.toml 里的 `edition` 改成新版本，重新编译、跑测试
    // 3. 重点检查闭包捕获粒度变化带来的 drop 顺序变化，以及预导入新增造成的方法歧义
    // 4. 逐步采用新特性，而不是一次性重写

    println!();
}

// ===========================================
// 13. Rust 1.70+ OnceLock 和 OnceCell（OnceLock and OnceCell）
// ===========================================

// OnceLock 和 OnceCell 是 Rust 1.70 引入的重要类型
// 它们提供了线程安全的单次初始化机制，是延迟初始化和缓存场景的理想选择
// 这些类型填补了 Rust 标准库在延迟初始化方面的重要空白

// OnceLock 和 OnceCell 的核心价值：
// 1. 线程安全：OnceLock 是线程安全的，OnceCell 是单线程的
// 2. 延迟初始化：只在第一次访问时进行初始化
// 3. 高效缓存：避免重复计算，提高性能
// 4. 内存安全：保证初始化的原子性和不可变性
// 5. 简洁 API：提供易用的 get_or_init 方法

fn once_lock_and_once_cell() {
    println!("=== Rust 1.70+ OnceLock 和 OnceCell ===");

    // 1. OnceLock 基础用法（线程安全）
    // OnceLock 是线程安全的一次性初始化类型，适合并发场景
    use std::sync::OnceLock;

    // 创建 OnceLock 来存储配置信息
    static CONFIG: OnceLock<String> = OnceLock::new();

    // 第一次访问：初始化配置
    let config = CONFIG.get_or_init(|| {
        println!("正在初始化配置...");
        String::from("database_url=localhost\nport=5432")
    });
    println!("配置内容：{}", config);

    // 第二次访问：直接返回已初始化的值
    let config2 = CONFIG.get_or_init(|| {
        panic!("这段代码不会执行");
    });
    println!("再次获取配置：{}", config2);
    assert_eq!(config, config2);

    // OnceCell 的实际应用：缓存计算结果
    fn expensive_computation(input: u32) -> u32 {
        println!("执行昂贵的计算：{}", input);
        // 模拟耗时计算
        std::thread::sleep(std::time::Duration::from_millis(100));
        input * input
    }

    let cache = OnceCell::new();

    // 第一次：执行计算并缓存结果
    let result1 = cache.get_or_init(|| expensive_computation(42));
    println!("计算结果1：{}", result1);

    // 第二次：直接从缓存获取
    let result2 = cache.get_or_init(|| expensive_computation(42));
    println!("计算结果2：{}", result2);
    assert_eq!(result1, result2);

    // 2. OnceLock 基础用法（多线程安全）
    // OnceLock 是线程安全的，适合并发场景
    // use std::sync::OnceLock;  // 已经在上面导入

    static GLOBAL_CACHE: OnceLock<Vec<i32>> = OnceLock::new();

    // 并发访问测试
    use std::thread;
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                // 多个线程同时访问，但初始化只执行一次
                let data = GLOBAL_CACHE.get_or_init(|| {
                    println!("线程 {} 正在初始化全局缓存", i);
                    (1..=100).collect()
                });
                println!("线程 {} 获取到数据，长度：{}", i, data.len());
                data.len()
            })
        })
        .collect();

    let lengths: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("所有线程获取的数据长度：{:?}", lengths);
    assert!(lengths.iter().all(|&len| len == 100));

    // 3. OnceLock 在实际应用中的使用
    // 示例：数据库连接池的单例模式

    #[derive(Debug)]
    struct DatabaseConnection {
        url: String,
        connection_count: u32,
    }

    impl DatabaseConnection {
        fn new(url: &str) -> Self {
            println!("创建数据库连接：{}", url);
            DatabaseConnection {
                url: url.to_string(),
                connection_count: 0,
            }
        }

        fn execute_query(&mut self, query: &str) {
            self.connection_count += 1;
            println!(
                "执行查询：{} (总查询次数：{})",
                query, self.connection_count
            );
        }
    }

    static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

    // 获取数据库连接（单例）- 使用更安全的方式
    fn get_db_connection() -> &'static DatabaseConnection {
        DB_CONNECTION.get_or_init(|| DatabaseConnection::new("postgresql://localhost:5432/mydb"))
    }

    let conn1 = get_db_connection();
    println!("连接1: {}", conn1.url);

    let conn2 = get_db_connection();
    println!("连接2: {}", conn2.url);

    // 4. OnceCell 在配置管理中的应用
    #[derive(Debug)]
    struct AppConfig {
        api_key: String,
        timeout_seconds: u32,
        retry_count: u32,
    }

    impl AppConfig {
        fn from_env() -> Self {
            // 在实际应用中，这里会从环境变量读取
            AppConfig {
                api_key: "sk-1234567890abcdef".to_string(),
                timeout_seconds: 30,
                retry_count: 3,
            }
        }
    }

    static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

    fn get_config() -> &'static AppConfig {
        APP_CONFIG.get_or_init(|| {
            println!("从环境变量加载配置...");
            AppConfig::from_env()
        })
    }

    let config = get_config();
    println!("应用配置：{:?}", config);

    // 再次获取：直接返回缓存的配置
    let config2 = get_config();
    println!("配置重用：{:?}", config2);

    // 5. OnceLock 在缓存系统中的应用
    static FIBONACCI_CACHE: OnceLock<Vec<u64>> = OnceLock::new();

    fn get_fibonacci_sequence(n: usize) -> &'static [u64] {
        FIBONACCI_CACHE.get_or_init(|| {
            println!("生成斐波那契数列...");
            let mut fib = vec![0, 1];
            for i in 2..=n {
                fib.push(fib[i - 1] + fib[i - 2]);
            }
            fib
        })
    }

    let seq1 = get_fibonacci_sequence(20);
    println!("斐波那契数列前10项：{:?}", &seq1[..10]);

    let seq2 = get_fibonacci_sequence(20);
    println!("缓存重用，第15项：{}", seq2[14]);

    // 6. OnceLock 在懒加载中的应用
    use std::cell::OnceCell; // OnceCell 用于单线程懒加载

    struct LazyImage {
        width: u32,
        height: u32,
        pixels: OnceCell<Vec<u8>>,
    }

    impl LazyImage {
        fn new(width: u32, height: u32) -> Self {
            LazyImage {
                width,
                height,
                pixels: OnceCell::new(),
            }
        }

        fn get_pixels(&self) -> &[u8] {
            self.pixels.get_or_init(|| {
                println!("生成图像像素数据... {}x{}", self.width, self.height);
                vec![255u8; (self.width * self.height) as usize]
            })
        }

        fn get_pixel(&self, x: u32, y: u32) -> u8 {
            let pixels = self.get_pixels();
            let index = (y * self.width + x) as usize;
            pixels[index]
        }
    }

    let image = LazyImage::new(800, 600);
    println!("图像大小：{}x{}", image.width, image.height);

    // 第一次访问像素：生成数据
    let pixel = image.get_pixel(10, 20);
    println!("像素 (10, 20)：{}", pixel);

    // 第二次访问：使用缓存
    let pixel2 = image.get_pixel(30, 40);
    println!("像素 (30, 40)：{}", pixel2);

    // 7. 错误处理和 OnceLock/OnceCell
    use std::io;

    static FILE_CONTENT: OnceLock<Result<String, io::Error>> = OnceLock::new();

    fn read_file_cached(filename: &str) -> Result<&'static str, io::Error> {
        let content = FILE_CONTENT.get_or_init(|| {
            println!("读取文件：{}", filename);
            if filename == "existent.txt" {
                Ok(String::from("文件内容"))
            } else {
                Err(io::Error::new(io::ErrorKind::NotFound, "文件不存在"))
            }
        });

        match content {
            Ok(s) => Ok(s.as_str()),
            Err(e) => Err(io::Error::new(e.kind(), "文件读取失败")),
        }
    }

    match read_file_cached("existent.txt") {
        Ok(content) => println!("文件内容：{}", content),
        Err(e) => println!("错误：{}", e),
    }

    // OnceLock 和 OnceCell 的最佳实践：
    // 1. 选择合适的类型：并发场景用 OnceLock，单线程用 OnceCell
    // 2. 避免可变状态：尽量使用不可变数据，减少同步开销
    // 3. 合理初始化：初始化函数应该快速且无副作用
    // 4. 错误处理：考虑初始化可能失败的情况
    // 5. 内存管理：注意静态数据的生命周期和内存占用

    println!();
}

// ===========================================
// 14. 最新的const函数和泛型增强（Latest Const Functions and Generic Enhancements）
// ===========================================

// 随着Rust语言的不断发展，const函数和泛型系统在最新版本中获得了显著的增强
// 这些改进极大地扩展了编译时计算的能力，提高了代码的性能和类型安全性
// 本节汇总 const 函数与 const 泛型的进阶用法。涉及的关键版本节点（已对照官方发布说明核实）：
// - const 函数：1.46 控制流、1.57 panic!/assert!、1.61 泛型约束、1.82 浮点运算、1.83 `&mut`
//   （前面几节已展开），1.86 起 `str::split_at` 等字符串方法可在 const 中使用
// - const 泛型：1.51 稳定、1.59 参数默认值、1.89 允许用 `_` 让编译器推断 const 参数
// - 仍未稳定：const trait、对泛型参数做运算的常量表达式（generic_const_exprs）

// 最新const函数改进的核心价值：
// 1. 更强大的编译时计算：支持更复杂的逻辑和操作
// 2. 更好的类型安全：在编译时捕获更多错误
// 3. 性能优化：减少运行时开销，提高执行效率
// 4. 代码简化：减少样板代码，提高可读性
// 5. 泛型增强：更灵活的编译时编程能力

fn latest_const_and_generic_enhancements() {
    println!("=== 最新的const函数和泛型增强 ===");

    // 1. 高级const函数模式（Advanced Const Function Patterns）
    // 最新的Rust版本扩展了const函数的能力，支持更复杂的编译时计算

    // const函数中的字符串操作
    // [Rust 1.86] `str::split_at`、`split_at_checked`、`is_char_boundary` 等方法可以在 const 上下文中使用。
    // 在此之前，const 里对字符串基本只能做 `len()`、`as_bytes()` 这类只读查询；
    // 用下标切片 `&s[..n]` 到今天依然不能在 const 中使用（它走的是 `Index` trait）。
    // 注意：`split_at` 要求切分位置落在字符边界上，否则会 panic——
    // 在 const 求值时 panic，就意味着编译失败，这正是“编译期校验”想要的效果。
    const fn split_greeting() -> (&'static str, &'static str) {
        const INPUT: &str = "Hello, World!";
        INPUT.split_at(5)
    }

    const GREETING_PARTS: (&str, &str) = split_greeting();
    println!("编译时字符串切分：{:?}", GREETING_PARTS);

    // const函数中的数组操作
    const fn array_operations() -> [i32; 5] {
        let mut arr = [0; 5];

        // 使用循环在编译时初始化数组
        let mut i = 0;
        while i < 5 {
            arr[i] = (i * i) as i32; // 显式转换为 i32
            i += 1;
        }

        arr
    }

    const SQUARES: [i32; 5] = array_operations();
    println!("编译时数组初始化：{:?}", SQUARES);

    // const函数中的条件编译
    const fn conditional_compilation() -> u32 {
        #[cfg(debug_assertions)]
        {
            100 // 调试模式
        }
        #[cfg(not(debug_assertions))]
        {
            200 // 发布模式
        }
    }

    const CONDITIONAL_VALUE: u32 = conditional_compilation();
    println!("条件编译值：{}", CONDITIONAL_VALUE);

    // 2. const泛型增强（Const Generic Enhancements）
    // [Rust 1.89] 支持显式推断const参数 (Explicitly Inferred Const Arguments)
    // const泛型允许在类型级别使用编译时常量，提供更强大的抽象能力

    // [Rust 1.89] 显式推断 const 泛型实参：用 `_` 让编译器从上下文推断数组长度。
    // 以前必须把长度写死（或另外声明一个常量）；现在 `[0; _]` 的长度由左侧的类型标注 `[u8; 4]` 推断出来。
    let inferred: [u8; 4] = [0; _];
    println!("`[0; _]` 推断出的数组长度：{}", inferred.len());

    // 动态数组大小验证
    const fn validate_array_size<const N: usize>() -> bool {
        N > 0 && N <= 1000
    }

    struct ValidatedArray<T, const N: usize> {
        data: [T; N],
    }

    impl<T, const N: usize> ValidatedArray<T, N> {
        const fn new(data: [T; N]) -> Self {
            assert!(validate_array_size::<N>(), "数组大小必须在1到1000之间");
            Self { data }
        }
    }

    // 编译时验证数组大小
    let valid_array = ValidatedArray::new([1, 2, 3, 4, 5]);
    println!("验证后的数组：{:?}", valid_array.data);

    // const泛型与trait约束
    // 带 const 泛型参数的 trait 本身是稳定的；
    // 不稳定的只是下面被注释掉的 `{ ROWS * COLS }` ——对泛型参数做运算再当常量用，需要 nightly
    trait ConstMath<const N: usize> {
        const SIZE: usize = N;
        fn compute(&self) -> usize;
    }

    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    // 注意：这个实现需要 generic_const_exprs 特性，暂时注释掉
    /*
    impl<T: std::ops::Add<Output = T> + Copy + Default, const ROWS: usize, const COLS: usize>
    ConstMath<{ ROWS * COLS }> for Matrix<T, ROWS, COLS> {
        fn compute(&self) -> usize {
            ROWS * COLS
        }
    }
    */

    // 使用具体的实现来展示 const generics 的概念
    struct Matrix2x3<T> {
        data: [[T; 3]; 2],
    }

    impl<T: std::ops::Add<Output = T> + Copy + Default> ConstMath<6> for Matrix2x3<T> {
        fn compute(&self) -> usize {
            6 // 2 * 3 = 6
        }
    }

    let matrix = Matrix2x3 { data: [[0; 3]; 2] };
    println!("矩阵大小：{}", matrix.compute());

    // 3. 通过 trait 的关联常量参与编译期计算
    // 注意：真正的 `const trait`（让 trait 方法能在 const fn 里被调用）目前仍是不稳定特性（const_trait_impl）。
    // stable 上能做的是下面这种：trait 带一个关联常量，const fn 通过泛型参数读取它。

    // 带关联常量的普通 trait
    trait ConstToString {
        const DESCRIPTION: &'static str;
        fn const_to_string(&self) -> String;
    }

    #[derive(Debug)]
    struct Config {
        name: &'static str,
        version: u32,
    }

    impl ConstToString for Config {
        const DESCRIPTION: &'static str = "应用程序配置";

        fn const_to_string(&self) -> String {
            format!("{} v{}", self.name, self.version)
        }
    }

    const fn describe_config<T: ConstToString>() -> &'static str {
        T::DESCRIPTION
    }

    const CONFIG_DESC: &str = describe_config::<Config>();
    println!("配置描述：{}", CONFIG_DESC);

    // 4. 编译时集合操作（Compile-time Collection Operations）
    // 在const函数中进行更复杂的集合操作

    const fn find_max(arr: &[i32; 10]) -> i32 {
        let mut max = arr[0];
        let mut i = 1;

        while i < 10 {
            if arr[i] > max {
                max = arr[i];
            }
            i += 1;
        }

        max
    }

    const NUMBERS: [i32; 10] = [1, 5, 3, 9, 2, 8, 4, 7, 6, 0];
    const MAX_NUMBER: i32 = find_max(&NUMBERS);
    println!("编译时最大值：{}", MAX_NUMBER);

    // const函数中的搜索操作
    const fn contains_value(arr: &[i32; 5], target: i32) -> bool {
        let mut i = 0;

        while i < 5 {
            if arr[i] == target {
                return true;
            }
            i += 1;
        }

        false
    }

    const SEARCH_ARRAY: [i32; 5] = [10, 20, 30, 40, 50];
    const HAS_THIRTY: bool = contains_value(&SEARCH_ARRAY, 30);
    const HAS_SIXTY: bool = contains_value(&SEARCH_ARRAY, 60);
    println!("包含30：{}，包含60：{}", HAS_THIRTY, HAS_SIXTY);

    // 5. const函数与错误处理（Const Functions with Error Handling）
    // 在const函数中进行编译时错误处理

    #[derive(Debug, PartialEq)]
    enum ConstError {
        DivisionByZero,
        NegativeNumber,
        OutOfRange,
    }

    const fn safe_divide(a: i32, b: i32) -> Result<i32, ConstError> {
        if b == 0 {
            Err(ConstError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    const fn safe_sqrt(n: u32) -> Result<u32, ConstError> {
        // 简单的整数平方根实现
        let mut i = 0;

        while i * i <= n {
            if i * i == n {
                return Ok(i);
            }
            i += 1;
        }

        Err(ConstError::OutOfRange)
    }

    const DIV_RESULT: Result<i32, ConstError> = safe_divide(10, 2);
    const SQRT_RESULT: Result<u32, ConstError> = safe_sqrt(16);

    println!("除法结果：{:?}", DIV_RESULT);
    println!("平方根结果：{:?}", SQRT_RESULT);

    // 6. const函数与内存布局（Const Functions and Memory Layout）
    // 使用const函数分析类型和内存布局

    const fn type_size<T>() -> usize {
        std::mem::size_of::<T>()
    }

    const fn type_align<T>() -> usize {
        std::mem::align_of::<T>()
    }

    struct Point {
        x: f64,
        y: f64,
        z: f64,
    }

    const POINT_SIZE: usize = type_size::<Point>();
    const POINT_ALIGN: usize = type_align::<Point>();
    println!("Point大小：{}字节，对齐：{}字节", POINT_SIZE, POINT_ALIGN);

    // 7. const泛型与算法优化（Const Generics and Algorithm Optimization）
    // 使用const泛型进行编译时算法优化

    const fn is_power_of_two_generic<const N: usize>() -> bool {
        N > 0 && (N & (N - 1)) == 0
    }

    // 编译时选择最优算法（简化版本）
    const fn optimal_sort<const N: usize>(arr: &[i32; N]) -> [i32; N] {
        // 由于 sort() 不能在 const fn 中使用，这里返回原数组
        // 在实际应用中，可以实现编译时排序算法
        *arr
    }

    const SORT_INPUT: [i32; 8] = [5, 2, 8, 1, 9, 3, 7, 4];
    const SORTED_OUTPUT: [i32; 8] = optimal_sort(&SORT_INPUT);
    println!("编译时排序结果：{:?}", SORTED_OUTPUT);

    // 8. const函数与位操作（Const Functions and Bit Operations）
    // 在const函数中进行复杂的位操作

    const fn bit_count(n: u32) -> u32 {
        let mut count = 0;
        let mut remaining = n;

        while remaining != 0 {
            count += remaining & 1;
            remaining >>= 1;
        }

        count
    }

    const fn reverse_bits(n: u32) -> u32 {
        let mut result = 0;
        let mut input = n;
        let mut i = 0;

        // 使用 while 循环代替 for 循环
        while i < 32 {
            result = (result << 1) | (input & 1);
            input >>= 1;
            i += 1;
        }

        result
    }

    const NUMBER: u32 = 0b10101010_10101010_10101010_10101010;
    const BIT_COUNT: u32 = bit_count(NUMBER);
    const REVERSED: u32 = reverse_bits(NUMBER);

    println!("数字：{:032b}", NUMBER);
    println!("位计数：{}", BIT_COUNT);
    println!("位反转：{:032b}", REVERSED);

    // Rust 1.95: ControlFlow::is_break / is_continue 可在 const 中使用
    //
    // ControlFlow 是标准库内部常用的“提前返回”枚举，
    // 现在它的两个判断方法可以在常量求值里使用。
    use std::ops::ControlFlow;
    const IS_BREAK: bool = ControlFlow::<u32, u32>::Break(3).is_break();
    const IS_CONTINUE: bool = ControlFlow::<u32, u32>::Continue(1).is_continue();
    println!("const 求值: Break.is_break = {IS_BREAK}, Continue.is_continue = {IS_CONTINUE}");

    // 最新const函数和泛型的最佳实践：
    // 1. 合理使用：只在真正需要编译时计算时使用const函数
    // 2. 性能考虑：复杂的const函数可能增加编译时间
    // 3. 类型安全：利用const泛型提供更强的类型约束
    // 4. 可读性：保持const函数的逻辑清晰易懂
    // 5. 测试覆盖：为const函数编写充分的测试用例
    // 6. 文档化：详细说明const函数的使用场景和限制

    // 最新const函数和泛型的应用场景：
    // 1. 性能优化：将运行时计算移到编译时
    // 2. 类型安全：在编译时验证数据的有效性
    // 3. 资源管理：编译时计算资源需求和分配
    // 4. 算法优化：根据编译时常量选择最优算法
    // 5. 配置验证：在编译时验证配置参数的有效性
    // 6. 元编程：使用const泛型实现更灵活的抽象

    println!();
}

// ===========================================
// 15. 高级示例程序（Advanced Example Programs）
// ===========================================

// 高级示例程序展示了 Rust 高级特性的实际应用
// 这些示例涵盖了内存管理、类型安全、性能优化等多个方面
// 通过这些实际案例，可以更好地理解 Rust 高级特性的价值和使用场景

// 高级示例程序的核心价值：
// 1. 实际应用：展示特性在真实场景中的应用
// 2. 最佳实践：展示如何正确和安全地使用高级特性
// 3. 性能优化：展示如何使用 Rust 进行性能优化
// 4. 设计模式：展示 Rust 中的设计模式实现
// 5. 综合运用：展示多种特性的协同使用

fn advanced_example_program() {
    println!("=== 高级示例程序 ===");

    // 示例 1：内存池管理器（Memory Pool Manager）
    // 内存池是一种高效的内存管理技术，通过预分配大块内存来减少分配开销
    // 这个示例展示了 unsafe Rust 在内存管理中的应用
    struct MemoryPool {
        pool: Vec<u8>, // 内存池底层存储
        used: usize,   // 已使用的内存量
    }

    impl MemoryPool {
        // 创建指定大小的内存池
        fn new(size: usize) -> Self {
            MemoryPool {
                pool: vec![0; size], // 预分配内存
                used: 0,
            }
        }

        // 分配指定大小的内存块
        // 使用 unsafe 因为涉及裸指针操作
        unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
            // 检查是否有足够的内存
            if self.used + size > self.pool.len() {
                return std::ptr::null_mut(); // 内存不足，返回空指针
            }

            // 计算分配位置的指针
            let ptr = unsafe { self.pool.as_mut_ptr().add(self.used) };
            self.used += size;
            ptr
        }

        // 释放内存块（简化版本）
        // 实际的内存池需要更复杂的内存管理策略
        unsafe fn deallocate(&mut self, _ptr: *mut u8, _size: usize) {
            // 简化的内存释放逻辑
            // 实际实现需要：
            // 1. 跟踪空闲块
            // 2. 内存碎片整理
            // 3. 合并相邻的空闲块
        }

        // 获取内存池统计信息
        fn stats(&self) -> (usize, usize) {
            (self.used, self.pool.len())
        }
    }

    // 使用内存池
    let mut pool = MemoryPool::new(1024);

    unsafe {
        // 分配两个内存块
        let ptr1 = pool.allocate(100);
        let ptr2 = pool.allocate(200);

        println!("分配的指针1: {:p}", ptr1);
        println!("分配的指针2: {:p}", ptr2);

        // 使用分配的内存
        if !ptr1.is_null() {
            *ptr1 = 42;
            println!("通过指针1写入的值: {}", *ptr1);
        }

        // 显示内存池使用情况
        let (used, total) = pool.stats();
        println!("内存池使用情况: {} / {} 字节", used, total);
    }

    // 内存池的优势：
    // 1. 性能：减少频繁的内存分配和释放
    // 2. 碎片化：减少内存碎片
    // 3. 可预测性：内存使用模式更可预测
    // 4. 局部性：提高数据局部性

    // 示例 2：类型安全的矩阵运算（Type-Safe Matrix Operations）
    // 这个示例展示了 Rust 泛型系统在数学运算中的应用
    // 通过泛型和 trait，我们可以实现类型安全的矩阵运算
    #[derive(Debug, Clone)]
    struct Matrix<T> {
        data: Vec<T>, // 存储矩阵数据
        rows: usize,  // 矩阵行数
        cols: usize,  // 矩阵列数
    }

    // 为所有类型 T 实现基本矩阵操作
    impl<T> Matrix<T> {
        // 创建新矩阵，用默认值填充
        fn new(rows: usize, cols: usize, default: T) -> Self
        where
            T: Clone,
        {
            Matrix {
                data: vec![default; rows * cols],
                rows,
                cols,
            }
        }

        // 安全地获取矩阵元素
        fn get(&self, row: usize, col: usize) -> &T {
            &self.data[row * self.cols + col]
        }

        // 安全地获取矩阵元素的可变引用
        fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
            &mut self.data[row * self.cols + col]
        }
    }

    // 为 f64 类型实现特定的矩阵运算
    impl Matrix<f64> {
        // 矩阵加法
        fn add(&self, other: &Matrix<f64>) -> Result<Matrix<f64>, String> {
            // 检查维度是否匹配
            if self.rows != other.rows || self.cols != other.cols {
                return Err("矩阵维度不匹配".to_string());
            }

            let mut result = Matrix::new(self.rows, self.cols, 0.0);

            // 执行矩阵加法
            for i in 0..self.rows {
                for j in 0..self.cols {
                    let sum = *self.get(i, j) + *other.get(i, j);
                    *result.get_mut(i, j) = sum;
                }
            }

            Ok(result)
        }

        // 矩阵乘法
        fn multiply(&self, other: &Matrix<f64>) -> Result<Matrix<f64>, String> {
            // 检查乘法条件：第一个矩阵的列数等于第二个矩阵的行数
            if self.cols != other.rows {
                return Err("矩阵维度不匹配，无法相乘".to_string());
            }

            let mut result = Matrix::new(self.rows, other.cols, 0.0);

            // 执行矩阵乘法
            for i in 0..self.rows {
                for j in 0..other.cols {
                    let mut sum = 0.0;
                    for k in 0..self.cols {
                        sum += *self.get(i, k) * *other.get(k, j);
                    }
                    *result.get_mut(i, j) = sum;
                }
            }

            Ok(result)
        }
    }

    // 测试矩阵运算
    let m1 = Matrix::new(2, 2, 1.0);
    let m2 = Matrix::new(2, 2, 2.0);

    match m1.add(&m2) {
        Ok(sum) => println!("矩阵加法结果: {:?}", sum),
        Err(e) => println!("矩阵加法错误: {}", e),
    }

    // 创建特定值的矩阵
    let m3 = Matrix {
        data: vec![1.0, 2.0, 3.0, 4.0],
        rows: 2,
        cols: 2,
    };

    let m4 = Matrix {
        data: vec![5.0, 6.0, 7.0, 8.0],
        rows: 2,
        cols: 2,
    };

    match m3.multiply(&m4) {
        Ok(product) => println!("矩阵乘法结果: {:?}", product),
        Err(e) => println!("矩阵乘法错误: {}", e),
    }

    // 矩阵运算的特点：
    // 1. 类型安全：编译时保证类型正确性
    // 2. 泛型支持：支持多种数据类型
    // 3. 错误处理：明确的错误类型和处理机制
    // 4. 内存效率：零成本的抽象

    // 示例 3：高性能计算示例（High-Performance Computing）
    // 这个示例比较了安全实现和 unsafe 优化的性能差异
    // 展示了如何在保证安全性的同时进行性能优化
    fn compute_fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    // 使用 unsafe 进行优化
    // 通过指针操作减少临时变量的创建
    unsafe fn compute_fibonacci_unsafe(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let mut a = 0u64; // 明确指定为 u64 类型
        let mut b = 1u64; // 明确指定为 u64 类型
        let mut i = 2;

        // 使用指针算术进行优化
        let p_a = &mut a as *mut u64;
        let p_b = &mut b as *mut u64;

        while i <= n {
            unsafe {
                *p_a = *p_a + *p_b;
                // 手动交换指针指向的值
                let temp = *p_a;
                *p_a = *p_b;
                *p_b = temp;
            }
            i += 1;
        }

        b
    }

    // 比较两种实现的性能
    let n = 10;
    let result1 = compute_fibonacci(n);
    let result2 = unsafe { compute_fibonacci_unsafe(n) };

    println!("斐波那契数列第{}项: {}", n, result1);
    println!("unsafe 斐波那契数列第{}项: {}", n, result2);

    // 性能优化的注意事项：
    // 1. 安全性优先：只在必要时使用 unsafe
    // 2. 性能测试：验证优化确实提升了性能
    // 3. 可读性：确保代码仍然易于理解和维护
    // 4. 平台考虑：优化可能在不同平台上有不同效果

    println!();
}

// ===========================================
// 16. cfg(accessible) 配置谓词（RFC 2523，尚未实现）
// ===========================================

// 注意：截至 Rust 1.99，`cfg(accessible)` 没有稳定，而且 RFC 提出的写法编译器根本还没有实现：
// - RFC 2523 设想的写法是 `#[cfg(accessible(::path::to::thing))]`，
//   可以和 `not` / `all` / `any` 组合——这个谓词写法目前不被支持
// - 标准库里只有一个内置属性宏 `#[cfg_accessible(path)]`（追踪 issue #64797），
//   源码里明确标注为 "not fully implemented"，需要 nightly 并开启 `#![feature(cfg_accessible)]`，
//   而且只有“路径可访问就保留这一项”的单向语义，写不出 `not(...)`
// 所以：stable 上不可用，nightly 上也只是不完整的雏形。
//
// 它想解决的问题是：根据“某个路径是否存在/可访问”来做条件编译，
// 而不是只能按平台、Cargo feature、编译器版本来判断。
// 这里只介绍设想中的用法，帮助读者了解 Rust 条件编译的演进方向；
// 下面标注“设想语法”的代码全部以注释形式保留，当前任何工具链都无法编译它们。
//
// 今天在 stable 上要达到类似目的，常见做法是：
// 1. Cargo feature：由使用者显式打开
// 2. build.rs 探测：用 `autocfg` 之类的 crate 试编译一小段代码，
//    成功就输出 `cargo::rustc-cfg=某个名字`，源码里再用 `#[cfg(某个名字)]` 判断

fn cfg_accessible_predicate() {
    println!("=== cfg(accessible) 配置谓词（RFC 2523，尚未实现） ===");

    // #[cfg(accessible)] 的背景和意义：
    // 1. 精细控制：基于路径的可访问性进行条件编译
    // 2. 特性检测：检测特定类型或模块是否可用
    // 3. 兼容性处理：根据依赖项的存在调整代码
    // 4. 性能优化：有条件地包含昂贵的依赖
    // 5. 平台适配：根据特定平台功能调整实现

    // 基本语法：#[cfg_accessible(path)]
    // path 可以是：模块路径、类型路径、函数路径、trait 路径

    // 示例 1: 检测标准库类型是否可访问
    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(accessible(std::collections::HashMap))]
    {
        println!("HashMap 类型可用，使用标准库实现");
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("key", "value");
        println!("HashMap 创建成功: {:?}", map);
    }

    #[cfg(not(accessible(std::collections::HashMap)))]
    {
        println!("HashMap 类型不可用，使用备用实现");
        // 可以使用其他实现或自己实现
    }
    */

    // 示例 2: 检测外部 crate 的可用性
    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(accessible(serde::Serialize))]
    {
        println!("serde::Serialize trait 可用");
        // 可以安全地使用 serde 序列化功能
    }

    #[cfg(not(accessible(serde::Serialize)))]
    {
        println!("serde::Serialize trait 不可用");
        // 使用备用序列化方案
    }
    */

    // 示例 3: 条件性 trait 实现
    trait DataProcessor {
        fn process(&self) -> String;
    }

    // 只有当特定 trait 可用时才实现
    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(accessible(std::io::Read))]
    impl DataProcessor for String {
        fn process(&self) -> String {
            format!("处理字符串: {}", self)
        }
    }

    // 备用实现
    #[cfg(not(accessible(std::io::Read)))]
    impl DataProcessor for String {
        fn process(&self) -> String {
            format!("基本字符串处理: {}", self)
        }
    }
    */

    // 提供一个默认实现
    impl DataProcessor for String {
        fn process(&self) -> String {
            format!("基本字符串处理: {}", self)
        }
    }

    let data = "测试数据".to_string();
    println!("处理结果: {}", data.process());

    // 示例 4: 模块级别的条件包含
    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(accessible(tokio::runtime::Runtime))]
    mod async_features {
        use tokio::runtime::Runtime;

        pub fn async_operation() {
            println!("Tokio 运行时可用，执行异步操作");
            // 在实际应用中这里会有真正的异步代码
        }
    }

    #[cfg(not(accessible(tokio::runtime::Runtime)))]
    mod async_features {
        pub fn async_operation() {
            println!("Tokio 运行时不可用，使用同步替代方案");
        }
    }

    async_features::async_operation();
    */

    // 提供一个默认的同步实现
    mod async_features {
        pub fn async_operation() {
            println!("Tokio 运行时不可用，使用同步替代方案");
        }
    }

    async_features::async_operation();

    // 示例 5: 基于多个条件的复杂配置
    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(all(
        accessible(std::fs::File),
        accessible(std::io::BufReader),
        target_os = "linux"
    ))]
    {
        println!("Linux 平台上完整的文件 I/O 支持可用");
        // 使用 Linux 特定的文件操作优化
    }

    #[cfg(any(
        not(accessible(std::fs::File)),
        not(target_os = "linux")
    ))]
    {
        println!("使用跨平台文件操作方案");
        // 使用通用的跨平台实现
    }
    */

    // 提供一个默认实现
    #[cfg(target_os = "linux")]
    {
        println!("Linux 平台，使用标准文件 I/O");
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("非 Linux 平台，使用跨平台文件操作方案");
    }

    // 示例 6: 条件性导入
    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(accessible(num_bigint::BigInt))]
    {
        use num_bigint::BigInt;
        println!("BigInt 类型可用，支持大整数运算");
    }

    #[cfg(not(accessible(num_bigint::BigInt)))]
    {
        println!("BigInt 类型不可用，使用 i64 替代");
        type BigInt = i64;
    }
    */

    // 提供一个默认实现
    {
        println!("使用标准库整数（num_bigint 不可用）");
        type BigInt = i64;
    }

    // 示例 7: 检测内部模块的可访问性
    mod internal_module {
        pub struct InternalType;

        impl InternalType {
            pub fn new() -> Self {
                InternalType
            }
        }
    }

    // 设想语法（RFC 2523）：编译器尚未实现，所以以注释形式保留
    /*
    #[cfg(accessible(internal_module::InternalType))]
    {
        let _internal = internal_module::InternalType::new();
        println!("内部模块类型可访问");
    }
    */

    // 提供一个默认实现
    {
        let _internal = internal_module::InternalType::new();
        println!("内部模块类型可访问（默认实现）");
    }

    // practical_cfg_accessible_examples(); // 注释掉：它使用的是尚未实现的设想语法

    println!("cfg(accessible) 配置谓词演示完成");
    println!();
}

// 设想语法（RFC 2523）：编译器尚未实现，整个函数以注释形式保留
/*
fn practical_cfg_accessible_examples() {
    println!("=== #[cfg(accessible)] 实际应用示例 ===");

    // 场景 1: 数据库后端的条件编译
    mod database {
        // 根据可用的数据库驱动选择实现
        #[cfg(accessible(sqlx::Postgres))]
        pub mod postgres {
            pub fn connect() -> String {
                "PostgreSQL 连接已建立".to_string()
            }
        }

        #[cfg(accessible(diesel::prelude::*))]
        pub mod diesel_mysql {
            pub fn connect() -> String {
                "MySQL (Diesel) 连接已建立".to_string()
            }
        }

        #[cfg(not(any(
            accessible(sqlx::Postgres),
            accessible(diesel::prelude::*)
        )))]
        pub mod sqlite {
            pub fn connect() -> String {
                "SQLite 连接已建立（备用方案）".to_string()
            }
        }

        pub fn get_connection() -> String {
            #[cfg(accessible(sqlx::Postgres))]
            return postgres::connect();

            #[cfg(accessible(diesel::prelude::*))]
            return diesel_mysql::connect();

            #[cfg(not(any(
                accessible(sqlx::Postgres),
                accessible(diesel::prelude::*)
            )))]
            return sqlite::connect();
        }
    }

    println!("数据库连接: {}", database::get_connection());

    // 场景 2: 图形处理库的条件选择
    mod graphics {
        #[cfg(accessible(image::ImageBuffer))]
        pub mod image_processing {
            use image::ImageBuffer;

            pub fn process_image() -> String {
                "使用 image crate 处理图像".to_string()
            }
        }

        #[cfg(accessible(opencv::core::Mat))]
        pub mod opencv_processing {
            pub fn process_image() -> String {
                "使用 OpenCV 处理图像".to_string()
            }
        }

        #[cfg(not(any(
            accessible(image::ImageBuffer),
            accessible(opencv::core::Mat)
        )))]
        pub mod basic_processing {
            pub fn process_image() -> String {
                "使用基本图像处理算法".to_string()
            }
        }

        pub fn available_backends() -> Vec<String> {
            let mut backends = Vec::new();

            #[cfg(accessible(image::ImageBuffer))]
            backends.push("Image".to_string());

            #[cfg(accessible(opencv::core::Mat))]
            backends.push("OpenCV".to_string());

            #[cfg(not(any(
                accessible(image::ImageBuffer),
                accessible(opencv::core::Mat)
            )))]
            backends.push("Basic".to_string());

            backends
        }
    }

    println!("可用的图形处理后端: {:?}", graphics::available_backends());

    // 场景 3: 网络库的条件编译
    mod networking {
        #[cfg(accessible(reqwest::Client))]
        pub mod http_client {
            use reqwest::Client;

            pub async fn make_request() -> String {
                "使用 reqwest 发起 HTTP 请求".to_string()
            }
        }

        #[cfg(accessible(curl::easy::Easy))]
        pub mod curl_client {
            pub fn make_request() -> String {
                "使用 libcurl 发起 HTTP 请求".to_string()
            }
        }

        #[cfg(not(any(
            accessible(reqwest::Client),
            accessible(curl::easy::Easy)
        )))]
        pub mod basic_client {
            pub fn make_request() -> String {
                "使用标准库 HTTP 客户端".to_string()
            }
        }

        pub fn best_available_client() -> &'static str {
            #[cfg(accessible(reqwest::Client))]
            return "reqwest (异步 HTTP 客户端)";

            #[cfg(accessible(curl::easy::Easy))]
            return "libcurl (同步 HTTP 客户端)";

            #[cfg(not(any(
                accessible(reqwest::Client),
                accessible(curl::easy::Easy)
            )))]
            return "标准库 HTTP 客户端";
        }
    }

    println!("最佳可用 HTTP 客户端: {}", networking::best_available_client());

    // 场景 4: 序列化框架的条件选择
    mod serialization {
        #[cfg(accessible(serde_json::to_string))]
        pub mod json_serializer {
            use serde_json;

            pub fn serialize(data: &str) -> String {
                format!("JSON (serde): {}", data)
            }
        }

        #[cfg(accessible(quick_xml::se::to_string))]
        pub mod xml_serializer {
            use quick_xml::se;

            pub fn serialize(data: &str) -> String {
                format!("XML (quick-xml): {}", data)
            }
        }

        #[cfg(not(any(
            accessible(serde_json::to_string),
            accessible(quick_xml::se::to_string)
        )))]
        pub mod basic_serializer {
            pub fn serialize(data: &str) -> String {
                format!("Basic: {}", data)
            }
        }

        pub fn supported_formats() -> Vec<&'static str> {
            let mut formats = Vec::new();

            #[cfg(accessible(serde_json::to_string))]
            formats.push("JSON");

            #[cfg(accessible(quick_xml::se::to_string))]
            formats.push("XML");

            #[cfg(not(any(
                accessible(serde_json::to_string),
                accessible(quick_xml::se::to_string)
            )))]
            formats.push("Basic");

            formats
        }
    }

    println!("支持的序列化格式: {:?}", serialization::supported_formats());

    // 场景 5: 条件性特性实现
    trait Logger {
        fn log(&self, message: &str);
    }

    #[cfg(accessible(log::Logger))]
    struct StandardLogger;

    #[cfg(accessible(log::Logger))]
    impl Logger for StandardLogger {
        fn log(&self, message: &str) {
            println!("[Standard Logger] {}", message);
        }
    }

    #[cfg(not(accessible(log::Logger)))]
    struct BasicLogger;

    #[cfg(not(accessible(log::Logger)))]
    impl Logger for BasicLogger {
        fn log(&self, message: &str) {
            println!("[Basic Logger] {}", message);
        }
    }

    #[cfg(accessible(log::Logger))]
    let logger: Box<dyn Logger> = Box::new(StandardLogger);

    #[cfg(not(accessible(log::Logger)))]
    let logger: Box<dyn Logger> = Box::new(BasicLogger);

    logger.log("测试日志消息");

    println!("实际应用示例演示完成");
}
*/

// ===========================================
// 17. Rust 1.83: Const 上下文中的可变引用 (Const Context Mutable References)
// ===========================================

// Rust 1.83 稳定了在 const 上下文（const fn / const 块）中支持 `&mut`、`*mut`、
// `&Cell` 和 `*const Cell`，这极大扩展了编译时计算的能力。
// 在 1.83 之前，const fn 里只能通过传值反复运算；现在可以在 const fn 内部
// 使用可变引用原地修改数据，写起来更接近普通代码。

pub fn const_mut_refs() {
    println!("=== Rust 1.83: Const 上下文中的可变引用 ===");

    // 编译时交换值
    const fn const_swap(arr: &mut [i32], x: usize, y: usize) {
        let temp = arr[x];
        arr[x] = arr[y];
        arr[y] = temp;
    }

    // 编译时数据处理
    const fn process_data() -> [i32; 3] {
        let mut data = [1, 2, 3];
        let mut i = 0;
        while i < 3 {
            data[i] *= 2;
            i += 1;
        }
        const_swap(&mut data, 0, 2);
        data
    }

    const PROCESSED: [i32; 3] = process_data();
    println!("编译时处理结果: {:?}", PROCESSED);

    // 这一特性的意义：
    // 1. 允许更复杂的编译时算法
    // 2. 减少运行时开销
    // 3. 提高 const fn 的实用性

    println!();
}

// ===========================================
// 18. LazyCell / LazyLock 的访问 API（Rust 1.94）
// ===========================================

// `LazyCell`（单线程）和 `LazyLock`（线程安全）都表示“懒初始化”：第一次访问时才运行初始化闭包。
// 1.94 新增的 `get` / `get_mut` / `force_mut`，让你可以在不触发初始化的前提下观察状态，
// 或者强制初始化之后直接拿到可变引用。

fn lazy_access_apis() {
    println!("=== LazyCell / LazyLock 的访问 API（Rust 1.94） ===");

    // Rust 1.94: LazyCell / LazyLock 的访问 API 更完整了
    //
    // 这两个类型都表示“懒初始化”：
    // - LazyCell: 单线程版本
    // - LazyLock: 线程安全版本
    //
    // 新增的 `get` / `get_mut` / `force_mut` 让我们可以更精细地观察和控制初始化状态。
    use std::cell::LazyCell;
    use std::sync::LazyLock;

    // `get` 不会触发初始化，只是问一句：“里面已经有值了吗？”
    // 由于我们还没真正访问过闭包，所以这里得到 None。
    let mut local_cache = LazyCell::new(|| vec![1, 2, 3]);
    println!("LazyCell 初始化前 get: {:?}", LazyCell::get(&local_cache));
    // `force_mut` 会在必要时先完成初始化，然后直接返回可变引用。
    // 这很适合“第一次创建对象后，还要立刻继续补内容”的写法。
    LazyCell::force_mut(&mut local_cache).push(4);
    println!("LazyCell 初始化后 get: {:?}", LazyCell::get(&local_cache));

    // LazyLock 是多线程版本，常见于全局配置、全局缓存和共享字典。
    // 这里同样先观察初始化状态，再强制初始化并继续修改结果。
    let mut banner = LazyLock::new(|| String::from("rust"));
    println!("LazyLock 初始化前 get: {:?}", LazyLock::get(&banner));
    LazyLock::force_mut(&mut banner).push_str("-1.94");
    println!("LazyLock 初始化后内容: {}", LazyLock::force(&banner));

    // 小结：
    // 1. `get` 只观察，不触发初始化；`force_mut` 会先初始化再给出可变引用
    // 2. 单线程用 LazyCell，需要跨线程共享用 LazyLock
    println!();
}

// ===========================================
// 19. Rust 2024 Edition：与 2021 的主要区别（Rust 1.85）
// ===========================================

// Rust 2024 是目前最大的一个 edition，随 Rust 1.85 稳定（2025-02-20 发布）。
// 本项目的 Cargo.toml 里写的就是 `edition = "2024"`，所以这一节的演示都是直接运行的。
// 迁移已有项目：先运行 `cargo fix --edition` 让工具自动改写，再把 Cargo.toml 里的 edition 改成 "2024"。
//
// 官方 Edition Guide 把变化按语言、标准库、Cargo 等分组。这里按“最常遇到”的顺序梳理：
//
// 语言
// 1. `impl Trait` 返回类型会自动捕获作用域内所有的泛型参数和生命周期；
//    想少捕获，用 `use<..>`（见第 8 章 traits）
// 2. `if let` 的临时值在进入 `else` 之前就销毁
// 3. 块末尾表达式里的临时值，先于块内的局部变量销毁
// 4. if let 链：`if let .. && let ..`（见第 5 章模式匹配）
// 5. `extern` 块必须写成 `unsafe extern`；`no_mangle`、`export_name`、`link_section` 必须写成 `#[unsafe(..)]`
// 6. `unsafe fn` 的函数体里，不安全的操作也必须显式写 `unsafe {}`（`unsafe_op_in_unsafe_fn` 默认 warn）
// 7. 禁止对 `static mut` 取引用（`static_mut_refs` 默认 deny），请改用 `&raw const` / `&raw mut`
// 8. 从 `!`（never 类型）回退的类型是 `!`，而不是 `()`
// 9. `gen` 成为保留字（给将来的生成器块预留）；已有的叫 `gen` 的标识符要写成 `r#gen`
// 10. 宏的 `expr` 片段说明符现在也匹配 `const { .. }` 和 `_`；想保持旧行为用 `expr_2021`
// 11. 模式里，在“默认绑定模式”下不能再写 `mut`、`ref`、`&`（match ergonomics 的保留规则）
//
// 标准库
// 12. `Future` 和 `IntoFuture` 进入 prelude
// 13. `Box<[T]>` 的 `.into_iter()` 变成按值迭代
// 14. `std::env::set_var` / `remove_var` 变成 unsafe 函数：多线程程序里修改环境变量并不安全
//
// Cargo
// 15. 依赖解析器默认使用 v3，会考虑各依赖声明的 `rust-version`
//
// 同样重要的是：edition 只负责“必须靠不兼容改动才能实现的变化”，
// 新的库函数、新的语法（let-else、async 闭包……）和 edition 无关，所有 edition 都能用。

use std::cell::RefCell;
use std::sync::RwLock;

// 2：`if let` 的临时值作用域
// 读锁是 `value.read().unwrap()` 产生的临时值。
// - 2021：它一直活到整个 `if let ... else ...` 结束，else 里再申请写锁就会死锁
// - 2024：进入 else 之前它已经被销毁，下面的写法才能正常工作
fn init_if_empty(value: &RwLock<Option<bool>>) -> bool {
    if let Some(existing) = *value.read().unwrap() {
        existing
    } else {
        let mut guard = value.write().unwrap();
        *guard = Some(true);
        true
    }
}

// 3：块末尾表达式的临时值作用域
// `cell.borrow()` 产生一个临时的 `Ref`，它借用了局部变量 `cell`。
// - 2021：这个临时值比 `cell` 活得更久，编译器报 E0597（`cell` 活得不够长）
// - 2024：临时值先于局部变量销毁，直接编译通过
fn tail_expression_len() -> usize {
    let cell = RefCell::new("2024 edition");
    cell.borrow().len()
}

// 1：返回的迭代器借用了参数 `numbers`。
// 2021 里必须额外写成 `impl Iterator<Item = i32> + '_`，否则报错说隐藏类型捕获了没有写出来的生命周期；
// 2024 里自动捕获，不必再写。
fn evens(numbers: &[i32]) -> impl Iterator<Item = i32> {
    numbers.iter().copied().filter(|n| n % 2 == 0)
}

// 9：`gen` 是保留字，当变量名要写成原始标识符 `r#gen`；
// 当生命周期名要写成原始生命周期 `'r#gen`（Rust 1.83 起可以这样写）。
// 原始生命周期的设计初衷，就是让旧代码迁移到引入新关键字的 edition 时，
// 仍然可以保留原来的名字（迁移工具会自动改写）。
fn pick_first<'r#gen>(first: &'r#gen str, _second: &str) -> &'r#gen str {
    first
}

fn rust_2024_edition_features() {
    println!("=== Rust 2024 Edition：与 2021 的主要区别（Rust 1.85） ===");

    // ---------- if let 的临时值先于 else 销毁 ----------
    let config = RwLock::new(None);
    println!("第一次调用（空，走 else 分支，写入默认值）: {}", init_if_empty(&config));
    println!("第二次调用（已有值，走 if 分支）: {}", init_if_empty(&config));

    // ---------- 块末尾表达式的临时值先于局部变量销毁 ----------
    println!("块末尾表达式里的 Ref 先于 cell 销毁，长度 = {}", tail_expression_len());

    // ---------- impl Trait 自动捕获 ----------
    let data = [1, 2, 3, 4, 5, 6];
    println!("evens 返回的迭代器借用了 data，不需要 + '_: {:?}", evens(&data).collect::<Vec<_>>());

    // ---------- gen 保留字与原始标识符 / 原始生命周期 ----------
    let r#gen = 7;
    println!("原始标识符 r#gen = {}，原始生命周期: {}", r#gen, pick_first("第一个", "第二个"));

    // ---------- Box<[T]> 的 into_iter 按值迭代 ----------
    // 1.80 起 Box<[T]> 实现了 IntoIterator（所有 edition）；
    // 但在 2024 之前，方法调用 `.into_iter()` 仍然会自动解引用成 `&[T]`，得到的是引用。
    // 2024 起，`.into_iter()` 才真正调用 Box<[T]> 自己的实现，按值迭代。
    let boxed: Box<[String]> = vec!["甲".to_string(), "乙".to_string()].into_boxed_slice();
    let owned: Vec<String> = boxed.into_iter().collect(); // 元素类型是 String，不是 &String
    println!("按值迭代得到的所有权: {owned:?}");

    // ---------- Future 进入 prelude ----------
    // 不再需要 `use std::future::Future;` 就可以直接写 `Future` 约束
    async fn answer() -> u8 {
        42
    }
    fn run<F: Future<Output = u8>>(future: F) -> u8 {
        futures::executor::block_on(future)
    }
    println!("不用 use 就能写 Future 约束: {}", run(answer()));

    // ---------- 只能用注释说明的变化 ----------
    // 5. 不安全的 extern 块 / 属性：本项目里的 FFI 声明都是这种写法，例如
    //        unsafe extern "C" { fn strlen(s: *const c_char) -> usize; }
    //        #[unsafe(no_mangle)] pub extern "C" fn exported() {}
    // 6. 不安全的操作要显式写 unsafe 块，哪怕已经在 unsafe fn 里：
    //        unsafe fn read(ptr: *const u8) -> u8 { unsafe { *ptr } }
    // 7. static mut：
    //        static mut COUNTER: u32 = 0;
    //        let r = unsafe { &COUNTER };          // 2024 里是编译错误（static_mut_refs）
    //        let p = unsafe { &raw const COUNTER }; // 正确：直接取裸指针，不制造引用
    // 8. never 类型回退：以前一个类型推断不出来的位置遇到 `!` 时，会回退成 `()`；2024 起回退成 `!`。
    //    依赖旧行为的代码需要写明类型。1.92 起，检测这类风险的两个 lint
    //    `never_type_fallback_flowing_into_unsafe` 和 `dependency_on_unit_never_type_fallback`
    //    在所有 edition 里都默认 deny，所以在升级 edition 之前就会收到提示。
    // 14. env::set_var：
    //        unsafe { std::env::set_var("KEY", "value") }  // 2024 里必须写 unsafe
    //     原因是在 Unix 上，其他线程可能同时在读取环境变量（包括 C 库里的代码），修改它会造成数据竞争。
    //     最佳做法：在程序启动、还没有创建任何线程之前设置好，或者干脆通过参数把配置传下去，而不是改全局环境。

    // 最佳实践：
    // 1. 新项目直接用最新的 edition；旧项目按 `cargo fix --edition` 迁移，一次迁移一个 edition
    // 2. 迁移后先看编译警告，再跑测试，重点留意 drop 顺序和锁的持有时间——这两类变化是静默发生的
    println!();
}

// ===========================================
// 20. use 导入路径关键字并重命名（Rust 1.95）
// ===========================================

// `crate`、`super`、`self` 是路径里的“关键字段”，分别代表当前 crate 的根、上一级模块、当前模块。
// 以前只能写 `use crate as name;`，另外两个不能这样用。
// 1.95 起三者统一了：都可以用 `as` 导入并改名；反过来，不改名的 `use $crate::{self};` 这类写法被禁止了。
//     use crate as app;      // 当前 crate 的根；也可以写 use crate::{self as app};
//     use super as parent;   // 上一级模块；链式的 `super::super` 同样可以：use super::super as grand;
//     use self as current;   // 当前模块自己
//
// 典型场景：
// - 测试模块里，用 `use super as subject;`（被测模块）代替 `use super::*;`，调用时写 `subject::parse(..)`，
//   一眼能看出哪些调用是被测代码，不会因为通配导入让名字来源变模糊
// - 层级很深的模块里，用 `use crate as app;` 代替满屏的 `crate::`
// - 宏生成的代码里，用 `use $crate as name;` 给宏所在的 crate 取个固定的名字

mod path_keyword_renaming_demo {
    pub fn base_value() -> u32 {
        10
    }

    pub mod nested {
        use crate as app;
        use self as current;
        use super as parent;

        pub fn offset() -> u32 {
            5
        }

        // 三个来源一目了然：app 是整个 crate，parent 是上一级模块，current 是本模块
        pub fn total() -> u32 {
            parent::base_value() + current::offset() + app::mod_14_advanced::path_keyword_marker()
        }
    }
}

// 放在 crate 根下的 mod_14_advanced 模块里，供上面的 `app::mod_14_advanced::...` 引用
fn path_keyword_marker() -> u32 {
    100
}

fn use_path_keyword_renaming() {
    println!("=== use 导入路径关键字并重命名（Rust 1.95） ===");

    println!("parent + current + app 三个来源的结果相加 = {}", path_keyword_renaming_demo::nested::total());

    // 小结：
    // 1. 想给 crate / super / self 取别名，直接 `use .. as 别名;`
    // 2. 别名让代码里的路径来源更清楚，尤其适合测试模块和深层嵌套的模块
    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 高级特性演示");
    println!("=================");

    unsafe_rust_basics();
    unsafe_functions();
    mutable_static_variables();
    union_types();
    advanced_lifetime_annotations();
    advanced_trait_features();
    advanced_type_features();
    advanced_functions_and_closures();
    advanced_error_handling();
    const_fn_improvements();
    const_generic_parameters();
    rust_2021_edition_features();
    once_lock_and_once_cell();
    cfg_accessible_predicate();
    latest_const_and_generic_enhancements();
    const_mut_refs();
    lazy_access_apis();
    rust_2024_edition_features();
    use_path_keyword_renaming();
    advanced_example_program();

    println!("高级特性演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_pointer_dereference() {
        let mut x = 10;
        let ptr = &mut x as *mut i32;

        unsafe {
            *ptr = 20;
            assert_eq!(x, 20);
        }
    }

    #[test]
    fn test_mutable_static() {
        unsafe {
            MUTABLE_COUNTER = 0;
            MUTABLE_COUNTER += 1;
            assert_eq!(std::ptr::addr_of!(MUTABLE_COUNTER).read(), 1);
        }
    }

    #[test]
    fn test_union_operations() {
        #[repr(C)]
        union TestUnion {
            i: i32,
            u: u32,
        }

        let u = TestUnion { i: -1 };

        unsafe {
            assert_eq!(u.i, -1);
            assert_eq!(u.u, 0xFFFFFFFF);
        }
    }

    #[test]
    fn test_lifetime_annotations() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        let s1 = "hello";
        let s2 = "world";

        let result = longest(s1, s2);
        // "world" 比 "hello" 长，所以应该返回 "world"
        assert_eq!(result, "world");
    }

    #[test]
    fn test_newtype_pattern() {
        struct Kilometers(i32);

        impl Kilometers {
            fn to_meters(&self) -> i32 {
                self.0 * 1000
            }
        }

        let km = Kilometers(5);
        assert_eq!(km.to_meters(), 5000);
    }

    #[test]
    fn test_function_pointers() {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        let f: fn(i32, i32) -> i32 = add;
        assert_eq!(f(3, 4), 7);
    }

    // 为测试添加的矩阵类型
    struct Matrix {
        data: Vec<f64>,
        rows: usize,
        cols: usize,
    }

    impl Matrix {
        fn new(rows: usize, cols: usize, value: f64) -> Self {
            Self {
                data: vec![value; rows * cols],
                rows,
                cols,
            }
        }

        fn get(&self, row: usize, col: usize) -> &f64 {
            &self.data[row * self.cols + col]
        }

        fn add(&self, other: &Matrix) -> Option<Matrix> {
            if self.rows != other.rows || self.cols != other.cols {
                return None;
            }

            let mut result = Matrix::new(self.rows, self.cols, 0.0);
            for i in 0..self.data.len() {
                result.data[i] = self.data[i] + other.data[i];
            }
            Some(result)
        }
    }

    #[test]
    fn test_matrix_operations() {
        let m1 = Matrix::new(2, 2, 1.0);
        let m2 = Matrix::new(2, 2, 2.0);

        let result = m1.add(&m2).unwrap();
        assert_eq!(*result.get(0, 0), 3.0);
        assert_eq!(*result.get(1, 1), 3.0);
    }

    // 为测试添加的函数
    fn compute_fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(compute_fibonacci(0), 0);
        assert_eq!(compute_fibonacci(1), 1);
        assert_eq!(compute_fibonacci(2), 1);
        assert_eq!(compute_fibonacci(3), 2);
        assert_eq!(compute_fibonacci(10), 55);
    }

    // 为测试添加的内存池类型
    struct MemoryPool {
        buffer: Vec<u8>,
        offset: usize,
    }

    impl MemoryPool {
        fn new(size: usize) -> Self {
            Self {
                buffer: vec![0; size],
                offset: 0,
            }
        }

        unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
            if self.offset + size > self.buffer.len() {
                return std::ptr::null_mut();
            }
            let ptr = unsafe { self.buffer.as_mut_ptr().add(self.offset) };
            self.offset += size;
            ptr
        }

        fn stats(&self) -> (usize, usize) {
            (self.offset, self.buffer.len())
        }
    }

    #[derive(Debug)]
    enum TestMyError {
        OutOfMemory,
        Parse(String),
    }

    impl std::fmt::Display for TestMyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                TestMyError::OutOfMemory => write!(f, "内存不足"),
                TestMyError::Parse(msg) => write!(f, "解析错误: {}", msg),
            }
        }
    }

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(100);

        unsafe {
            let ptr1 = pool.allocate(50);
            let ptr2 = pool.allocate(30);

            assert!(!ptr1.is_null());
            assert!(!ptr2.is_null());

            let (used, _) = pool.stats();
            assert_eq!(used, 80);
        }
    }

    #[test]
    fn test_custom_error() {
        let error = TestMyError::Parse("解析错误".to_string());
        assert_eq!(format!("{}", error), "解析错误: 解析错误");
        assert_eq!(format!("{:?}", error), "Parse(\"解析错误\")");
    }

    #[test]
    fn test_const_fn_basic() {
        const fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        const SUM: i32 = add(10, 20);
        assert_eq!(SUM, 30);

        // 测试运行时调用 const fn
        assert_eq!(add(5, 7), 12);
    }

    #[test]
    fn test_const_fn_conditional() {
        const fn abs(x: i32) -> i32 {
            if x >= 0 { x } else { -x }
        }

        const ABS_POS: i32 = abs(42);
        const ABS_NEG: i32 = abs(-42);

        assert_eq!(ABS_POS, 42);
        assert_eq!(ABS_NEG, 42);
        assert_eq!(abs(-100), 100);
    }

    #[test]
    fn test_const_fn_loops() {
        const fn factorial(n: u32) -> u32 {
            let mut result = 1;
            let mut i = 2;
            while i <= n {
                result *= i;
                i += 1;
            }
            result
        }

        const FACT_5: u32 = factorial(5);
        assert_eq!(FACT_5, 120);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn test_const_fn_match() {
        const fn describe_number(n: i32) -> &'static str {
            match n {
                0 => "零",
                1 => "一",
                2 => "二",
                3..=10 => "小数字",
                _ => "大数字",
            }
        }

        const DESC_0: &str = describe_number(0);
        const DESC_5: &str = describe_number(5);
        const DESC_15: &str = describe_number(15);

        assert_eq!(DESC_0, "零");
        assert_eq!(DESC_5, "小数字");
        assert_eq!(DESC_15, "大数字");
    }

    #[test]
    fn test_const_fn_array_ops() {
        const fn array_sum(arr: &[i32; 3]) -> i32 {
            let mut sum = 0;
            let mut i = 0;
            while i < arr.len() {
                sum += arr[i];
                i += 1;
            }
            sum
        }

        const ARRAY: [i32; 3] = [1, 2, 3];
        const SUM: i32 = array_sum(&ARRAY);

        assert_eq!(SUM, 6);
        assert_eq!(array_sum(&[10, 20, 30]), 60);
    }

    #[test]
    fn test_const_fn_bit_ops() {
        const fn count_ones(n: u32) -> u32 {
            let mut count = 0;
            let mut x = n;
            while x != 0 {
                count += x & 1;
                x >>= 1;
            }
            count
        }

        const ONES: u32 = count_ones(0b10101010);
        assert_eq!(ONES, 4);
        assert_eq!(count_ones(0b11110000), 4);
    }

    #[test]
    fn test_const_fn_validation() {
        const fn is_power_of_two(n: u32) -> bool {
            n > 0 && (n & (n - 1)) == 0
        }

        const IS_POW2_16: bool = is_power_of_two(16);
        const IS_POW2_15: bool = is_power_of_two(15);

        assert!(IS_POW2_16);
        assert!(!IS_POW2_15);
        assert!(is_power_of_two(8));
        assert!(!is_power_of_two(7));
    }

    #[test]
    fn test_const_fn_fibonacci() {
        const fn fibonacci(n: u32) -> u32 {
            match n {
                0 => 0,
                1 => 1,
                _ => {
                    let mut a = 0;
                    let mut b = 1;
                    let mut i = 2;
                    while i <= n {
                        let temp = a + b;
                        a = b;
                        b = temp;
                        i += 1;
                    }
                    b
                }
            }
        }

        const FIB_10: u32 = fibonacci(10);
        assert_eq!(FIB_10, 55);
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn test_const_generic_array() {
        fn process_array<const N: usize>(arr: [i32; N]) -> [i32; N] {
            let mut result = [0; N];
            for i in 0..N {
                result[i] = arr[i] * 2;
            }
            result
        }

        let input = [1, 2, 3];
        let result = process_array(input);
        assert_eq!(result, [2, 4, 6]);

        let input5 = [1, 2, 3, 4, 5];
        let result5 = process_array(input5);
        assert_eq!(result5, [2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_const_generic_buffer() {
        fn create_buffer<const SIZE: usize>() -> [u8; SIZE] {
            [0; SIZE]
        }

        let buffer_8 = create_buffer::<8>();
        let buffer_16 = create_buffer::<16>();

        assert_eq!(buffer_8.len(), 8);
        assert_eq!(buffer_16.len(), 16);
        assert_eq!(buffer_8, [0u8; 8]);
        assert_eq!(buffer_16, [0u8; 16]);
    }

    #[test]
    fn test_const_generic_matrix() {
        struct Matrix<T, const ROWS: usize, const COLS: usize> {
            data: [[T; COLS]; ROWS],
        }

        impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
            fn new() -> Self {
                Matrix {
                    data: [[T::default(); COLS]; ROWS],
                }
            }

            fn get(&self, row: usize, col: usize) -> &T {
                &self.data[row][col]
            }

            fn set(&mut self, row: usize, col: usize, value: T) {
                self.data[row][col] = value;
            }
        }

        let mut matrix = Matrix::<i32, 2, 2>::new();
        matrix.set(0, 0, 1);
        matrix.set(0, 1, 2);
        matrix.set(1, 0, 3);
        matrix.set(1, 1, 4);

        assert_eq!(*matrix.get(0, 0), 1);
        assert_eq!(*matrix.get(0, 1), 2);
        assert_eq!(*matrix.get(1, 0), 3);
        assert_eq!(*matrix.get(1, 1), 4);
    }

    #[test]
    fn test_const_generic_fixed_buffer() {
        struct FixedSizeBuffer<const SIZE: usize> {
            data: [u8; SIZE],
            position: usize,
        }

        impl<const SIZE: usize> FixedSizeBuffer<SIZE> {
            fn new() -> Self {
                FixedSizeBuffer {
                    data: [0; SIZE],
                    position: 0,
                }
            }

            fn write(&mut self, byte: u8) -> Result<(), &'static str> {
                if self.position < SIZE {
                    self.data[self.position] = byte;
                    self.position += 1;
                    Ok(())
                } else {
                    Err("缓冲区已满")
                }
            }

            fn capacity(&self) -> usize {
                SIZE
            }

            fn len(&self) -> usize {
                self.position
            }
        }

        let mut buffer = FixedSizeBuffer::<4>::new();
        assert_eq!(buffer.capacity(), 4);
        assert_eq!(buffer.len(), 0);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();
        buffer.write(4).unwrap();

        assert_eq!(buffer.len(), 4);
        assert!(buffer.write(5).is_err()); // 缓冲区已满
    }

    #[test]
    fn test_const_generic_bit_field() {
        // 注意：这个测试需要 generic_const_exprs 特性，暂时注释掉
        /*
        struct BitField<const WIDTH: u32> {
            value: u32,
        }

        impl<const WIDTH: u32> BitField<WIDTH>
        where
            [(); WIDTH as usize]:,
        {
            fn new(value: u32) -> Self {
                let mask = (1 << WIDTH) - 1;
                BitField {
                    value: value & mask,
                }
            }

            fn get(&self) -> u32 {
                let mask = (1 << WIDTH) - 1;
                self.value & mask
            }

            fn set(&mut self, new_value: u32) {
                let mask = (1 << WIDTH) - 1;
                self.value = new_value & mask;
            }
        }

        let mut bit_field = BitField::<4>::new(0x0F);
        assert_eq!(bit_field.get(), 0x0F);

        bit_field.set(0x1F); // 超出 4 位
        assert_eq!(bit_field.get(), 0x0F); // 应该被截断为 4 位

        let mut bit_field_8 = BitField::<8>::new(0x123);
        assert_eq!(bit_field_8.get(), 0x23); // 截断为 8 位
        */

        // 使用具体的实现来展示 const generics 的概念
        struct BitField4 {
            value: u32,
        }

        impl BitField4 {
            fn new(value: u32) -> Self {
                let mask = (1 << 4) - 1;
                BitField4 {
                    value: value & mask,
                }
            }

            fn get(&self) -> u32 {
                let mask = (1 << 4) - 1;
                self.value & mask
            }
        }

        let bit_field = BitField4::new(0x0F);
        assert_eq!(bit_field.get(), 0x0F);
    }

    #[test]
    fn test_const_generic_with_const_fn() {
        const fn array_size<T, const N: usize>(_arr: &[T; N]) -> usize {
            N
        }

        const fn is_even<const N: usize>() -> bool {
            N % 2 == 0
        }

        let test_array = [1, 2, 3, 4];
        let size: usize = array_size(&test_array);
        const IS_EVEN: bool = is_even::<4>();

        assert_eq!(size, 4);
        assert!(IS_EVEN);

        let odd_array = [1, 2, 3];
        let odd_size: usize = array_size(&odd_array);
        const IS_ODD_EVEN: bool = is_even::<3>();

        assert_eq!(odd_size, 3);
        assert!(!IS_ODD_EVEN);
    }

    #[test]
    fn test_rust_2021_edition_features() {
        // 闭包按字段分别捕获：可变借用 retries 的同时，仍可读取 name
        struct Config {
            name: String,
            retries: u32,
        }
        let mut cfg = Config {
            name: "svc".to_string(),
            retries: 3,
        };
        let mut bump = || cfg.retries += 1;
        assert_eq!(cfg.name, "svc");
        bump();
        assert_eq!(cfg.retries, 4);

        // move 闭包只移走用到的字段
        let owner = Config {
            name: "owner".to_string(),
            retries: 0,
        };
        let take_name = move || owner.name;
        assert_eq!(owner.retries, 0);
        assert_eq!(take_name(), "owner");

        // 预导入项：不写 use 也能使用 TryInto / TryFrom / FromIterator
        let small: i32 = 1000_i64.try_into().unwrap();
        assert_eq!(small, 1000);
        assert!(u8::try_from(300_i32).is_err());
        assert_eq!(Vec::from_iter(1..=3), vec![1, 2, 3]);

        // FromStr 不在预导入项里，需要手动 use
        use std::str::FromStr;
        assert_eq!(i32::from_str("42"), Ok(42));

        // 数组按值迭代
        let words = ["a".to_string(), "b".to_string()];
        let owned: Vec<String> = words.into_iter().collect();
        assert_eq!(owned, ["a", "b"]);

        // $p:pat 可以匹配顶层或模式
        macro_rules! is_vowel {
            ($c:expr, $p:pat) => {
                matches!($c, $p)
            };
        }
        assert!(is_vowel!('e', 'a' | 'e' | 'i' | 'o' | 'u'));
        assert!(!is_vowel!('z', 'a' | 'e' | 'i' | 'o' | 'u'));

        // 格式化字符串捕获（Rust 1.58，与 edition 无关）
        let name = "Alice";
        assert_eq!(format!("姓名={name}"), "姓名=Alice");
    }

    #[test]
    fn test_once_cell() {
        use std::cell::OnceCell;

        // 测试 OnceCell 的基本功能 - 使用局部变量而不是静态变量
        let cache = OnceCell::new();

        // 第一次初始化
        let value1 = cache.get_or_init(|| "第一次初始化".to_string());
        assert_eq!(value1, "第一次初始化");

        // 第二次访问应该返回缓存的值
        let value2 = cache.get_or_init(|| "这段代码不会执行".to_string());
        assert_eq!(value2, "第一次初始化");
        assert_eq!(value1.as_ptr(), value2.as_ptr());

        // 测试 OnceCell 的计算缓存功能
        fn expensive_computation(n: u32) -> u32 {
            n * n
        }

        let computation_cache = OnceCell::new();
        let result1 = computation_cache.get_or_init(|| expensive_computation(42));
        let result2 = computation_cache.get_or_init(|| expensive_computation(42));

        assert_eq!(*result1, 1764);
        assert_eq!(*result2, 1764);
        assert_eq!(result1 as *const u32, result2 as *const u32);
    }

    #[test]
    fn test_once_lock() {
        use std::sync::OnceLock;
        use std::thread;

        // 测试 OnceLock 的线程安全性
        static GLOBAL_CACHE: OnceLock<Vec<i32>> = OnceLock::new();

        // 创建多个线程并发访问
        let handles: Vec<_> = (0..5)
            .map(|i| {
                thread::spawn(move || {
                    let data = GLOBAL_CACHE.get_or_init(|| {
                        // 这个闭包应该只执行一次
                        vec![1, 2, 3, 4, 5]
                    });
                    (i, data.len())
                })
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // 所有线程都应该获得相同的数据
        for (i, len) in results {
            assert_eq!(len, 5, "线程 {} 获得了错误的数据长度", i);
        }

        // 测试 OnceLock 的缓存功能
        static FIB_CACHE: OnceLock<Vec<u64>> = OnceLock::new();

        let fib_seq = FIB_CACHE.get_or_init(|| {
            let mut fib = vec![0, 1];
            for i in 2..=10 {
                fib.push(fib[i - 1] + fib[i - 2]);
            }
            fib
        });

        assert_eq!(fib_seq.len(), 11);
        assert_eq!(fib_seq[10], 55);

        // 再次访问应该返回相同的引用
        let fib_seq2 = FIB_CACHE.get_or_init(|| panic!("不应该执行"));
        assert_eq!(fib_seq.as_ptr(), fib_seq2.as_ptr());
    }

    #[test]
    fn test_once_lock_error_handling() {
        use std::io;
        use std::sync::OnceLock;

        // 测试 OnceLock 存储错误结果
        static RESULT_CACHE: OnceLock<Result<String, io::Error>> = OnceLock::new();

        // 初始化一个成功的结果
        let success_result = RESULT_CACHE.get_or_init(|| Ok("成功的结果".to_string()));

        assert!(success_result.is_ok());
        assert_eq!(success_result.as_ref().unwrap(), "成功的结果");

        // 再次访问应该返回缓存的错误结果
        let cached_result =
            RESULT_CACHE.get_or_init(|| Err(io::Error::other("不应该执行")));

        assert!(cached_result.is_ok());
        assert_eq!(cached_result.as_ref().unwrap(), "成功的结果");
    }

    #[test]
    fn test_once_cell_in_struct() {
        use std::cell::OnceCell;

        // 测试 OnceCell 在结构体中的使用
        #[derive(Debug)]
        struct LazyConfig {
            database_url: OnceCell<String>,
            timeout: OnceCell<u32>,
        }

        let config = LazyConfig {
            database_url: OnceCell::new(),
            timeout: OnceCell::new(),
        };

        // 延迟初始化数据库 URL
        let db_url = config
            .database_url
            .get_or_init(|| "postgresql://localhost:5432/mydb".to_string());
        assert_eq!(db_url, "postgresql://localhost:5432/mydb");

        // 延迟初始化超时时间
        let timeout = config.timeout.get_or_init(|| 30);
        assert_eq!(*timeout, 30);

        // 再次访问应该返回缓存的值
        let db_url2 = config.database_url.get_or_init(|| panic!("不应该执行"));
        assert_eq!(db_url.as_ptr(), db_url2.as_ptr());
    }

    #[test]
    fn test_rust_2024_edition_features() {
        // if let 的临时值先于 else 销毁：2021 里这里会死锁
        let config = RwLock::new(None);
        assert!(init_if_empty(&config));
        assert_eq!(*config.read().unwrap(), Some(true));
        assert!(init_if_empty(&config));

        // 块末尾表达式的临时值先于局部变量销毁
        assert_eq!(tail_expression_len(), "2024 edition".len());

        // impl Trait 自动捕获参数的生命周期
        assert_eq!(evens(&[1, 2, 3, 4, 5, 6]).collect::<Vec<_>>(), [2, 4, 6]);

        // 原始标识符与原始生命周期
        let r#gen = 7;
        assert_eq!(r#gen, 7);
        assert_eq!(pick_first("a", "b"), "a");

        // Box<[T]>::into_iter 按值迭代
        let boxed: Box<[String]> = vec!["x".to_string(), "y".to_string()].into_boxed_slice();
        let owned: Vec<String> = boxed.into_iter().collect();
        assert_eq!(owned, ["x", "y"]);
    }

    #[test]
    fn test_use_path_keyword_renaming() {
        assert_eq!(path_keyword_renaming_demo::nested::total(), 10 + 5 + 100);
    }

    #[test]
    fn test_arrays_of_any_length_implement_traits() {
        let buffer = [7_u8; 128];
        assert_eq!(buffer.clone(), buffer);
        assert!(format!("{buffer:?}").starts_with("[7, 7"));
    }

    #[test]
    fn test_lazy_access_apis() {
        use std::cell::LazyCell;
        use std::sync::LazyLock;

        let mut cell = LazyCell::new(|| vec![1, 2]);
        assert!(LazyCell::get(&cell).is_none());
        LazyCell::force_mut(&mut cell).push(3);
        assert_eq!(LazyCell::get(&cell), Some(&vec![1, 2, 3]));

        let mut lock = LazyLock::new(|| String::from("stable"));
        assert!(LazyLock::get(&lock).is_none());
        LazyLock::force_mut(&mut lock).push('!');
        assert_eq!(LazyLock::force(&lock), "stable!");
    }

    #[test]
    fn test_controlflow_const_methods() {
        use std::ops::ControlFlow;

        // is_break / is_continue 可以在常量求值里使用（Rust 1.95）
        const IS_BREAK: bool = ControlFlow::<u32, u32>::Break(3).is_break();
        const IS_CONTINUE: bool = ControlFlow::<u32, u32>::Continue(1).is_continue();
        assert_eq!([IS_BREAK, IS_CONTINUE], [true, true]);
    }
}

// 全局可变静态变量
static mut MUTABLE_COUNTER: i32 = 0;
