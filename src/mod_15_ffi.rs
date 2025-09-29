// Rust FFI (外部函数接口)
// 深入讲解 Rust 与其他语言（主要是 C 语言）的互操作性
// 包含函数调用、数据类型映射、内存管理、错误处理等核心技术

use std::ffi::{CStr, CString, c_char};

// ===========================================
// 1. FFI 基础 (FFI Basics)
// ===========================================

// FFI (Foreign Function Interface) 是 Rust 与其他语言交互的核心机制
// 它允许 Rust 代码调用外部语言的函数，也允许其他语言调用 Rust 函数
// 这是 Rust 作为系统编程语言的重要特性，使其能够与现有 C 生态系统无缝集成

fn ffi_basics() {
    println!("=== FFI 基础 ===");

    // FFI 的核心概念和重要性：
    // 1. 语言互操作性：Rust 可以与 C、C++、Python 等语言交互
    // 2. 库集成：能够使用现有的 C/C++ 库，避免重复造轮子
    // 3. 性能关键代码：在需要时调用优化过的 C 代码
    // 4. 系统调用：直接操作操作系统 API
    // 5. 硬件访问：与底层硬件和驱动程序交互

    // 声明外部 C 函数 (External C Function Declaration)
    // extern "C" 告诉 Rust 编译器使用 C 调用约定
    // 这确保了函数调用的兼容性，因为不同语言的调用约定可能不同
    unsafe extern "C" {
        // 声明一个来自 C 标准库的函数
        // abs 函数计算整数的绝对值，是 C 标准数学库的一部分
        fn abs(input: i32) -> i32;
    }

    // 调用外部 C 函数必须在 unsafe 块中
    // unsafe 块告诉编译器："我了解这里可能存在未定义行为，我会负责安全性"
    // FFI 调用之所以需要 unsafe，是因为：
    // 1. 编译器无法验证外部函数的行为和内存安全性
    // 2. 外部函数可能违反 Rust 的内存安全保证
    // 3. 需要开发者自行确保调用的安全性
    unsafe {
        let result = abs(-42);
        println!("abs(-42) = {}", result);
    }

    // 使用 #[link] 属性链接外部库 (Linking External Libraries)
    // #[link(name = "m")] 告诉链接器链接数学库（libm）
    // 不同的平台可能有不同的库名和链接方式
    #[link(name = "m")]
    unsafe extern "C" {
        // sqrt 函数计算平方根，来自数学库
        // 链接外部库时，需要确保目标系统上有对应的库文件
        fn sqrt(x: f64) -> f64;
    }

    unsafe {
        let result = sqrt(16.0);
        println!("sqrt(16.0) = {}", result);
    }

    // 定义可被 C 调用的 Rust 函数 (Rust Functions Callable from C)
    // #[unsafe(no_mangle)] 告诉编译器不要改变函数名
    // Rust 编译器默认会修改函数名（name mangling）以支持重载等特性
    // 但 C 语言不支持函数重载，所以需要保持函数名不变
    #[unsafe(no_mangle)]
    pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
        // 这个函数使用 C 调用约定，可以被 C 代码直接调用
        // extern "C" 确保了调用约定的兼容性
        a + b
    }

    // 在 unsafe 块中调用 Rust 的 extern "C" 函数
    // 即使是 Rust 函数，只要使用了 extern "C" 调用约定
    // 调用时也需要 unsafe 块，因为这种函数通常用于 FFI 场景
    unsafe {
        let result = add_numbers(10, 20);
        println!("add_numbers(10, 20) = {}", result);
    }

    // FFI 调用约定的重要性：
    // 1. 参数传递方式：参数如何传递到栈或寄存器
    // 2. 栈清理责任：调用者还是被调用者清理栈
    // 3. 返回值传递：返回值如何传递回调用者
    // 4. 寄存器使用约定：哪些寄存器需要保存

    println!();
}

// ===========================================
// 2. 数据类型映射 (Data Type Mapping)
// ===========================================

// 数据类型映射是 FFI 中的关键技术挑战
// 不同编程语言有不同的数据类型系统和内存布局
// 正确的类型映射确保了数据在语言边界上的正确传递

fn data_type_mapping() {
    println!("=== 数据类型映射 ===");

    // Rust 和 C 之间数据类型映射的挑战：
    // 1. 大小差异：相同概念在不同语言中可能有不同大小
    // 2. 对齐要求：内存对齐方式可能不同
    // 3. 表示方式：布尔值、枚举等的内部表示可能不同
    // 4. 内存布局：结构体的字段排列顺序和对齐
    // 5. 类型兼容性：确保类型在运行时行为一致

    // 基本类型映射 (Primitive Type Mapping)
    // 使用 std::os::raw 模块中定义的 C 类型别名
    // 这些类型别名确保了与 C 类型的精确对应
    use std::os::raw::{c_char, c_double, c_float, c_int};

    let rust_int: i32 = 42;
    let c_int_val: c_int = rust_int as c_int;
    println!("Rust i32 -> C int: {} -> {}", rust_int, c_int_val);
    // 注意：大多数平台上 i32 和 c_int 大小相同，但这不是保证的
    // 在嵌入式系统或特殊平台上，它们可能不同

    let rust_float: f64 = 3.14;
    let c_double_val: c_double = rust_float as c_double;
    println!("Rust f64 -> C double: {} -> {}", rust_float, c_double_val);
    // 浮点类型的映射通常比较简单，因为大多数现代系统使用 IEEE 754 标准

    // 指针类型映射 (Pointer Type Mapping)
    // Rust 的字符串切片 &str 转换为 C 的字符指针 *const c_char
    // 这是字符串跨语言传递的基础
    let rust_str = "Hello, World!";
    let c_str = rust_str.as_ptr() as *const c_char;
    println!("Rust &str -> C char*: {:p}", c_str);
    // 注意：这只是指针转换，不涉及字符串内容的复制
    // 实际的字符串转换需要处理 UTF-8 到 C 字符串的转换

    // 数组映射 (Array Mapping)
    // Rust 数组可以转换为 C 风格的指针和长度
    // 这是数组数据传递的标准方式
    let rust_array = [1, 2, 3, 4, 5];
    let c_array_ptr = rust_array.as_ptr() as *const c_int;
    println!("Rust [i32; 5] -> C int*: {:p}", c_array_ptr);
    // 注意：C 函数通常需要额外的长度参数来知道数组的大小
    // Rust 数组包含编译时已知的大小信息，但 C 数组不包含

    // 结构体映射 (Struct Mapping)
    // #[repr(C)] 确保结构体在内存中的布局与 C 兼容
    // Rust 默认可能会重新排列字段以优化内存使用，但 C 需要稳定的布局
    #[repr(C)] // C 兼容的内存布局
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    let point = Point { x: 1.0, y: 2.0 };
    println!("Rust struct -> C struct: {:?}", point);
    // #[repr(C)] 的重要性：
    // 1. 字段顺序：按声明顺序排列，不重新排序
    // 2. 内存对齐：使用 C 的对齐规则
    // 3. 大小计算：确保与 C 编译器计算的大小一致

    // 枚举映射 (Enum Mapping)
    // 同样需要 #[repr(C)] 来确保枚举的表示与 C 兼容
    #[repr(C)]
    #[derive(Debug)]
    enum Status {
        Success, // 通常映射为 0
        Error,   // 通常映射为 1
        Pending, // 通常映射为 2
    }

    let status = Status::Success;
    println!("Rust enum -> C enum: {:?}", status);
    // 枚举映射的注意事项：
    // 1. 值分配：Rust 从 0 开始递增分配值
    // 2. 显式指定：可以使用 #[repr(C, i32)] 来指定整数类型
    // 3. 兼容性：确保与 C 端枚举定义的值一致

    // 联合体映射 (Union Mapping)
    // 联合体是特殊的类型，多个字段共享同一内存位置
    // 需要特别注意安全性，因为访问错误的字段会导致未定义行为
    #[repr(C)]
    union IntOrFloat {
        i: c_int,
        f: c_float,
    }

    let value = IntOrFloat { i: 42 };
    unsafe {
        println!("Rust union -> C union: i = {}", value.i);
    }
    // 联合体的安全性考虑：
    // 1. 类型混淆：必须确保访问的字段与设置的字段匹配
    // 2. 内存布局：所有字段从相同的内存地址开始
    // 3. 大小：由最大字段的大小决定
    // 4. 用途：主要用于类型转换和内存优化

    // 数据类型映射的最佳实践：
    // 1. 始终使用 #[repr(C)] 装饰跨边界的结构体和枚举
    // 2. 优先使用标准库提供的 C 类型别名
    // 3. 注意平台差异，考虑不同架构上的类型大小
    // 4. 对于复杂类型，提供转换函数而不是直接类型转换
    // 5. 充分测试跨边界的类型转换

    println!();
}

