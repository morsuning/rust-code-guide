// Rust 所有权系统
// 深入讲解所有权、借用、切片等 Rust 内存安全的核心机制

// ===========================================
// 1. 所有权规则 (Ownership Rules)
// ===========================================

// Rust 的所有权系统是其最独特的特性，也是保证内存安全的核心机制
// 通过编译时的所有权检查，Rust 消除了整个类的内存错误，包括：
// - 空指针解引用（null pointer dereferencing）
// - 悬垂指针（dangling pointers）
// - 数据竞争（data races）
// - 内存泄漏（memory leaks）
// - 迭代器失效（iterator invalidation）

fn ownership_rules() {
    println!("=== 所有权规则 ===");

    // 三大所有权规则（内存安全的基石）：
    // 规则 1: Rust 中的每个值都有一个被称为其所有者（owner）的变量
    // 规则 2: 值在任一时刻有且只有一个所有者
    // 规则 3: 当所有者离开作用域时，该值将被丢弃（内存被释放）

    // 字符串字面量（String Literal）
    // 这些字符串在编译时就被硬编码到程序的二进制文件中
    // 它们存储在程序的只读数据段中，具有 'static 生命周期
    let s = "hello";
    println!("字符串字面量: {}", s);
    // s 的类型是 &str，即字符串切片，指向程序二进制文件中的静态数据

    // String 类型：堆分配的动态字符串
    // String 类型用于存储在运行时才能确定大小的文本
    // 它在堆上分配内存，允许在运行时修改
    let mut s = String::from("hello");
    println!("初始字符串: {}", s);

    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("修改后的字符串: {}", s);

    // 内存布局理解：
    // - s 在栈上存储：指向堆内存的指针、长度、容量
    // - 实际的字符串内容 "hello, world!" 存储在堆上

    // 移动语义（Move Semantics）：所有权的转移
    // 这是 Rust 区别于其他语言的关键特性
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2

    // 此时发生了什么：
    // 1. s1 的数据（指针、长度、容量）被复制到 s2
    // 2. s1 被标记为无效，不再能使用
    // 3. 当 s2 离开作用域时，会自动释放堆内存

    // println!("{}", s1); // 编译错误：s1 不再有效（borrow of moved value）
    println!("移动后的字符串: {}", s2);

    // 移动语义的重要性：
    // - 避免了双重释放（double free）
    // - 确保了内存的唯一所有权
    // - 编译时就能发现错误，而不是运行时

    // 克隆（Clone）：显式的深度复制
    // 当确实需要复制数据时，必须显式调用 clone()
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 在堆上创建 s1 数据的完整副本

    println!("克隆的字符串: {} {}", s1, s2);
    // s1 和 s2 都有效，但它们指向不同的堆内存

    // 克隆的性能考虑：
    // - clone() 会复制所有堆数据，可能很昂贵
    // - 对于大数据结构，应考虑其他方案（如引用）
    // - 只在确实需要多个独立副本时使用

    // 栈数据的复制（Copy Trait）
    // 对于实现 Copy trait 的类型，赋值操作是复制而不是移动
    let x = 5;
    let y = x; // 栈数据会自动复制

    println!("栈数据复制: {} {}", x, y);
    // x 和 y 都有效，因为整数类型实现了 Copy trait

    // Copy trait 的特点：
    // - 只有在内存中可以按位复制的类型才能实现 Copy
    // - 如果类型或其任何部分实现了 Drop，则不能实现 Copy
    // - 实现 Copy 的类型：所有整数类型、bool、char、浮点数、元组（如果元素都实现了 Copy）

    // 移动 vs 复制的决策：
    // - 移动：用于堆分配数据（String、Vec等），避免昂贵的复制
    // - 复制：用于小的栈分配数据，复制成本低廉

    println!();
}

// ===========================================
// 2. 所有权与函数 (Ownership and Functions)
// ===========================================

// 函数与所有权的交互是 Rust 内存管理的重要组成部分
// 理解函数如何影响所有权对于编写安全的 Rust 代码至关重要

fn ownership_and_functions() {
    println!("=== 所有权与函数 ===");

    // 函数参数的所有权转移
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里

    // println!("{}", s);           // 编译错误：s 不再有效
    // 这是因为 String 类型没有实现 Copy trait，所以发生移动

    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 移动到函数里，但 i32 是 Copy 的
    println!("复制后的值: {}", x);   // x 仍然有效

    // 函数调用时的所有权变化：
    // 1. 传递给函数的变量会发生移动（如果类型没有实现 Copy）
    // 2. 函数返回值的所有权会转移给调用者
    // 3. 函数参数的所有权会影响变量的后续使用

    // 实际编程中的考虑：
    // - 如果还需要使用变量，考虑传递引用而不是所有权
    // - 如果函数确实需要拥有数据，确保这是设计意图
    // - 使用 Copy trait 对于小类型是安全的

    println!();
}

