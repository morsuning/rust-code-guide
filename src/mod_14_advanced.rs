// Rust 高级特性（Advanced Rust Features）
// 包含 unsafe Rust、内联汇编、高级生命周期、高级 trait 等深度特性的详细讲解
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
        println!("可变静态变量 MUTABLE_COUNTER: {}", std::ptr::addr_of!(MUTABLE_COUNTER).read());

        // 修改可变静态变量
        // 这种操作在多线程环境中是不安全的
        MUTABLE_COUNTER += 1;
        println!("修改后的 MUTABLE_COUNTER: {}", std::ptr::addr_of!(MUTABLE_COUNTER).read());

        MUTABLE_COUNTER += 1;
        println!("再次修改后的 MUTABLE_COUNTER: {}", std::ptr::addr_of!(MUTABLE_COUNTER).read());
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
    // 1. 使用 Arc<Mutex<T>>：线程安全的可变状态
    // 2. 使用 OnceCell：延迟初始化的静态变量
    // 3. 使用 lazy_static：宏简化延迟初始化
    // 4. 使用函数参数：避免全局状态

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
        u.f = 3.14;
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
// 5. 内联汇编（Inline Assembly）- Rust 1.54
// ===========================================

// 内联汇编是 Rust 1.54 版本开始稳定的重要特性
// 它允许在 Rust 代码中直接嵌入汇编语言指令，实现对硬件的直接控制
// 这是系统编程、性能优化和底层开发的重要工具

// 内联汇编的核心价值：
// 1. 硬件控制：直接控制 CPU 指令和寄存器
// 2. 性能优化：生成最优的机器代码，零成本抽象
// 3. 系统调用：实现系统调用和底层操作
// 4. 算法优化：使用 CPU 特定的指令集和 SIMD
// 5. 原子操作：实现低级别的原子操作和无锁编程