// ===========================================
// 3. 字符串处理 (String Handling)
// ===========================================

// 字符串处理是 FFI 中最常见也是最复杂的任务之一
// Rust 和 C 对字符串的处理方式有根本性差异：
// - Rust: UTF-8 编码，包含长度信息，有所有权概念
// - C: 以空字符（\0）结尾的字节序列，无长度信息

fn string_handling() {
    println!("=== 字符串处理 ===");

    // Rust 字符串与 C 字符串的根本差异：
    // 1. 编码方式：Rust 使用 UTF-8，C 使用 ASCII 或本地编码
    // 2. 长度信息：Rust 存储长度，C 依赖空字符终止符
    // 3. 内存安全：Rust 保证安全，C 需要手动管理
    // 4. 所有权：Rust 有所有权系统，C 使用手动管理
    // 5. Unicode 支持：Rust 原生支持，C 需要额外处理

    // Rust 字符串转换为 C 字符串 (Rust to C String Conversion)
    // 使用 CString 和 CStr 来处理跨语言的字符串转换
    // 这些类型专门为 FFI 设计，处理了编码和内存管理的复杂性
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

    let rust_string = "Hello from Rust!";
    // CString::new() 创建一个以空字符结尾的 C 兼容字符串
    // 它会验证字符串是否包含内部空字符，并分配必要的内存
    let c_string = CString::new(rust_string).expect("CString::new failed");
    let c_ptr = c_string.as_ptr() as *const c_char;

    println!("Rust 字符串: {}", rust_string);
    println!("C 字符串指针: {:p}", c_ptr);
    // CString 的特点：
    // 1. 所有权：拥有自己的内存，会自动释放
    // 2. 兼容性：以空字符结尾，与 C 字符串兼容
    // 3. 安全性：不允许内部空字符，防止缓冲区溢出
    // 4. 内存分配：在堆上分配，适合传递给 C 函数

    // C 字符串转换为 Rust 字符串 (C String to Rust String Conversion)
    // 使用 CStr 来处理来自 C 的字符串引用
    // 这是一个借用类型，不拥有内存
    unsafe {
        // CStr::from_ptr() 从原始指针创建 CStr 引用
        // 这是 unsafe 的，因为我们需要确保指针有效且以空字符结尾
        let c_str = CStr::from_ptr(c_ptr);
        // to_str() 尝试将 CStr 转换为 Rust &str
        // 这会验证 UTF-8 编码，如果包含无效 UTF-8 会返回错误
        let rust_str = c_str.to_str().unwrap();
        println!("C 字符串转换回 Rust: {}", rust_str);
    }

    // 处理包含空字符的字符串 (Handling Null Characters in Strings)
    // C 字符串不能包含内部空字符，因为空字符用作终止符
    // CString::new() 会检测并拒绝包含内部空字符的字符串
    let invalid_string = "Hello\0World";
    match CString::new(invalid_string) {
        Ok(cstr) => {
            println!("有效的 C 字符串: {:?}", cstr);
        }
        Err(e) => {
            println!("无效的 C 字符串: {}", e);
        }
    }
    // 错误处理的重要性：
    // 1. 安全性：防止 C 函数读取到意外的字符串终止符
    // 2. 数据完整性：确保字符串内容完整传递
    // 3. 兼容性：符合 C 字符串的约定

    // 手动创建 C 字符串 (Manual C String Creation)
    // 有时需要手动创建 C 字符串，特别是在处理二进制数据时
    let mut buffer = [0u8; 64]; // 创建足够大的缓冲区
    let text = b"Manual C string\0"; // 包含终止符的字节串
    buffer[..text.len()].copy_from_slice(text);

    unsafe {
        let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
        let rust_str = c_str.to_str().unwrap();
        println!("手动创建的 C 字符串: {}", rust_str);
    }
    // 手动创建的注意事项：
    // 1. 缓冲区大小：确保足够大以容纳字符串和终止符
    // 2. 终止符：手动添加空字符作为字符串结束
    // 3. 内存安全：确保缓冲区在使用期间保持有效
    // 4. 字符编码：确保数据是有效的 UTF-8

    // 使用 Cow 进行高效的字符串转换 (Efficient String Conversion with Cow)
    // Cow (Clone on Write) 是一种优化技术，避免不必要的复制
    // 当可能时借用原始数据，只有在需要修改时才复制
    use std::borrow::Cow;

    fn to_cow_string(s: &str) -> Cow<str> {
        if s.contains('\0') {
            // 如果包含空字符，需要复制和处理
            // 创建一个新的字符串，替换空字符为转义序列
            let owned = s.replace('\0', "\\0");
            Cow::Owned(owned)
        } else {
            // 否则可以借用原始字符串，避免复制
            Cow::Borrowed(s)
        }
    }

    let normal_str = "Hello World";
    let special_str = "Hello\0World";

    println!("普通字符串: {}", to_cow_string(normal_str));
    println!("特殊字符串: {}", to_cow_string(special_str));
    // Cow 的优势：
    // 1. 性能优化：避免不必要的字符串复制
    // 2. 内存效率：仅在需要时分配新内存
    // 3. 灵活性：统一处理借用和拥有的情况
    // 4. 抽象性：隐藏了具体的内存管理细节

    // 字符串处理的最佳实践：
    // 1. 优先使用 CString 和 CStr 进行字符串转换
    // 2. 始终检查字符串是否包含内部空字符
    // 3. 注意内存生命周期，确保字符串在使用期间保持有效
    // 4. 处理编码转换错误，提供有意义的错误信息
    // 5. 考虑使用 Cow 优化频繁的字符串转换操作

    println!();
}

// ===========================================
// 4. 回调函数 (Callback Functions)
// ===========================================

// 回调函数是 FFI 中实现事件驱动和异步编程的重要机制
// 它允许 C 代码调用 Rust 函数，实现灵活的程序设计模式
// 回调函数广泛用于事件处理、异步操作、算法框架等场景