// 接收所有权的函数
// 这个函数获得 some_string 的所有权，当函数结束时，some_string 被丢弃
fn takes_ownership(some_string: String) {
    println!("获取所有权的字符串: {}", some_string);
    // 在这里可以使用 some_string
    // 当函数结束时，Rust 会自动调用 drop() 释放内存
} // some_string 离开作用域，内存被自动释放

// 接收副本的函数
// 对于实现了 Copy trait 的类型，参数会被复制而不是移动
fn makes_copy(some_integer: i32) {
    println!("获取复制的整数: {}", some_integer);
} // some_integer 离开作用域，不会做特殊处理（整数在栈上）

// ===========================================
// 3. 返回值与作用域 (Return Values and Scope)
// ===========================================

// 返回值是所有权转移的重要方式
// 理解返回值如何影响所有权对于函数设计很重要

fn return_values_and_scope() {
    println!("=== 返回值与作用域 ===");

    // 从函数返回所有权
    let s1 = gives_ownership();         // gives_ownership 将返回值移动给 s1

    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到函数中，
                                        // 函数返回值移动给 s3
    // println!("{}", s2);              // 编译错误：s2 不再有效

    println!("返回的字符串: {} {}", s1, s3);

    // 所有权转移的几种方式：
    // 1. 变量之间的赋值（移动）
    // 2. 传递给函数（移动）
    // 3. 从函数返回（移动）
    // 4. 其他方式将在后续章节讨论

    println!();
}

// 返回所有权的函数
// 函数创建 String 并将其所有权返回给调用者
fn gives_ownership() -> String {
    // some_string 进入作用域
    let some_string = String::from("yours");
    // some_string 被返回并移动出函数
    some_string
}

// 接收并返回所有权的函数
// 这个函数接收一个 String，修改它，然后返回所有权
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // a_string 被返回并移动出函数
}

// ===========================================
// 4. 引用与借用 (References and Borrowing)
// ===========================================

// 引用（References）和借用（Borrowing）是 Rust 避免不必要所有权转移的机制
// 通过引用，可以在不获取所有权的情况下访问数据

fn references_and_borrowing() {
    println!("=== 引用与借用 ===");

    // 基本引用概念
    let s1 = String::from("hello");

    // 创建引用：& 操作符创建对数据的引用
    let len = calculate_length(&s1);

    println!("字符串 '{}' 的长度是 {}", s1, len);
    // s1 仍然有效，因为我们只传递了引用而不是所有权

    // 引用的本质：
    // - 引用是内存地址的别名，类似于其他语言中的指针
    // - 引用保证指向有效的数据
    // - 引用的大小是固定的（通常是一个指针的大小）
    // - 引用不拥有数据，所以当引用离开作用域时不会丢弃数据

    // 可变引用（Mutable References）
    let mut s = String::from("hello");

    change_string(&mut s); // 传入可变引用
    println!("修改后的字符串: {}", s);

    // 可变引用的规则：
    // 1. 在同一作用域内，对特定数据只能有一个可变引用
    // 2. 不能在拥有可变引用的同时拥有不可变引用
    // 3. 引用必须始终有效（不能有悬垂引用）

    // 可变引用的限制示例：
    let mut s = String::from("hello");

    let r1 = &mut s; // 第一个可变引用
    // let r2 = &mut s; // 编译错误：不能有多个可变引用
    // println!("{}, {}", r1, r2);

    // 为什么需要这个限制？
    // 1. 防止数据竞争（data race）
    // 2. 确保编译时能确定访问模式
    // 3. 使代码更容易理解和推理

    // 可变和不可变引用的互斥性
    let mut s = String::from("hello");

    let r1 = &s; // 不可变引用
    // let r2 = &mut s; // 编译错误：不能在有不可变引用时创建可变引用

    println!("不可变引用: {}", r1);

    // 当 r1 不再使用后，可以创建可变引用
    let r2 = &mut s;
    r2.push_str(", world");
    println!("可变引用修改后: {}", r2);

    // 悬垂引用（Dangling References）的避免
    // Rust 编译器确保引用永远不会悬垂
    let reference_to_nothing = dangle();
    println!("安全返回的引用: {}", reference_to_nothing);

    // 引用的使用最佳实践：
    // 1. 优先使用不可变引用，只在必要时使用可变引用
    // 2. 保持引用的生命周期尽可能短
    // 3. 避免长时间持有引用
    // 4. 在复杂情况下考虑使用智能指针

    println!();
}

