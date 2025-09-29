// Rust 闭包（Closures）
// 深入讲解闭包语法、环境捕获、Fn trait 系统等函数式编程特性

// ===========================================
// 1. 闭包基础 (Closure Basics)
// ===========================================

// 闭包是 Rust 中函数式编程的核心特性，它们是能够捕获其创建环境的匿名函数
// 闭包提供了简洁、灵活的函数定义方式，特别适用于需要将函数作为参数或返回值的场景
// 闭包在 Rust 中被广泛应用于迭代器适配器、事件处理、回调函数等场景

fn closure_basics() {
    println!("=== 闭包基础 ===");

    // 基本闭包语法
    // 闭包使用 || 定义，参数在 || 之间，函数体在 {} 中
    // 与普通函数不同，闭包可以捕获并访问其创建环境中的变量
    let expensive_closure = |num: u32| -> u32 {
        println!("计算中...");
        num * num * num
    };

    let result = expensive_closure(3);
    println!("3 的立方: {}", result);

    // 闭包的类型推断
    // Rust 编译器能够自动推断闭包的参数和返回类型
    // 这使得闭包的语法比普通函数更加简洁
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 环境变量的捕获
    // 闭包可以捕获其作用域中的变量，这是闭包与普通函数的关键区别
    // 捕获行为取决于变量的使用方式和闭包的类型
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("4 == 4: {}", equal_to_x(4));
    println!("5 == 4: {}", equal_to_x(5));

    // 闭包对可变环境的修改
    // 当闭包需要修改环境中的变量时，该变量必须是可变的
    // 闭包通过 FnMut trait 实现对环境的可变访问
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("count: {}", count);
    };

    increment();
    increment();
    println!("最终 count: {}", count);

    // 闭包的使用场景：
    // 1. 短期使用的匿名函数
    // 2. 需要访问环境的函数
    // 3. 回调函数和事件处理器
    // 4. 迭代器适配器
    // 5. 配置器和建造者模式

    println!();
}

// ===========================================
// 2. 闭包类型和捕获 (Closure Types and Capturing)
// ===========================================

// 闭包的类型由其对环境的捕获方式决定
// Rust 提供了三种主要的闭包 trait：FnOnce、FnMut 和 Fn
// 每种 trait 对应不同的捕获语义和使用限制

fn closure_types_and_capturing() {
    println!("=== 闭包类型和捕获 ===");

    // FnOnce - 消费环境的闭包
    // FnOnce 闭包只能调用一次，因为它会消费（take ownership of）其捕获的变量
    // 当闭包需要移动或销毁其捕获的变量时，它必须实现 FnOnce trait
    let x = String::from("hello");
    let consume = || {
        println!("消费: {}", x);
        // x 被移动，不能再次使用
        // x 的所有权被转移到闭包中，调用闭包时 x 被消耗（drop）
    };
    consume();
    // consume(); // 编译错误：x 已被消费，无法再次调用

    // FnMut - 可变借用环境的闭包
    // FnMut 闭包可以多次调用，并且可以修改其捕获的变量
    // 它通过可变借用来访问环境，因此不会取得变量的所有权
    let mut x = 5;
    let mut mutable = || {
        x += 1;
        println!("修改后的 x: {}", x);
    };
    mutable();
    mutable();
    println!("最终的 x: {}", x);

    // Fn - 不可变借用环境的闭包
    // Fn 闭包可以多次调用，但不能修改其捕获的变量
    // 它通过不可变借用来访问环境，是最受限制的闭包类型
    let x = 5;
    let immutable = || {
        println!("读取的 x: {}", x);
    };
    immutable();
    immutable();
    println!("x 仍然可用: {}", x);

    // move 关键字强制移动所有权
    // move 关键字强制闭包取得其捕获变量的所有权，而不是借用
    // 这在需要将闭包传递到新线程或延长闭包生命周期时非常有用
    let x = String::from("hello");
    let moved_closure = move || {
        println!("移动的 x: {}", x);
    };
    moved_closure();
    // println!("{}", x); // 编译错误：x 已被移动

    // 捕获方式的自动选择：
    // Rust 编译器会根据闭包对变量的使用方式自动选择最合适的捕获方式
    // 1. 如果闭包移动了变量（如 String），则使用 FnOnce
    // 2. 如果闭包修改了变量，则使用 FnMut
    // 3. 如果闭包只是读取变量，则使用 Fn

    // 性能考虑：
    // 1. Fn 闭包通常是最灵活的，可以在任何需要 Fn 或 FnMut 的地方使用
    // 2. FnMut 闭包提供了修改能力，但使用限制更多
    // 3. FnOnce 闭包的限制最多，但提供了最大的灵活性

    println!();
}