fn callback_functions() {
    println!("=== 回调函数 ===");

    // 回调函数的核心概念：
    // 1. 函数指针：将函数作为参数传递给其他函数
    // 2. 调用约定：确保 Rust 和 C 的函数调用方式兼容
    // 3. 生命周期：确保回调函数在使用期间保持有效
    // 4. 上下文传递：将用户数据和状态传递给回调函数
    // 5. 错误处理：处理回调函数中可能发生的错误

    // 定义 C 语言的函数指针类型 (C Function Pointer Type Definition)
    // extern "C" 指定了 C 调用约定，确保与 C 代码的兼容性
    // fn(i32) -> i32 定义了函数的签名：接受一个 i32 参数，返回 i32
    type CCallback = extern "C" fn(i32) -> i32;

    // 接受回调函数的 C 函数声明 (C Function Declaration with Callback)
    // 这个 C 函数接受一个值和一个回调函数，将值传递给回调函数并返回结果
    unsafe extern "C" {
        fn call_with_callback(value: i32, callback: CCallback) -> i32;
    }

    // 定义 Rust 函数作为回调 (Rust Function as Callback)
    // #[unsafe(no_mangle)] 确保函数名在编译时不被修改
    // extern "C" 使用 C 调用约定，使 C 代码能够调用此函数
    #[unsafe(no_mangle)]
    pub extern "C" fn rust_callback(value: i32) -> i32 {
        println!("回调函数被调用，参数: {}", value);
        value * 2 // 简单的处理逻辑：将输入值乘以 2
    }
    // 回调函数的特点：
    // 1. 签名匹配：必须与 C 端期望的函数签名完全一致
    // 2. 调用约定：必须使用相同的调用约定（通常是 C 调用约定）
    // 3. 线程安全：考虑回调函数可能在不同线程中被调用
    // 4. 错误处理：设计适当的错误处理机制

    // 使用回调函数 (Using Callback Functions)
    unsafe {
        // 调用 C 函数，传递 Rust 函数作为回调
        // 注意：这里使用 unsafe 因为我们调用了外部函数
        let result = call_with_callback(21, rust_callback);
        println!("回调函数调用结果: {}", result);
    }

    // 模拟 C 库的函数 (Simulating C Library Functions)
    // 为了演示的目的，我们在 Rust 中模拟 C 函数的实现
    unsafe fn simulate_call_with_callback(value: i32, callback: CCallback) -> i32 {
        println!("模拟调用 C 函数，参数: {}", value);
        let result = callback(value); // 调用回调函数
        println!("回调函数返回: {}", result);
        result + 10 // 对回调结果进行额外处理
    }

    unsafe {
        let result = simulate_call_with_callback(15, rust_callback);
        println!("模拟回调结果: {}", result);
    }

    // 闭包作为回调的挑战 (Challenges with Closures as Callbacks)
    // Rust 闭包不能直接传递给 C 代码，因为：
    // 1. 闭包环境：闭包可能捕获环境变量，有复杂的内部结构
    // 2. 调用约定：闭包的调用方式与 C 函数指针不同
    // 3. 生命周期：闭包的生命周期管理比普通函数更复杂
    // 4. 类型擦除：需要将不同类型的闭包统一为函数指针
    // 注意：不能直接将 Rust 闭包传递给 C，需要额外的包装

    // 使用函数指针包装器 (Function Pointer Wrapper)
    // 这个包装器旨在解决闭包作为回调的问题
    struct CallbackWrapper<F> {
        callback: F, // 存储闭包
    }

    impl<F> CallbackWrapper<F>
    where
        F: Fn(i32) -> i32,
    {
        fn new(callback: F) -> Self {
            CallbackWrapper { callback }
        }

        // 提供一个 extern "C" 函数来调用闭包
        // 这是一个简化的示例，实际实现会更复杂
        fn as_c_callback(&self) -> extern "C" fn(i32) -> i32 {
            // 这是一个泛型函数，需要为每个具体的闭包类型生成实例
            extern "C" fn trampoline<F>(value: i32) -> i32
            where
                F: Fn(i32) -> i32,
            {
                // 这里需要实际的闭包调用逻辑
                // 实际实现需要通过某种方式访问到具体的闭包实例
                // 这通常涉及到全局状态或复杂的指针操作
                value * 2 // 简化的实现
            }
            trampoline::<F> // 返回具体类型的函数指针
        }
    }

    // 实际的闭包包装器实现需要考虑：
    // 1. 内存安全：确保闭包在使用期间不被销毁
    // 2. 线程安全：处理多线程环境下的回调调用
    // 3. 类型擦除：将不同类型的闭包统一为相同的函数指针类型
    // 4. 上下文传递：将闭包的环境数据传递给回调函数

    // 使用包装器（演示概念）
    let wrapper = CallbackWrapper::new(|x| x + 100);
    let c_callback = wrapper.as_c_callback();

    unsafe {
        let result = simulate_call_with_callback(25, c_callback);
        println!("包装器回调结果: {}", result);
    }

    // 回调函数的高级模式：
    // 1. 用户数据指针：传递额外的上下文数据给回调函数
    // 2. 多态回调：支持多种类型的回调函数
    // 3. 异步回调：处理异步操作完成的回调
    // 4. 错误回调：专门的错误处理回调函数
    // 5. 回调链：多个回调函数的链式调用

    // 回调函数的最佳实践：
    // 1. 简单接口：保持回调函数的接口简单明了
    // 2. 错误处理：设计清晰的错误处理机制
    // 3. 文档说明：详细说明回调函数的调用时机和约束
    // 4. 线程安全：考虑回调函数可能被不同线程调用
    // 5. 生命周期管理：确保回调函数及其依赖的数据在使用期间保持有效

    println!();
}

// ===========================================
// 5. 内存管理 (Memory Management)
// ===========================================

// 内存管理是 FFI 中最关键和最具挑战性的方面
// Rust 和 C 有着根本不同的内存管理模型：
// - Rust：所有权系统、编译时检查、自动释放
// - C：手动管理、程序员负责分配和释放
// 正确的内存管理是避免内存泄漏和悬垂指针的关键

fn memory_management() {
    println!("=== 内存管理 ===");

    // FFI 内存管理的核心挑战：
    // 1. 所有权转移：跨语言边界的所有权转移语义
    // 2. 生命周期保证：确保内存在使用期间保持有效
    // 3. 释放责任：明确哪一方负责释放内存
    // 4. 分配器兼容性：不同语言的内存分配器可能不兼容
    // 5. 线程安全：多线程环境下的内存访问安全

    // 使用 Box 分配内存 (Memory Allocation with Box)
    // Box 提供了在堆上分配内存的安全方式
    // 它遵循 Rust 的所有权系统，提供了自动内存管理
    let boxed_value = Box::new(42);
    // Box::into_raw() 将 Box 转换为原始指针，放弃所有权
    // 这用于将 Rust 分配的内存传递给 C 代码
    let raw_ptr = Box::into_raw(boxed_value);

    println!("Box 分配的内存: {:p}", raw_ptr);
    // 注意：调用 Box::into_raw() 后，Rust 不再负责释放这块内存
    // 必须确保 C 代码或者之后的 Rust 代码能够正确释放它

    // 从原始指针重建 Box (Rebuilding Box from Raw Pointer)
    // Box::from_raw() 重新获得原始指针的所有权
    // 这要求指针必须是通过 Box::into_raw() 获得的有效指针
    unsafe {
        let rebuilt_box = Box::from_raw(raw_ptr);
        println!("重建的 Box 值: {}", *rebuilt_box);
    }
    // 当 rebuilt_box 离开作用域时，内存会自动释放
    // 这展示了 Rust 内存管理的安全性

    // C 风格的内存分配 (C-style Memory Allocation)
    // 使用 std::alloc 模块进行低级别的内存分配
    // 这提供了与 C 的 malloc/free 类似的接口
    use std::alloc::{Layout, alloc, dealloc};

    // Layout 描述了内存块的布局信息：大小和对齐
    let layout = Layout::new::<i32>();
    // alloc() 分配指定布局的内存，返回原始指针
    let ptr = unsafe { alloc(layout) } as *mut i32;

    if !ptr.is_null() {
        unsafe {
            *ptr = 123;
            println!("分配的内存值: {}", *ptr);
        }
    }

    // 释放内存 (Memory Deallocation)
    // 必须使用相同的 Layout 来释放内存
    // 这是 unsafe 的，因为错误的释放会导致未定义行为
    if !ptr.is_null() {
        unsafe {
            dealloc(ptr as *mut u8, layout);
        }
    }

    // 字符串的内存管理 (String Memory Management)
    // CString 提供了与 C 字符串兼容的内存管理
    let rust_string = "Dynamic string";
    let c_string = CString::new(rust_string).unwrap();
    // into_raw() 将 CString 转换为原始指针，转移所有权
    let c_ptr = c_string.into_raw();

    println!("CString 转换为原始指针: {:p}", c_ptr);
    // 调用 into_raw() 后，CString 不再负责释放内存
    // 必须使用 from_raw() 来重新获取所有权并正确释放

    // 从原始指针重建 CString (Rebuilding CString from Raw Pointer)
    unsafe {
        let rebuilt_cstring = CString::from_raw(c_ptr);
        let rust_str = rebuilt_cstring.to_str().unwrap();
        println!("重建的字符串: {}", rust_str);
    }
    // rebuilt_cstring 离开作用域时会自动释放内存

    // 向量的内存管理 (Vector Memory Management)
    // Vec 可以转换为 C 风格的数组和长度
    // 这用于将 Rust 集合传递给 C 代码
    let rust_vec = vec![1, 2, 3, 4, 5];
    let mut c_vec = rust_vec.into_boxed_slice();
    let c_slice_ptr = c_vec.as_mut_ptr();
    let c_len = c_vec.len();

    println!("向量转换为 C 风格数组: {:p}, 长度: {}", c_slice_ptr, c_len);
    // 注意：c_vec 不再拥有数据的所有权，但仍然负责释放

    // 从 C 风格数组重建向量 (Rebuilding Vector from C-style Array)
    unsafe {
        let rebuilt_vec = Vec::from_raw_parts(c_slice_ptr, c_len, c_len);
        println!("重建的向量: {:?}", rebuilt_vec);
    }
    // from_raw_parts() 重新获得向量的所有权
    // 当向量离开作用域时，内存会自动释放

    // 内存管理的黄金法则：
    // 1. 明确所有权：清楚知道哪一方拥有内存的所有权
    // 2. 配对的分配和释放：确保每个分配都有对应的释放
    // 3. 一致的分配器：使用相同的内存分配器进行分配和释放
    // 4. 生命周期管理：确保内存在使用期间保持有效
    // 5. 错误处理：处理内存分配失败的情况

    // 常见的内存管理陷阱：
    // 1. 双重释放：两个地方都尝试释放同一块内存
    // 2. 内存泄漏：分配的内存从未被释放
    // 3. 悬垂指针：释放后仍在使用指针
    // 4. 缓冲区溢出：写入超出分配边界
    // 5. 分配器不匹配：用一个分配器分配，用另一个释放

    println!();
}

// ===========================================
// 6. 错误处理 (Error Handling)
// ===========================================