// 使用引用计算字符串长度
// 参数是 &String，表示对 String 的引用（借用的 String）
fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用，不是 String 本身
    // s 进入作用域，但由于它没有所有权，当 s 离开作用域时不会被丢弃

    let length = s.len(); // len() 方法返回字符串的长度

    length // 返回长度
    // s 离开作用域，但由于它没有所有权，不会发生任何事情
}

// 修改字符串的可变引用
fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}

// Rust 编译器会阻止悬垂引用的创建
// fn dangle() -> &String { // 编译错误：期望返回引用但实际返回了 String
//     let s = String::from("hello"); // s 进入作用域
//     &s // 返回 s 的引用
// } // s 离开作用域被丢弃，引用变成悬垂的

// 正确的引用返回方式
fn dangle() -> String {
    let s = String::from("hello");
    s // 直接返回 String，而不是引用
}

// ===========================================
// 5. 切片 (Slices)
// ===========================================

// 切片是一种不拥有所有权的数据类型
// 它允许你引用集合中连续的一部分元素

fn slices() {
    println!("=== 切片 ===");

    // 字符串切片
    let s = String::from("hello world");

    let hello = &s[0..5]; // [start..end] 包含 start，不包含 end
    let world = &s[6..11]; // 从索引 6 到 11
    let all = &s[..];      // 整个字符串的切片

    println!("字符串切片: {} {} {}", hello, world, all);

    // 切片的内存表示：
    // - 切片包含一个指向数据的指针和长度
    // - 不包含容量信息（因为切片不是所有者）
    // - 大小固定（通常两个机器字：指针 + 长度）

    // 切片的安全性：
    // Rust 会在编译时和运行时检查切片边界
    // let invalid = &s[0..20]; // 运行时 panic：超出字符串边界

    // 字符串字面量就是切片
    let literal = "hello world"; // 类型是 &str
    println!("字面量切片: {}", literal);

    // 切片的实用场景：查找单词
    let word = first_word(&s);
    println!("第一个单词: {}", word);

    // 切片的优势：
    // 1. 不需要所有权转移
    // 2. 避免了不必要的数据复制
    // 3. 提供了灵活的数据访问方式
    // 4. 编译时和运行时的边界检查

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
    println!("数组切片: {:?}", slice);

    // 切片与其他类型的区别：
    // - &String：字符串引用，包含长度和容量信息
    // - &str：字符串切片，只包含长度信息
    // - &[T]：数组切片，引用数组的一部分

    println!();
}

// 返回字符串第一个单词的切片
fn first_word(s: &str) -> &str {
    // 将输入转换为字节切片
    let bytes = s.as_bytes();

    // 使用 iter().enumerate() 创建迭代器，同时获取索引和字节
    for (i, &item) in bytes.iter().enumerate() {
        // 如果遇到空格，返回从开始到空格的切片
        if item == b' ' {
            return &s[0..i];
        }
    }

    // 如果没有空格，返回整个字符串
    &s[..]
}

// ===========================================
// 6. 生命周期 (Lifetimes)
// ===========================================

// 生命周期是 Rust 中另一个重要的概念
// 它确保引用始终有效，防止悬垂引用

fn lifetimes() {
    println!("=== 生命周期 ===");

    // 生命周期注解确保引用的有效性
    let string1 = String::from("长字符串");
    let string2 = "短字符串";

    let result = longest(&string1, string2);
    println!("较长的字符串: {}", result);

    // 生命周期的本质：
    // - 生命周期是引用有效性的范围
    // - 编译器使用生命周期来检查引用是否有效
    // - 大多数情况下，编译器可以推断生命周期（生命周期省略）

    // 生命周期省略规则（Lifetime Elision Rules）：
    // 1. 每个引用参数都有自己的生命周期参数
    // 2. 如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数
    // 3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
    //    那么 self 的生命周期被赋予所有输出生命周期参数

    // 生命周期的实际应用：
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("重要摘录: {:?}", i);

    // 静态生命周期：'static
    let static_str: &'static str = "这是一个静态生命周期的字符串";
    println!("静态生命周期字符串: {}", static_str);

    // 'static 生命周期：
    // - 表示引用在整个程序运行期间都有效
    // - 用于字符串字面量和全局变量
    // - 是最长的生命周期

    // 生命周期的最佳实践：
    // 1. 尽量让编译器推断生命周期
    // 2. 只在必要时才添加显式生命周期注解
    // 3. 保持生命周期尽可能短
    // 4. 在复杂函数签名中，使用生命周期注解提高代码清晰度

    println!();
}