// 内联汇编的风险和注意事项：
// 1. 平台依赖：代码可能只在特定平台上工作
// 2. 调试困难：汇编代码难以调试和维护
// 3. 编译器优化限制：可能阻止某些优化
// 4. 安全性：直接操作硬件可能导致安全问题
// 5. 可移植性：降低代码的可移植性

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn inline_assembly() {
    println!("=== 内联汇编 (Rust 1.54+) ===");

    // Rust 1.54 内联汇编的基本语法
    // asm!("汇编模板", 操作数1, 操作数2, ...);
    // 汇编模板：包含汇编指令和占位符
    // 操作数：指定输入、输出和标签

    // 基础示例 1：常量赋值
    // 展示最基本的内联汇编语法和操作数使用
    let result: u64;
    unsafe {
        asm!(
            "mov {}, 42", // 将立即数 42 移动到输出寄存器
            out(reg) result, // 输出操作数：使用通用寄存器
            options(nomem, nostack, preserves_flags) // 选项：不访问内存、不修改栈、不修改标志位
        );
    }
    println!("内联汇编常量赋值: {}", result);

    // 基础示例 2：算术运算
    // 展示如何在汇编中进行基本计算
    let x = 10;
    let y = 20;
    let sum: i32;

    unsafe {
        asm!(
            "add {sum}, {x}, {y}", // 加法：sum = x + y
            x = in(reg) x,      // 输入操作数：x 放在通用寄存器中
            y = in(reg) y,      // 输入操作数：y 放在通用寄存器中
            sum = out(reg) sum, // 输出操作数：sum 放在通用寄存器中
            options(nomem, nostack, preserves_flags) // 优化选项
        );
    }
    println!("内联汇编加法: {} + {} = {}", x, y, sum);

    // 基础示例 3：乘法运算
    // 展示更复杂的算术运算
    let a = 5;
    let b = 3;
    let product: i32;

    unsafe {
        asm!(
            "imul {result}, {a}, {b}", // 有符号乘法：result = a * b
            a = in(reg) a,           // 输入：a 放在寄存器中
            b = in(reg) b,           // 输入：b 放在寄存器中
            result = out(reg) product, // 输出：product 放在寄存器中
            options(nomem, nostack, preserves_flags) // 优化选项
        );
    }
    println!("内联汇编乘法: {} * {} = {}", a, b, product);

    // Rust 1.54 内联汇编的操作数类型详解
    // in(reg)：输入到通用寄存器
    // out(reg)：从通用寄存器输出
    // inout(reg)：输入输出通用寄存器
    // in(reg) late_out(reg)：先输入后输出
    // const：编译时常量
    // sym：符号/标签

    // 演示 inout 操作数的使用
    let mut value = 10;
    unsafe {
        asm!(
            "add {value}, 5", // 将 5 加到 value 上
            value = inout(reg) value, // 同时作为输入和输出
            options(pure, nomem, nostack)
        );
    }
    println!("inout 操作数结果: {}", value);

    // 内联汇编的选项详解
    // pure：函数无副作用，允许编译器优化重复调用
    // nomem：不访问内存，允许编译器进行内存相关优化
    // readonly：只读内存访问
    // preserves_flags：不修改标志位，允许更好的指令调度
    // noreturn：不返回，如 jmp 指令
    // nostack：不修改栈

    // CPU 特性检测示例
    // 演示如何使用内联汇编检测 CPU 特性
    let has_sse2: bool;
    unsafe {
        let mut eax: u32;
        asm!(
            "mov eax, 1", // CPUID 功能号 1
            "cpuid",
            "test edx, 1 << 26", // 检查 SSE2 位
            "setnz {has_sse2}", // 如果 SSE2 位被设置，has_sse2 = 1
            has_sse2 = out(reg_byte) has_sse2,
            eax = out(reg) eax,
            in("ecx") 0u32,
            options(nomem, nostack, preserves_flags)
        );
        has_sse2 = has_sse2 != 0;
    }
    println!("CPU 支持 SSE2: {}", has_sse2);

    // 系统调用示例
    // 演示如何使用内联汇编进行系统调用
    let pid: u64;
    unsafe {
        asm!(
            "syscall",
            in("rax") 39u64, // getpid 系统调用号
            inout("rdi") 0u64 => _, // 参数寄存器
            out("rax") pid, // 结果在 rax 中
            options(nostack, preserves_flags)
        );
    }
    println!("当前进程 ID: {}", pid);

    // 原子操作示例
    // 演示如何使用内联汇编实现原子操作
    let mut atomic_value: u32 = 42;
    unsafe {
        let old_value: u32;
        asm!(
            "lock xchg [{value}], {new_value}", // 原子交换
            value = in(reg) &mut atomic_value,
            new_value = in(reg) 100u32,
            out("eax") old_value, // 旧值在 eax 中
            options(nostack, preserves_flags)
        );
        println!("原子交换操作: 旧值 {}, 新值 {}", old_value, atomic_value);
    }

    // SIMD 指令示例（如果可用）
    // 演示如何使用内联汇编进行 SIMD 优化
    if has_sse2 {
        unsafe {
            let mut a = [1.0f32, 2.0f32, 3.0f32, 4.0f32];
            let mut b = [5.0f32, 6.0f32, 7.0f32, 8.0f32];
            let mut result = [0.0f32; 4];

            asm!(
                "movups xmm0, [{a}]",     // 加载第一个向量
                "movups xmm1, [{b}]",     // 加载第二个向量
                "addps xmm0, xmm1",        // 向量加法
                "movups [{result}], xmm0", // 存储结果
                a = in(reg) a.as_ptr(),
                b = in(reg) b.as_ptr(),
                result = in(reg) result.as_mut_ptr(),
                options(nostack, preserves_flags)
            );

            println!("SIMD 向量加法: {:?}", result);
        }
    }

    // 内存屏障示例
    // 演示如何使用内联汇编实现内存屏障
    let shared_var = 42;
    unsafe {
        // 确保之前的内存操作完成
        asm!("sfence", options(nostack, preserves_flags, nomem));
        // 内存屏障后的操作
        let value = shared_var;
        println!("内存屏障后读取: {}", value);
    }

    // Rust 1.54 内联汇编的实际应用场景
    // 场景 1：性能关键的内联函数
    #[inline(always)]
    unsafe fn fast_abs(x: i32) -> i32 {
        let result: i32;
        asm!(
            "cdq",           // 符号扩展 eax 到 edx:eax
            "xor eax, edx",  // 如果 x 为负数，edx=0xFFFFFFFF，否则 edx=0
            "sub eax, edx",  // 如果 x 为负数，eax = x - (-1) = -x，否则 eax = x - 0 = x
            in("eax") x,
            out("eax") result,
            options(pure, nomem, nostack, preserves_flags)
        );
        result
    }

    let abs_result = unsafe { fast_abs(-100) };
    println!("快速绝对值: {}", abs_result);

    // 场景 2：位操作优化
    #[inline(always)]
    unsafe fn bit_reverse(x: u32) -> u32 {
        let result: u32;
        asm!(
            // 使用 BMI2 指令集（如果可用）
            "mov {result}, {x}",
            "bswap {result}", // 字节交换
            inlateout(reg) x => result,
            options(pure, nomem, nostack, preserves_flags)
        );
        result
    }

    let reversed = unsafe { bit_reverse(0x12345678) };
    println!("位反转结果: 0x{:08X}", reversed);

    // 场景 3：编译时常量计算
    const fn const_asm_multiply(a: i32, b: i32) -> i32 {
        // 在编译时进行乘法计算
        // 注意：const fn 中的内联汇编有更多限制
        let result: i32;
        unsafe {
            asm!(
                "imul {result}, {a}, {b}",
                a = const a,
                b = const b,
                result = out(reg) result,
                options(pure, nomem, nostack, preserves_flags)
            );
        }
        result
    }

    const CONST_PRODUCT: i32 = const_asm_multiply(6, 7);
    println!("编译时常量乘法: {}", CONST_PRODUCT);

    // 内联汇编的最佳实践和注意事项
    // 1. 安全性：始终提供安全的 Rust 接口封装
    // 2. 平台兼容性：使用条件编译处理不同平台
    // 3. 性能测试：验证汇编代码确实带来性能提升
    // 4. 文档化：详细说明汇编代码的功能和约束
    // 5. 错误处理：处理可能的错误情况
    // 6. 备用方案：提供非汇编的实现作为后备

    // 内联汇编的未来发展
    // 1. 更多架构支持：支持 ARM、RISC-V 等更多架构
    // 2. 更丰富的操作数类型：支持更多寄存器类型
    // 3. 更好的集成：与 Rust 类型系统的深度集成
    // 4. 编译器优化：更好的编译器优化支持
    // 5. 调试支持：改进调试体验

    println!();
}