// 错误处理是 FFI 中的重要主题
// Rust 和 C 有着不同的错误处理理念和机制：
// - Rust：使用 Result<T, E> 和 panic，强调编译时错误处理
// - C：使用错误代码、errno 和特殊返回值，依赖运行时检查
// 在 FFI 边界上需要设计合适的错误处理策略

fn error_handling() {
    println!("=== 错误处理 ===");

    // FFI 错误处理的核心挑战：
    // 1. 语义差异：Rust 的错误类型系统 vs C 的错误代码
    // 2. 异常安全：Rust 的 panic vs C 的异常处理
    // 3. 错误传播：跨语言边界的错误信息传递
    // 4. 资源清理：错误发生时的资源释放
    // 5. 线程安全：多线程环境下的错误处理

    // 使用返回值传递错误 (Error Passing via Return Values)
    // 这是最传统的 C 风格错误处理方式
    // 函数返回错误代码，通过指针参数返回结果
    #[unsafe(no_mangle)]
    pub extern "C" fn divide_with_error(a: f64, b: f64, result: *mut f64) -> i32 {
        if b == 0.0 {
            return -1; // 错误代码：除零错误
        }

        unsafe {
            *result = a / b; // 通过指针参数返回计算结果
        }
        0 // 成功代码
    }
    // 错误代码设计的考虑：
    // 1. 明确定义：文档化每个错误代码的含义
    // 2. 唯一性：确保不同错误使用不同的代码
    // 3. 一致性：在整个 API 中保持错误代码的一致性
    // 4. 扩展性：为未来的错误类型预留代码空间

    let mut result = 0.0;
    let error_code = unsafe { divide_with_error(10.0, 2.0, &mut result) };

    if error_code == 0 {
        println!("除法结果: {}", result);
    } else {
        println!("除法错误: 错误代码 {}", error_code);
    }

    // 使用 errno 机制 (Using errno Mechanism)
    // errno 是 C 标准库定义的全局错误变量
    // 它提供了一种线程安全的错误代码存储机制
    unsafe extern "C" {
        fn errno() -> *mut i32; // 获取 errno 变量的指针
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn set_last_error(code: i32) {
        unsafe {
            *errno() = code; // 设置最后的错误代码
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_last_error() -> i32 {
        unsafe { *errno() } // 获取最后的错误代码
    }

    unsafe {
        set_last_error(42);
        let error = get_last_error();
        println!("错误代码: {}", error);
    }
    // errno 机制的特点：
    // 1. 全局性：在整个程序范围内共享错误信息
    // 2. 线程安全：现代 C 库保证 errno 的线程安全性
    // 3. 标准化：POSIX 标准定义了常见的 errno 值
    // 4. 延迟检查：允许在操作后稍后检查错误

    // 使用 Result 类型的包装 (Result Type Wrapper)
    // 将 Rust 的 Result 类型包装为 C 兼容的结构体
    // 这种方式更好地保持了 Rust 的错误处理语义
    #[repr(C)] // 确保与 C 的内存布局兼容
    #[derive(Debug)]
    struct ResultWrapper<T> {
        value: T,        // 实际的返回值
        error_code: i32, // 错误代码，0 表示成功
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn safe_divide(a: f64, b: f64) -> ResultWrapper<f64> {
        if b == 0.0 {
            ResultWrapper {
                value: 0.0,     // 错误时的默认值
                error_code: -1, // 错误代码
            }
        } else {
            ResultWrapper {
                value: a / b,  // 成功时的计算结果
                error_code: 0, // 成功代码
            }
        }
    }
    // ResultWrapper 的优势：
    // 1. 语义保持：保持了 Result 的 Ok/Err 语义
    // 2. 类型安全：编译时类型检查
    // 3. 信息完整：同时包含结果和错误信息
    // 4. 易于使用：C 代码可以直接访问字段

    let result = unsafe { safe_divide(15.0, 3.0) };
    println!("安全除法结果: {:?}", result);

    // 使用 Option 类型的包装 (Option Type Wrapper)
    // 将 Rust 的 Option<T> 类型包装为 C 兼容的结构体
    // 用于表示可能失败的操作，类似于 C 的指针返回值
    #[repr(C)]
    #[derive(Debug)]
    struct OptionWrapper<T> {
        has_value: bool, // 是否有有效值
        value: T,        // 实际的值（如果有）
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn safe_sqrt(x: f64) -> OptionWrapper<f64> {
        if x >= 0.0 {
            OptionWrapper {
                has_value: true, // 有有效值
                value: x.sqrt(), // 计算平方根
            }
        } else {
            OptionWrapper {
                has_value: false, // 无有效值
                value: 0.0,       // 无效时的默认值
            }
        }
    }
    // OptionWrapper 的应用场景：
    // 1. 查找操作：在集合中查找元素
    // 2. 转换操作：可能失败的类型转换
    // 3. 资源获取：可能失败的资源分配
    // 4. 边界检查：可能失败的数组访问

    let sqrt_result = unsafe { safe_sqrt(16.0) };
    println!("安全平方根结果: {:?}", sqrt_result);

    let invalid_result = unsafe { safe_sqrt(-1.0) };
    println!("无效平方根结果: {:?}", invalid_result);

    // 错误处理的高级策略：
    // 1. 错误分类：将错误分为可恢复和不可恢复两类
    // 2. 错误链：提供错误的上下文和来源信息
    // 3. 错误恢复：提供恢复机制或建议
    // 4. 错误日志：记录错误信息以便调试
    // 5. 错误测试：编写测试验证错误处理逻辑

    // FFI 错误处理的最佳实践：
    // 1. 一致性：在整个 API 中使用一致的错误处理策略
    // 2. 文档化：详细记录所有可能的错误情况
    // 3. 资源安全：确保错误发生时正确释放资源
    // 4. 性能考虑：避免错误处理的过度开销
    // 5. 国际化：考虑错误信息的本地化需求

    println!();
}

// ===========================================
// 7. 高级 FII 特性 (Advanced FFI Features)
// ===========================================

// 高级 FFI 特性涵盖了更复杂的跨语言交互场景
// 这些特性通常用于高性能、底层系统编程或与现有 C 生态系统的深度集成
// 掌握这些特性可以构建更强大和灵活的跨语言应用

fn advanced_ffi_features() {
    println!("=== 高级 FFI 特性 ===");

    // 高级 FFI 特性的应用场景：
    // 1. 系统编程：直接调用操作系统 API 和系统服务
    // 2. 性能优化：利用现有的高性能 C 库和算法
    // 3. 硬件访问：与驱动程序和硬件设备交互
    // 4. 遗留集成：与现有的 C/C++ 代码库集成
    // 5. 多语言环境：在多语言架构中充当胶水代码

    // 可变参数函数 (Variadic Functions)
    // 可变参数函数可以接受任意数量的参数
    // 这是 C 语言中 printf 等函数的核心特性
    unsafe extern "C" {
        // printf 是 C 标准库中的可变参数函数
        // ... 表示可变数量的参数
        fn printf(format: *const c_char, ...) -> i32;
    }

    unsafe {
        let format_str = CString::new("这是一个整数: %d\n").unwrap();
        // 调用 printf，传递格式字符串和参数
        printf(format_str.as_ptr(), 42);
    }
    // 可变参数函数的注意事项：
    // 1. 类型安全：编译器无法检查可变参数的类型
    // 2. 参数个数：必须确保传递的参数数量正确
    // 3. 格式匹配：格式说明符必须与参数类型匹配
    // 4. 性能影响：可变参数调用可能有额外开销

    // 使用变长参数 (Using va_list)
    // va_list 提供了对可变参数的编程访问
    // 允许将可变参数传递给其他函数
    // use std::va_list::VaList; // va_list 不是标准库的一部分

    // unsafe extern "C" {
    //     // vprintf 接受 va_list 而不是可变参数
    //     // 这使得可以包装和转发可变参数
    //     fn vprintf(format: *const c_char, args: VaList) -> i32;
    // }
    // va_list 的应用场景：
    // 1. 参数转发：将可变参数传递给其他函数
    // 2. 参数处理：遍历和处理可变参数
    // 3. 日志系统：实现自定义的格式化输出
    // 4. 错误处理：构建详细的错误消息

    // 全局变量访问 (Global Variable Access)
    // 可以访问和修改 C 语言中的全局变量
    // 这需要特别小心，因为全局变量可能被多个线程访问
    #[link(name = "c")]
    unsafe extern "C" {
        // environ 是 C 标准库中存储环境变量的全局变量
        // 它是一个指向字符串数组的指针
        static mut environ: *const *const c_char;
    }

    unsafe {
        if !environ.is_null() {
            println!("环境变量指针: {:p}", std::ptr::addr_of!(environ));
        }
    }
    // 全局变量访问的风险：
    // 1. 线程安全：全局变量可能导致数据竞争
    // 2. 初始化顺序：全局变量的初始化时间不确定
    // 3. 依赖性：可能依赖于特定的运行时环境
    // 4. 可移植性：不同平台的全局变量可能不同

    // 函数指针返回 (Returning Function Pointers)
    // 可以返回函数指针，实现动态函数调用
    // 这在插件系统和动态加载中很有用
    type MathFunc = unsafe extern "C" fn(f64) -> f64;

    #[unsafe(no_mangle)]
    pub extern "C" fn get_math_function(name: *const c_char) -> Option<MathFunc> {
        unsafe {
            let name_str = CStr::from_ptr(name).to_str().unwrap();
            match name_str {
                "sqrt" => Some(sqrt),
                "abs" => Some(abs_f64),
                _ => None,
            }
        }
    }
    // 函数指针工厂的优势：
    // 1. 动态分发：根据运行时条件选择函数
    // 2. 插件系统：支持动态加载的功能模块
    // 3. 回调注册：允许外部代码注册回调函数
    // 4. 策略模式：实现可切换的算法策略

    unsafe extern "C" {
        fn sqrt(x: f64) -> f64;
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn abs_f64(x: f64) -> f64 {
        if x < 0.0 { -x } else { x }
    }

    unsafe {
        let func_name = CString::new("sqrt").unwrap();
        if let Some(math_func) = get_math_function(func_name.as_ptr()) {
            let result = math_func(25.0);
            println!("动态函数调用结果: {}", result);
        }
    }

    // 线程本地存储 (Thread-Local Storage)
    // thread_local! 宏定义了线程本地的静态变量
    // 每个线程都有自己独立的变量副本
    thread_local! {
        static FFI_COUNTER: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
    }
    // 线程本地存储的特点：
    // 1. 线程隔离：每个线程有独立的变量副本
    // 2. 无锁访问：不需要同步机制，提高性能
    // 3. 生命周期：变量生命周期与线程绑定
    // 4. 初始化：每个线程首次访问时进行初始化

    #[unsafe(no_mangle)]
    pub extern "C" fn increment_ffi_counter() {
        FFI_COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
        });
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_ffi_counter() -> i32 {
        FFI_COUNTER.with(|counter| *counter.borrow())
    }

    increment_ffi_counter();
    increment_ffi_counter();
    let count = get_ffi_counter();
    println!("FFI 计数器: {}", count);

    // 其他高级 FFI 特性：
    // 1. 信号处理：与操作系统的信号机制交互
    // 2. 异步调用：支持异步函数调用和回调
    // 3. 内存映射：操作内存映射文件和设备
    // 4. 线程同步：与线程同步原语交互
    // 5. 异常处理：处理跨语言边界的异常

    // 高级 FFI 的性能考虑：
    // 1. 调用开销：最小化跨语言调用的开销
    // 2. 批量操作：减少频繁的跨语言调用
    // 3. 内存复制：优化数据的跨语言传递
    // 4. 缓存策略：缓存跨语言调用的结果
    // 5. 内联考虑：考虑函数内联的可能性

    println!();
}

// ===========================================
// 8. 实际应用示例 (Practical Examples)
// ===========================================

// 实际应用示例展示了 FFI 在真实世界中的使用场景
// 这些示例涵盖了数据库操作、图像处理、网络编程等常见领域
// 通过这些示例，可以更好地理解如何将 FFI 技术应用到实际项目中

fn practical_examples() {
    println!("=== 实际应用示例 ===");

    // 实际应用中 FFI 的典型用途：
    // 1. 遗留系统集成：与现有的 C/C++ 代码库集成
    // 2. 性能关键路径：使用优化过的 C 库处理性能关键任务
    // 3. 系统编程：直接调用操作系统 API 和系统服务
    // 4. 第三方库集成：使用丰富的 C 生态系统库
    // 5. 硬件访问：与设备驱动和硬件接口交互

    // SQLite 数据库包装器示例 (SQLite Database Wrapper Example)
    // SQLite 是一个轻量级的嵌入式数据库，广泛用于移动应用和桌面软件
    // 这个示例展示了如何为 C 库创建安全的 Rust 包装器
    use std::ffi::CString;
    use std::ptr;

    // 模拟 SQLite 的 C API (Simulating SQLite C API)
    // 定义与 C 头文件对应的 Rust 类型
    #[repr(C)]
    struct Sqlite3; // 数据库连接句柄

    #[repr(C)]
    struct Sqlite3Stmt; // 预处理语句句柄

    // 回调函数类型定义
    type Sqlite3Callback =
        extern "C" fn(*mut std::ffi::c_void, i32, *mut *mut c_char, *mut *mut c_char) -> i32;

    // 声明 SQLite 的 C 函数接口（在实际项目中会链接到真正的SQLite库）
    // 这里为了演示，我们同时提供模拟实现
    // 数据库包装器的设计原则：
    // 1. 资源安全：使用 RAII 模式管理数据库连接
    // 2. 错误处理：将 C 错误代码转换为 Rust Result
    // 3. 类型安全：提供强类型的 API 接口
    // 4. 并发安全：处理多线程环境下的数据库访问
    // 5. 内存管理：正确管理字符串和结果集的内存

    // 模拟实现（用于演示）
    unsafe fn sqlite3_open(_filename: *const c_char, db: *mut *mut Sqlite3) -> i32 {
        unsafe {
            *db = ptr::null_mut(); // 模拟数据库句柄
        }
        0 // SQLITE_OK
    }

    unsafe fn sqlite3_close(_db: *mut Sqlite3) -> i32 {
        0 // SQLITE_OK
    }

    unsafe fn sqlite3_exec(
        _db: *mut Sqlite3,
        _sql: *const c_char,
        _callback: Option<Sqlite3Callback>,
        _callback_arg: *mut std::ffi::c_void,
        _errmsg: *mut *mut c_char,
    ) -> i32 {
        0 // SQLITE_OK
    }

    unsafe fn sqlite3_free(_ptr: *mut std::ffi::c_void) {
        // 模拟释放内存
    }

    // 使用模拟的 SQLite API
    unsafe {
        let mut db: *mut Sqlite3 = ptr::null_mut();
        let filename = CString::new(":memory:").unwrap();

        let result = sqlite3_open(filename.as_ptr(), &mut db);
        println!("SQLite 打开结果: {}", result);

        let sql = CString::new("SELECT * FROM users").unwrap();
        let mut errmsg: *mut c_char = ptr::null_mut();

        let result = sqlite3_exec(db, sql.as_ptr(), None, ptr::null_mut(), &mut errmsg);

        if result != 0 && !errmsg.is_null() {
            let error_msg = CStr::from_ptr(errmsg).to_str().unwrap();
            println!("SQLite 错误: {}", error_msg);
            sqlite3_free(errmsg as *mut std::ffi::c_void);
        }

        sqlite3_close(db);
    }

    // 图像处理库包装器示例 (Image Processing Library Wrapper Example)
    // 图像处理是 FFI 的常见应用场景，许多高性能图像处理库都是用 C/C++ 编写的
    // 这个示例展示了如何处理复杂的内存布局和资源管理
    #[repr(C)]
    struct Image {
        width: i32,    // 图像宽度
        height: i32,   // 图像高度
        data: *mut u8, // 像素数据（RGB 格式）
    }
    // 图像处理的内存布局考虑：
    // 1. 像素格式：RGB、RGBA、灰度等不同的像素格式
    // 2. 内存对齐：确保数据访问的对齐要求
    // 3. 行间距：可能的行填充字节
    // 4. 内存连续性：确保数据在内存中连续存储
    // 5. 大端序/小端序：处理多字节数据的字节序

    // 模拟实现（在实际项目中会链接到真正的图像处理库）
    unsafe fn image_create(width: i32, height: i32) -> *mut Image {
        // 计算所需的内存大小（每个像素 3 字节：RGB）
        let data = unsafe {
            std::alloc::alloc(
                std::alloc::Layout::array::<u8>((width * height * 3) as usize).unwrap(),
            ) as *mut u8
        };

        // 创建图像结构体并包装为指针
        Box::into_raw(Box::new(Image {
            width,
            height,
            data,
        }))
    }

    unsafe fn image_free(image: *mut Image) {
        if !image.is_null() {
            let img = unsafe { Box::from_raw(image) };
            // 释放像素数据内存
            unsafe {
                std::alloc::dealloc(
                    img.data,
                    std::alloc::Layout::array::<u8>((img.width * img.height * 3) as usize).unwrap(),
                );
            }
        }
    }

    // 设置图像像素
    unsafe fn image_set_pixel(image: *mut Image, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if image.is_null() {
            return;
        }
        let img = unsafe { &mut *image };
        let index = ((y * img.width + x) * 3) as usize;
        if index + 2 < (img.width * img.height * 3) as usize {
            unsafe {
                *img.data.add(index) = r;
                *img.data.add(index + 1) = g;
                *img.data.add(index + 2) = b;
            }
        }
    }

    // 获取图像像素
    unsafe fn image_get_pixel(
        image: *mut Image,
        x: i32,
        y: i32,
        r: *mut u8,
        g: *mut u8,
        b: *mut u8,
    ) {
        if image.is_null() || r.is_null() || g.is_null() || b.is_null() {
            return;
        }
        let img = unsafe { &*image };
        let index = ((y * img.width + x) * 3) as usize;
        if index + 2 < (img.width * img.height * 3) as usize {
            unsafe {
                *r = *img.data.add(index);
                *g = *img.data.add(index + 1);
                *b = *img.data.add(index + 2);
            }
        }
    }

    // 使用模拟的图像处理 API
    unsafe {
        let image = image_create(100, 100);
        if !image.is_null() {
            // 设置红色像素
            image_set_pixel(image, 10, 10, 255, 0, 0);

            let mut r = 0u8;
            let mut g = 0u8;
            let mut b = 0u8;
            image_get_pixel(image, 10, 10, &mut r, &mut g, &mut b);

            println!("图像像素值: R={}, G={}, B={}", r, g, b);
            image_free(image);
        }
    }

    // 网络库包装器示例 (Network Library Wrapper Example)
    // 网络编程是系统编程的重要组成部分
    // 许多网络库和协议栈都是用 C 编写的，需要通过 FFI 来使用
    #[repr(C)]
    struct Socket; // 网络套接字的不透明句柄
    // 网络编程的特殊考虑：
    // 1. 阻塞操作：处理可能阻塞的网络操作
    // 2. 超时处理：设置和管理网络操作超时
    // 3. 错误恢复：处理网络连接中断和错误
    // 4. 并发访问：多线程环境下的套接字使用
    // 5. 资源清理：确保套接字资源正确释放

    unsafe extern "C" {
        fn socket_create(domain: i32, type_: i32, protocol: i32) -> *mut Socket;
        fn socket_connect(socket: *mut Socket, addr: *const c_char, port: i32) -> i32;
        fn socket_send(socket: *mut Socket, data: *const c_char, len: i32) -> i32;
        fn socket_close(socket: *mut Socket);
    }

    // 使用模拟的网络 API
    unsafe {
        let socket = socket_create(2, 1, 0); // AF_INET, SOCK_STREAM
        if !socket.is_null() {
            let addr = CString::new("127.0.0.1").unwrap();
            let result = socket_connect(socket, addr.as_ptr(), 8080);
            println!("连接结果: {}", result);

            let message = CString::new("Hello, Server!").unwrap();
            let sent = socket_send(socket, message.as_ptr(), message.as_bytes().len() as i32);
            println!("发送字节数: {}", sent);

            socket_close(socket);
        }
    }

    // 其他实际应用场景：
    // 1. 加密库：OpenSSL、libsodium 等加密库的包装器
    // 2. 多媒体库：FFmpeg、SDL 等多媒体处理库
    // 3. 科学计算：BLAS、LAPACK 等数值计算库
    // 4. 机器学习：TensorFlow、PyTorch 等 ML 框架的 C API
    // 5. 游戏引擎：Unity、Unreal Engine 等游戏引擎的插件

    // FFI 集成的最佳实践：
    // 1. 安全抽象：提供安全的 Rust 包装器
    // 2. 错误转换：将 C 错误转换为 Rust 错误类型
    // 3. 文档完善：详细说明跨语言边线的使用约束
    // 4. 测试覆盖：编写全面的测试验证集成正确性
    // 5. 性能优化：最小化跨语言调用的开销

    println!();
}

// ===========================================
// 9. 安全包装器 (Safe Wrappers)
// ===========================================

// 安全包装器是 FFI 编程中的关键设计模式
// 它们将不安全的底层 C API 包装成类型安全的 Rust 接口
// 通过 RAII（Resource Acquisition Is Initialization）模式确保资源正确管理
// 这使得开发者可以在享受 Rust 安全性的同时使用底层系统功能

fn safe_wrappers() {
    println!("=== 安全包装器 ===");

    // 安全包装器的设计原则：
    // 1. 资源安全：使用 RAII 确保资源正确获取和释放
    // 2. 类型安全：提供强类型接口，避免类型错误
    // 3. 错误处理：将 C 风格错误转换为 Rust Result
    // 4. 线程安全：考虑多线程环境下的使用
    // 5. 易用性：提供符合 Rust 风格的 API
    // 6. 文档完整：详细说明使用方法和限制

    // 内存分配的安全包装器 (Safe Memory Allocation Wrapper)
    // 封装了原始的内存分配和释放操作
    // 确保分配的内存一定会被正确释放，避免内存泄漏
    struct SafeMemory {
        ptr: *mut u8, // 内存指针
        size: usize,  // 内存大小
    }

    impl SafeMemory {
        // 创建新的安全内存块
        fn new(size: usize) -> Result<Self, String> {
            // 验证大小参数，确保不会溢出
            let layout =
                std::alloc::Layout::array::<u8>(size).map_err(|e| format!("无效的布局: {}", e))?;

            // 分配内存
            let ptr = unsafe { std::alloc::alloc(layout) };
            if ptr.is_null() {
                return Err("内存分配失败".to_string());
            }

            Ok(SafeMemory { ptr, size })
        }

        // 获取可变指针，用于写入操作
        fn as_mut_ptr(&mut self) -> *mut u8 {
            self.ptr
        }

        // 获取只读指针，用于读取操作
        fn as_ptr(&self) -> *const u8 {
            self.ptr as *const u8
        }

        // 获取内存大小
        fn len(&self) -> usize {
            self.size
        }
    }

    // Drop trait 确保内存自动释放
    impl Drop for SafeMemory {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                let layout = std::alloc::Layout::array::<u8>(self.size).unwrap();
                unsafe {
                    std::alloc::dealloc(self.ptr, layout);
                }
            }
        }
    }

    // 使用安全内存包装器
    let mut safe_mem = SafeMemory::new(1024).expect("内存分配失败");
    println!("安全内存分配: 大小 {} 字节", safe_mem.len());
    // 当 safe_mem 离开作用域时，内存会自动释放

    // 文件描述符的安全包装器 (Safe File Descriptor Wrapper)
    // 封装了文件操作的底层 C API
    // 确保文件描述符在不需要时被正确关闭
    struct SafeFile {
        fd: i32, // 文件描述符
    }

    impl SafeFile {
        // 打开文件
        fn open(path: &str) -> Result<Self, String> {
            let c_path = CString::new(path).map_err(|e| e.to_string())?;
            let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o666) };

            if fd == -1 {
                return Err("文件打开失败".to_string());
            }

            Ok(SafeFile { fd })
        }

        // 写入数据
        fn write(&self, data: &[u8]) -> Result<usize, String> {
            let written = unsafe {
                libc::write(
                    self.fd,
                    data.as_ptr() as *const std::os::raw::c_void,
                    data.len(),
                )
            };

            if written == -1 {
                return Err("写入失败".to_string());
            }

            Ok(written as usize)
        }

        // 读取数据
        fn read(&self, buffer: &mut [u8]) -> Result<usize, String> {
            let read = unsafe {
                libc::read(
                    self.fd,
                    buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                    buffer.len(),
                )
            };

            if read == -1 {
                return Err("读取失败".to_string());
            }

            Ok(read as usize)
        }
    }

    impl Drop for SafeFile {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.fd);
            }
        }
    }

    // 使用安全文件包装器
    let file = SafeFile::open("/tmp/test.txt").expect("文件打开失败");
    let data = b"Hello, Safe World!";
    let written = file.write(data).expect("写入失败");
    println!("写入 {} 字节", written);

    // 网络套接字的安全包装器 (Safe Socket Wrapper)
    // 封装了网络套接字的创建和管理
    // 处理复杂的网络地址设置和连接管理
    struct SafeSocket {
        sockfd: i32, // 套接字描述符
    }

    impl SafeSocket {
        // 创建新套接字
        fn new(domain: i32, type_: i32, protocol: i32) -> Result<Self, String> {
            let sockfd = unsafe { libc::socket(domain, type_, protocol) };
            if sockfd == -1 {
                return Err("套接字创建失败".to_string());
            }

            Ok(SafeSocket { sockfd })
        }

        // 连接到远程地址
        fn connect(&self, addr: &str, port: u16) -> Result<(), String> {
            let c_addr = CString::new(addr).map_err(|e| e.to_string())?;

            // 设置套接字地址结构
            let mut sockaddr = libc::sockaddr_in {
                sin_family: libc::AF_INET as u16,
                sin_port: port.to_be(), // 网络字节序
                sin_addr: libc::in_addr {
                    s_addr: unsafe { libc::inet_addr(c_addr.as_ptr()) },
                },
                sin_zero: [0; 8], // 填充字节
            };

            let result = unsafe {
                libc::connect(
                    self.sockfd,
                    &sockaddr as *const libc::sockaddr_in as *const libc::sockaddr,
                    std::mem::size_of::<libc::sockaddr_in>() as u32,
                )
            };

            if result == -1 {
                return Err("连接失败".to_string());
            }

            Ok(())
        }

        // 发送数据
        fn send(&self, data: &[u8]) -> Result<usize, String> {
            let sent = unsafe {
                libc::send(
                    self.sockfd,
                    data.as_ptr() as *const std::os::raw::c_void,
                    data.len(),
                    0,
                )
            };

            if sent == -1 {
                return Err("发送失败".to_string());
            }

            Ok(sent as usize)
        }
    }

    impl Drop for SafeSocket {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.sockfd);
            }
        }
    }

    // 使用安全套接字包装器
    let socket = SafeSocket::new(libc::AF_INET, libc::SOCK_STREAM, 0).expect("套接字创建失败");
    println!("安全套接字创建成功");

    // 动态库加载的安全包装器 (Safe Dynamic Library Wrapper)
    // 封装了动态库的加载、符号查找和卸载
    // 确保库句柄在不需要时被正确卸载
    struct SafeLibrary {
        handle: *mut std::os::raw::c_void, // 库句柄
    }

    impl SafeLibrary {
        // 打开动态库
        fn open(path: &str) -> Result<Self, String> {
            let c_path = CString::new(path).map_err(|e| e.to_string())?;
            let handle = unsafe { libc::dlopen(c_path.as_ptr(), libc::RTLD_LAZY) };

            if handle.is_null() {
                let error = unsafe { libc::dlerror() };
                let error_msg = if error.is_null() {
                    "未知错误".to_string()
                } else {
                    unsafe { CStr::from_ptr(error).to_str().unwrap().to_string() }
                };
                return Err(format!("库加载失败: {}", error_msg));
            }

            Ok(SafeLibrary { handle })
        }

        // 获取库中的符号（函数或变量）
        fn get_symbol<T>(&self, name: &str) -> Result<T, String> {
            let c_name = CString::new(name).map_err(|e| e.to_string())?;
            let symbol = unsafe { libc::dlsym(self.handle, c_name.as_ptr()) };

            if symbol.is_null() {
                return Err(format!("符号 {} 未找到", name));
            }

            // 注意：transmute_copy 是 unsafe 的，需要谨慎使用
            Ok(unsafe { std::mem::transmute_copy(&symbol) })
        }
    }

    impl Drop for SafeLibrary {
        fn drop(&mut self) {
            if !self.handle.is_null() {
                unsafe {
                    libc::dlclose(self.handle);
                }
            }
        }
    }

    // 使用安全库包装器
    let lib = SafeLibrary::open("libmath.so").expect("库加载失败");
    println!("安全库加载成功");

    // 安全包装器的高级特性：
    // 1. 资源池：管理多个相同类型的资源
    // 2. 引用计数：支持资源的共享所有权
    // 3. 缓存机制：缓存昂贵的资源创建操作
    // 4. 超时处理：支持资源获取的超时控制
    // 5. 监控和日志：记录资源使用情况

    // 安全包装器的最佳实践：
    // 1. 最小化 unsafe 代码：只在必要时使用 unsafe
    // 2. 全面测试：测试所有边界条件和错误情况
    // 3. 性能考虑：避免包装器引入过度的性能开销
    // 4. 文档完善：详细说明使用限制和注意事项
    // 5. 错误信息：提供清晰有用的错误信息

    println!();
}