// ===========================================
// 3. Fn trait 系列 (Fn Trait Series)
// ===========================================

// Fn trait 系列是 Rust 中函数式编程的基础
// 它们定义了不同类型的可调用对象（callable objects）的行为
// 理解这些 trait 对于编写通用的函数式代码至关重要

fn fn_traits() {
    println!("=== Fn trait 系列 ===");

    // 通用函数接受不同类型的闭包
    // 通过 trait bound，我们可以编写接受不同类型闭包的通用函数
    // 这使得代码更加灵活和可重用
    fn apply<F>(f: F) -> i32
    where
        F: FnOnce() -> i32,
    {
        f() // 调用闭包并返回结果
    }

    fn apply_mut<F>(mut f: F) -> i32
    where
        F: FnMut() -> i32,
    {
        f() // 注意：需要 mut 关键字，因为闭包可能修改其环境
    }

    fn apply_ref<F>(f: F) -> i32
    where
        F: Fn() -> i32,
    {
        f() // 最灵活的调用方式
    }

    // Fn 闭包示例
    // 这个闭包只是读取环境中的变量，因此实现了 Fn trait
    let x = 5;
    let fn_closure = || x;
    println!("Fn 闭包结果: {}", apply_ref(fn_closure));

    // FnMut 闭包示例
    // 这个闭包修改环境中的变量，因此实现了 FnMut trait
    let mut x = 5;
    let fnmut_closure = || {
        x = x + 1;
        x
    };
    println!("FnMut 闭包结果: {}", apply_mut(fnmut_closure));

    // FnOnce 闭包示例
    // 这个闭包消费了 String 类型，因此只能实现 FnOnce trait
    let x = String::from("hello");
    let fnonce_closure = || {
        println!("FnOnce 闭包: {}", x);
        42
    };
    println!("FnOnce 闭包结果: {}", apply(fnonce_closure));

    // 闭包作为返回值
    // 闭包可以作为函数的返回值，这在函数式编程中非常常见
    // 使用 move 关键字确保闭包捕获的变量具有适当的生命周期
    fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
        // move 关键字确保 factor 被移动到闭包中
        // 这样闭包就可以独立于原函数的作用域存在
    }

    let multiply_by_3 = create_multiplier(3);
    let multiply_by_5 = create_multiplier(5);

    println!("3 * 4 = {}", multiply_by_3(4));
    println!("5 * 4 = {}", multiply_by_5(4));

    // Trait 层次结构：
    // 所有 Fn 闭包都实现了 FnMut
    // 所有 FnMut 闭包都实现了 FnOnce
    // 这意味着：
    // - Fn 可以用在任何需要 Fn 或 FnMut 的地方
    // - FnMut 可以用在任何需要 FnOnce 的地方
    // - FnOnce 只能用在需要 FnOnce 的地方

    // 选择合适的 trait bound：
    // 1. 如果闭包只是读取数据，使用 Fn
    // 2. 如果闭包需要修改数据，使用 FnMut
    // 3. 如果闭包需要消费数据，使用 FnOnce

    println!();
}

// ===========================================
// 4. 闭包和迭代器 (Closures and Iterators)
// ===========================================

// 闭包和迭代器是 Rust 函数式编程的两个核心特性
// 它们经常一起使用，提供强大的数据处理能力
// 迭代器适配器（adapters）通常接受闭包作为参数，定义数据的转换逻辑