// 其他平台的内联汇编处理
#[cfg(not(all(target_arch = "x86_64", target_os = "linux")))]
fn inline_assembly() {
    println!("=== 内联汇编 (Rust 1.54+) ===");
    println!("内联汇编在当前平台上可能受限或不支持");
    println!("内联汇编需要 Rust 1.54+ 版本和特定平台支持");
    println!("当前架构: {}", std::env::consts::ARCH);
    println!("当前系统: {}", std::env::consts::OS);

    // 显示支持内联汇编的平台
    println!("\n支持内联汇编的平台包括：");
    println!("- x86/x86_64 (Linux, Windows, macOS)");
    println!("- ARM (部分支持)");
    println!("- AArch64 (部分支持)");
    println!("- RISC-V (实验性支持)");

    println!("\n内联汇编的主要应用场景：");
    println!("1. 系统编程：操作系统内核、驱动程序");
    println!("2. 性能优化：关键路径的汇编优化");
    println!("3. 嵌入式系统：直接硬件控制");
    println!("4. 密码学：特定算法的汇编实现");
    println!("5. 虚拟机：JIT 编译器的代码生成");

    println!();
}

// ===========================================
// 6. 高级生命周期注解（Advanced Lifetime Annotations）
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
// 7. 高级 trait 特性（Advanced Trait Features）
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
// 8. 高级类型特性（Advanced Type Features）
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
// 9. 高级函数和闭包（Advanced Functions and Closures）
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
// 10. 高级错误处理（Advanced Error Handling）
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
    // 1. thiserror：简化自定义错误类型的创建
    // 2. anyhow：提供简化的错误处理和上下文添加
    // 3. eyre：提供彩色错误报告和改进的错误链
    // 4. error-chain：传统的错误链处理库

    println!();
}

// ===========================================
// 11. const fn 改进 (Rust 1.46)
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

    // Rust 1.46 新增：更多基本操作支持
    // 1.46 版本开始，const fn 支持更多基本操作和类型
    const fn calculate_circle_area(radius: f64) -> f64 {
        // 现在可以在 const fn 中使用浮点运算
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
    // const fn 不能包含闭包或函数指针，但可以调用其他 const fn
    // 移除这个函数，因为它在 const fn 中不能工作
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
    // 1. 不能进行堆分配
    // 2. 不能使用 trait 方法（除了内置 trait）
    // 3. 不能使用闭包
    // 4. 不能进行 I/O 操作
    // 5. 不能使用 panic! 宏（在某些情况下）
    // 6. 不能使用递归（在 1.46 版本中有限支持）

    // const fn 的最佳实践
    // 1. 纯函数：const fn 应该是纯函数，没有副作用
    // 2. 简单逻辑：保持 const fn 的逻辑简单明了
    // 3. 性能考虑：只在确实需要编译时计算时使用
    // 4. 测试覆盖：为 const fn 编写测试，确保编译时和运行时行为一致
    // 5. 文档化：清楚说明 const fn 的约束和使用条件

    // const fn 的发展历程
    // 1.31：基础 const fn 支持
    // 1.39：更多控制流支持
    // 1.46：大量新特性，包括循环、条件等
    // 1.47：支持更多标准库函数
    // 1.50：支持数组迭代
    // 1.56：支持更多字符串操作
    // 1.57：支持更多 trait 方法
    // 1.61：支持 const 泛型参数

    // const fn 的未来发展方向
    // 1. 更多的标准库支持
    // 2. 更强大的编译时编程能力
    // 3. 更好的错误处理
    // 4. 与 const 泛型的深度集成
    // 5. 编译时反射和元编程

    println!();
}

// ===========================================
// 12. const 泛型参数 (Rust 1.51)
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
    // 注意：这是不稳定的特性，需要 nightly 编译器和以下配置：
    // 在 Cargo.toml 中添加：
    // ```toml
    // [unstable]
    // build-std-features = ["generic_const_exprs"]
    // build-std = true
    // ```
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
    trait BitWidth<const WIDTH: u32> {
        fn max_value() -> Self;
        fn mask() -> Self;
    }

    impl BitWidth<8> for u8 {
        fn max_value() -> Self {
            u8::MAX
        }
        fn mask() -> Self {
            0xFF
        }
    }

    impl BitWidth<16> for u16 {
        fn max_value() -> Self {
            u16::MAX
        }
        fn mask() -> Self {
            0xFFFF
        }
    }

    impl BitWidth<32> for u32 {
        fn max_value() -> Self {
            u32::MAX
        }
        fn mask() -> Self {
            0xFFFFFFFF
        }
    }

    println!("8位最大值: {}", u8::max_value());
    println!("16位最大值: {}", u16::max_value());
    println!("32位最大值: {}", u32::max_value());

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
    // 注意：这是不稳定的特性，需要 nightly 编译器和配置
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