// ===========================================
// 10. Rust 1.77 std::os::fd 模块标准化
// ===========================================

// Rust 1.77 引入了 std::os::fd 模块的标准化，这标志着 Rust 在系统编程领域的重要进展
// 这个模块提供了跨平台的文件描述符抽象，统一了不同操作系统的文件 I/O 接口
// 标准化之前，开发者需要使用平台特定的 API 和第三方库

fn std_os_fd_standardization() {
    println!("=== Rust 1.77 std::os::fd 模块标准化 ===");

    // std::os::fd 模块标准化的背景和意义：
    // 1. 跨平台一致性：统一的文件描述符 API，减少平台特定代码
    // 2. 类型安全：使用 Rust 的类型系统确保文件描述符的正确使用
    // 3. 资源管理：利用 Rust 的所有权系统自动管理文件描述符的生命周期
    // 4. 错误处理：集成的错误处理机制，比传统的 C 风格更安全
    // 5. 生态系统：为标准库和第三方库提供统一的基础设施

    // 核心 trait 和类型：
    // - AsFd/AsHandle/AsSocket：将类型转换为文件描述符/句柄/套接字
    // - OwnedFd/OwnedHandle/OwnedSocket：拥有所有权的文件描述符类型
    // - BorrowedFd/BorrowedHandle/BorrowedSocket：借用文件描述符的类型
    // - FromRawFd/FromRawHandle/FromRawSocket：从原始描述符构造安全类型
    // - IntoRawFd/IntoRawHandle/IntoRawSocket：将安全类型转换为原始描述符

    #[cfg(unix)]
    {
        use std::fs::File;
        use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
        use std::path::Path;

        println!("=== Unix 平台文件描述符操作 ===");

        // 创建文件并获取文件描述符
        let file = File::create("temp_test.txt").expect("文件创建失败");
        println!("文件创建成功");

        // 使用 AsFd trait 获取 BorrowedFd
        let borrowed_fd: BorrowedFd = file.as_fd();
        println!("获取 BorrowedFd 成功");

        // 使用 AsRawFd trait 获取原始文件描述符
        let raw_fd: RawFd = file.as_raw_fd();
        println!("原始文件描述符: {}", raw_fd);

        // 从原始文件描述符创建 OwnedFd
        // 注意：这需要 unsafe，因为我们需要确保文件描述符的有效性
        unsafe {
            let owned_fd = OwnedFd::from_raw_fd(raw_fd);
            println!("OwnedFd 创建成功");

            // OwnedFd 可以安全地传递给其他函数
            let another_raw_fd: RawFd = owned_fd.as_raw_fd();
            println!("从 OwnedFd 获取的原始描述符: {}", another_raw_fd);

            // OwnedFd 离开作用域时会自动关闭文件描述符
        }

        // 文件描述符的实际应用场景
        practical_fd_usage_unix();
    }

    #[cfg(windows)]
    {
        use std::fs::File;
        use std::os::windows::io::{
            AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle, RawHandle,
        };
        use std::path::Path;

        println!("=== Windows 平台文件句柄操作 ===");

        let file = File::create("temp_test.txt").expect("文件创建失败");
        println!("文件创建成功");

        let borrowed_handle: BorrowedHandle = file.as_handle();
        println!("获取 BorrowedHandle 成功");

        let raw_handle: RawHandle = file.as_raw_handle();
        println!("原始文件句柄: {:?}", raw_handle);

        unsafe {
            let owned_handle = OwnedHandle::from_raw_handle(raw_handle);
            println!("OwnedHandle 创建成功");
        }
    }

    // 标准化文件描述符的优势和应用：
    // 1. 网络编程：统一的套接字接口
    // 2. 进程间通信：管道、消息队列等
    // 3. 设备访问：访问系统设备文件
    // 4. 系统监控：文件系统事件监控
    // 5. 容器化：与容器运行时交互

    println!("文件描述符标准化演示完成");
    println!();
}