fn closures_and_iterators() {
    println!("=== 闭包和迭代器 ===");

    // 使用闭包过滤数据
    // filter 方法接受一个返回 bool 的闭包，保留满足条件的元素
    // 闭包使得过滤逻辑可以动态定义，非常灵活
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers: Vec<&i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)  // 过滤偶数
        .collect();
    println!("偶数: {:?}", even_numbers);

    // 使用闭包映射数据
    // map 方法接受一个闭包，对每个元素进行转换
    // 这是函数式编程中最常用的操作之一
    let doubled: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)  // 每个元素乘以 2
        .collect();
    println!("翻倍: {:?}", doubled);

    // 使用闭包折叠数据
    // fold 方法接受一个初始值和一个闭包，将所有元素折叠成一个值
    // 这对求和、连接字符串等操作非常有用
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("总和: {}", sum);

    // 使用闭包查找元素
    // find 方法接受一个闭包，返回第一个满足条件的元素
    // 闭包使得查找条件可以动态定义
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);

    // 使用闭包排序数据
    // sort_by 方法接受一个比较闭包，定义元素的排序规则
    // 闭包的灵活性使得可以按任意规则排序
    let mut words = vec!["apple", "banana", "cherry", "date"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));  // 按长度排序
    println!("按长度排序: {:?}", words);

    // 复杂的迭代器链
    // 迭代器适配器可以链式调用，形成复杂的数据处理管道
    // 每个适配器都接受一个闭包，定义特定的转换逻辑
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x > 2)    // 过滤大于 2 的数
        .map(|&x| x * x)       // 计算平方
        .take(3)               // 只取前 3 个
        .collect();            // 收集到向量中
    println!("大于 2 的平方（前3个）: {:?}", result);

    // 迭代器闭包的优势：
    // 1. 惰性求值：迭代器操作是惰性的，只有在需要时才执行
    // 2. 链式调用：可以将多个操作链式组合
    // 3. 内存效率：迭代器通常按元素处理，不需要存储中间结果
    // 4. 表达力强：可以用简洁的代码表达复杂的数据转换

    // 常用的迭代器适配器：
    // - map: 转换每个元素
    // - filter: 过滤元素
    // - fold/reduce: 折叠元素
    // - find: 查找元素
    // - take/skip: 取或跳过前 N 个元素
    // - enumerate: 给元素添加索引
    // - flat_map: 将每个元素转换为迭代器然后展平
    // - any/all: 检查是否有/所有元素满足条件

    println!();
}

// ===========================================
// 5. 闭包模式 (Closure Patterns)
// ===========================================

// 闭包在软件设计中形成了多种常用的模式
// 这些模式利用了闭包的特性，解决了特定的编程问题
// 理解这些模式有助于编写更加优雅和灵活的代码

fn closure_patterns() {
    println!("=== 闭包模式 ===");

    // 模式 1: 惰性求值（Lazy Evaluation）
    // 惰性求值推迟昂贵的计算，直到真正需要结果时才执行
    // 这可以避免不必要的计算，提高程序性能
    fn expensive_computation(x: i32) -> i32 {
        println!("执行昂贵计算...");
        x * x * x
    }

    fn lazy_compute<F>(condition: bool, computation: F) -> Option<i32>
    where
        F: FnOnce() -> i32,
    {
        if condition {
            Some(computation())  // 只有当 condition 为 true 时才执行计算
        } else {
            None
        }
    }

    let result1 = lazy_compute(true, || expensive_computation(3));
    let result2 = lazy_compute(false, || expensive_computation(3));
    println!("惰性计算结果: {:?} {:?}", result1, result2);

    // 模式 2: 回调函数（Callback Functions）
    // 回调模式允许调用者提供自定义的逻辑，由被调用者在适当时机执行
    // 这是事件驱动编程和异步编程的基础
    fn execute_callback<F>(callback: F)
    where
        F: FnOnce(),
    {
        println!("准备执行回调...");
        callback();
        println!("回调执行完成");
    }

    execute_callback(|| {
        println!("这是回调函数");
        println!("执行一些操作");
    });

    // 模式 3: 配置器模式（Builder Pattern）
    // 配置器模式使用闭包来构建和配置复杂对象
    // 这种模式使得配置过程更加灵活和可读
    struct Config {
        timeout: u32,
        retries: u32,
        debug: bool,
    }

    impl Config {
        fn new() -> Self {
            Config {
                timeout: 30,     // 默认超时 30 秒
                retries: 3,      // 默认重试 3 次
                debug: false,    // 默认不开启调试
            }
        }

        fn configure<F>(mut self, config_fn: F) -> Self
        where
            F: FnOnce(&mut Config),
        {
            config_fn(&mut self);  // 闭包配置对象
            self
        }
    }

    let config = Config::new()
        .configure(|c| {
            c.timeout = 60;     // 修改超时时间
            c.retries = 5;      // 修改重试次数
            c.debug = true;     // 开启调试模式
        });

    println!("配置结果: timeout={}, retries={}, debug={}",
             config.timeout, config.retries, config.debug);

    // 模式 4: 工厂函数（Factory Functions）
    // 工厂函数使用闭包创建和返回定制化的函数
    // 这种模式在函数式编程中非常常见
    fn create_adder(add_value: i32) -> impl Fn(i32) -> i32 {
        move |x| x + add_value
        // move 关键字确保 add_value 被移动到闭包中
        // 这样返回的函数可以独立于原函数存在
    }

    let add_ten = create_adder(10);
    let add_twenty = create_adder(20);

    println!("工厂函数: {} {}", add_ten(5), add_twenty(5));

    // 其他常见的闭包模式：
    // 1. 装饰器模式（Decorator）：在不修改原函数的情况下增加功能
    // 2. 策略模式（Strategy）：将算法封装在闭包中，动态选择算法
    // 3. 观察者模式（Observer）：使用闭包作为事件处理器
    // 4. 中间件模式（Middleware）：使用闭包链处理请求
    // 5. 高阶函数（Higher-order Functions）：接受或返回函数的函数

    // 闭包模式的优势：
    // 1. 代码复用：避免重复编写相似的逻辑
    // 2. 灵活性：可以在运行时动态改变行为
    // 3. 可读性：使代码意图更加清晰
    // 4. 模块化：将关注点分离到不同的闭包中

    println!();
}