// Rust 2021 Edition 是 Rust 语言的第三个主要版本，于 2021 年 10 月随 Rust 1.56 发布
// 它引入了许多重要的语言改进和新特性，使 Rust 更加现代化和易用
// 这些改进包括更简洁的语法、更好的错误处理、增强的并发支持等

// Rust 2021 Edition 的核心改进：
// 1. 默认 Edition 变更：新项目默认使用 Rust 2021
// 2. 闭包改进：更直观的闭包捕获和推断
// 3. 格式化宏：更强大的格式化字符串功能
// 4. Panic 语法：更一致的 panic 宏语法
// 5. 智能提示：改进的编译器错误和提示
// 6. 预导入项调整：优化的标准库预导入内容

fn rust_2021_edition_features() {
    println!("=== Rust 2021 Edition 新特性 ===");

    // 1. 闭包捕获改进（Closure Capture Improvements）
    // Rust 2021 Edition 改进了闭包对变量的捕获规则，使推断更加智能
    // 这减少了显式 move 关键字的需要，使代码更简洁

    struct Container {
        value: i32,
    }

    impl Container {
        fn get_value(&self) -> i32 {
            self.value
        }

        fn set_value(&mut self, new_value: i32) {
            self.value = new_value;
        }
    }

    let mut container = Container { value: 42 };

    // Rust 2021 Edition 中的闭包捕获更加智能
    let get_closure = || container.get_value(); // 不可变引用
    println!("闭包捕获改进：获取值 {}", get_closure());

    // 分开处理可变和不可变引用
    {
        let mut mut_closure = || container.set_value(100); // 可变引用
        mut_closure();
    }

    let get_closure2 = || container.get_value(); // 重新创建不可变引用
    println!("修改后的值：{}", get_closure2());

    // 闭包捕获的实际影响：
    // - 减少显式 move 关键字的需求
    // - 更精确地捕获语义（只捕获需要的部分）
    // - 更好的错误消息和建议
    // - 与异步代码更好的兼容性

    // 2. 预导入项调整（Prelude Changes）
    // Rust 2021 Edition 调整了标准库的预导入项，移除了一些过时的导入
    // 主要移除：TryInto、TryFrom、FromStr（现在需要显式导入）

    // 以前版本：这些类型自动可用
    // Rust 2021 Edition：需要显式导入
    use std::convert::{TryFrom, TryInto};
    use std::str::FromStr;

    // TryInto 示例
    let big_number: i64 = 1000;
    let small_number: i32 = big_number.try_into().unwrap_or(0);
    println!("TryInto 转换：{} -> {}", big_number, small_number);

    // TryFrom 示例
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl TryFrom<(i32, i32)> for Point {
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

    let point = Point::try_from((10, 20));
    println!("TryFrom 转换：{:?}", point);

    // FromStr 示例
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

    // 预导入项调整的意义：
    // - 减少标准库的隐式依赖
    // - 使导入关系更加明确
    // - 避免命名冲突
    // - 提高编译速度

    // 3. 格式化宏改进（Format Macro Improvements）
    // Rust 2021 Edition 改进了格式化宏的语法和错误处理
    // 这些改进在 Rust 1.58 中进一步完善为格式化字符串捕获

    let name = "Alice";
    let age = 30;

    // 改进的格式化语法（为后续的格式化字符串捕获做准备）
    println!("格式化宏改进：姓名={}, 年龄={}", name, age);

    // 更复杂的格式化示例
    println!("二进制：{:b}", age);
    println!("八进制：{:o}", age);
    println!("十六进制：{:x}", age);
    println!("指针地址：{:p}", &name);

    // 3.1. 格式化字符串捕获（Rust 1.58 新特性）
    // Rust 1.58 引入了格式化字符串捕获功能，允许在格式化宏中直接使用变量名
    // 这大大简化了格式化代码，减少了参数重复和错误

    let user = "Bob";
    let score = 95;
    let level = 3;

    // 传统的格式化方式
    println!("传统方式：用户={}, 分数={}, 等级={}", user, score, level);

    // Rust 1.58+ 的格式化字符串捕获
    println!("捕获方式：用户={user}, 分数={score}, 等级={level}");

    // 格式化字符串捕获的优势：
    // - 更简洁：不需要重复变量名
    // - 更安全：减少参数顺序错误
    // - 更易读：代码更加直观
    // - 更维护：修改变量名时只需要改一处

    // 复杂的格式化捕获示例
    let debug_info = true;
    let timestamp = "2023-12-01 10:30:00";
    let memory_usage = 1024 * 1024 * 512; // 512MB

    if debug_info {
        println!("[{timestamp}] 调试信息：内存使用={memory_usage} bytes ({memory_usage:#x} hex)",);
    }

    // 混合使用捕获和传统参数
    let operation = "calculate";
    let input = 42;
    let result = input * 2;
    println!("{operation}({input}) = {result} (二进制: {result:#b})");

    // 格式化捕获的边界情况
    let complex_expr = 10 + 20;
    println!("复杂表达式需要括号：{}", complex_expr); // 捕获不能用于复杂表达式
    println!("复杂表达式需要括号：{sum}", sum = 10 + 20); // 但可以使用命名参数

    // 格式化捕获的最佳实践：
    // 1. 优先使用捕获语法，代码更简洁
    // 2. 对于复杂表达式，使用命名参数
    // 3. 在调试和日志中使用捕获语法提高可读性
    // 4. 注意变量作用域，确保捕获的变量在作用域内

    // 4. Panic 宏语法一致性
    // Rust 2021 Edition 统一了 panic! 宏的语法
    // 现在 panic! 宏在所有上下文中都有一致的行为

    // 在不同上下文中的 panic! 语法
    let result: Result<i32, &str> = Err("发生错误");

    // 使用 panic! 进行错误处理
    let _value = result.unwrap_or_else(|e| {
        panic!("错误：{}", e); // 一致的 panic 语法
    });

    // 5. 智能提示和错误信息改进
    // Rust 2021 Edition 显著改进了编译器的错误信息和提示
    // 这些改进使开发者更容易理解和修复错误

    // 示例：常见的类型错误现在有更好的提示
    let numbers = vec![1, 2, 3];
    // let sum: &str = numbers.iter().sum(); // 编译错误：类型不匹配
    // Rust 2021 Edition 会提供更清晰的错误信息和修复建议

    let sum: i32 = numbers.iter().sum();
    println!("改进的错误提示：{}", sum);

    // 6. 并发特性增强
    // Rust 2021 Edition 包含了对并发编程的改进
    // 这些改进与后续版本中的并发增强相呼应

    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let data_clone = Arc::clone(&data);

    thread::spawn(move || {
        let sum: i32 = data_clone.iter().sum();
        println!("并发计算：{}", sum);
    })
    .join()
    .unwrap();

    // Rust 2021 Edition 的实际影响：
    // - 提升开发体验：更简洁的语法和更好的错误信息
    // - 增强类型安全：更精确的类型推断和检查
    // - 改进并发支持：更好的闭包和线程交互
    // - 现代化语言特性：使 Rust 更加现代化和易用
    // - 向后兼容：保持与之前版本的兼容性

    // 迁移到 Rust 2021 Edition 的建议：
    // 1. 使用 cargo fix --edition 自动修复大多数兼容性问题
    // 2. 检查并更新显式导入（特别是 TryInto、TryFrom、FromStr）
    // 3. 测试闭包捕获行为的变化
    // 4. 利用新的错误信息和提示优化代码
    // 5. 逐步采用新特性，而不是一次性重写

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
        DB_CONNECTION
            .get_or_init(|| DatabaseConnection::new("postgresql://localhost:5432/mydb"))
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
// 15. 最新的const函数和泛型增强（Latest Const Functions and Generic Enhancements）
// ===========================================

// 随着Rust语言的不断发展，const函数和泛型系统在最新版本中获得了显著的增强
// 这些改进极大地扩展了编译时计算的能力，提高了代码的性能和类型安全性
// 本节涵盖了Rust 1.86到1.90版本中最重要的const函数和泛型增强功能

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
    const fn string_operations() -> &'static str {
        // 在编译时进行字符串处理
        const INPUT: &str = "Hello, World!";
        const LEN: usize = INPUT.len();

        // 编译时字符串切片（简化版本）
        if LEN > 5 {
            "Hello" // 直接返回字符串字面量
        } else {
            INPUT
        }
    }

    const SLICE: &str = string_operations();
    println!("编译时字符串切片：{}", SLICE);

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
    // const泛型允许在类型级别使用编译时常量，提供更强大的抽象能力

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
    // 注意：这是不稳定的特性，需要 nightly 编译器和配置
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

    // 3. const trait实现（Const Trait Implementations）
    // 最新的Rust版本支持更多的trait在const上下文中使用

    // 自定义const trait
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
// 16. 高级示例程序（Advanced Example Programs）
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
// 14. Rust 1.80 #[cfg(accessible)] 配置谓词
// ===========================================

// Rust 1.80 引入了 #[cfg(accessible)] 配置谓词，这是一个强大的条件编译特性
// 它允许基于项目路径的可访问性进行条件编译，为大型项目和库提供更精细的控制

fn cfg_accessible_predicate() {
    println!("=== Rust 1.80 #[cfg(accessible)] 配置谓词 ===");

    // #[cfg(accessible)] 的背景和意义：
    // 1. 精细控制：基于路径的可访问性进行条件编译
    // 2. 特性检测：检测特定类型或模块是否可用
    // 3. 兼容性处理：根据依赖项的存在调整代码
    // 4. 性能优化：有条件地包含昂贵的依赖
    // 5. 平台适配：根据特定平台功能调整实现

    // 基本语法：#[cfg_accessible(path)]
    // path 可以是：模块路径、类型路径、函数路径、trait 路径

    // 示例 1: 检测标准库类型是否可访问
    // 注意：#[cfg(accessible)] 是不稳定的特性，需要 nightly 编译器和以下配置：
    // 在 Cargo.toml 中添加：
    // ```toml
    // [package]
    // name = "rust-code-guide"
    // version = "1.0.0"
    // edition = "2021"
    //
    // [dependencies]
    //
    // [profile.dev]
    // panic = "abort"
    //
    // [unstable]
    // build-std-features = ["compiler-builtins-mem"]
    // build-std = true
    // ```
    // 并在 rust-toolchain.toml 中指定 nightly：
    // ```toml
    // [toolchain]
    // channel = "nightly"
    // ```
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
    // 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉
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
    // 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉
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
    // 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉
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
    // 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉
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
    // 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉
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

    // 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉
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

    // practical_cfg_accessible_examples(); // 注释掉，因为使用了不稳定的特性

    println!("#[cfg(accessible)] 配置谓词演示完成");
    println!();
}

// 注意：#[cfg(accessible)] 是不稳定的特性，暂时注释掉整个函数
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
// 15. Rust 1.82 #[repr(transparent)] 对结构体支持
// ===========================================

// Rust 1.82 完善了对结构体的 #[repr(transparent)] 支持，这是一个重要的内存布局控制特性
// 它允许结构体在内存布局上与其内部的唯一字段完全相同，为 FFI 和类型安全的封装提供了更好的支持

fn repr_transparent_structs() {
    println!("=== Rust 1.82 #[repr(transparent)] 对结构体支持 ===");

    // #[repr(transparent)] 的背景和意义：
    // 1. FFI 互操作性：在 C 语言接口中透明传递 Rust 类型
    // 2. 类型安全封装：为原始类型提供强类型包装器
    // 3. 零成本抽象：包装器不会增加任何运行时开销
    // 4. 内存布局保证：确保包装后的类型与原始类型具有完全相同的内存表示
    // 5. 智能指针支持：为自定义智能指针提供正确的内存布局

    // 基本语法：#[repr(transparent)] struct Wrapper(T);
    // 要求：结构体必须只有一个非零大小的字段

    // 示例 1: 基本的透明包装器
    #[repr(transparent)]
    struct UserId(i32);

    impl UserId {
        fn new(id: i32) -> Self {
            UserId(id)
        }

        fn value(&self) -> i32 {
            self.0
        }
    }

    // 验证内存布局相同
    let user_id = UserId::new(42);
    println!("用户 ID: {}", user_id.value());
    println!("UserId 大小: {} 字节", std::mem::size_of::<UserId>());
    println!("i32 大小: {} 字节", std::mem::size_of::<i32>());

    // 示例 2: FFI 互操作性的透明结构体
    #[repr(C)]
    struct CPoint {
        x: f64,
        y: f64,
    }

    #[repr(transparent)]
    struct SafePoint(CPoint);

    impl SafePoint {
        fn new(x: f64, y: f64) -> Self {
            SafePoint(CPoint { x, y })
        }

        fn distance_from_origin(&self) -> f64 {
            (self.0.x * self.0.x + self.0.y * self.0.y).sqrt()
        }
    }

    let point = SafePoint::new(3.0, 4.0);
    println!("点距离原点的距离: {}", point.distance_from_origin());
    println!("SafePoint 大小: {} 字节", std::mem::size_of::<SafePoint>());
    println!("CPoint 大小: {} 字节", std::mem::size_of::<CPoint>());

    // 示例 3: 带有 ZST 字段的透明结构体
    #[repr(transparent)]
    struct DebugWrapper<T> {
        value: T,
        #[allow(dead_code)]
        debug_marker: (), // 零大小类型，不影响透明性
    }

    impl<T> DebugWrapper<T> {
        fn new(value: T) -> Self {
            DebugWrapper {
                value,
                debug_marker: (),
            }
        }

        fn inner(&self) -> &T {
            &self.value
        }
    }

    let debug_int = DebugWrapper::new(100);
    println!("包装的整数值: {}", debug_int.inner());
    println!(
        "DebugWrapper<i32> 大小: {} 字节",
        std::mem::size_of::<DebugWrapper<i32>>()
    );

    // 示例 4: 通用指针类型的透明包装
    #[repr(transparent)]
    struct NonNullPtr<T> {
        ptr: *const T,
    }

    impl<T> NonNullPtr<T> {
        fn new(ptr: *const T) -> Option<Self> {
            if ptr.is_null() {
                None
            } else {
                Some(NonNullPtr { ptr })
            }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }
    }

    let value = 42;
    let non_null_ptr = NonNullPtr::new(&value as *const i32).unwrap();
    println!("非空指针: {:?}", non_null_ptr.as_ptr());

    // 示例 5: 枚举类型的透明包装
    #[repr(u16)]
    enum StatusCode {
        Ok = 200,
        NotFound = 404,
        InternalError = 500,
    }

    #[repr(transparent)]
    struct SafeStatusCode(StatusCode);

    impl SafeStatusCode {
        fn is_success(&self) -> bool {
            matches!(self.0, StatusCode::Ok)
        }

        fn as_u16(&self) -> u16 {
            match self.0 {
                StatusCode::Ok => 200,
                StatusCode::NotFound => 404,
                StatusCode::InternalError => 500,
            }
        }
    }

    let status = SafeStatusCode(StatusCode::Ok);
    println!(
        "状态码: {}, 是否成功: {}",
        status.as_u16(),
        status.is_success()
    );

    // 示例 6: 复杂嵌套的透明结构体
    #[repr(transparent)]
    struct Matrix3x3([[f32; 3]; 3]);

    impl Matrix3x3 {
        fn identity() -> Self {
            Matrix3x3([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        }

        fn determinant(&self) -> f32 {
            let m = &self.0;
            m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
        }
    }

    let identity_matrix = Matrix3x3::identity();
    println!("单位矩阵行列式: {}", identity_matrix.determinant());
    println!("Matrix3x3 大小: {} 字节", std::mem::size_of::<Matrix3x3>());
    println!(
        "[[f32; 3]; 3] 大小: {} 字节",
        std::mem::size_of::<[[f32; 3]; 3]>()
    );

    practical_transparent_examples();

    println!("#[repr(transparent)] 结构体支持演示完成");
    println!();
}

fn practical_transparent_examples() {
    println!("=== #[repr(transparent)] 实际应用示例 ===");

    // 场景 1: C 库绑定的类型安全包装
    mod c_bindings {
        // 模拟 C 语言的类型定义
        #[repr(C)]
        struct CFileHandle {
            fd: i32,
            flags: u32,
        }

        // 透明包装器提供类型安全
        #[repr(transparent)]
        pub struct SafeFileHandle(CFileHandle);

        impl SafeFileHandle {
            pub fn from_raw(fd: i32) -> Self {
                SafeFileHandle(CFileHandle {
                    fd,
                    flags: 0, // 默认标志
                })
            }

            pub fn raw_fd(&self) -> i32 {
                self.0.fd
            }

            pub fn is_valid(&self) -> bool {
                self.0.fd >= 0
            }
        }

        // 可以安全地传递给 C 函数
        #[repr(C)]
        pub struct CFileStats {
            size: u64,
            modified: u64,
        }

        unsafe extern "C" {
            fn get_file_stats(handle: CFileHandle) -> CFileStats;
        }

        pub fn get_safe_file_stats(handle: &SafeFileHandle) -> Result<CFileStats, String> {
            if !handle.is_valid() {
                return Err("无效的文件句柄".to_string());
            }

            // 模拟文件统计信息，因为我们没有真正的 C 函数
            Ok(CFileStats {
                size: 1024,
                modified: 1234567890,
            })
        }
    }

    let file_handle = c_bindings::SafeFileHandle::from_raw(1); // stdout
    println!("文件句柄是否有效: {}", file_handle.is_valid());

    // 场景 2: 网络协议的类型包装
    mod network_protocol {
        #[repr(C, packed)]
        struct IpAddress {
            bytes: [u8; 4],
        }

        #[repr(transparent)]
        pub struct SafeIpAddress(IpAddress);

        impl SafeIpAddress {
            pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
                SafeIpAddress(IpAddress {
                    bytes: [a, b, c, d],
                })
            }

            pub fn is_loopback(&self) -> bool {
                self.0.bytes[0] == 127
            }

            pub fn as_array(&self) -> [u8; 4] {
                self.0.bytes
            }
        }

        #[repr(transparent)]
        pub struct PortNumber(u16);

        impl PortNumber {
            pub fn new(port: u16) -> Option<Self> {
                if port == 0 {
                    None
                } else {
                    Some(PortNumber(port))
                }
            }

            pub fn value(&self) -> u16 {
                self.0
            }
        }

        #[repr(C)]
        struct SocketAddress {
            ip: IpAddress,
            port: u16,
        }

        #[repr(transparent)]
        pub struct SafeSocketAddress(SocketAddress);

        impl SafeSocketAddress {
            pub fn new(ip: SafeIpAddress, port: PortNumber) -> Self {
                SafeSocketAddress(SocketAddress {
                    ip: ip.0,
                    port: port.0,
                })
            }

            pub fn is_loopback(&self) -> bool {
                // 直接检查 IP 地址的第一个字节
                self.0.ip.bytes[0] == 127
            }
        }
    }

    use network_protocol::{PortNumber, SafeIpAddress, SafeSocketAddress};
    let ip = SafeIpAddress::new(127, 0, 0, 1);
    let port = PortNumber::new(8080).unwrap();
    let socket_addr = SafeSocketAddress::new(ip, port);
    println!("Socket 地址是否回环: {}", socket_addr.is_loopback());

    // 场景 3: 加密算法的密钥包装
    mod crypto {
        #[repr(transparent)]
        pub struct AesKey([u8; 32]);

        impl AesKey {
            pub fn from_bytes(bytes: [u8; 32]) -> Self {
                AesKey(bytes)
            }

            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
        }

        // #[repr(transparent)]  // transparent struct 只能有一个非零大小的字段
        pub struct RsaPublicKey {
            modulus: [u8; 256],
            exponent: [u8; 3],
        }

        impl RsaPublicKey {
            pub fn new(modulus: [u8; 256], exponent: [u8; 3]) -> Self {
                RsaPublicKey { modulus, exponent }
            }

            pub fn modulus(&self) -> &[u8; 256] {
                &self.modulus
            }
        }
    }

    let aes_key = crypto::AesKey::from_bytes([0u8; 32]);
    println!("AES 密钥是否为零密钥: {}", aes_key.is_zero());

    // 场景 4: 硬件寄存器的类型安全访问
    mod hardware {
        #[repr(C)]
        struct DeviceRegisters {
            control: u32,
            status: u32,
            data: u32,
        }

        #[repr(transparent)]
        pub struct SafeDeviceRegisters(*mut DeviceRegisters);

        impl SafeDeviceRegisters {
            pub unsafe fn from_ptr(ptr: *mut DeviceRegisters) -> Self {
                SafeDeviceRegisters(ptr)
            }

            pub fn read_control(&self) -> u32 {
                unsafe { (*self.0).control }
            }

            pub fn write_control(&mut self, value: u32) {
                unsafe {
                    (*self.0).control = value;
                }
            }

            pub fn is_ready(&self) -> bool {
                unsafe { (*self.0).status & 0x01 != 0 }
            }
        }
    }

    // 场景 5: 数据库连接句柄的透明包装
    mod database {
        #[repr(C)]
        struct DBConnection {
            connection_ptr: *mut std::ffi::c_void,
            connection_id: u32,
        }

        #[repr(transparent)]
        pub struct SafeDBConnection(DBConnection);

        impl SafeDBConnection {
            pub fn connection_id(&self) -> u32 {
                self.0.connection_id
            }

            pub fn is_connected(&self) -> bool {
                !self.0.connection_ptr.is_null()
            }
        }
    }

    // 场景 6: 音频处理的样本格式包装
    mod audio {
        #[repr(transparent)]
        struct AudioSample(i16);

        impl AudioSample {
            pub fn new(sample: i16) -> Self {
                AudioSample(sample)
            }

            pub fn normalize(&self) -> f32 {
                self.0 as f32 / i16::MAX as f32
            }

            pub fn apply_gain(&mut self, gain_db: f32) {
                let multiplier = 10.0f32.powf(gain_db / 20.0);
                let normalized = self.normalize() * multiplier;
                self.0 = (normalized * i16::MAX as f32) as i16;
            }
        }

        #[repr(transparent)]
        struct StereoFrame([AudioSample; 2]);

        impl StereoFrame {
            pub fn new(left: i16, right: i16) -> Self {
                StereoFrame([AudioSample::new(left), AudioSample::new(right)])
            }

            pub fn apply_pan(&mut self, pan: f32) {
                // pan: -1.0 (全左) 到 1.0 (全右)
                let left_gain = (1.0 - pan) / 2.0;
                let right_gain = (1.0 + pan) / 2.0;

                self.0[0].apply_gain(20.0 * left_gain.log10());
                self.0[1].apply_gain(20.0 * right_gain.log10());
            }
        }
    }

    println!("实际应用示例演示完成");
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
    inline_assembly();
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
    repr_transparent_structs();
    latest_const_and_generic_enhancements();
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
    fn test_rust_2021_edition_closure_capture() {
        struct Container {
            value: i32,
        }

        impl Container {
            fn get_value(&self) -> i32 {
                self.value
            }

            fn set_value(&mut self, new_value: i32) {
                self.value = new_value;
            }
        }

        let mut container = Container { value: 42 };

        // 测试闭包捕获改进 - 分别测试不可变和可变访问
        {
            let get_closure = || container.get_value();
            assert_eq!(get_closure(), 42);
        }

        {
            let mut mut_closure = || container.set_value(100);
            mut_closure();
        }

        {
            let get_closure2 = || container.get_value();
            assert_eq!(get_closure2(), 100);
        }
    }

    #[test]
    fn test_rust_2021_edition_prelude_changes() {
        use std::convert::{TryFrom, TryInto};
        use std::str::FromStr;

        // 测试 TryInto
        let big_number: i64 = 1000;
        let small_number: i32 = big_number.try_into().unwrap_or(0);
        assert_eq!(small_number, 1000);

        // 测试 TryFrom
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl TryFrom<(i32, i32)> for Point {
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

        let point = Point::try_from((10, 20)).unwrap();
        assert_eq!(point, Point { x: 10, y: 20 });

        // 测试 FromStr
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

        let color: Color = "red".parse().unwrap();
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_rust_2021_edition_format_macros() {
        let name = "Alice";
        let age = 30;

        // 测试格式化宏改进
        let formatted = format!("姓名={}, 年龄={}", name, age);
        assert_eq!(formatted, "姓名=Alice, 年龄=30");

        // 测试不同格式化选项
        assert_eq!(format!("{:b}", age), "11110");
        assert_eq!(format!("{:o}", age), "36");
        assert_eq!(format!("{:x}", age), "1e");
    }

    #[test]
    fn test_rust_2021_edition_concurrency() {
        use std::sync::Arc;
        use std::thread;

        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || data_clone.iter().sum::<i32>());

        let sum = handle.join().unwrap();
        assert_eq!(sum, 15);
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
            RESULT_CACHE.get_or_init(|| Err(io::Error::new(io::ErrorKind::Other, "不应该执行")));

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
}

// 全局可变静态变量
static mut MUTABLE_COUNTER: i32 = 0;