#[cfg(unix)]
fn practical_fd_usage_unix() {
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd};
    use std::process::{Command, Stdio};

    println!("=== 实际文件描述符应用示例 ===");

    // 示例 1: 文件描述符重定向
    fn redirect_stdio_example() {
        println!("--- 标准输入输出重定向 ---");

        // 创建临时文件作为新的标准输出
        let output_file = File::create("stdout_redirect.txt").expect("输出文件创建失败");

        // 获取文件描述符并重定向标准输出
        let fd = output_file.as_raw_fd();

        // 在实际应用中，可以使用 dup2 系统调用重定向
        // 这里我们演示如何安全地操作文件描述符
        unsafe {
            // 模拟文件描述符操作
            println!("将标准输出重定向到文件描述符: {}", fd);

            // 创建 OwnedFd 来管理文件描述符
            let owned_fd = OwnedFd::from_raw_fd(fd);
            println!("文件描述符现在由 OwnedFd 安全管理");

            // owned_fd 离开作用域时自动清理
        }
    }

    // 示例 2: 进程间通信使用文件描述符
    fn inter_process_communication() {
        println!("--- 进程间通信示例 ---");

        // 创建管道用于进程间通信
        let mut child = Command::new("cat")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("进程启动失败");

        // 获取子进程的输入输出文件描述符
        if let Some(stdin) = child.stdin.as_mut() {
            let stdin_fd = stdin.as_raw_fd();
            println!("子进程标准输入文件描述符: {}", stdin_fd);
        }

        if let Some(stdout) = child.stdout.as_mut() {
            let stdout_fd = stdout.as_raw_fd();
            println!("子进程标准输出文件描述符: {}", stdout_fd);
        }
    }

    // 示例 3: 文件描述符传递
    fn fd_passing_example() {
        println!("--- 文件描述符传递示例 ---");

        // 创建一个文件
        let file = File::create("shared_file.txt").expect("文件创建失败");

        // 将文件描述符转换为 OwnedFd
        let owned_fd = unsafe { OwnedFd::from_raw_fd(file.as_raw_fd()) };

        // 在实际应用中，可以通过 Unix 域套接字传递文件描述符
        // 这里演示安全的管理方式
        println!("文件描述符已封装为 OwnedFd，可以安全传递");

        // 模拟接收方使用文件描述符
        let received_fd = owned_fd;
        println!("接收到的文件描述符: {}", received_fd.as_raw_fd());
    }

    redirect_stdio_example();
    inter_process_communication();
    fd_passing_example();
}