// ===========================================
// 6. 闭包和生命周期 (Closures and Lifetimes)
// ===========================================

// 生命周期是 Rust 中保证内存安全的重要机制
// 闭包的生命周期管理比普通函数更复杂，因为闭包可以捕获环境变量
// 理解闭包和生命周期的关系对于编写安全的代码至关重要

fn closures_and_lifetimes() {
    println!("=== 闭包和生命周期 ===");

    // 返回闭包时的生命周期问题
    // 当闭包需要返回给调用者时，必须确保其捕获的变量具有足够的生命周期
    // 使用 move 关键字可以解决这个问题
    fn create_counter() -> impl FnMut() -> i32 {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
        // move 关键字将 count 移动到闭包中
        // 这样闭包就拥有了 count 的所有权，可以独立存在
    }

    let mut counter = create_counter();
    println!("计数器: {}", counter());
    println!("计数器: {}", counter());
    println!("计数器: {}", counter());

    // 闭包和引用的生命周期
    // 闭包可以捕获引用，但必须确保引用在闭包使用期间保持有效
    // Rust 编译器会自动检查这种生命周期约束
    let text = String::from("hello");
    let create_closure = || {
        // text 被借用，闭包的生命周期不能超过 text
        println!("文本: {}", text);
    };

    create_closure();
    println!("文本仍然可用: {}", text);

    // 返回闭包时的生命周期问题
    // 当需要返回闭包时，可以使用 trait 对象来处理复杂的生命周期问题
    // trait 对象提供了动态分发的能力，可以处理不同生命周期的闭包
    fn returns_closure<'a>() -> Box<dyn Fn() + 'a> {
        Box::new(|| println!("返回的闭包"))
    }

    let closure = returns_closure();
    closure();

    // 闭包和 'static 生命周期
    // 'static 生命周期表示闭包可以在整个程序运行期间存在
    // 这通常用于闭包不捕获任何变量或只捕获静态数据的情况
    let static_closure: &'static dyn Fn() = &|| println!("静态闭包");
    static_closure();

    // 生命周期约束的最佳实践：
    // 1. 优先使用 move 关键字：当需要返回闭包时
    // 2. 理解借用规则：闭包不能超过其捕获的引用的生命周期
    // 3. 使用 trait 对象：当需要处理复杂生命周期时
    // 4. 避免长时间持有闭包：减少生命周期管理的复杂性

    // 常见错误和解决方案：
    // 1. "borrowed value does not live long enough"：使用 move
    // 2. "closure may outlive current function"：使用 Box<dyn Fn>
    // 3. "cannot move out of captured variable"：重新设计闭包

    // 生命周期省略规则在闭包中的应用：
    // 1. 闭包参数的生命周期是独立的
    // 2. 闭包返回值的生命周期与其参数相关
    // 3. 如果闭包捕获了引用，其生命周期受限于这些引用

    println!();
}

// ===========================================
// 7. 高级闭包特性 (Advanced Closure Features)
// ===========================================

// 除了基本的闭包用法外，Rust 还提供了许多高级特性
// 这些特性使得闭包可以用于更复杂的场景，包括 trait 对象、错误处理等
// 理解这些特性对于充分利用闭包的能力至关重要