// 带有显式生命周期注解的函数
// 生命周期 'a 表示两个引用和返回值的生命周期必须相同或更长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
// 如果结构体包含引用，必须为引用添加生命周期注解
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// ===========================================
// 7. 实际应用示例
// ===========================================

fn practical_examples() {
    println!("=== 实际应用示例 ===");

    // 示例 1：字符串处理函数
    // 使用切片和引用来避免不必要的所有权转移
    fn count_words(text: &str) -> usize {
        text.split_whitespace().count()
    }

    let text = "Hello world this is a test";
    let word_count = count_words(text);
    println!("文本 \"{}\" 有 {} 个单词", text, word_count);
    // text 仍然可以使用，因为我们只传递了引用

    // 示例 2：安全的字符串修改
    fn safe_append(original: &mut String, suffix: &str) {
        original.push_str(suffix);
    }

    let mut message = String::from("Hello");
    safe_append(&mut message, " World!");
    println!("安全修改后的字符串: {}", message);

    // 示例 3：函数返回引用
    fn find_number(numbers: &[i32], target: i32) -> Option<&i32> {
        numbers.iter().find(|&&num| num == target)
    }

    let numbers = [1, 2, 3, 4, 5];
    match find_number(&numbers, 3) {
        Some(&num) => println!("找到数字: {}", num),
        None => println!("未找到数字"),
    }

    // 示例 4：避免悬垂引用
    fn create_safe_string() -> String {
        String::from("安全字符串")
    }

    let safe_string = create_safe_string();
    println!("安全字符串: {}", safe_string);

    // 示例 5：生命周期实践
    fn get_longest_prefix<'a>(prefixes: &'a [&str], word: &'a str) -> Option<&'a str> {
        prefixes
            .iter()
            .copied() // <- 把 &&str 变成 &str
            .filter(|prefix| word.starts_with(prefix))
            .max_by_key(|prefix| prefix.len())
    }

    let prefixes = &["pre", "prefix", "pre"];
    let word = "prefix";

    if let Some(longest_prefix) = get_longest_prefix(prefixes, word) {
        println!("最长前缀: {}", longest_prefix);
    }

    // 实际编程中的所有权模式：
    // 1. 使用引用避免不必要的数据复制
    // 2. 在函数签名中明确表达所有权意图
    // 3. 合理使用可变和不可变引用
    // 4. 注意生命周期的影响

    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 所有权系统演示");
    println!("===================");

    ownership_rules();
    ownership_and_functions();
    return_values_and_scope();
    references_and_borrowing();
    slices();
    lifetimes();
    practical_examples();

    println!("所有权系统演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_rules() {
        let s1 = String::from("hello");
        let s2 = s1;
        // assert_eq!(s1, "hello"); // 编译错误：s1 已移动
        assert_eq!(s2, "hello");

        let x = 5;
        let y = x;
        assert_eq!(x, 5); // x 仍然有效
        assert_eq!(y, 5);
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        let length = calculate_length(&s);
        assert_eq!(length, 5);
        assert_eq!(s, "hello"); // s 仍然有效
    }

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        let word = first_word(&s);
        assert_eq!(word, "hello");
        assert_eq!(s, "hello world"); // s 仍然有效
    }

    #[test]
    fn test_longest() {
        let s1 = "long string";
        let s2 = "short";
        let result = longest(s1, s2);
        assert_eq!(result, s1);
    }

    #[test]
    fn test_count_words() {
        let text = "Hello world this is a test";
        let count = count_words(text);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_find_number() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(find_number(&numbers, 3), Some(&3));
        assert_eq!(find_number(&numbers, 10), None);
    }

    #[test]
    fn test_struct_with_lifetime() {
        let novel = String::from("Call me Ishmael.");
        let excerpt = ImportantExcerpt {
            part: novel.split('.').next().unwrap(),
        };
        assert_eq!(excerpt.part, "Call me Ishmael");
    }
}

// 辅助函数
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn find_number(numbers: &[i32], target: i32) -> Option<&i32> {
    numbers.iter().find(|&&num| num == target)
}