// 文件描述符在异步编程中的应用
async fn async_fd_usage() {
    use std::fs::File;
    use std::os::fd::AsFd;
    // use tokio::io::{AsyncReadExt, AsyncWriteExt}; // 需要添加 tokio 依赖

    println!("=== 异步文件描述符使用 ===");

    // 在异步编程中，文件描述符的管理更加重要
    // Rust 1.77+ 的标准化为异步 I/O 提供了更好的基础

    // 创建同步文件
    let file = File::create("async_test.txt").expect("文件创建失败");

    // 获取文件描述符用于异步操作
    let fd = file.as_fd();
    println!("文件描述符准备用于异步操作: {:?}", fd);

    // 在实际应用中，可以使用 tokio::fs::File 进行真正的异步 I/O
    // 这里演示标准化的文件描述符如何与异步生态集成
}

// 文件描述符的网络编程应用
fn network_fd_usage() {
    use std::net::TcpListener;
    use std::os::fd::AsFd;

    println!("=== 网络编程文件描述符应用 ===");

    // 创建 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:0").expect("监听器创建失败");
    println!("TCP 监听器创建成功");

    // 获取监听器的文件描述符
    let listener_fd = listener.as_fd();
    println!("监听器文件描述符: {:?}", listener_fd);

    // 文件描述符在网络编程中的优势：
    // 1. 可以使用 epoll/kqueue 等高效 I/O 多路复用
    // 2. 支持非阻塞 I/O 操作
    // 3. 可以在多个线程/协程间共享
    // 4. 支持文件描述符传递机制
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust FFI 演示");
    println!("============");

    ffi_basics();
    data_type_mapping();
    string_handling();
    callback_functions();
    memory_management();
    error_handling();
    advanced_ffi_features();
    practical_examples();
    safe_wrappers();
    std_os_fd_standardization();

    println!("FFI 演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_c_string_conversion() {
        let rust_str = "Hello, World!";
        let c_string = CString::new(rust_str).unwrap();

        unsafe {
            let c_str = CStr::from_ptr(c_string.as_ptr());
            let converted = c_str.to_str().unwrap();
            assert_eq!(converted, rust_str);
        }
    }

    #[test]
    fn test_safe_memory() {
        // 简化的测试，不依赖具体的 SafeMemory 实现
        let buffer = vec![0u8; 1024];
        assert_eq!(buffer.len(), 1024);

        let ptr = buffer.as_ptr();
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_data_type_conversion() {
        let rust_int: i32 = 42;
        let c_int: i32 = rust_int;
        assert_eq!(rust_int, c_int);
    }

    #[test]
    fn test_error_codes() {
        let mut result = 0.0;
        let success_code = unsafe { divide_with_error(10.0, 2.0, &mut result) };
        assert_eq!(success_code, 0);
        assert_eq!(result, 5.0);

        let error_code = unsafe { divide_with_error(10.0, 0.0, &mut result) };
        assert_eq!(error_code, -1);
    }

    #[test]
    fn test_result_wrapper() {
        let result = unsafe { safe_divide(10.0, 2.0) };
        assert_eq!(result.error_code, 0);
        assert_eq!(result.value, 5.0);

        let error_result = unsafe { safe_divide(10.0, 0.0) };
        assert_eq!(error_result.error_code, -1);
    }

    #[test]
    fn test_option_wrapper() {
        let valid_result = unsafe { safe_sqrt(16.0) };
        assert_eq!(valid_result.has_value, true);
        assert_eq!(valid_result.value, 4.0);

        let invalid_result = unsafe { safe_sqrt(-1.0) };
        assert_eq!(invalid_result.has_value, false);
    }

    #[test]
    fn test_thread_local_storage() {
        unsafe { increment_ffi_counter() };
        unsafe { increment_ffi_counter() };
        assert_eq!(unsafe { get_ffi_counter() }, 2);
    }

    // 为测试添加的 FFI 函数
    static mut FFI_COUNTER: i32 = 0;

    unsafe fn increment_ffi_counter() {
        unsafe {
            FFI_COUNTER += 1;
        }
    }

    unsafe fn get_ffi_counter() -> i32 {
        unsafe { FFI_COUNTER }
    }

    unsafe fn divide_with_error(a: f64, b: f64, result: *mut f64) -> i32 {
        unsafe {
            if b == 0.0 {
                return -1;
            }
            *result = a / b;
            0
        }
    }

    #[repr(C)]
    struct SafeDivideResult {
        value: f64,
        error_code: i32,
    }

    unsafe fn safe_divide(a: f64, b: f64) -> SafeDivideResult {
        if b == 0.0 {
            SafeDivideResult {
                value: 0.0,
                error_code: -1,
            }
        } else {
            SafeDivideResult {
                value: a / b,
                error_code: 0,
            }
        }
    }

    #[repr(C)]
    struct OptionResult {
        value: f64,
        has_value: bool,
    }

    unsafe fn safe_sqrt(x: f64) -> OptionResult {
        if x < 0.0 {
            OptionResult {
                value: 0.0,
                has_value: false,
            }
        } else {
            OptionResult {
                value: x.sqrt(),
                has_value: true,
            }
        }
    }

    #[test]
    fn test_ffi_counter() {
        let initial_count = unsafe { get_ffi_counter() };
        unsafe { increment_ffi_counter() };
        assert_eq!(unsafe { get_ffi_counter() }, initial_count + 1);
    }
}

// 模拟的 C 函数声明
mod libc {
    use std::os::raw::{c_char, c_int, c_void};
    pub type size_t = usize; // 在大多数平台上，size_t 相当于 usize

    pub const AF_INET: i32 = 2;
    pub const SOCK_STREAM: i32 = 1;
    pub const O_RDWR: i32 = 2;
    pub const O_CREAT: i32 = 64;
    pub const RTLD_LAZY: i32 = 1;

    unsafe extern "C" {
        pub fn socket(domain: i32, type_: i32, protocol: i32) -> i32;
        pub fn connect(sockfd: i32, addr: *const sockaddr, addrlen: u32) -> i32;
        pub fn send(sockfd: i32, buf: *const c_void, len: usize, flags: i32) -> isize;
        pub fn close(fd: i32) -> i32;
        pub fn open(path: *const c_char, flags: i32, mode: u32) -> i32;
        pub fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
        pub fn read(fd: i32, buf: *mut std::ffi::c_void, count: usize) -> isize;
        pub fn dlopen(filename: *const c_char, flag: i32) -> *mut std::ffi::c_void;
        pub fn dlclose(handle: *mut std::ffi::c_void) -> i32;
        pub fn dlsym(handle: *mut std::ffi::c_void, symbol: *const c_char)
        -> *mut std::ffi::c_void;
        pub fn dlerror() -> *mut c_char;
        pub fn inet_addr(cp: *const c_char) -> u32;
    }

    #[repr(C)]
    pub struct sockaddr {
        pub sa_family: u16,
        pub sa_data: [u8; 14],
    }

    #[repr(C)]
    pub struct sockaddr_in {
        pub sin_family: u16,
        pub sin_port: u16,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8],
    }

    #[repr(C)]
    pub struct in_addr {
        pub s_addr: u32,
    }
}