fn advanced_closure_features() {
    println!("=== 高级闭包特性 ===");

    // 闭包和 trait 对象
    // 闭包可以包装成 trait 对象，这为函数式编程提供了多态性
    // 这种模式在需要统一处理不同操作的系统中非常有用
    trait Operation {
        fn execute(&self, x: i32) -> i32;
    }

    struct Adder(i32);
    impl Operation for Adder {
        fn execute(&self, x: i32) -> i32 {
            x + self.0
        }
    }

    struct Multiplier(i32);
    impl Operation for Multiplier {
        fn execute(&self, x: i32) -> i32 {
            x * self.0
        }
    }

    fn create_operation<F>(f: F) -> Box<dyn Operation>
    where
        F: Fn(i32) -> i32 + 'static,
    {
        struct ClosureOperation<F> {
            f: F,
        }

        impl<F> Operation for ClosureOperation<F>
        where
            F: Fn(i32) -> i32,
        {
            fn execute(&self, x: i32) -> i32 {
                (self.f)(x)
            }
        }

        Box::new(ClosureOperation { f })
        // 将闭包装成实现了 Operation trait 的对象
        // 这样可以统一处理闭包和其他实现了 Operation 的类型
    }

    let add_op = create_operation(|x| x + 10);
    let mul_op = create_operation(|x| x * 3);

    println!("闭包 trait 对象: {} {}", add_op.execute(5), mul_op.execute(5));

    // 递归算法的闭包实现
    // 虽然 Rust 不支持直接的递归闭包语法，但可以通过间接方式实现递归
    // 这里使用迭代实现阶乘计算，展示了闭包在算法实现中的应用
    let factorial = |n| {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    };

    println!("5! = {}", factorial(5));
    println!("6! = {}", factorial(6));

    // 闭包和错误处理
    // 闭包可以与 Result 类型结合，提供安全的错误处理机制
    // 这种模式可以将错误处理逻辑与业务逻辑分离
    fn safe_divide<F>(a: i32, b: i32, operation: F) -> Result<i32, String>
    where
        F: FnOnce(i32, i32) -> i32,
    {
        if b == 0 {
            return Err("除数不能为零".to_string());
        }
        Ok(operation(a, b))
    }

    let result = safe_divide(10, 2, |a, b| a / b);
    println!("安全除法: {:?}", result);

    let result = safe_divide(10, 0, |a, b| a / b);
    println!("安全除法: {:?}", result);

    // 高级闭包特性总结：
    // 1. Trait 对象：提供运行时多态性
    // 2. 生命周期管理：确保内存安全
    // 3. 错误处理集成：与 Result 和 Option 类型无缝结合
    // 4. 泛型约束：通过 trait bounds 实现编译时检查
    // 5. 零成本抽象：闭包调用没有运行时开销

    // 性能优化技巧：
    // 1. 使用 #[inline] 提示编译器内联闭包
    // 2. 避免不必要的闭包分配
    // 3. 选择合适的 trait bound（优先使用 Fn，然后是 FnMut）
    // 4. 对于小的闭包，编译器会自动内联优化

    println!();
}

// ===========================================
// 8. 闭包性能考虑 (Closure Performance)
// ===========================================

// 闭包在 Rust 中被设计为零成本抽象（zero-cost abstraction）
// 这意味着在不使用闭包功能时，不会带来任何运行时开销
// 理解闭包的性能特性对于编写高效的代码至关重要

fn closure_performance() {
    println!("=== 闭包性能考虑 ===");

    // 闭包和函数的性能比较
    // 在大多数情况下，闭包的性能与普通函数相当
    // 这得益于 Rust 编译器的优化能力，包括内联和单态化
    fn regular_function(x: i32) -> i32 {
        x * x
    }

    let closure = |x: i32| -> i32 { x * x };

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        let _ = regular_function(5);
    }
    let function_duration = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        let _ = closure(5);
    }
    let closure_duration = start.elapsed();

    println!("函数调用耗时: {:?}", function_duration);
    println!("闭包调用耗时: {:?}", closure_duration);

    // 闭包内联优化
    // 内联是提升闭包性能的重要优化手段
    // 编译器会自动决定是否内联，但也可以使用 #[inline] 属性提供提示
    #[inline]
    fn inline_function(x: i32) -> i32 {
        x * x
    }

    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        let _ = inline_function(5);
    }
    let inline_duration = start.elapsed();

    println!("内联函数耗时: {:?}", inline_duration);

    // 闭包与数据访问的性能比较
    // 闭包捕获大型数据结构时，可能会影响性能
    // 理解这种影响对于编写高效代码很重要
    let mut data = vec![0; 1000];
    let mut closure = |i: usize| data[i] = i;

    let start = std::time::Instant::now();
    for i in 0..1000 {
        closure(i);
    }
    let closure_data_duration = start.elapsed();

    let start = std::time::Instant::now();
    for i in 0..1000 {
        data[i] = i;
    }
    let direct_duration = start.elapsed();

    println!("闭包数据访问耗时: {:?}", closure_data_duration);
    println!("直接数据访问耗时: {:?}", direct_duration);

    // 闭包性能的关键考虑因素：
    // 1. 内联优化：大多数小闭包会被完全内联
    // 2. 单态化：泛型闭包会为每个具体类型生成专门的代码
    // 3. 分配开销：闭包本身通常在栈上分配，没有堆分配开销
    // 4. 捕获开销：捕获环境的成本取决于捕获的方式

    // 性能优化策略：
    // 1. 优先使用 Fn trait：限制最少的 trait 提供最大的优化空间
    // 2. 避免过度捕获：只捕获需要的变量
    // 3. 使用引用：对于大型数据结构，使用引用而不是移动
    // 4. 合理使用 move：在需要时使用 move，避免不必要的借用

    // 编译器优化的限制：
    // 1. 间接函数调用：通过 trait 对象调用会有虚函数开销
    // 2. 递归闭包：不能内联，有函数调用开销
    // 3. 大型闭包：编译器可能选择不内联过大的闭包
    // 4. 动态分发：trait 对象调用需要运行时查找

    // 性能测试建议：
    // 1. 使用基准测试：使用 criterion crate 进行准确测量
    // 2. 比较替代方案：测试闭包与其他实现的性能差异
    // 3. 检查汇编输出：查看编译器生成的实际代码
    // 4. 考虑编译器优化级别：测试时使用 release 模式

    println!();
}

// ===========================================
// 9. 闭包示例程序 (Closure Example Program)
// ===========================================

// 通过完整的示例程序来展示闭包在实际应用中的强大能力
// 这些示例涵盖了事件处理、数据流、配置系统等常见场景
// 理解这些示例有助于在实际项目中更好地使用闭包

fn closure_example_program() {
    println!("=== 闭包示例程序 ===");

    // 示例 1: 事件驱动系统
    // 闭包是事件驱动编程的核心，它们提供了灵活的事件处理机制
    // 这种模式在 GUI 应用、游戏引擎、网络服务器等领域广泛应用
    #[derive(Debug)]
    enum Event {
        Click { x: i32, y: i32 },
        KeyPress(char),
        MouseMove { x: i32, y: i32 },
        Timer,
    }

    struct EventHandler<F> {
        callback: F,
    }

    impl<F> EventHandler<F>
    where
        F: Fn(&Event),
    {
        fn new(callback: F) -> Self {
            EventHandler { callback }
        }

        fn handle(&self, event: &Event) {
            (self.callback)(event);
        }
    }

    // 创建各种事件处理器
    // 每个处理器都是一个闭包，专门处理特定类型的事件
    let click_handler = EventHandler::new(|event| {
        if let Event::Click { x, y } = event {
            println!("点击事件: 坐标({}, {})", x, y);
        }
    });

    let key_handler = EventHandler::new(|event| {
        if let Event::KeyPress(key) = event {
            println!("按键事件: '{}'", key);
        }
    });

    let move_handler = EventHandler::new(|event| {
        if let Event::MouseMove { x, y } = event {
            println!("鼠标移动: ({}, {})", x, y);
        }
    });

    let timer_handler = EventHandler::new(|event| {
        if let Event::Timer = event {
            println!("定时器事件");
        }
    });

    // 模拟事件流
    let events = vec![
        Event::Click { x: 100, y: 200 },
        Event::KeyPress('a'),
        Event::MouseMove { x: 150, y: 250 },
        Event::Timer,
        Event::Click { x: 300, y: 400 },
    ];

    println!("=== 事件处理系统 ===");
    for event in &events {
        println!("处理事件: {:?}", event);
        click_handler.handle(event);
        key_handler.handle(event);
        move_handler.handle(event);
        timer_handler.handle(event);
        println!();
    }

    // 示例 2: 数据处理管道
    // 闭包可以用来构建复杂的数据处理管道
    // 每个处理步骤都是一个闭包，可以灵活地组合和重用
    fn create_pipeline<F1, F2, F3, T, U, V, W>(
        step1: F1,
        step2: F2,
        step3: F3,
    ) -> impl Fn(T) -> W
    where
        F1: Fn(T) -> U,
        F2: Fn(U) -> V,
        F3: Fn(V) -> W,
    {
        move |input| {
            let step1_result = step1(input);
            let step2_result = step2(step1_result);
            let step3_result = step3(step2_result);
            step3_result
        }
    }

    let pipeline = create_pipeline(
        |x: i32| x * 2,      // 第一步：乘以 2
        |x: i32| x + 10,     // 第二步：加 10
        |x: i32| x.to_string(), // 第三步：转换为字符串
    );

    println!("\n=== 数据处理管道 ===");
    let input = 5;
    let output = pipeline(input);
    println!("管道处理: {} -> {}", input, output);

    // 示例 3: 配置系统
    // 闭包提供了优雅的配置接口，支持链式调用和灵活的配置选项
    #[derive(Debug)]
    struct AppConfig {
        name: String,
        version: String,
        debug: bool,
        max_connections: u32,
    }

    impl AppConfig {
        fn new() -> Self {
            AppConfig {
                name: "MyApp".to_string(),
                version: "1.0.0".to_string(),
                debug: false,
                max_connections: 100,
            }
        }

        fn configure<F>(mut self, config_fn: F) -> Self
        where
            F: FnOnce(&mut Self),
        {
            config_fn(&mut self);
            self
        }
    }

    let config = AppConfig::new()
        .configure(|config| {
            config.name = "Advanced App".to_string();
            config.version = "2.0.0".to_string();
            config.debug = true;
            config.max_connections = 1000;
        });

    println!("\n=== 配置系统 ===");
    println!("最终配置: {:?}", config);

    // 闭包在实际项目中的应用场景：
    // 1. 异步编程：Future 和 Stream 的组合器
    // 2. Web 框架：路由处理器和中间件
    // 3. 游戏开发：事件系统和行为树
    // 4. 数据处理：ETL（提取、转换、加载）流程
    // 5. 测试框架：测试用例和断言
    // 6. UI 框架：事件处理器和动画
    // 7. 网络编程：请求处理器和协议解析
    // 8. 系统编程：回调和信号处理

    // 闭包设计原则：
    // 1. 单一职责：每个闭包应该专注于一个任务
    // 2. 可组合性：设计可以轻松组合的闭包
    // 3. 错误处理：在闭包中合理处理错误
    // 4. 性能考虑：避免在闭包中进行昂贵的操作
    // 5. 可测试性：使闭包易于测试和调试

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 闭包演示");
    println!("=============");

    closure_basics();
    closure_types_and_capturing();
    fn_traits();
    closures_and_iterators();
    closure_patterns();
    closures_and_lifetimes();
    advanced_closure_features();
    closure_performance();
    closure_example_program();

    println!("闭包演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_closure() {
        let add_one = |x| x + 1;
        assert_eq!(add_one(5), 6);
    }

    #[test]
    fn test_closure_capture_environment() {
        let x = 5;
        let capture = || x;
        assert_eq!(capture(), 5);
    }

    #[test]
    fn test_closure_mutate_environment() {
        let mut x = 5;
        let mut increment = || {
            x += 1;
            x
        };
        assert_eq!(increment(), 6);
        assert_eq!(increment(), 7);
    }

    #[test]
    fn test_closure_iterator_methods() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_closure_factory() {
        fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
            move |x| x * factor
        }

        let multiply_by_3 = create_multiplier(3);
        let multiply_by_5 = create_multiplier(5);

        assert_eq!(multiply_by_3(4), 12);
        assert_eq!(multiply_by_5(4), 20);
    }

    #[test]
    fn test_closure_counter() {
        fn create_counter() -> impl FnMut() -> i32 {
            let mut count = 0;
            move || {
                count += 1;
                count
            }
        }

        let mut counter = create_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_closure_pattern_matching() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let even_numbers: Vec<&i32> = numbers.iter()
            .filter(|&&x| x % 2 == 0)
            .collect();
        assert_eq!(even_numbers, vec![&2, &4, &6]);
    }
}