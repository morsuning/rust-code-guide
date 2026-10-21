// 教程里的类型、字段和方法经常只为演示语法而定义，不一定会被全部读取或调用，所以允许 dead_code；
// 其他 lint（未使用的导入、变量、可变性等）必须保持干净，不要再给模块添加新的 allow。
#![allow(dead_code)]

// Rust 进阶参考（Advanced Reference）
// 本章收纳「低频、偏底层」的语言与标准库特性：裸指针与内存布局、ABI 与 FFI 细节、
// 内联汇编、全局分配器，以及工具链 / 构建配置。
//
// 为什么单独成章？
// 前 18 章讲的是写业务代码时天天用到的东西；这一章里的特性有三个共同点——
// 1. 多数需要 `unsafe`，或者只在特定平台、特定场景下才有意义
// 2. 日常业务代码几乎碰不到，但读标准库源码、写底层库、对接 C 代码、排查链接错误时会反复遇到
// 3. 版本迭代里它们的更新很频繁，如果散落在各个主题章里，会让新手误以为“这些也是必修内容”
//
// 怎么读这一章：
// - 建议学完前面的章节再来，第一遍通读一遍即可，不必逐节啃
// - 每一节自成一体，遇到问题再回来查
// - 凡是标了版本号的特性，都用本机编译器验证过；仍未稳定的特性只以注释形式出现，并标明“需要 nightly”

use std::ffi::{VaList, c_double, c_int};

// ===========================================
// 1. 指针的地址与来源：严格来源 API 与按字节偏移（Rust 1.84 / 1.75）
// ===========================================

// 一个指针不只是一个数字。在 Rust 的内存模型里，指针 = 地址（address）+ 来源（provenance）：
// - 地址：它指向内存里的哪个位置，也就是打印出来的那个数字
// - 来源：这个指针“有权访问”哪一块分配（哪个变量、哪块堆内存）。
//   编译器靠它做别名分析：来源不同的两个指针，即使地址碰巧相同，也可以被当作互不相干。
//
// 老办法 `ptr as usize` / `addr as *mut T` 把两件事搅在一起：来源“藏”在整数里，
// 整数变回指针时，没人说得清这个指针该继承哪一块分配的来源。
// 这是 unsafe 代码里最难推理的灰色地带之一，也是 Miri 等检测工具最常报错的地方。
//
// 严格来源（strict provenance）API 把两件事拆开，让你明确表态：
// - `ptr.addr()`：只读地址，来源被丢弃
// - `ptr.with_addr(addr)`：沿用 `ptr` 的来源，换成新地址
// - `ptr.map_addr(f)`：对地址做一次变换，来源保持不变（最常用）
// - `ptr.expose_provenance()` 与 `ptr::with_exposed_provenance(addr)`：
//   明确承认“我就是要把指针变成整数再变回来”，这是不得不走整数中转时的正规写法
// - `ptr::without_provenance(addr)`：造一个没有任何来源的指针，只能比较、不能解引用
//
// 另外，`byte_add` / `byte_sub` / `byte_offset` / `byte_offset_from`（1.75）让指针直接按**字节**偏移，
// 不用先 `cast::<u8>()` 再转回来。

fn pointer_provenance_and_byte_offsets() {
    println!("=== 指针的地址与来源：严格来源 API 与按字节偏移（Rust 1.84 / 1.75） ===");

    // ---------- 典型场景：在指针的低位里藏一个标记 ----------
    //
    // 8 字节对齐的指针，地址的低 3 位一定是 0，这 3 位可以拿来存状态标记（比如“已删除”“有锁”）。
    // 无锁数据结构和高性能运行时经常这样省内存。
    // 正确写法是用 `map_addr`：只改地址，来源原封不动，所以改完的指针依然能指向原来那块分配。
    #[repr(align(8))]
    struct Node(u64);
    const TAG_MASK: usize = 0b111;

    let raw: *mut Node = Box::into_raw(Box::new(Node(42)));
    assert_eq!(raw.addr() & TAG_MASK, 0); // 对齐保证了低 3 位为 0
    let tagged = raw.map_addr(|addr| addr | 0b101); // 写入标记 0b101
    println!("指针里藏的标记 = {:#05b}", tagged.addr() & TAG_MASK);
    // 取回数据前必须先清掉标记，否则这个指针没有对齐，不能解引用
    let clean = tagged.map_addr(|addr| addr & !TAG_MASK);
    // SAFETY: clean 与 raw 地址相同、来源相同，指向 Box::new 的那次分配，并且只收回这一次。
    let node = unsafe { Box::from_raw(clean) };
    println!("清掉标记后读到的数据 = {}", node.0);

    // ---------- 不得不走整数中转：expose / with_exposed ----------
    //
    // 有些场景必须把指针变成整数——比如把地址当作“句柄”交给 C 代码、写进日志、塞进 u64 字段。
    // 这时请用 `expose_provenance` 取整数，再用 `with_exposed_provenance` 取回：
    // 前者在编译器那里登记“这个来源之后可能会被整数借走”，后者从登记过的来源里挑一个。
    let value = 7_u32;
    let handle = (&value as *const u32).expose_provenance();
    let back = std::ptr::with_exposed_provenance::<u32>(handle);
    // SAFETY: handle 来自指向 value 的指针，value 仍然存活。
    println!("经由整数句柄读回: {}", unsafe { *back });

    // 没有来源的指针只携带一个地址：可以比较、可以做地址运算，但解引用它是未定义行为。
    // `ptr::dangling()` 等价于“对齐值作为地址、没有来源”，常用来做 `Vec` 容量为 0 时的占位。
    let fake: *const u32 = std::ptr::without_provenance(0x1000);
    println!("无来源指针的地址 = {:#x}（不能解引用）", fake.addr());

    // ---------- 按字节偏移：行步长不等于元素大小 ----------
    //
    // 图像、纹理、GPU 映射内存里，“一行”占多少字节（stride）常常比“像素个数 × 像素大小”更大，
    // 因为每行末尾要补齐对齐。这时偏移量是以字节计的，与元素类型无关。
    // `ptr.add(n)` 的单位是元素个数，用起来要自己换算；`byte_add` 直接按字节走，指针类型保持不变。
    let image: [u32; 8] = [1, 2, 3, 0, 4, 5, 6, 0]; // 两行，每行 3 个像素 + 1 个填充
    let stride_bytes = 4 * size_of::<u32>(); // 一行占 16 字节
    let first_row: *const u32 = image.as_ptr();
    // SAFETY: 偏移后的地址仍在 image 这一块分配之内。
    let second_row = unsafe { first_row.byte_add(stride_bytes) };
    println!(
        "第二行第一个像素 = {}，两行相距 {} 字节",
        unsafe { *second_row },
        unsafe { second_row.byte_offset_from(first_row) }
    );

    // 小结：
    // 1. 想改指针的地址，用 `map_addr` / `with_addr`，别写 `as usize` 再 `as *mut T`
    // 2. 必须走整数时，成对使用 `expose_provenance` 与 `with_exposed_provenance`
    // 3. 偏移量以字节计时，直接用 `byte_*`，不要手动 cast 成 `*const u8`
    println!();
}

// ===========================================
// 2. AtomicPtr 的指针运算与位运算（Rust 1.91）
// ===========================================

// `AtomicPtr<T>` 以前只有 load / store / swap / compare_exchange。
// 想原子地“让指针前进一格”，必须自己写 compare_exchange 循环：
//     loop {
//         let old = pointer.load(..);
//         if pointer.compare_exchange(old, old.wrapping_add(1), ..).is_ok() { break old; }
//     }
// 1.91 起标准库直接提供了原子的指针运算和位运算，一次完成“读取旧值并修改”，返回修改之前的指针：
// - `fetch_ptr_add` / `fetch_ptr_sub`：以 `T` 的大小为单位前进 / 后退
// - `fetch_byte_add` / `fetch_byte_sub`：以字节为单位
// - `fetch_or` / `fetch_and` / `fetch_xor`：对指针的**地址**做位运算（指针的来源保持不变）
// 它们都是“回绕”运算，本身不检查是否越界；越过缓冲区之后再解引用，仍然是未定义行为，这一点要靠你自己保证。
//
// 两个典型场景：
// 1. 无锁地“认领下一个槽位”：多个线程共享一个游标（指向预先分配好的缓冲区），
//    每个线程 `fetch_ptr_add(1)` 就拿到一个属于自己的槽位，互不重叠，不需要锁
// 2. 无锁数据结构里用指针的低位当标记（比如“这个节点已逻辑删除”）：`fetch_or(1)` 一步完成标记

fn atomic_pointer_arithmetic() {
    println!("=== AtomicPtr 的指针运算与位运算（Rust 1.91） ===");

    use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
    use std::thread;

    // ---------- 认领槽位 ----------
    // 预先分配好的缓冲区，每个槽位记录“是哪个线程认领的”
    let slots: [AtomicU32; 8] = std::array::from_fn(|_| AtomicU32::new(0));
    // 游标指向下一个可认领的槽位
    let cursor = AtomicPtr::new(slots.as_ptr().cast_mut());
    let cursor_ref = &cursor;

    thread::scope(|scope| {
        for worker in 1..=8_u32 {
            scope.spawn(move || {
                // 原子地把游标前进一格，并返回前进之前指向的槽位：每个线程拿到的槽位各不相同
                let claimed = cursor_ref.fetch_ptr_add(1, Ordering::Relaxed);
                // SAFETY: 总共只有 8 个线程，游标起点就在 slots 的开头，最多前进 8 次，
                // 所以认领到的槽位都在 slots 之内，并且互不重叠。
                unsafe { (*claimed).store(worker, Ordering::Relaxed) };
            });
        }
    });

    let mut owners: Vec<u32> = slots.iter().map(|slot| slot.load(Ordering::Relaxed)).collect();
    owners.sort();
    println!("8 个槽位的认领者（排序后）: {owners:?}");
    // SAFETY: 游标和 slots.as_ptr() 指向同一个数组，游标最多指到数组末尾之后一格。
    let advanced = unsafe { cursor.load(Ordering::Relaxed).offset_from(slots.as_ptr()) };
    println!("游标一共前进了 {advanced} 格");

    // ---------- 用地址低位当标记 ----------
    // 8 字节对齐的节点，地址的低 3 位一定是 0，可以拿来存标记
    #[repr(align(8))]
    struct Node(u32);
    let node = Node(7);
    let shared = AtomicPtr::new((&node as *const Node).cast_mut());
    let before = shared.fetch_or(0b1, Ordering::AcqRel); // 设置标记位，返回设置之前的指针
    let marked = shared.load(Ordering::Acquire);
    println!("设置前标记位 = {}，设置后标记位 = {}", before.addr() & 1, marked.addr() & 1);
    // 解引用之前必须先清掉标记，否则指针没有对齐
    let clean = marked.map_addr(|addr| addr & !1);
    // SAFETY: clean 与原指针地址相同，指向仍然存活的 node，这里只读取。
    println!("清掉标记后读到节点的值: {}", unsafe { (*clean).0 });

    // 最佳实践：
    // 1. 需要“原子地前进 / 后退一格”时，用 fetch_ptr_add / fetch_ptr_sub，不要手写 compare_exchange 循环
    // 2. 这些运算不检查边界：缓冲区的容量必须由你另外保证，或者在认领之后检查游标是否越界
    // 3. 能用 `AtomicUsize` 表示下标就用下标，只有确实需要指针时才用 AtomicPtr
    println!();
}

// ===========================================
// 3. 字段偏移：offset_of!（Rust 1.77，嵌套字段 1.82）
// ===========================================

// 问题：想知道结构体的某个字段距离结构体起点多少字节。
// 过去的“标准写法”要凭空造一个未初始化的值，再用指针相减，既啰嗦又容易踩进未定义行为。
// `std::mem::offset_of!(类型, 字段)` 在编译期直接给出答案：返回 `usize` 常量，不需要实例，也不需要 unsafe。
//
// 典型用途：
// 1. 校验与 C 头文件约定的布局一致（编译期断言，布局一变就编译失败）
// 2. 已知字段指针反推整个结构体指针（C 里的 `container_of`），侵入式链表常用
// 3. 手写二进制协议的读写，按偏移取字段
//
// 注意：只有 `#[repr(C)]` 的布局是语言规定的；默认的 `repr(Rust)` 允许编译器重排字段，
// `offset_of!` 虽然仍能给出当前的真实值，但不能把这个值写死到别处。

fn field_offsets_with_offset_of() {
    println!("=== 字段偏移：offset_of!（Rust 1.77，嵌套字段 1.82） ===");

    use std::mem::offset_of;

    #[repr(C)]
    struct Header {
        magic: u32,
        version: u16,
        flags: u16,
    }
    #[repr(C)]
    struct Packet {
        header: Header,
        length: u32,
        checksum: u64,
    }

    // 编译期布局断言：与协议文档约定的偏移不一致，立刻编译失败，而不是等到运行时数据错乱
    const _: () = assert!(offset_of!(Packet, length) == 8);
    // 嵌套字段（1.82 起）：直接写路径 `header.version`
    println!("header.version 的偏移 = {}", offset_of!(Packet, header.version));
    // checksum 是 u64，要求 8 字节对齐，所以 length 之后补了 4 字节填充，偏移是 16 而不是 12
    println!("checksum 的偏移 = {}，整个结构体 {} 字节", offset_of!(Packet, checksum), size_of::<Packet>());

    // ---------- 配合 byte_add：由结构体指针取字段指针，再反推回来 ----------
    let packet = Packet {
        header: Header { magic: 0xCAFE_BABE, version: 2, flags: 1 },
        length: 128,
        checksum: 0xAB,
    };
    // 注意要从“整个结构体的裸指针”出发。用 `&raw const` 取指针，不会制造中间引用。
    let base: *const Packet = &raw const packet;
    // SAFETY: 偏移量来自 offset_of!，结果落在 packet 内部；字段类型与 cast 的目标类型一致。
    let length_ptr = unsafe { base.byte_add(offset_of!(Packet, length)).cast::<u32>() };
    println!("按偏移读出 length = {}", unsafe { length_ptr.read() });

    // 反推（container_of）：已知字段地址，减去偏移量就是整个结构体的地址
    // SAFETY: length_ptr 由 base 派生而来，减回同样的偏移量，回到 packet 的起点。
    let recovered = unsafe { length_ptr.byte_sub(offset_of!(Packet, length)).cast::<Packet>() };
    println!("反推出的结构体指针与原指针相同: {}", std::ptr::eq(recovered, base));
    // 为什么一定要从整个结构体的指针出发？因为字段指针的“来源”可能只覆盖那一个字段：
    // 如果指针来自 `&packet.length` 这样的字段引用，再用它反推并访问整个结构体，
    // 在 Miri 等工具使用的别名模型里会被判为未定义行为。
    // 链表节点这类场景里，指向字段的指针本来就是从整个节点的指针派生出来的，所以是安全的。

    // 小结：
    // 1. 布局约定写成 `const _: () = assert!(offset_of!(..) == ..)`，让编译器替你把关
    // 2. 用到偏移的结构体请标 `#[repr(C)]`，否则偏移不是语言保证的
    // 3. 反推结构体指针时，字段指针必须派生自整个结构体的指针
    println!();
}

// ===========================================
// 4. #[repr(transparent)]：与内部字段布局完全相同的透明包装器（Rust 1.28）
// ===========================================

// `#[repr(transparent)]` 早已稳定（struct 自 1.28 起，单变体枚举自 1.42 起），
// 它让结构体在内存布局上与其内部的唯一字段完全相同，
// 为 FFI 和类型安全的封装提供了更好的支持。
// 本节介绍它的用途与限制。

fn repr_transparent_structs() {
    println!("=== #[repr(transparent)] 透明包装器 ===");

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
        #[allow(
            dead_code,
            unused_variables,
            unused_imports,
            unused_mut,
            unused_assignments
        )]
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
// 5. 从裸指针查询布局，以及几处底层小改进（Rust 1.98-1.99）
// ===========================================

// 这一节补齐和底层开发（unsafe、内存布局、内联汇编）相关的几处改进：
// - 不经过引用，直接从裸指针查询大小/对齐/布局（1.99）：
//   `mem::size_of_val_raw` / `mem::align_of_val_raw` / `Layout::for_value_raw`
// - `mem::transmute_copy` 的源类型放宽到动态大小类型（`Src: ?Sized`，1.99）
// - unsize 转换时允许缩短 `&mut` 的生命周期，即使它处在“不变”的位置（1.98）
// - `asm!` 在 x86 上可以通过向量寄存器传递 128 位整数（1.99）
// - 新增 allow-by-default 的 lint `raw_borrows_via_references`（1.99）
//
// 这些都属于“让 unsafe 代码少踩坑”的基础设施改进：
// 日常业务代码几乎不会直接用到，但读标准库和写底层库时会反复遇到。

fn layout_queries_and_low_level_tweaks() {
    println!("=== 从裸指针查询布局，以及几处底层小改进（Rust 1.98-1.99） ===");

    use std::alloc::Layout;
    use std::mem::{align_of_val_raw, size_of_val_raw};

    // ---------- 从裸指针查询布局（Rust 1.99） ----------
    //
    // 背景：安全的 `size_of_val(&x)` 需要一个**引用**。
    // 对动态大小类型（DST）——切片 `[T]`、`str`、`dyn Trait`——大小要靠“胖指针”里携带的元数据算出来
    // （切片是长度，trait 对象是 vtable）。
    // 但在 unsafe 代码里，你经常手里只有一个裸指针：它指向的内存可能还没初始化，
    // 也可能别处正在修改。把它转成引用只为了问一句“多大”，这个转换本身就可能是未定义行为——
    // 制造引用意味着向编译器承诺“这块内存有效，且符合别名规则”。
    //
    // 1.99 稳定了不需要引用的版本，三个函数都是 `unsafe` 的 const fn：
    // - `size_of_val_raw(ptr)` / `align_of_val_raw(ptr)`：大小与对齐
    // - `Layout::for_value_raw(ptr)`：直接给出完整的 `Layout`
    // 它们的安全要求比“转成引用”宽松：对 `Sized` 类型总是安全；
    // 对切片、`str`、trait 对象，只要求“整个值的大小不超过 isize::MAX”；
    // 凡是能安全转成共享引用的指针，调用它们一定也安全。
    let numbers: [u32; 5] = [1, 2, 3, 4, 5];
    let slice_ptr: *const [u32] = &numbers[..3]; // 胖指针：地址 + 长度 3
    // SAFETY: slice_ptr 指向有效的切片，3 个 u32 的大小远小于 isize::MAX。
    let (size, align) = unsafe { (size_of_val_raw(slice_ptr), align_of_val_raw(slice_ptr)) };
    println!("切片指针指向的 3 个 u32: 大小 = {size} 字节，对齐 = {align} 字节");

    // 典型用途：手动释放一个 `Box<dyn Trait>`。
    // 同样是 `*mut dyn Shape` 这个类型，指向 `Circle` 还是 `Rect`，大小并不相同，
    // 这个信息藏在 vtable 里；释放内存时必须用**正确的** `Layout`，所以要先查询。
    trait Shape {
        fn area(&self) -> f64;
    }
    struct Circle(f64);
    struct Rect {
        w: f64,
        h: f64,
    }
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.0 * self.0
        }
    }
    impl Shape for Rect {
        fn area(&self) -> f64 {
            self.w * self.h
        }
    }

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle(1.0)), Box::new(Rect { w: 2.0, h: 3.0 })];
    for boxed in shapes {
        let raw: *mut dyn Shape = Box::into_raw(boxed); // 放弃 Box 的自动释放，改由我们手动负责
        // SAFETY: raw 刚由 into_raw 得到，指向一个有效且已初始化的值。
        let (size, area) = unsafe { (size_of_val_raw(raw), (*raw).area()) };
        println!("trait 对象实际占 {size} 字节，面积 = {area:.2}");
        // SAFETY: raw 来自 into_raw，只在这里释放一次；layout 与分配时一致。
        unsafe {
            let layout = Layout::for_value_raw(raw); // 先查布局（析构之后 vtable 信息仍在指针里）
            std::ptr::drop_in_place(raw); // 再运行析构函数
            if layout.size() != 0 {
                // 大小为 0 的类型 Box 根本不会分配，没有东西可以释放
                std::alloc::dealloc(raw as *mut u8, layout);
            }
        }
    }

    // ---------- transmute_copy 放宽到 ?Sized（Rust 1.99） ----------
    //
    // `mem::transmute_copy::<Src, Dst>(&src)` 按字节复制 `src` 开头的 `size_of::<Dst>()` 个字节，
    // 并把它们重新解释成 `Dst`。与 `transmute` 不同，它不要求两个类型大小相同，
    // 只要求 `Dst` 不比 `Src` 大。1.99 起，`Src` 可以是切片、`str` 这类动态大小类型，
    // 于是可以直接从字节切片里“读出”一个整数：
    let bytes: &[u8] = &[0x78, 0x56, 0x34, 0x12, 0xFF];
    // SAFETY: 切片至少有 4 字节；u32 的任何位模式都是合法值。
    let word: u32 = unsafe { std::mem::transmute_copy::<[u8], u32>(bytes) };
    println!("从字节切片按本机字节序读出 u32: {word:#x}（小端机器上是 0x12345678）");
    // 如果 `Dst` 比 `Src` 还大，就会读到范围之外的内存，属于库层面的未定义行为：
    // 在开启 debug assertions 的构建（默认的 dev profile）里会被检查出来并 panic
    // （1.99 起这个 panic 不再展开，因为它是不可恢复的契约违反），release 构建则不检查。
    // 实际工程里读字节通常更推荐 `u32::from_le_bytes` / `from_ne_bytes`，它们完全是安全代码。

    // ---------- unsize 转换时缩短 &mut 的生命周期（Rust 1.98） ----------
    //
    // `Cell<T>` 在 `T` 上是**不变**的：一般来说，`Cell<&'long mut i32>`
    // 不能当成 `Cell<&'short mut i32>` 使用，否则可以往里塞进一个短命的引用，
    // 而外面还以为里面装的是长命的引用，这会造成悬垂引用。
    // 但 unsize 转换（`&mut i32` → `&mut dyn Send`）本来就会按值“消耗”旧的 Cell、产生一个新值，
    // 1.98 起编译器允许在这个转换里顺便把 `&mut` 的生命周期缩短，哪怕它处在不变的位置：
    use std::cell::Cell;
    fn shorten<'long: 'short, 'short>(cell: Cell<&'long mut i32>) -> Cell<&'short mut dyn Send> {
        cell // 转换同时完成两件事：i32 -> dyn Send，以及 'long -> 'short
    }
    let mut value = 7;
    let _shortened = shorten(Cell::new(&mut value));
    println!("Cell<&'long mut i32> -> Cell<&'short mut dyn Send> 转换成功（这在 1.98 之前无法编译）");

    // ---------- asm! 通过向量寄存器传递 128 位整数（Rust 1.99，仅 x86） ----------
    //
    // x86 的 `xmm` 寄存器恰好是 128 位。1.99 起，`u128` / `i128` 可以直接作为 `xmm_reg` 操作数。
    // 下面用 `pxor`（128 位按位异或）演示。其他架构没有 xmm 寄存器，所以用 cfg 排除。
    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::asm;
        let a: u128 = 0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000;
        let b: u128 = 0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF;
        let out: u128;
        // SAFETY: pxor 只在寄存器之间做按位异或，不访问内存，也没有副作用。
        unsafe {
            asm!(
                "pxor {0}, {1}",
                inout(xmm_reg) a => out,
                in(xmm_reg) b,
                options(pure, nomem, nostack)
            );
        }
        println!("经由 xmm 寄存器做 128 位异或: {out:#034x}");
    }
    #[cfg(not(target_arch = "x86_64"))]
    println!("当前平台不是 x86_64，跳过 asm! 的 128 位向量寄存器示例");

    // ---------- raw_borrows_via_references lint（Rust 1.99） ----------
    //
    // 这是一个默认关闭（allow-by-default）的 lint，需要时用 `#![warn(raw_borrows_via_references)]` 打开。
    // 它检查“先取引用、立刻转成裸指针”的写法，例如 `&mut x as *mut T`：
    // 中间那个引用会带来别名要求（比如让此前派生出的其他裸指针失效）。
    // 推荐直接用 `&raw const` / `&raw mut`（1.82 起稳定），全程不制造引用：
    let mut counter = 0_u32;
    let ptr = &raw mut counter;
    // SAFETY: ptr 指向存活的局部变量，期间没有别的引用在使用它。
    unsafe {
        ptr.write(41);
        *ptr += 1;
    }
    println!("经由 &raw mut 修改后的 counter = {counter}");

    // 小结：
    // 1. 手里只有裸指针时，用 `*_val_raw` 查布局，而不是先转成引用——后者本身就可能是 UB
    // 2. `transmute_copy` 能读切片了，但读字节优先用安全的 `from_*_bytes`
    // 3. `&mut` 的生命周期缩短现在可以和 unsize 转换同时发生
    // 4. 想取裸指针时，直接 `&raw`，别绕一道引用
    println!();
}


// ===========================================
// 6. Box、Vec 与裸指针的往返，以及 ManuallyDrop 与 UnsafeCell（Rust 1.98-1.99）
// ===========================================

// 围绕“智能指针和裸指针之间如何安全地来回转换”补充了不少内容：
// - `Box::as_ptr` / `Box::as_mut_ptr`（1.98）——不消耗 Box、也不制造中间引用就拿到裸指针
// - `ManuallyDrop<Box<T>>` 在被手动 drop 之后再移动不是未定义行为（1.98，文档明确保证）
// - `Box::into_non_null` / `Box::from_non_null`（1.99）——用 `NonNull<T>` 代替 `*mut T` 做往返
// - `Vec::into_raw_parts` / `String::into_raw_parts`（1.93）与 `Vec::into_parts` / `Vec::from_parts`（1.99）——
//   把 `Vec` / `String` 拆成指针 + 长度 + 容量，再原样组装回来（后者用 `NonNull`，类型里就保证非空）
// - 官方建议不要对 `Box::leak` 的结果“再收回”（1.99）；同时放宽了 `UnsafeCell` 的别名规则：
//   内容可以不经过 `get()` 直接访问（1.99）
//
// 这一节主要是“认识这些工具长什么样、各自解决什么问题”，
// 大部分操作都需要 unsafe，日常业务代码里很少直接用到；
// 但读标准库、读 FFI 绑定、写自己的数据结构（链表、arena）时，这些概念会反复出现。

fn pointer_roundtrips_and_unsafe_cell() {
    println!("=== Box、Vec 与裸指针的往返，以及 ManuallyDrop 与 UnsafeCell（Rust 1.98-1.99） ===");

    use std::cell::UnsafeCell;
    use std::mem::ManuallyDrop;
    use std::ptr::NonNull;

    // ---------- Box::into_non_null / Box::from_non_null（Rust 1.99） ----------
    //
    // 想把 Box 的所有权“交出去”（比如交给 C 代码，或者存进自己写的链表节点），
    // 一直以来的做法是 `Box::into_raw(b)` 得到 `*mut T`，之后用 `Box::from_raw` 收回。
    // 1.99 新增了 `NonNull<T>` 版本：
    // - `NonNull<T>` 在类型层面保证不是空指针，省去到处写 `is_null()` 判断
    // - `Option<NonNull<T>>` 与裸指针一样大（空指针优化），用来表示“可能没有下一个节点”最合适
    // - 链表、树这类自引用数据结构的节点指针，标准库自己也都用 `NonNull`
    //
    // 约定和 `into_raw` 完全一样：调用 `into_non_null` 之后，Box 不会再自动释放内存，
    // 你必须恰好调用一次 `from_non_null` 把它收回来（或者自己负责释放），否则就是内存泄漏。
    let boxed = Box::new(String::from("堆上的数据"));
    let handle: NonNull<String> = Box::into_non_null(boxed); // 注意是关联函数，不是方法
    let copy_of_handle = handle; // NonNull 是 Copy，可以随便传递，但“释放”只能发生一次
    // SAFETY: handle 来自 into_non_null，尚未被收回过，且这里是唯一一次收回。
    let recovered = unsafe { Box::from_non_null(copy_of_handle) };
    println!("经由 NonNull 往返之后: {recovered}");

    // ---------- Vec::into_raw_parts / String::into_raw_parts（Rust 1.93） ----------
    //
    // 它们把 Vec / String 拆成三部分：指针 `ptr`、当前长度 `len`、当前容量 `cap`。
    // 这在 FFI 中非常常见，因为 C / C++ 的 API 往往只认这些原始部件。
    // 重要理解：调用 `into_raw_parts` 之后，原来的 Vec 不再由 Rust 自动回收，因为你已经把堆分配“拆走”了。
    // 下面立刻用 `from_raw_parts` 重建回来，是为了演示一个安全的闭环；
    // 实际项目里，这一步可能发生在外部库回调回来之后，或者在资源释放阶段。
    let numbers = vec![10_i32, 20, 30];
    let (ptr, len, cap) = numbers.into_raw_parts();
    // SAFETY: 三个部件原封不动地来自同一个 Vec。
    let rebuilt = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("Vec::into_raw_parts 往返结果: {rebuilt:?}");

    // `String::into_raw_parts` 的思路完全一样：String 本质上就是一块 UTF-8 字节缓冲区。
    // 常见场景：把字符串缓冲区交给底层系统去填写；与要求裸缓冲区的库对接；自己管理分配与释放的边界。
    let message = String::from("ffi-buffer");
    let (ptr, len, cap) = message.into_raw_parts();
    // SAFETY: 同上，部件来自同一个 String。
    let rebuilt = unsafe { String::from_raw_parts(ptr, len, cap) };
    println!("String::into_raw_parts 往返结果: {rebuilt}");
    // 对新手最关键的提醒：`from_raw_parts` 必须和原始分配方式严格匹配，
    // 指针、长度、容量只要有任何不一致，就是未定义行为。

    // ---------- Vec::into_parts / from_parts（Rust 1.99） ----------
    //
    // Rust 1.93 的 `into_raw_parts` 返回裸指针 `*mut T`；
    // 1.99 的 `into_parts` 返回 `NonNull<T>`，类型本身就保证“非空”：
    // - 不会误把空指针传回去
    // - `Option<NonNull<T>>` 与指针等大小（空指针优化），便于放进 FFI 或自定义容器
    // 取出三个部件之后，缓冲区由你负责：必须用 `from_parts` 还原（或手动释放），否则就是内存泄漏。
    let numbers = vec![1_u32, 2, 3];
    let (ptr, len, cap) = numbers.into_parts(); // numbers 已被消耗
    println!("拆成部件: len = {len}, cap >= len: {}", cap >= len);
    // SAFETY: 三个部件原封不动地来自同一个 Vec，指针、长度、容量都没有被改动。
    let rebuilt = unsafe { Vec::from_parts(ptr, len, cap) };
    println!("重新组装: {rebuilt:?}");

    // ---------- Box::as_ptr / Box::as_mut_ptr（Rust 1.98） ----------
    //
    // 只想临时拿到内容的地址，又不想放弃所有权，怎么办？
    // 常见写法是 `&*b as *const T` ——但它先制造了一个**引用**，再转成裸指针。
    // 按 Rust 的别名模型，制造引用这一步会让之前派生出的其他裸指针失效。
    // 这两个新函数是“原语解引用”，不会制造中间引用，因此多次取指针互不干扰：
    // - 连续两次 `as_mut_ptr` 取出的指针可以交错使用，通过其中一个写入不会使另一个失效（下面的代码就是这样）
    // - `as_ptr` 取出的只读指针则不同：一旦通过 `as_mut_ptr` 的指针写入，它就不能再用了
    // 此外，真正“制造引用”（比如 `&mut *b`）同样会让之前派生出的指针失效。
    // 注意它们是关联函数（`Box::as_ptr(&b)`），这是 Box 的惯例，为了不遮蔽内部类型的同名方法。
    let mut value = Box::new(10_i32);
    let first = Box::as_mut_ptr(&mut value);
    let second = Box::as_mut_ptr(&mut value); // 再取一次，first 依然有效
    // SAFETY: value 一直存活，first/second 都指向它的内容，且没有同时存在的其他引用。
    unsafe {
        first.write(11);
        second.write(first.read() + 1);
    }
    println!("经由 as_mut_ptr 修改后: {}，只读指针看到: {}", value, unsafe {
        Box::as_ptr(&value).read()
    });

    // ---------- 不要“收回”Box::leak 的结果（Rust 1.99） ----------
    //
    // `Box::leak` 故意把堆内存泄漏，换回一个 `&'a mut T`（`T` 里没有非 `'static` 的引用时可以取 `'static`），
    // 适合那种与进程同寿命、本来就不打算释放的数据，比如启动时创建的全局配置。
    // 1.99 在文档里加了明确的建议：如果内存之后还要释放，请改用 `Box::into_raw` 或 `Box::into_non_null`；
    // 不要先 `leak`、再用 `Box::from_raw` 把它“收回来”。
    // 文档给出的理由是：这样“收回”只有在分配器是 `Global` 时才有可能，
    // 而且即便如此也属于灰色地带——只在特定条件下成立，许多看似无害的写法其实是未定义行为。
    // 1.99 的发布公告还补充说，这类往返写法与编译器现有和未来的优化存在冲突，
    // 在即将稳定的自定义分配器面前尤其成问题，并且这条建议同样适用于标准库里其他的 leak 类函数。
    let app_name: &'static mut String = Box::leak(Box::new(String::from("rust-code-guide")));
    app_name.push_str(" (进程级常量)");
    println!("leak 出来的 'static 引用: {app_name}（它不会被释放，也不要试图释放）");

    // ---------- ManuallyDrop<Box<T>> 的明确保证（Rust 1.98） ----------
    //
    // `ManuallyDrop<T>` 是一个包装器：它包住的值在离开作用域时**不会**被自动 drop，
    // 要不要 drop、什么时候 drop，完全由你用 `ManuallyDrop::drop` 手动决定。
    // 标准库的容器、`Vec::into_parts` 这类函数的实现里到处都有它。
    struct Noisy(&'static str);
    impl Drop for Noisy {
        fn drop(&mut self) {
            println!("  Noisy({}) 被 drop", self.0);
        }
    }
    let mut manual = ManuallyDrop::new(Noisy("手动管理"));
    println!("ManuallyDrop 包住的值离开作用域时不会自动 drop，需要时手动 drop：");
    // SAFETY: manual 内部的值有效，且只 drop 这一次，之后不再使用它。
    unsafe { ManuallyDrop::drop(&mut manual) };

    // 一个曾经很微妙的问题：Rust 1.96 之前，“移动一个已经被释放的 Box”被视为未定义行为，
    // 而 `ManuallyDrop<Box<T>>` 在手动 drop 之后，里面的 Box 恰好就处于“已释放”状态，
    // 所以下面这个动作（drop 之后再把整个 ManuallyDrop 移动走）在 1.96 之前是 UB
    // （发布公告把它称为编译器里的缺陷）。1.96 修复之后，1.98 又把结论写进文档作为长期保证：
    // 这个移动操作**不是**未定义行为。
    let mut slot = ManuallyDrop::new(Box::new(String::from("data")));
    // SAFETY: slot 内部的 Box 有效，且只 drop 这一次。
    unsafe { ManuallyDrop::drop(&mut slot) };
    let moved = slot; // 1.98 起文档保证：这一步不是 UB
    // 但要牢记：保证的只是“移动”。此后再读取内容、或者再 drop 一次，依然是未定义行为。
    let _ = &moved;
    println!("已 drop 内容的 ManuallyDrop<Box<_>> 被移动了一次（没有再碰它的内容）");

    // ---------- UnsafeCell：可以不经过 get() 访问内容（Rust 1.99） ----------
    //
    // `UnsafeCell<T>` 是 Rust 里**唯一**合法的内部可变性原语：
    // `Cell`、`RefCell`、`Mutex`、`RwLock`、`OnceCell`、各种原子类型，全都建立在它之上。
    // 平时我们通过 `cell.get()` 拿到 `*mut T` 来读写内容。
    // 1.99 之前，文档明确规定：共享的 `UnsafeCell<T>` 里内容的 `*mut T` 只能经由 `get()`（或 `raw_get()`）获得，
    // 直接把 `*const UnsafeCell<T>` 强转成 `*mut T` 来访问内容是未定义行为——
    // 尽管 `T` 与 `UnsafeCell<T>` 的内存布局一直是相同的。
    // 1.99 放宽了这条别名规则：**不经过 `get()`**，直接强转之后访问内容，现在也是合法的；
    // `invalid_reference_casting` lint 也相应调整，不会再对这种写法报错。
    // 注意这并没有改变 `&mut` 的唯一性规则：只要某个 `&mut` 引用还活着，
    // 就不能用任何别的指针或引用去访问同一块内存。
    let cell = UnsafeCell::new(5_u32);
    let direct = &cell as *const UnsafeCell<u32> as *mut u32; // 没有调用 get()
    // SAFETY: cell 存活，且此刻没有任何其他线程或引用在访问它的内容。
    unsafe { *direct += 1 };
    println!("绕过 get() 直接写入之后: {}", cell.into_inner());
    // 注意：“布局相同”只对 `UnsafeCell<T>` 与 `T` 本身成立。把 `Outer<T>` 整个换成 `Outer<UnsafeCell<T>>`
    // 就不一定合法——`UnsafeCell` 会禁用 niche 优化，在 64 位平台上，`Option<NonNull<u8>>` 是 8 字节，
    // 而 `Option<UnsafeCell<NonNull<u8>>>` 要占 16 字节。

    // 小结：
    // 1. 往返裸指针时优先 `into_non_null` / `into_parts`；它们把“非空”写进了类型
    // 2. 想临时拿地址又不想制造引用，用 `Box::as_ptr` / `as_mut_ptr`
    // 3. 需要释放的内存别用 `leak`；`ManuallyDrop` 之后的移动与访问要分清
    // 4. 想理解 Cell/RefCell/Mutex 的本质，就去看 UnsafeCell
    println!();
}


// ===========================================
// 7. 未初始化与零初始化内存：MaybeUninit 数组转换、new_zeroed（Rust 1.95 / 1.92）
// ===========================================

// Rust 要求每个值在使用前都已经初始化，这是内存安全的基础。
// 但系统编程里常有这样的需求：先拿到一块内存，再决定怎么填——
// 比如把缓冲区交给 C 函数去写入，或者逐个元素地构造一个大数组。
// 标准库为此提供了两条路：
// - `MaybeUninit<T>`：明确表示“这里可能还没初始化”，由你在确认写入完成之后再 `assume_init`
// - `new_zeroed`：直接在堆上分配一块全零的内存（`Box` / `Rc` / `Arc`，1.92 起）
// 两条路都需要 unsafe，因为“已经初始化好了”这个承诺，只有你能做出。
// 这一节的重点不只是怎么用，还包括什么时候不能乱用：全零的位模式不一定是合法的值。

fn uninitialized_and_zeroed_memory() {
    println!("=== 未初始化与零初始化内存：MaybeUninit 数组转换、new_zeroed（Rust 1.95 / 1.92） ===");

    use std::mem::MaybeUninit;
    use std::rc::Rc;
    use std::sync::Arc;

    // Rust 1.95: MaybeUninit 数组转换
    //
    // MaybeUninit<T> 表示“可能未初始化的 T”。
    // 当你手头是 `[MaybeUninit<T>; N]`（逐个准备元素）时，
    // 现在可以直接转换成 `MaybeUninit<[T; N]>`（一整块待初始化内存），
    // 反之亦然，无需自己手工转指针。
    //
    // 典型场景：FFI 缓冲区、SIMD 打包、用数组拼装结构体。
    let mut slots: [MaybeUninit<u32>; 3] = [MaybeUninit::uninit(); 3];
    slots[0].write(10);
    slots[1].write(20);
    slots[2].write(30);
    // 元素都已就绪，转成“整块未初始化内存”的形式。
    let whole: MaybeUninit<[u32; 3]> = slots.into();
    // 反向转换也支持：从整块内存重新拆回逐个元素。
    let slots_again: [MaybeUninit<u32>; 3] = whole.into();
    let sum: u32 = unsafe { slots_again.into_iter().map(|s| s.assume_init()).sum() };
    println!("MaybeUninit 数组转换后的元素之和: {sum}");


    // ---------- new_zeroed（Rust 1.92） ----------
    //
    // Rust 1.92 为 Box、Rc、Arc 提供了 new_zeroed：
    // “先分配一块全零内存，确认初始化完成之后再 assume_init”成为标准库支持的流程。
    // 它比普通 `new` 更危险，因为“所有字节都为 0”并不一定是一个合法的值。
    // `Box::new_zeroed()`：
    // 在堆上分配足够容纳 `u32` 的内存，并把所有字节清零。
    //
    // 返回值还不是普通的 `Box<u32>`，
    // 而是一个“可能尚未完全初始化”的中间状态，因此最后要 `assume_init()`。
    //
    // 这里是安全的，因为 `u32` 的全零位模式就是合法值 `0`。
    let boxed = Box::<u32>::new_zeroed();
    let boxed = unsafe { boxed.assume_init() };
    println!("Box::new_zeroed -> {}", boxed);

    // `Rc::new_zeroed()`：
    // 用于单线程共享所有权场景。
    // 示例里仍然选一个“全零合法”的基础整数类型，避免误导新手。
    let rc = Rc::<u16>::new_zeroed();
    let rc = unsafe { rc.assume_init() };
    println!("Rc::new_zeroed -> {}", rc);

    // `Arc::new_zeroed()`：
    // 与 Rc 类似，但面向多线程共享所有权。
    // 如果某些共享数据需要“先分配，再初始化，再共享”，这个 API 会很有价值。
    let arc = Arc::<u8>::new_zeroed();
    let arc = unsafe { arc.assume_init() };
    println!("Arc::new_zeroed -> {}", arc);

    // 非常重要的警告：
    // 下面这些类型通常不能随便用 `new_zeroed + assume_init`：
    // 1. 引用类型 `&T`、`Box<T>`、`NonNull<T>`、函数指针：它们都不允许是空指针
    // 2. `NonZeroU32` 这类“保证非零”的整数
    // 3. 具有内部不变量的结构体
    // 4. 依赖特定标签值的枚举（没有取值为 0 的变体时，全零就是非法的判别值）
    //
    // 原因是：全零位模式不一定是这些类型的合法值。
    // 对照一下：`bool` 的全零位模式是 `false`，`char` 的是 '\0'，它们都是合法的，
    // 可以放心使用；但 `bool` 的 2..=255 这些位模式是非法的，不能随便凑出来。
}

// ===========================================
// 8. 裸指针与布局的小工具：union 字段指针、as_ref_unchecked、Layout::repeat、Cell 数组转换（Rust 1.92 / 1.95）
// ===========================================

// 手写底层数据结构时，会反复遇到这几件小事，标准库在 1.92 和 1.95 里把它们补得更顺手：
// - 在安全代码里对 union 字段取原始指针（1.92）
// - 裸指针转引用的 `as_ref_unchecked` / `as_mut_unchecked`（1.95）
// - `Layout` 的 `repeat` / `repeat_packed` / `extend_packed` / `dangling_ptr`（1.95）
// - `Cell<[T; N]>` 当作 `[Cell<T>; N]` 或 `[Cell<T>]` 看待（1.95）
//
// 阅读时建议留意：哪些地方仍然需要 unsafe，哪些地方标准库已经替你处理好了溢出和对齐。

fn pointer_and_layout_helpers() {
    println!("=== 裸指针与布局的小工具（Rust 1.92 / 1.95） ===");

    use std::cell::Cell;

    // Rust 1.92: 在安全代码里对 union 字段创建原始指针
    //
    // 这里定义一个典型的 C 风格包头：
    // - `raw` 表示把整块内存看成一个 32 位整数
    // - `bytes` 表示把同一块内存按字节观察
    //
    // union 常见于协议解析、FFI、驱动和寄存器映射。
    #[repr(C)]
    union PacketHeader {
        raw: u32,
        bytes: [u8; 4],
    }

    // `&raw const header.bytes` 的含义是：
    // “我只是想取得这个字段的原始地址，并不在这里直接读取它”。
    //
    // 注意区分两件事：
    // 1. 取地址
    // 2. 解引用读取内容
    //
    // 真正解引用仍然是 unsafe，因为程序员必须自己保证读取这个 union 字段是合法的。
    let header = PacketHeader { raw: 0x12345678 };
    let bytes_ptr = &raw const header.bytes;
    let bytes = unsafe { *bytes_ptr };
    println!("union 原始字节视图: {bytes:02X?}");


    // Rust 1.95: 裸指针的 as_ref_unchecked / as_mut_unchecked
    //
    // 以前要把裸指针转成引用，必须写 `unsafe { &*ptr }`。
    // 新方法把意图写得更清楚：`as_ref_unchecked` 表示“我保证指针此时有效”。
    // 注意：它仍然是 unsafe 的——调用者有责任保证对齐、有效和非别名约束。
    let value = 42u32;
    let raw_const = &value as *const u32;
    // 把“只读裸指针”转成共享引用。
    let shared: &u32 = unsafe { raw_const.as_ref_unchecked() };
    println!("as_ref_unchecked 得到: {shared}");

    let mut mutable = 7u32;
    let raw_mut = &mut mutable as *mut u32;
    // 把“可变裸指针”转成可变引用。
    let exclusive: &mut u32 = unsafe { raw_mut.as_mut_unchecked() };
    *exclusive += 1;
    println!("as_mut_unchecked 修改后: {mutable}");


    // Rust 1.95: Layout 的 repeat 系列
    //
    // Layout 描述“一块内存怎么对齐、多大”。手动分配数组内存时，
    // 需要按元素大小重复布局：
    // - `repeat(n)`：n 个元素，返回 (新布局, 步长)。步长用于手动算偏移
    // - `repeat_packed(n)`：n 个元素，但要求对齐必须是元素对齐的幂
    // - `extend_packed(layout)`：在本布局后面接上另一段布局
    //
    // 它们都返回 Result，因为计算可能溢出（元素数量太大时）。
    let elem = std::alloc::Layout::new::<u32>();
    let (array_layout, stride) = elem.repeat(4).unwrap();
    println!("Layout::repeat(4): stride={stride}, size={}", array_layout.size());
    let packed = elem.repeat_packed(4).unwrap();
    println!("Layout::repeat_packed(4): size={}", packed.size());
    let extended = elem.extend_packed(elem).unwrap();
    println!("Layout::extend_packed: size={}", extended.size());
    // dangling_ptr 返回“该布局对应的合法悬垂指针”，
    // 常用于“还没有数据可指向”时的占位指针。
    let _dangling = elem.dangling_ptr();
    println!("Layout::dangling_ptr 可用作占位指针");


    // Rust 1.95: Cell<[T; N]> 的 AsRef
    //
    // 之前 `Cell<[T; N]>` 只能整体读写，现在可以把它当成
    // `[Cell<T>; N]` 或 `[Cell<T>]` 的引用看待。
    // 这有什么用？写锁步（lock-step）算法时，
    // 可以“逐元素原地修改”，而不必每次取 Cell 的可变引用。
    let cells = Cell::new([1u32, 2, 3]);
    // 两个 AsRef 实现同时存在，需要显式标注类型来消歧。
    let as_array: &[Cell<u32>; 3] = <Cell<[u32; 3]> as AsRef<[Cell<u32>; 3]>>::as_ref(&cells);
    let as_slice: &[Cell<u32>] = <Cell<[u32; 3]> as AsRef<[Cell<u32>]>>::as_ref(&cells);
    // 逐元素原地加 1（Cell 的 get/set 不需要 &mut self）。
    as_array[0].set(as_array[0].get() + 1);
    as_slice[1].set(as_slice[1].get() + 1);
    println!("Cell 数组原地修改后: {:?}", cells.get());


    // 小结：
    // 1. union 字段取原始指针让“只取地址”这种操作表达得更清晰，真正解引用仍然是 unsafe
    // 2. `as_ref_unchecked` 把“我保证指针有效”的意图写进了方法名
    // 3. Layout 的计算有 Result 兜底，溢出时不会悄悄返回一个错误的布局
    // 4. Cell 数组的 AsRef 让内部可变性可以精确到元素
    println!();
}

// ===========================================
// 9. C 变参函数的定义与运行时符号 lint（Rust 1.98-1.99）
// ===========================================

// 这一节围绕“Rust 与 C 的调用约定边界”补充三件事：
// - 用 Rust **定义** C 变参函数（`unsafe extern "C" fn f(mut args: ...)`，1.99 稳定），
//   配套类型是 `core::ffi::VaList`，读取参数用 `next_arg::<T>()`
// - 三个新 lint 帮你拦住“FFI 声明写错签名”这类极难排查的 bug（1.98/1.99）：
//   `invalid_runtime_symbol_definitions`、`suspicious_runtime_symbol_definitions`、`c_void_returns`
// - 修复 x86_64 上 FFI 返回 `bool` 被错误编译的问题（1.99.1）
//
// 这一节的主线是：FFI 的危险往往不在 Rust 一侧，而在“两侧对签名的理解不一致”，
// 编译器正在一点点把这类不一致变成编译期错误。

// 变参函数：把前 count 个 c_int 参数加起来。
// 规则（已用编译器逐条验证）：
// 1. `...` 必须是最后一个参数，它的类型是 `VaList<'_>`
// 2. 变参函数定义必须是 `unsafe extern "C"`（或 "C-unwind"），写成安全函数会报错
//    "functions with a C variable argument list must be unsafe"
// 3. 前面的具名参数可以没有，但通常会用一个具名参数告诉函数“后面跟着什么、有几个”
// 4. 读取参数用 `args.next_arg::<T>()`，等价于 C 的 `va_arg`，是 unsafe 的——
//    读取的类型和个数必须与调用者实际传入的完全一致，否则就是未定义行为
unsafe extern "C" fn variadic_sum(count: c_int, mut args: ...) -> c_int {
    let mut total = 0;
    for _ in 0..count {
        // SAFETY: 调用者保证传入了至少 count 个 c_int 参数。
        total += unsafe { args.next_arg::<c_int>() };
    }
    total
}

// 混合类型：读取的顺序和类型要与调用者传入的完全一致。
// 这里没有具名参数，只有 `...`，这也是允许的。
unsafe extern "C" fn variadic_weighted(mut args: ...) -> c_double {
    // SAFETY: 调用者保证依次传入一个 c_double 和一个 c_int。
    let weight = unsafe { args.next_arg::<c_double>() };
    let times = unsafe { args.next_arg::<c_int>() };
    weight * f64::from(times)
}

// `VaList` 就是 C 的 `va_list`：布局和 ABI 与平台的 va_list 完全一致，可以跨 FFI 边界传递。
// C 标准库因此成对地提供 `printf(fmt, ...)` 和 `vprintf(fmt, va_list)`——
// 前者收变参，后者收 va_list，真正的逻辑写在后者里，前者只负责转交。
// 在 Rust 里也是这个写法：逻辑放进一个接收 `VaList` 的普通函数，变参函数只做转交。
unsafe fn variadic_sum_and_max(count: c_int, mut args: VaList<'_>) -> (c_int, c_int) {
    // 克隆一个 VaList 等价于 C 的 `va_copy`：得到一个互相独立的读取游标，读它不会影响原来的那个。
    // VaList 被 drop 时则相当于 `va_end`，不需要手动调用。
    let mut copy = args.clone();
    let mut sum = 0;
    for _ in 0..count {
        sum += unsafe { args.next_arg::<c_int>() };
    }
    let mut max = c_int::MIN;
    for _ in 0..count {
        max = max.max(unsafe { copy.next_arg::<c_int>() });
    }
    (sum, max)
}

// 变参函数不应该直接返回元组（元组不是 FFI 安全类型，会触发 improper_ctypes_definitions 警告），所以最大值通过输出指针带回。
unsafe extern "C" fn variadic_sum_max(count: c_int, max_out: *mut c_int, args: ...) -> c_int {
    // SAFETY: 调用者保证 max_out 有效，并传入至少 count 个 c_int。
    let (sum, max) = unsafe { variadic_sum_and_max(count, args) };
    unsafe { max_out.write(max) };
    sum
}

fn c_variadic_and_symbol_lints() {
    println!("=== C 变参函数的定义与运行时符号 lint（Rust 1.98-1.99） ===");

    // ---------- 用 Rust 定义 C 变参函数（Rust 1.99） ----------
    //
    // 背景：像 C 的 `printf(const char *fmt, ...)` 这样参数个数和类型都不固定的函数叫“变参函数”。
    // Rust 一直能**调用**外部定义的变参函数（第 15 章里的 printf 示例），
    // 却无法**定义**一个：想写一个能被 C 代码按变参方式调用的回调，只能用 C 来写。
    // 1.99 补上了这块拼图。上面三个模块级函数就是用法示例。
    //
    // 能读哪些类型，由 `VaArgSafe` 约束决定：`i32/u32/i64/u64/isize/usize`、`f64`、裸指针等。
    // `i8` / `i16` / `f32` 不在其中——C 传递变参时会先做“默认实参提升”：
    // 小整数提升成 int，float 提升成 double，所以接收端实际读到的永远是提升后的类型。
    // 从 Rust 调用变参函数时同样受此约束：传 `1.0f32` 或 `7u8` 会被编译器拒绝（E0617）。
    println!("variadic_sum(3, 10, 20, 30) = {}", unsafe { variadic_sum(3, 10, 20, 30) });
    println!(
        "variadic_weighted(2.5, 4) = {}",
        unsafe { variadic_weighted(2.5_f64, 4 as c_int) }
    );

    let mut max = 0;
    let sum = unsafe { variadic_sum_max(4, &mut max, 3, 9, 4, 7) };
    println!("经由 VaList 转交 + va_copy: 总和 = {sum}, 最大值 = {max}");

    // 如果要把 Rust 函数导出给 C 代码调用，加上 `#[unsafe(no_mangle)]` 和 `pub` 即可：
    //     #[unsafe(no_mangle)]
    //     pub unsafe extern "C" fn my_log(level: c_int, fmt: *const c_char, mut args: ...) { ... }
    // 另外 1.99 也稳定了用 `#[unsafe(naked)]` 定义变参函数，那需要自己用内联汇编处理 va_list，
    // 只有写运行时、调试器之类的极少数场景才会遇到。

    // ---------- 运行时符号 lint（Rust 1.98/1.99） ----------
    //
    // 什么是“运行时符号”？是 Rust 的 core / std（以及 LLVM 生成的代码）自己会去链接、
    // 并且对其签名有固定假设的符号：`memcpy`、`memset`、`memcmp`、`strlen`……
    // 1.99 又把 `open`、`read`、`write`、`close` 等标准库内部会用到的 POSIX 函数加了进来。
    // 如果你声明或定义了同名符号，却把签名写错，链接时不会有任何报错，
    // 运行时调用约定对不上，轻则数据错乱，重则崩溃——这类 bug 极难排查。
    // 所以编译器新增了检查：
    //
    // 1. invalid_runtime_symbol_definitions（默认 deny，即直接编译失败，1.98 引入）
    //    签名明确不对。典型例子就是把变参的 open 写成固定三个参数：
    //        unsafe extern "C" { fn open(path: *const c_char, flags: c_int, mode: u32) -> c_int; }
    //    报错：expected `unsafe extern "C" fn(*const i8, i32, ...) -> i32`
    //          found    `unsafe extern "C" fn(*const i8, i32, u32) -> i32`
    //    原因很具体：在 Apple Silicon 等平台上，变参走栈、定参走寄存器，
    //    被调用的 open 去栈上读 mode，读到的是垃圾值。
    //    （手写 libc 绑定时这个错误特别常见，所以第 15 章“安全包装器”一节
    //    直接使用 `libc` crate，而不是自己声明 open。）
    //
    // 2. suspicious_runtime_symbol_definitions（默认 warn，1.98 引入）
    //    签名“看起来不一样，但也许仍然兼容”，例如 strlen 的参数写成 `*const u8` 而不是 `*const c_char`
    //    （在 aarch64 macOS 上 c_char 是 i8）。确认兼容的话可以 `#[allow]` 掉。
    //
    // 3. c_void_returns（默认 warn，1.98 引入）
    //    `-> c_void` 并不等于 C 的 `void` 返回值：`c_void` 只用来表示 `void*` 指针指向的“不透明数据”。
    //    返回 void 就什么都别写（Rust 里返回 `()` 与 C 的 `void` 等价）；返回 `void*` 则写 `*mut c_void`。
    //        extern "C" fn callback() -> c_void   // 错误：应写成 extern "C" fn callback()
    //
    // 正确的声明长这样。下面的块只在 Unix 上编译，并且把声明真正调用了一遍：
    // 既证明它们能通过这几条 lint，也证明它们确实可用。
    #[cfg(unix)]
    {
        use std::ffi::{c_char, c_int, c_void};
        unsafe extern "C" {
            // open 是变参函数，声明里必须带 `...`
            fn open(path: *const c_char, flags: c_int, ...) -> c_int;
            fn close(fd: c_int) -> c_int;
            // strlen 的参数是 const char*，用 c_char 而不是 u8
            fn strlen(s: *const c_char) -> usize;
            // 返回 void 就不写返回类型；返回 void* 才用 *mut c_void
            fn malloc(size: usize) -> *mut c_void;
            fn free(ptr: *mut c_void);
        }
        // SAFETY: 传入的都是有效的 C 字符串；malloc 的结果检查非空后恰好释放一次；fd 打开后恰好关闭一次。
        unsafe {
            println!("strlen(\"hello\") = {}", strlen(c"hello".as_ptr()));
            let block = malloc(16);
            println!("malloc(16) 成功: {}", !block.is_null());
            free(block);
            // O_RDONLY 在所有 Unix 上都是 0；只读打开不需要变参里的 mode 参数
            let fd = open(c"/dev/null".as_ptr(), 0);
            println!("open(\"/dev/null\") 得到有效的 fd: {}", fd >= 0);
            close(fd);
        }
    }

    // ---------- FFI bool 返回值修复（Rust 1.99.1） ----------
    //
    // 1.99.1 修复了一个 x86_64 上的编译器缺陷：从 FFI 函数返回的 `bool` 值可能被错误地编译，
    // 导致读到不正确的 true/false。如果你的 FFI 接口里有返回 bool 的函数（比如 C 的 `_Bool` / C++ 的 `bool`），
    // 请至少升级到 1.99.1。点版本的发布说明很短，但这类“只在特定平台特定类型上出错”的修复值得留意。
    // 同一个点版本还修复了 UEFI / 嵌入式目标上缺失 `wcslen` 符号，以及 macOS 上 rust-lld 的 libLLVM rpath。

    // 小结：
    // 1. 现在 Rust 既能调用也能定义 C 变参函数；`VaList` 对应 `va_list`，`next_arg` 对应 `va_arg`
    // 2. 读变参时，类型、个数必须与调用者完全一致，这一点编译器无法替你检查
    // 3. 手写 FFI 声明时，签名要和 C 头文件逐项核对；编译器会拦下其中最危险的一类错误
    // 4. 能用 `libc` 之类现成绑定的，就别手写声明
    println!();
}


// ===========================================
// 10. extern 声明的 ABI 规范：显式 ABI、C-unwind、变参 ABI 与 raw-dylib（Rust 1.71-1.93）
// ===========================================

// ABI（Application Binary Interface，应用二进制接口）规定了函数调用时参数放在哪里、
// 返回值怎么传、谁来清理栈。Rust 与 C、与操作系统 API 互相调用时，双方必须在 ABI 上达成一致，
// 否则编译能过、运行就崩，而且崩得毫无头绪。这一节是 ABI 书写方面的几条规范：
// - 一律显式写出 ABI 字符串（`missing_abi` lint，1.86 起默认 warn）
// - 需要让 panic 穿过 FFI 边界时，用 "C-unwind"（1.71）
// - 其他调用约定也可以声明 C 风格的变参函数：sysv64 / win64 / efiapi / aapcs（1.91）、system（1.93）
// - Windows 上不依赖导入库就能链接系统 DLL：`raw-dylib`（1.71）

// 导出给 C 当回调的函数：显式写出 "C"。不写的话 `missing_abi` lint 会警告：
//     warning: `extern` declarations without an explicit ABI are deprecated
//     help: explicitly specify the "C" ABI: `extern "C"`
// 默认值虽然也是 "C"，但“默认是什么”对读代码的人并不明显，而 ABI 对不上正是 FFI 里最难查的一类 bug。
extern "C" fn increment(value: c_int) -> c_int {
    value + 1
}

// "C-unwind"：声明“panic 可以穿过这个函数的边界”。
// 普通的 `extern "C"` 函数如果有 panic 想要展开到调用者，进程会直接 abort（Rust 1.81 起的行为），
// 因为 C 一侧并不认识 Rust 的展开机制，硬展开是未定义行为。
// 但 Rust → C++ → Rust 这样的调用链，两边都认识展开，这时就该用 "C-unwind"。
// 带 `-unwind` 后缀的 ABI 还有 "system-unwind"、"sysv64-unwind" 等，规则相同。
extern "C-unwind" fn checked_divide(dividend: c_int, divisor: c_int) -> c_int {
    if divisor == 0 {
        panic!("除数不能为零");
    }
    dividend / divisor
}

// Windows 上的 raw-dylib：传统做法链接 DLL 里的函数，要求构建机器上有它的导入库（kernel32.lib 之类），
// 也就是要装 Windows SDK 或 MinGW，在 Linux / macOS 上交叉编译 Windows 程序时尤其麻烦。
// `kind = "raw-dylib"` 让 rustc 根据下面的声明自己生成导入表，构建时不再需要这些库。
// 还可以配合 `#[link_ordinal(n)]` 按序号而不是按名字绑定符号。
// Win32 API 的调用约定用 "system"：32 位 Windows 上是 stdcall，64 位上是 C。
#[cfg(windows)]
fn windows_tick_count() -> u32 {
    #[link(name = "kernel32", kind = "raw-dylib")]
    unsafe extern "system" {
        fn GetTickCount() -> u32;
    }
    // SAFETY: GetTickCount 没有参数，也没有前置条件。
    unsafe { GetTickCount() }
}

// 1.91 起，"sysv64" 等 ABI 也能在 extern 块里声明 `...` 变参函数。
// x86_64 的 Linux / macOS 都使用 System V ABI，所以 libc 的 snprintf 可以按 "sysv64" 声明。
// 返回值是写入的字符数（不含结尾的 \0）。
#[cfg(all(target_arch = "x86_64", unix))]
fn sysv64_snprintf_len() -> c_int {
    use std::ffi::c_char;
    unsafe extern "sysv64" {
        fn snprintf(buffer: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    }
    let mut buffer = [0 as c_char; 32];
    // SAFETY: buffer 足够容纳输出；格式串里有两个 %d，传入的恰好是两个 c_int。
    unsafe { snprintf(buffer.as_mut_ptr(), buffer.len(), c"%d-%d".as_ptr(), 12 as c_int, 34 as c_int) }
}

fn abi_declarations() {
    println!("=== extern 声明的 ABI 规范：显式 ABI、C-unwind、变参 ABI 与 raw-dylib（Rust 1.71-1.93） ===");

    // 把 Rust 函数当作 C 回调传出去时，类型里也要带上 ABI：`extern "C" fn(参数) -> 返回值`
    let callback: extern "C" fn(c_int) -> c_int = increment;
    println!("经由 extern \"C\" 函数指针调用: increment(41) = {}", callback(41));

    // "C-unwind" 的函数可以正常调用；除数为 0 时它会 panic，并且这个 panic 可以被 catch_unwind 接住。
    // 如果它是 extern "C"，同样的 panic 会让整个进程被 abort。
    println!("checked_divide(100, 5) = {}", checked_divide(100, 5));
    // 经验法则：导出给 C 调用的函数，要在函数内部自己 catch_unwind，把 panic 转成错误码；
    // 只有明确知道调用链两端都能处理展开时，才用 "C-unwind"。

    #[cfg(all(target_arch = "x86_64", unix))]
    println!("按 sysv64 声明的 snprintf 写入了 {} 个字符", sysv64_snprintf_len());
    // 其他 ABI 的写法完全一样，只是把 "sysv64" 换成 "win64"（Windows x64）、
    // "aapcs"（32 位 ARM）或 "efiapi"（UEFI）；1.93 起 "system" 也能声明变参函数。

    #[cfg(windows)]
    println!("系统启动后经过的毫秒数 = {}", windows_tick_count());

    // 1.93 起，"system" 也可以声明 C 风格的变参函数。这里不真正调用系统函数，
    // 只用函数指针类型说明：“system ABI + 可变参数”这种签名现在可以稳定地表达。
    // "system" 的含义是“当前平台系统 API 默认使用的调用约定”：大多数平台上和 "C" 相同，32 位 Windows 上是 stdcall。
    type SystemVariadicPrinter = unsafe extern "system" fn(*const std::ffi::c_char, ...) -> c_int;
    println!("extern \"system\" 变参函数指针大小: {} 字节", size_of::<SystemVariadicPrinter>());

    // 小结：
    // 1. 永远显式写出 ABI，别依赖默认值
    // 2. 默认的 "C" 遇到 panic 会 abort；需要 panic 穿过边界才用 "C-unwind"
    // 3. 声明外部的变参函数不再局限于 "C"；*定义*变参函数见上一节
    // 4. 交叉编译 Windows 程序时，raw-dylib 能省掉对导入库的依赖
    println!();
}

// ===========================================
// 11. 内联汇编（Rust 1.59）
// ===========================================

// 内联汇编是 Rust 1.59 版本开始稳定的重要特性
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

// 把 x86_64 Linux 上的几个典型内联汇编封装成安全函数：演示（inline_assembly）和测试共用同一份实现。
// 把 unsafe 的汇编藏在安全函数里，调用者就不需要写 unsafe——这是使用内联汇编的最佳实践。
// 单独成模块并配上测试还有一个现实原因：这类只在特定平台编译的代码，
// 在别的平台上不会被编译到，一旦写错就很难被发现；有了测试，在目标平台上跑 `cargo test` 就能暴露问题。
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
mod asm_x86_64_linux {
    use std::arch::asm;

    /// 用 cpuid 指令检测 CPU 是否支持 SSE2。
    ///
    /// 演示 cpuid 这类“会破坏多个寄存器”的指令怎么写。要点有两个：
    /// 1. cpuid 会写入 eax/ebx/ecx/edx 四个寄存器，每一个都必须在操作数里声明；
    /// 2. rbx 被 LLVM 保留用于内部用途，不能直接出现在 asm! 的操作数里。
    ///    惯用法是先把 rbx 存进临时寄存器，执行完 cpuid 之后用 xchg 换回来——
    ///    换回之后临时寄存器里装的恰好是 cpuid 写入的 ebx 值。
    ///
    /// 实际工程中不必手写：`std::arch::is_x86_feature_detected!("sse2")` 内部就是这么做的，还带缓存。
    pub fn cpu_has_sse2() -> bool {
        let edx: u32;
        // SAFETY: cpuid 在所有 x86_64 CPU 上都可用，只读取处理器信息，不访问内存。
        unsafe {
            asm!(
                "mov {tmp:r}, rbx",
                "cpuid",
                "xchg {tmp:r}, rbx",
                tmp = out(reg) _, // 临时寄存器：cpuid 之后装着 ebx 的值，这里用不到，用 `_` 丢弃
                inout("eax") 1u32 => _, // 功能号 1：处理器信息与特性位；返回的 eax 用不到
                inout("ecx") 0u32 => _,
                out("edx") edx,
                options(nomem, nostack)
            );
        }
        // 功能号 1 的 EDX 第 26 位就是 SSE2
        edx & (1 << 26) != 0
    }

    /// 不经过 libc，直接发起 Linux 的 getpid 系统调用，返回当前进程 ID。
    ///
    /// x86_64 Linux 的约定：系统调用号放在 rax，返回值也从 rax 取出。
    /// `syscall` 指令还会破坏 rcx 和 r11，必须声明为被占用，否则编译器可能把重要的值留在里面。
    /// rax 既是输入又是输出，用 inlateout 让两者共用同一个寄存器。
    pub fn getpid_via_syscall() -> u64 {
        let pid: u64;
        // SAFETY: getpid 系统调用没有参数、不会失败，也不访问我们的内存。
        unsafe {
            asm!(
                "syscall",
                inlateout("rax") 39u64 => pid, // 39 = SYS_getpid，返回值（进程 ID）也在 rax
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        pid
    }

    /// 用 xchg 指令原子地把 `slot` 的值换成 `new`，返回换出的旧值。
    ///
    /// x86 的 xchg 只要有一个操作数是内存，就自动带有 lock 语义，不需要再写 lock 前缀。
    /// xchg 把寄存器和内存里的值互换，所以旧值最终出现在“寄存器操作数”里，而不是 eax。
    pub fn atomic_swap(slot: &mut u32, new: u32) -> u32 {
        let old: u32;
        let ptr: *mut u32 = slot;
        // SAFETY: ptr 来自有效的 &mut u32，对齐且可写；汇编只读写这一个 u32。
        unsafe {
            asm!(
                "xchg [{ptr}], {new:e}",
                ptr = in(reg) ptr,         // 内存操作数：指向被交换变量的裸指针
                new = inout(reg) new => old, // 放入新值，交换之后这个寄存器里是旧值
                options(nostack, preserves_flags)
            );
        }
        old
    }

    /// 用 SSE 指令做 4 路 f32 的向量加法。
    ///
    /// 必须用 `out("xmm0") _` 声明 xmm0/xmm1 会被破坏，否则编译器可能把别的值放在里面，被我们悄悄覆盖。
    pub fn add_f32x4(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        let mut result = [0.0f32; 4];
        // SAFETY: 三个指针都指向恰好 4 个 f32（16 字节）的有效内存；movups 允许不对齐访问。
        unsafe {
            asm!(
                "movups xmm0, [{a}]",   // 加载第一个向量
                "movups xmm1, [{b}]",   // 加载第二个向量
                "addps xmm0, xmm1",     // 向量加法
                "movups [{dst}], xmm0", // 存储结果
                a = in(reg) a.as_ptr(),
                b = in(reg) b.as_ptr(),
                dst = in(reg) result.as_mut_ptr(),
                out("xmm0") _,
                out("xmm1") _,
                options(nostack, preserves_flags)
            );
        }
        result
    }

    /// 绝对值：用 cdq + xor + sub 的经典无分支写法。
    pub fn fast_abs(x: i32) -> i32 {
        let result: i32;
        // SAFETY: 这段汇编只在寄存器里计算，对任何 i32 输入都是良定义的。
        unsafe {
            asm!(
                "cdq",           // 符号扩展 eax 到 edx:eax：x 为负数时 edx = 0xFFFFFFFF，否则为 0
                "xor eax, edx",  // 负数：按位取反；非负数：不变
                "sub eax, edx",  // 负数：再加 1（减去 -1），合起来就是取相反数；非负数：减 0，不变
                inlateout("eax") x => result, // 输入输出共用 eax
                out("edx") _,                 // cdq 会写 edx，必须声明被破坏
                options(pure, nomem, nostack) // xor/sub 会修改标志位，所以不能声明 preserves_flags
            );
        }
        result
    }

    /// 字节序交换（bswap 指令），例如 0x12345678 变成 0x78563412。
    pub fn byte_swap(x: u32) -> u32 {
        let result: u32;
        // SAFETY: 纯寄存器运算，对任何 u32 输入都是良定义的。
        unsafe {
            asm!(
                "bswap {r:e}", // 反转 4 个字节的顺序
                r = inlateout(reg) x => result,
                options(pure, nomem, nostack, preserves_flags) // bswap 不影响标志位
            );
        }
        result
    }

    /// const 操作数：把编译期常量作为立即数嵌入指令（Rust 1.82 稳定），这里把 `x` 左移 3 位。
    ///
    /// 两点限制：
    /// 1. const 操作数的值必须是编译期常量（常量项、字面量、常量表达式），不能是函数参数
    /// 2. `asm!` 不能出现在 `const fn` 里：汇编是运行时才执行的机器指令，编译器没法在编译期求值它
    pub fn shift_left_by_const(x: u32) -> u32 {
        const SHIFT: u32 = 3;
        let shifted: u32;
        // SAFETY: 纯寄存器运算，对任何 u32 输入都是良定义的。
        unsafe {
            asm!(
                "shl {x:e}, {amount}",
                x = inlateout(reg) x => shifted,
                amount = const SHIFT, // 展开成立即数 3，即 `shl reg, 3`
                options(pure, nomem, nostack)
            );
        }
        shifted
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn inline_assembly() {
    use std::arch::asm;

    println!("=== 内联汇编 (Rust 1.59+) ===");

    // Rust 内联汇编的基本语法
    // asm!("汇编模板", 操作数1, 操作数2, ...);
    // 汇编模板：包含汇编指令和占位符；x86 默认使用 Intel 语法（目标操作数写在前面）
    // 操作数：声明哪些变量作为输入、输出，以及指令会破坏哪些寄存器
    //
    // 占位符 `{}` / `{name}` 会被替换成编译器分配的寄存器。
    // 默认展开成该寄存器的完整宽度（64 位的 rax 之类）；
    // 操作数是 32 位整数时，用修饰符 `{x:e}` 取 32 位名字（eax 之类），避免宽度不匹配的警告。
    //
    // 本函数里的代码只会在 x86_64 Linux 上编译和运行。

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
    // x86 的 add 只有两个操作数：目标 += 源（ARM 的 add 才是三个操作数）。
    // 所以让 x 既是输入又是输出：`inout(reg) x => sum`，结果写入 sum。
    let x: i32 = 10;
    let y: i32 = 20;
    let sum: i32;

    unsafe {
        asm!(
            "add {x:e}, {y:e}", // 加法：x += y
            x = inout(reg) x => sum, // 输入 x，输出写入 sum
            y = in(reg) y,           // 输入 y
            options(pure, nomem, nostack) // add 会修改标志位，所以这里不能写 preserves_flags
        );
    }
    println!("内联汇编加法: {} + {} = {}", x, y, sum);

    // 基础示例 3：乘法运算
    // x86 的 imul 也是两操作数形式：目标 *= 源。
    // （三操作数的 `imul r, r/m, imm` 第三个操作数只能是立即数，不能是寄存器。）
    let a: i32 = 5;
    let b: i32 = 3;
    let product: i32;

    unsafe {
        asm!(
            "imul {a:e}, {b:e}", // 有符号乘法：a *= b
            a = inout(reg) a => product, // 输入 a，输出写入 product
            b = in(reg) b,
            options(pure, nomem, nostack) // imul 也会修改标志位
        );
    }
    println!("内联汇编乘法: {} * {} = {}", a, b, product);

    // 操作数类型详解
    // in(reg) x：把 x 放进编译器选择的通用寄存器，作为输入
    // out(reg) y：汇编结束后从某个通用寄存器取出结果，写入变量 y
    // inout(reg) x：同一个寄存器既作输入也作输出（x 的值进去，结果写回 x）
    // inout(reg) x => y：输入用 x，输出写入 y
    // lateout / inlateout：声明“输出只在所有输入都读完之后才写入”，因此允许输出与输入共用同一个寄存器
    // in("rax") ... / out("rcx") _：指定具体寄存器；`out(..) _` 表示“这个寄存器会被破坏（clobber），但不关心结果”
    // const N：把编译期常量作为立即数嵌入指令（Rust 1.82 稳定）
    // sym path：函数或静态变量的符号地址（Rust 1.66 稳定）

    // 演示 inout 操作数的使用
    let mut value: i32 = 10;
    unsafe {
        asm!(
            "add {value:e}, 5", // 将 5 加到 value 上
            value = inout(reg) value, // 同时作为输入和输出
            options(pure, nomem, nostack)
        );
    }
    println!("inout 操作数结果: {}", value);

    // 内联汇编的选项详解
    // pure：无副作用，只依赖输入、只产生输出，允许编译器消除或合并重复调用（必须至少有一个输出）
    // nomem：不读写内存，允许编译器把前后的内存访问随意挪动
    // readonly：只读内存，不写内存
    // preserves_flags：不修改标志位（EFLAGS）
    // noreturn：汇编块不会返回，例如以 jmp 或 ud2 结尾
    // nostack：不使用栈（不 push/pop，也不动红区）

    // 下面几个示例的汇编代码都封装在上面的 asm_x86_64_linux 模块里（演示和测试共用同一份实现），
    // 每个函数的注释里写明了对应汇编的写法和注意事项，建议对照着读。

    // CPU 特性检测示例：用 cpuid 指令
    let has_sse2 = asm_x86_64_linux::cpu_has_sse2();
    println!("CPU 支持 SSE2: {}", has_sse2);

    // 系统调用示例：不经过 libc，直接发起 Linux 的 getpid 系统调用
    println!("当前进程 ID: {}", asm_x86_64_linux::getpid_via_syscall());

    // 原子操作示例：用 xchg 做原子交换
    let mut atomic_value: u32 = 42;
    let old_value = asm_x86_64_linux::atomic_swap(&mut atomic_value, 100);
    println!("原子交换操作: 旧值 {}, 新值 {}", old_value, atomic_value);

    // SIMD 指令示例：4 路 f32 向量加法
    // 在 x86_64 上 SSE/SSE2 属于基线指令集，这个判断一定为真；
    // 保留它是为了展示“用到可选指令集之前先检测”的好习惯（换成 AVX 这类可选扩展时就必须这么做）。
    if has_sse2 {
        let sum = asm_x86_64_linux::add_f32x4([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]);
        println!("SIMD 向量加法: {:?}", sum);
    }

    // 内存屏障示例：sfence 保证它之前的存储操作先于之后的存储操作对其他核心可见。
    // 注意这里不能写 `nomem`：nomem 等于告诉编译器“这段汇编不涉及内存”，
    // 编译器就可以把前后的内存读写随意挪到屏障另一侧，屏障就形同虚设。
    // （跨线程同步的实际代码请用 `std::sync::atomic::fence`，它是可移植的。）
    unsafe {
        asm!("sfence", options(nostack, preserves_flags));
    }
    println!("已执行 sfence 内存屏障");

    // 实际应用场景：性能关键的小函数 + 安全的 Rust 接口封装
    println!("快速绝对值: {}", asm_x86_64_linux::fast_abs(-100));
    println!("字节交换结果: 0x{:08X}", asm_x86_64_linux::byte_swap(0x12345678));
    println!("const 操作数: 5 << 3 = {}", asm_x86_64_linux::shift_left_by_const(5));

    // 内联汇编的最佳实践和注意事项
    // 1. 安全性：始终提供安全的 Rust 接口封装
    // 2. 平台兼容性：使用条件编译处理不同平台
    // 3. 性能测试：验证汇编代码确实带来性能提升
    // 4. 文档化：详细说明汇编代码的功能和约束
    // 5. 声明准确：指令破坏了哪些寄存器、是否访问内存、是否改标志位，都要如实写出，
    //    写错的后果是编译器生成错误代码，而且不会有任何报错
    // 6. 备用方案：提供非汇编的实现作为后备

    // 内联汇编的未来发展
    // 1. 继续扩展架构覆盖：近几个版本陆续稳定了 s390x、LoongArch、PowerPC 等
    // 2. 更丰富的操作数类型：支持更多寄存器类型
    // 3. 更好的集成：与 Rust 类型系统的深度集成
    // 4. 编译器优化：更好的编译器优化支持
    // 5. 调试支持：改进调试体验

    println!();
}

// 其他平台的内联汇编处理
#[cfg(not(all(target_arch = "x86_64", target_os = "linux")))]
fn inline_assembly() {
    println!("=== 内联汇编 (Rust 1.59+) ===");
    println!("内联汇编在当前平台上可能受限或不支持");
    println!("内联汇编需要 Rust 1.59+ 版本和特定平台支持");
    println!("当前架构: {}", std::env::consts::ARCH);
    println!("当前系统: {}", std::env::consts::OS);

    // 显示支持内联汇编的平台
    println!("\n支持内联汇编的平台包括：");
    println!("- x86 / x86_64（Rust 1.59 起稳定）");
    println!("- ARM / AArch64 / RISC-V（Rust 1.59 起稳定）");
    println!("- LoongArch64（1.72）、Arm64EC 与 s390x（1.84）、LoongArch32（1.91）、PowerPC / PowerPC64（1.95）");

    println!("\n内联汇编的主要应用场景：");
    println!("1. 系统编程：操作系统内核、驱动程序");
    println!("2. 性能优化：关键路径的汇编优化");
    println!("3. 嵌入式系统：直接硬件控制");
    println!("4. 密码学：特定算法的汇编实现");
    println!("5. 虚拟机：JIT 编译器的代码生成");

    println!();
}

// ===========================================
// 12. 裸函数：函数的序言和尾声也由你来写（Rust 1.88）
// ===========================================

// 普通函数开头和结尾的那几条指令（保存寄存器、调整栈指针、返回），是编译器自动生成的。
// 裸函数（naked function）把这部分也交还给你：函数体**只能**是一段 `naked_asm!`，编译器不会插入任何额外指令。
// 它是写这些东西的基础：
// - 操作系统内核、嵌入式固件的中断入口 / 系统调用入口：硬件规定了进入时寄存器的状态，不能被编译器的序言破坏
// - 协程、绿色线程的上下文切换：需要手动换栈、保存和恢复寄存器
// - 引导代码，以及各种跳板函数（trampoline）
//
// 编译器会逐条检查下面这些规则（都用 1.99.1 实测过）：
// 1. 属性写作 `#[unsafe(naked)]`：只写 `#[naked]` 会报 “unsafe attribute used without unsafe”。
//    加上 unsafe 是因为“汇编是否符合这个 ABI”这个责任完全在你
// 2. 必须显式声明非 Rust 的 ABI（比如 `extern "C"`），否则报
//    “`#[naked]` is currently unstable on `extern "Rust"` functions”。
//    因为调用者要按 ABI 把参数放进约定的寄存器，Rust 自己的 ABI 没有稳定的约定
// 3. 函数体里只能有一个 `naked_asm!`，多一条语句就报 E0787
// 4. 不能带 `#[inline]`（E0736），因为它没有“函数体”可以内联
// 5. 汇编里必须自己写返回指令；没写的话，执行会“掉”进后面毫不相干的代码里
//
// 1.99 又稳定了用裸函数定义 C 变参函数，那需要自己在汇编里处理 va_list，
// 只有写运行时、调试器之类的极少数场景才会遇到。

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
use std::arch::naked_asm;

// 两个 32 位整数相加。不同平台的 C ABI 把参数放在不同的寄存器里，所以要分平台写。
#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
extern "C" fn naked_add(a: i32, b: i32) -> i32 {
    // AAPCS64：第 1、2 个参数在 w0、w1，返回值也放在 w0
    naked_asm!("add w0, w0, w1", "ret")
}

#[cfg(all(target_arch = "x86_64", not(windows)))]
#[unsafe(naked)]
extern "C" fn naked_add(a: i32, b: i32) -> i32 {
    // System V：第 1、2 个参数在 edi、esi，返回值放在 eax
    naked_asm!("lea eax, [rdi + rsi]", "ret")
}

#[cfg(all(target_arch = "x86_64", windows))]
#[unsafe(naked)]
extern "C" fn naked_add(a: i32, b: i32) -> i32 {
    // Windows x64：第 1、2 个参数在 ecx、edx，返回值放在 eax
    naked_asm!("lea eax, [rcx + rdx]", "ret")
}

fn naked_functions() {
    println!("=== 裸函数：函数的序言和尾声也由你来写（Rust 1.88） ===");

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    println!("naked_add(40, 2) = {}（函数体只有 1 条加法指令和 1 条返回指令）", naked_add(40, 2));
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    println!("当前架构没有对应的裸函数示例，跳过");

    // 小结：
    // 1. 只有需要完全掌控进入 / 退出时的寄存器和栈时，才用裸函数；日常需求用普通函数 + `asm!` 就够了
    // 2. 函数体只能是一个 `naked_asm!`，并且要自己写返回指令
    // 3. 汇编依赖平台的调用约定，所以几乎总要配合 `#[cfg(target_arch = ...)]`
    println!();
}

// ===========================================
// 13. 内联汇编进阶：跳转到 Rust 代码块的 label 与 asm! 内的 #[cfg]（Rust 1.87 / 1.93）
// ===========================================

// `asm!` 的基础用法见本章第 11 节，这一节补两个进阶能力。
//
// (1) label 操作数（asm goto，1.87）
// 汇编里的条件跳转，目标本来只能是汇编内部的标号；想“跳回 Rust 代码”，
// 以前只能让汇编设置一个输出寄存器，再回到 Rust 里用 `if` 判断一次——多出一次判断，还丢掉了“这是一次跳转”的信息。
// `label { ... }` 操作数把一个 Rust 代码块当作跳转目标：在汇编模板里写 `{名字}` 就能直接跳进去。
// 典型用途：内核里的“静态键”（static key）和追踪点（tracepoint）——平时只是一条什么都不做的 nop，
// 需要时被运行时补丁改成跳转指令；还有把“检查失败就走慢路径”直接写在汇编里。
// 规则：label 块的类型必须是 `()` 或 `!`，通常用 `return` / `break` 离开；
//      label 与 output 操作数目前不能同时使用（编译器报 E0658，对应 issue #119364，仍然不稳定）。
//
// (2) asm! 内的 #[cfg]（1.93）
// 一份汇编模板里，有的指令只在某个平台或某个 CPU 特性下才存在。
// 以前只能把整个 `asm!` 复制成多份，每份套一层 `#[cfg]`；
// 现在可以直接在模板字符串和操作数上写 `#[cfg(...)]`，条件不满足的部分就像不存在一样。
// 条件不仅可以是平台，也可以是 CPU 特性：比如 `#[cfg(target_feature = "sse2")] "nop",`，
// 表示只有编译目标启用了 SSE2 时才插入这一行。

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn is_zero_via_asm_goto(value: u64) -> bool {
    use std::arch::asm;
    // SAFETY: 汇编只比较寄存器并跳转，不访问内存，也不动栈（nomem, nostack）。
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!(
            "cbz {value}, {zero}", // 寄存器为 0 就跳转到 zero 标号
            value = in(reg) value,
            zero = label { return true }, // 跳转目标：一个 Rust 代码块
            options(nomem, nostack),
        );
        #[cfg(target_arch = "x86_64")]
        asm!(
            "test {value}, {value}",
            "jz {zero}",
            value = in(reg) value,
            zero = label { return true },
            options(nomem, nostack),
        );
    }
    false // 没有跳转：值不为 0
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn forty_two_via_asm_cfg() -> u64 {
    use std::arch::asm;
    let result: u64;
    // SAFETY: 汇编只是往一个寄存器里写入常量，没有内存访问和其他副作用。
    unsafe {
        asm!(
            // 同一个 asm! 里，每个平台各取所需的那一行
            #[cfg(target_arch = "x86_64")]
            "mov {result}, 42",
            #[cfg(target_arch = "aarch64")]
            "mov {result}, #42",
            result = out(reg) result,
            options(pure, nomem, nostack),
        );
    }
    result
}

fn inline_asm_labels_and_cfg() {
    println!("=== 内联汇编进阶：label 跳转与 asm! 内的 #[cfg]（Rust 1.87 / 1.93） ===");

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        println!("is_zero_via_asm_goto(0) = {}", is_zero_via_asm_goto(0));
        println!("is_zero_via_asm_goto(7) = {}", is_zero_via_asm_goto(7));
        println!("forty_two_via_asm_cfg() = {}", forty_two_via_asm_cfg());
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    println!("当前架构没有对应的汇编示例，跳过");

    // 小结：
    // 1. `label { .. }` 让汇编直接跳进 Rust 代码块，省掉“设输出寄存器再判断”这一步
    // 2. label 不能和 output 操作数同时使用
    // 3. `#[cfg]` 可以直接写在 asm! 的模板行和操作数上，不必为每个平台复制整段 asm!
    println!();
}

// ===========================================
// 14. 安全函数上的 #[target_feature]（Rust 1.86）
// ===========================================

// CPU 的扩展指令集（x86 的 AVX2，ARM 的 NEON 等）并不是每台机器都有。
// `#[target_feature(enable = "avx2")]` 让编译器只为**这一个函数**打开该指令集，
// 程序的其他部分仍然能在没有 AVX2 的老机器上运行。
// 经典用法是“运行时检测 + 分发”：程序启动后检测 CPU 支持什么，挑最快的实现。
//
// 1.86 之前，`#[target_feature]` 只能用在 `unsafe fn` 上，因为在不支持该指令集的 CPU 上调用它是未定义行为。
// 1.86 起它可以标在**安全函数**上，规则变成：
// - 在“同样启用了这些特性”的上下文里调用它是安全的：也就是调用者自己也带有相同的 `#[target_feature]`
// - 在其他地方调用，必须写 `unsafe`，由你保证 CPU 确实支持（通常先做运行时检测）
// 编译器还特别强调：即使整个构建默认就启用了该特性（比如 aarch64 默认带 neon），也不能免除这个要求，
// 必须在函数自己的 `#[target_feature]` 里写出来：
//     the neon target feature being enabled in the build configuration
//     does not remove the requirement to list it in `#[target_feature]`

// 逐元素相乘再求和。函数体是普通的安全代码，指令集由编译器按 target_feature 自行选择。
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn dot_simd(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// 同样带 `#[target_feature]`，所以这里调用 dot_simd 不需要 unsafe（Rust 1.86 的新规则）
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn dot_simd_scaled(a: &[f32], b: &[f32], factor: f32) -> f32 {
    dot_simd(a, b) * factor
}

// 没有特殊指令集时的通用实现，也是检测失败时的后备
fn dot_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// 分发函数：先检测，再选实现。调用方不需要知道背后用了什么指令集。
fn scaled_dot_product(a: &[f32], b: &[f32], factor: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        // SAFETY: 刚刚检测过，当前 CPU 支持 avx2。
        return unsafe { dot_simd_scaled(a, b, factor) };
    }
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("neon") {
        // SAFETY: 刚刚检测过，当前 CPU 支持 neon。
        return unsafe { dot_simd_scaled(a, b, factor) };
    }
    dot_scalar(a, b) * factor
}

fn safe_target_feature() {
    println!("=== 安全函数上的 #[target_feature]（Rust 1.86） ===");

    let weights = [1.0_f32, 2.0, 3.0];
    let inputs = [4.0_f32, 5.0, 6.0];
    println!("缩放后的点积 = {}（1*4 + 2*5 + 3*6 = 32，再乘 2）", scaled_dot_product(&weights, &inputs, 2.0));

    // 小结：
    // 1. 指令集相关的实现用 `#[target_feature]` 标出来，通用实现作为后备
    // 2. 调用前用 `is_x86_feature_detected!` / `is_aarch64_feature_detected!` 做运行时检测
    // 3. 1.86 起它可以标在安全函数上；带同样特性的调用者能安全调用它，其他调用者要用 unsafe
    println!();
}

// ===========================================
// 15. 让 unsafe 代码更早暴露问题：悬垂指针 lint 与前置条件断言（Rust 1.91 / 1.78）
// ===========================================

// 两个互相配合的“安全网”：一个在编译期（lint），一个在运行期（debug 断言）。
//
// (1) dangling_pointers_from_locals（1.91，默认 warn）
// 函数返回指向局部变量的裸指针，是最经典的悬垂指针来源：函数一返回，局部变量就被销毁，
// 指针指向的栈空间随时会被覆盖。创建悬垂指针本身不是未定义行为，所以它只是 warn；
// 对它解引用才是未定义行为，而调用者往往根本意识不到手里的指针已经悬垂了。
//     fn make_ptr() -> *const i32 {
//         let x = 5;
//         &x
//     }
//     warning: function returns a dangling pointer to dropped local variable `x`
// 正确做法：让数据活得比函数更久（放到堆上用 `Box::into_raw`、放进 `static`、由调用者提供缓冲区），
// 或者干脆返回值本身而不是指针。
//
// (2) unsafe 前置条件断言（1.78）
// 标准库的许多 unsafe 函数有“前置条件”：`slice::from_raw_parts` 要求指针非空且对齐，
// `NonNull::new_unchecked` 要求不为空，`get_unchecked` 要求下标在范围内……
// 违反前置条件是未定义行为，而且常常“碰巧能跑”，直到换个编译器或优化级别才崩。
// 1.78 之前，这些检查只存在于 debug 版的标准库里，而分发给用户的标准库是 release 版，等于没有。
// 1.78 起，检查是否生效取决于**你自己**的 debug assertions 设置：
// 默认的 dev / test 构建里开启，release 构建里关闭，所以不会拖慢发布版本。
// 违反时进程会直接 abort，并打印类似这样的信息：
//     unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned
//     and non-null, and the total size of the slice not to exceed `isize::MAX`
//     thread caused non-unwinding panic. aborting.
// 因此用 `cargo test` / `cargo run` 跑 unsafe 代码，就等于免费做了一轮未定义行为抽查。
// 注意三点：
// - 这是 abort 而不是 panic，所以 `catch_unwind` 和 `#[should_panic]` 都拦不住它
// - 官方说明里明确写着，检查的范围没有稳定承诺（细节可能变），不能依赖它来保证安全
// - 想要更强的检测（别名规则、未初始化读取等），用 Miri：`cargo +nightly miri test`

// 把数据放到堆上再返回指针：数据比函数活得久，调用者负责用 `Box::from_raw` 收回
fn heap_pointer(value: i32) -> *mut i32 {
    Box::into_raw(Box::new(value))
}

fn dangling_pointers_and_precondition_checks() {
    println!("=== 悬垂指针 lint 与前置条件断言（Rust 1.91 / 1.78） ===");

    let ptr = heap_pointer(7);
    // SAFETY: ptr 刚由 Box::into_raw 得到，指向存活的堆数据；读取之后恰好收回并释放一次。
    unsafe {
        println!("堆上的数据在函数返回之后依然有效: {}", *ptr);
        drop(Box::from_raw(ptr));
    }

    // 小结：
    // 1. 返回指针前问一句：它指向的数据，比这个函数活得久吗？
    // 2. 用 debug 构建（`cargo test` / `cargo run`）跑 unsafe 代码，让标准库帮你检查前置条件
    // 3. 需要更严格的检查，用 Miri
    println!();
}

// ===========================================
// 16. 自定义全局分配器：GlobalAlloc 与 #[global_allocator]（Rust 1.28）
// ===========================================

// Rust 程序里所有的堆分配（`Box`、`Vec`、`String`、`Rc`……）最终都经过同一个“全局分配器”，
// 默认是系统分配器（Linux 上的 malloc 等）。想换成别的——
// 统计内存用量、限制内存上限、换成 jemalloc / mimalloc 来提升多线程性能、
// 在嵌入式环境里接上自己的内存池——只需要两步：
// 1. 为你的类型实现 `GlobalAlloc` trait（必须实现 `alloc` 和 `dealloc`，`realloc`、`alloc_zeroed` 可选）
// 2. 在一个 `static` 上标注 `#[global_allocator]`
//
// 约束（违反就是未定义行为或死循环）：
// - 整个程序（包括所有依赖）只能有一个 `#[global_allocator]`
// - `alloc` / `dealloc` 里**不能再分配内存**（`println!`、`Vec::push` 都不行），否则会递归调用自己
// - 实现必须是线程安全的，因为 `static` 要求 `Sync`
// - `dealloc` 收到的 `Layout` 必须和分配时用的一致
//
// 下面的示例是一个“计数分配器”：包装系统分配器，顺便统计当前存活的字节数。
// 演示里**没有**把它注册成全局分配器：`#[global_allocator]` 对整个程序生效，
// 会影响这个 crate 里所有的演示和测试，也会让并行运行的测试互相干扰计数。
// 在自己的项目里启用，只需在它后面再加两行：
//     #[global_allocator]
//     static GLOBAL: CountingAllocator = CountingAllocator::new();
//
// 另外，分配失败时怎么办？带 std 的程序会向 stderr 打印信息后 abort；
// 只依赖 `alloc` 的 `no_std` 程序，从 1.68 起（`default_alloc_error_handler` 稳定）默认会 panic，
// 可以通过 `#[panic_handler]` 调整；此前这需要 nightly 才有的 `#[alloc_error_handler]`。

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAllocator {
    live_bytes: AtomicUsize,
}

impl CountingAllocator {
    // const fn：这样才能用来初始化 `static`
    const fn new() -> Self {
        Self { live_bytes: AtomicUsize::new(0) }
    }

    // 当前存活（已分配、尚未释放）的字节数
    fn live_bytes(&self) -> usize {
        self.live_bytes.load(Ordering::Relaxed)
    }
}

// SAFETY: 所有请求原样转发给系统分配器，除此之外只维护一个原子计数器，
// 既没有再次分配内存，也满足线程安全。
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: 调用者保证 layout 的大小不为零，这里原样转交。
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            self.live_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: 调用者保证 ptr 来自本分配器，且 layout 与分配时一致。
        unsafe { System.dealloc(ptr, layout) };
        self.live_bytes.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

fn custom_global_allocator() {
    println!("=== 自定义全局分配器：GlobalAlloc 与 #[global_allocator]（Rust 1.28） ===");

    // 这里直接把分配器当作普通对象来调用，而不是通过 `Box` / `Vec` 间接使用
    let allocator = CountingAllocator::new();
    let layout = Layout::array::<u64>(8).expect("布局计算不会溢出"); // 8 个 u64 = 64 字节
    // SAFETY: layout 大小不为零；分配出来的指针只释放一次，并且用的是同一个 layout。
    unsafe {
        let ptr = allocator.alloc(layout);
        println!("分配 64 字节之后，存活字节数 = {}", allocator.live_bytes());
        allocator.dealloc(ptr, layout);
    }
    println!("释放之后，存活字节数 = {}", allocator.live_bytes());

    // 小结：
    // 1. 实现 `GlobalAlloc`，再用 `#[global_allocator]` 标在一个 static 上，整个程序的堆分配都会走它
    // 2. 分配器内部千万不能再触发堆分配
    // 3. 只想换性能更好的分配器，直接用成熟的 crate（mimalloc、jemallocator），不要自己写
    println!();
}

// ===========================================
// 附录：工具链与构建配置速查（只有文字，没有可运行的代码）
// ===========================================

// 下面这些特性发生在编译器、链接器、Cargo 层面，不是靠写 Rust 代码来使用的，所以在这里做一个速查。
// 每一条都说明：它解决什么问题，怎么开关。版本信息来自官方发布说明。
//
// 1. 基于源码的代码覆盖率（Rust 1.60）
//    `RUSTFLAGS="-C instrument-coverage" cargo test`，运行后生成 `.profraw` 文件（1.65 起默认生成不重名的文件，
//    多次运行不会互相覆盖）；再用 `llvm-profdata merge` 合并，`llvm-cov show` / `llvm-cov report` 出报告。
//    这些工具来自 `rustup component add llvm-tools`，而且必须与编译用的 rustc 是同一个版本。
//    实际项目里直接用 `cargo llvm-cov` 更省事，它把上面这些步骤封装好了。
//
// 2. 调试器可视化（Rust 1.71）
//    库作者可以把 Natvis（Visual Studio、WinDbg）或 GDB 脚本嵌进 crate，让调试器里自定义类型显示得更友好：
//        #![debugger_visualizer(natvis_file = "my_types.natvis")]
//        #![debugger_visualizer(gdb_script_file = "my_types.py")]
//    文件路径相对于写这条属性的源文件；文件不存在时是编译错误。
//
// 3. LLD 成为 x86_64 Linux 的默认链接器（Rust 1.90）
//    链接速度大幅提升（官方的例子：ripgrep 增量构建里链接耗时降到原来的七分之一）。
//    它对绝大多数项目是直接替换，但并不与 GNU ld 逐位兼容；遇到问题可以退回系统链接器：
//        RUSTFLAGS="-C linker-features=-lld"
//    或者写进 `.cargo/config.toml`：
//        [target.x86_64-unknown-linux-gnu]
//        rustflags = ["-Clinker-features=-lld"]
//
// 4. 符号修饰 v0 成为默认（Rust 1.97）
//    每个函数、静态变量在目标文件里都有一个全局唯一的“符号名”。旧方案基于 C++ 的 Itanium 修饰规则，
//    泛型参数的具体类型只体现为一个哈希；v0 方案则把模块路径和泛型实参都编进名字里，更容易被工具还原。
//    代价是较旧的调试器或性能分析工具可能无法解出名字，升级这些工具即可。
//    旧方案在稳定版里已经无法再开启（仅剩 nightly，并且计划移除）。
//
// 5. 链接器的输出不再被隐藏（Rust 1.97）
//    以前链接成功时，链接器打印的警告会被 rustc 悄悄吞掉，真正的问题因此被掩盖；现在它们以
//    `linker_messages` lint 的形式显示，默认级别是 warn。这是一个特殊的 lint：不受 `warnings` 这个 lint 组影响。
//    确认是误报、想要关掉的话，在 Cargo.toml 里写：
//        [lints.rust]
//        linker_messages = "allow"
//
// 6. Cargo 统一管控警告（Rust 1.97）
//    CI 里“有警告就失败”的老办法是 `RUSTFLAGS=-Dwarnings`，但改动 RUSTFLAGS 会让构建缓存失效。
//    现在可以让 Cargo 自己管：
//        # .cargo/config.toml
//        [build]
//        warnings = "deny"    # 可选 "allow"（静默）、"warn"（默认）、"deny"（有警告就失败）
//    也可以用环境变量临时覆盖，比如 `CARGO_BUILD_WARNINGS=allow cargo check`，不会使缓存失效。
//    CI 里配合 `--keep-going` 使用，可以一次性收集所有包的错误和警告，而不是遇到第一个失败的包就停下。

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 进阶参考");
    println!("=============");

    pointer_provenance_and_byte_offsets();
    atomic_pointer_arithmetic();
    field_offsets_with_offset_of();
    repr_transparent_structs();
    layout_queries_and_low_level_tweaks();
    pointer_roundtrips_and_unsafe_cell();
    uninitialized_and_zeroed_memory();
    pointer_and_layout_helpers();
    c_variadic_and_symbol_lints();
    abi_declarations();
    inline_assembly();
    naked_functions();
    inline_asm_labels_and_cfg();
    safe_target_feature();
    dangling_pointers_and_precondition_checks();
    custom_global_allocator();

    println!("进阶参考演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_provenance_and_byte_offsets() {
        // 低位标记：写入、读取、清除，数据完好
        #[repr(align(8))]
        struct Node(u64);
        let raw: *mut Node = Box::into_raw(Box::new(Node(42)));
        let tagged = raw.map_addr(|addr| addr | 0b101);
        assert_eq!(tagged.addr() & 0b111, 0b101);
        let clean = tagged.map_addr(|addr| addr & !0b111);
        assert_eq!(clean, raw);
        assert_eq!(unsafe { Box::from_raw(clean) }.0, 42);

        // expose / with_exposed 往返
        let value = 7_u32;
        let handle = (&value as *const u32).expose_provenance();
        assert_eq!(unsafe { *std::ptr::with_exposed_provenance::<u32>(handle) }, 7);

        // 无来源指针只有地址
        let fake: *const u32 = std::ptr::without_provenance(0x1000);
        assert_eq!(fake.addr(), 0x1000);

        // 按字节偏移：行步长 16 字节
        let image: [u32; 8] = [1, 2, 3, 0, 4, 5, 6, 0];
        let first = image.as_ptr();
        let second = unsafe { first.byte_add(16) };
        assert_eq!(unsafe { *second }, 4);
        assert_eq!(unsafe { second.byte_offset_from(first) }, 16);
    }

    #[test]
    fn test_field_offsets_with_offset_of() {
        use std::mem::offset_of;

        #[repr(C)]
        struct Header {
            magic: u32,
            version: u16,
            flags: u16,
        }
        #[repr(C)]
        struct Packet {
            header: Header,
            length: u32,
            checksum: u64,
        }

        assert_eq!(offset_of!(Packet, header.version), 4);
        assert_eq!(offset_of!(Packet, length), 8);
        assert_eq!(offset_of!(Packet, checksum), 16); // length 之后有 4 字节填充
        assert_eq!(size_of::<Packet>(), 24);

        let packet = Packet {
            header: Header { magic: 0xCAFE_BABE, version: 2, flags: 1 },
            length: 128,
            checksum: 0xAB,
        };
        let base: *const Packet = &raw const packet;
        let length_ptr = unsafe { base.byte_add(offset_of!(Packet, length)).cast::<u32>() };
        assert_eq!(unsafe { length_ptr.read() }, 128);
        let recovered = unsafe { length_ptr.byte_sub(offset_of!(Packet, length)).cast::<Packet>() };
        assert!(std::ptr::eq(recovered, base));
    }

    #[test]
    fn test_layout_queries_and_low_level_tweaks() {
        use std::alloc::Layout;
        use std::cell::Cell;
        use std::mem::{align_of_val_raw, size_of_val_raw};
        use std::rc::Rc;

        // 切片裸指针：大小 = 元素个数 × 元素大小
        let numbers: [u32; 5] = [1, 2, 3, 4, 5];
        let slice_ptr: *const [u32] = &numbers[..3];
        assert_eq!(unsafe { size_of_val_raw(slice_ptr) }, 12);
        assert_eq!(unsafe { align_of_val_raw(slice_ptr) }, 4);
        assert_eq!(unsafe { Layout::for_value_raw(slice_ptr) }, Layout::array::<u32>(3).unwrap());

        // str 也是动态大小类型
        let text_ptr: *const str = "你好";
        assert_eq!(unsafe { size_of_val_raw(text_ptr) }, 6);

        // trait 对象：布局取决于具体类型；手动释放 Box<dyn Trait> 不能泄漏也不能重复释放
        trait Probe {
            fn id(&self) -> u8;
        }
        struct Small(Rc<Cell<u32>>);
        struct Large(Rc<Cell<u32>>, [u64; 4]);
        impl Probe for Small {
            fn id(&self) -> u8 {
                1
            }
        }
        impl Probe for Large {
            fn id(&self) -> u8 {
                2 + self.1[0] as u8 // 读取第二个字段，让它参与计算
            }
        }
        impl Drop for Small {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        impl Drop for Large {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 10);
            }
        }
        let drops = Rc::new(Cell::new(0));
        let boxes: Vec<Box<dyn Probe>> = vec![
            Box::new(Small(Rc::clone(&drops))),
            Box::new(Large(Rc::clone(&drops), [0; 4])),
        ];
        let ids: Vec<u8> = boxes.iter().map(|probe| probe.id()).collect();
        assert_eq!(ids, [1, 2]);
        let mut sizes = vec![];
        for boxed in boxes {
            let raw: *mut dyn Probe = Box::into_raw(boxed);
            unsafe {
                let layout = Layout::for_value_raw(raw);
                sizes.push(layout.size());
                std::ptr::drop_in_place(raw);
                if layout.size() != 0 {
                    std::alloc::dealloc(raw as *mut u8, layout);
                }
            }
        }
        assert_eq!(sizes, [size_of::<Small>(), size_of::<Large>()]);
        assert_ne!(sizes[0], sizes[1]); // 同一个 *mut dyn Probe 类型，实际大小不同
        assert_eq!(drops.get(), 11); // 两个析构函数各运行了一次

        // transmute_copy 的源类型可以是切片
        let bytes: &[u8] = &[0x78, 0x56, 0x34, 0x12, 0xFF];
        let word: u32 = unsafe { std::mem::transmute_copy::<[u8], u32>(bytes) };
        assert_eq!(word, u32::from_ne_bytes([0x78, 0x56, 0x34, 0x12]));

        // 缩短 &mut 生命周期的 unsize 转换
        fn shorten<'long: 'short, 'short>(
            cell: Cell<&'long mut i32>,
        ) -> Cell<&'short mut dyn Send> {
            cell
        }
        let mut value = 7;
        let _ = shorten(Cell::new(&mut value)); // 临时值在语句结束时释放，借用随之结束
        assert_eq!(value, 7);

        // &raw mut：不经过引用直接取得裸指针
        let mut counter = 0_u32;
        let ptr = &raw mut counter;
        unsafe {
            ptr.write(41);
            *ptr += 1;
        }
        assert_eq!(counter, 42);

        // x86_64：128 位整数经由 xmm 寄存器做 asm! 操作数
        #[cfg(target_arch = "x86_64")]
        {
            use std::arch::asm;
            let a: u128 = 0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000;
            let b: u128 = 0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF;
            let out: u128;
            unsafe {
                asm!(
                    "pxor {0}, {1}",
                    inout(xmm_reg) a => out,
                    in(xmm_reg) b,
                    options(pure, nomem, nostack)
                );
            }
            assert_eq!(out, a ^ b);
        }
    }

    #[test]
    fn test_pointer_roundtrips_and_unsafe_cell() {
        use std::cell::{Cell, UnsafeCell};
        use std::mem::ManuallyDrop;
        use std::ptr::NonNull;
        use std::rc::Rc;

        // into_non_null / from_non_null 往返，内容不变
        let handle: NonNull<String> = Box::into_non_null(Box::new(String::from("堆上的数据")));
        let recovered = unsafe { Box::from_non_null(handle) };
        assert_eq!(*recovered, "堆上的数据");

        // Vec::into_parts / from_parts 往返
        let (ptr, len, cap) = vec![1_u32, 2, 3].into_parts();
        assert_eq!(len, 3);
        let rebuilt = unsafe { Vec::from_parts(ptr, len, cap) };
        assert_eq!(rebuilt, [1, 2, 3]);

        // as_mut_ptr 多次调用，先取出的指针依然有效
        let mut value = Box::new(10_i32);
        let first = Box::as_mut_ptr(&mut value);
        let second = Box::as_mut_ptr(&mut value);
        unsafe {
            first.write(11);
            second.write(first.read() + 1);
        }
        assert_eq!(*value, 12);
        assert_eq!(unsafe { Box::as_ptr(&value).read() }, 12);

        // ManuallyDrop：离开作用域不会自动 drop，手动 drop 恰好一次
        struct Counted(Rc<Cell<u32>>);
        impl Drop for Counted {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        {
            let mut manual = ManuallyDrop::new(Counted(Rc::clone(&drops)));
            assert_eq!(drops.get(), 0);
            unsafe { ManuallyDrop::drop(&mut manual) };
            assert_eq!(drops.get(), 1);
        }
        assert_eq!(drops.get(), 1);

        // 内容已被 drop 的 ManuallyDrop<Box<_>> 可以被移动（1.98 起的文档保证）
        let mut slot = ManuallyDrop::new(Box::new(String::from("data")));
        unsafe { ManuallyDrop::drop(&mut slot) };
        let moved = slot;
        let _ = &moved;

        // UnsafeCell：不经过 get()，直接转成裸指针读写
        let cell = UnsafeCell::new(5_u32);
        let direct = &cell as *const UnsafeCell<u32> as *mut u32;
        unsafe { *direct += 1 };
        assert_eq!(cell.into_inner(), 6);
    }

    #[test]
    fn test_abi_declarations() {
        // 显式 "C" 的函数指针
        let callback: extern "C" fn(c_int) -> c_int = increment;
        assert_eq!(callback(41), 42);

        // 1.93："system" 也能声明变参函数，函数指针和普通指针一样大
        assert_eq!(size_of::<unsafe extern "system" fn(*const std::ffi::c_char, ...) -> c_int>(), size_of::<usize>());

        // "C-unwind"：正常路径返回结果，panic 路径可以被 catch_unwind 接住
        assert_eq!(checked_divide(100, 5), 20);
        assert!(std::panic::catch_unwind(|| checked_divide(1, 0)).is_err());

        // x86_64 的 Unix：按 sysv64 声明的变参 snprintf 写入了 "12-34"，共 5 个字符
        #[cfg(all(target_arch = "x86_64", unix))]
        assert_eq!(sysv64_snprintf_len(), 5);

        #[cfg(windows)]
        assert!(windows_tick_count() > 0);
    }

    #[test]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    fn test_naked_functions() {
        assert_eq!(naked_add(40, 2), 42);
        assert_eq!(naked_add(-5, 3), -2); // 负数同样按 32 位补码相加
        assert_eq!(naked_add(i32::MAX, 1), i32::MIN); // 32 位回绕，不会变成 64 位
    }

    #[test]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    fn test_inline_asm_labels_and_cfg() {
        assert!(is_zero_via_asm_goto(0));
        assert!(!is_zero_via_asm_goto(7));
        assert!(!is_zero_via_asm_goto(u64::MAX));
        assert_eq!(forty_two_via_asm_cfg(), 42);
    }

    #[test]
    fn test_safe_target_feature() {
        let weights = [1.0_f32, 2.0, 3.0];
        let inputs = [4.0_f32, 5.0, 6.0];
        // 分发后的结果必须与通用实现一致
        assert_eq!(scaled_dot_product(&weights, &inputs, 2.0), dot_scalar(&weights, &inputs) * 2.0);
        assert_eq!(scaled_dot_product(&weights, &inputs, 2.0), 64.0);
    }

    #[test]
    fn test_dangling_pointers_and_precondition_checks() {
        // 返回的堆指针在函数返回之后依然有效，收回时恰好释放一次
        let ptr = heap_pointer(7);
        assert_eq!(unsafe { *ptr }, 7);
        unsafe { drop(Box::from_raw(ptr)) };
    }

    #[test]
    fn test_custom_global_allocator() {
        let allocator = CountingAllocator::new();
        let layout = Layout::array::<u64>(8).unwrap();
        assert_eq!(layout.size(), 64);
        unsafe {
            let first = allocator.alloc(layout);
            let second = allocator.alloc(layout);
            assert!(!first.is_null() && !second.is_null());
            assert_eq!(allocator.live_bytes(), 128);
            allocator.dealloc(first, layout);
            assert_eq!(allocator.live_bytes(), 64);
            allocator.dealloc(second, layout);
        }
        assert_eq!(allocator.live_bytes(), 0);
    }

    #[test]
    fn test_atomic_pointer_arithmetic() {
        use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
        use std::thread;

        // 多个线程并发认领槽位：每个槽位恰好被认领一次
        let slots: [AtomicU32; 8] = std::array::from_fn(|_| AtomicU32::new(0));
        let cursor = AtomicPtr::new(slots.as_ptr().cast_mut());
        let cursor_ref = &cursor;
        thread::scope(|scope| {
            for worker in 1..=8_u32 {
                scope.spawn(move || {
                    let claimed = cursor_ref.fetch_ptr_add(1, Ordering::Relaxed);
                    unsafe { (*claimed).store(worker, Ordering::Relaxed) };
                });
            }
        });
        let mut owners: Vec<u32> = slots.iter().map(|slot| slot.load(Ordering::Relaxed)).collect();
        owners.sort();
        assert_eq!(owners, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(unsafe { cursor.load(Ordering::Relaxed).offset_from(slots.as_ptr()) }, 8);

        // 以 T 为单位与以字节为单位、加与减
        let base = AtomicPtr::<u64>::new(std::ptr::null_mut());
        assert_eq!(base.fetch_ptr_add(2, Ordering::Relaxed).addr(), 0);
        assert_eq!(base.load(Ordering::Relaxed).addr(), 16); // 2 * size_of::<u64>()
        assert_eq!(base.fetch_byte_add(3, Ordering::Relaxed).addr(), 16);
        assert_eq!(base.load(Ordering::Relaxed).addr(), 19);
        assert_eq!(base.fetch_ptr_sub(1, Ordering::Relaxed).addr(), 19);
        assert_eq!(base.load(Ordering::Relaxed).addr(), 11);
        assert_eq!(base.fetch_byte_sub(11, Ordering::Relaxed).addr(), 11);
        assert_eq!(base.load(Ordering::Relaxed).addr(), 0);

        // 位运算：标记与清除
        #[repr(align(8))]
        struct Node(u32);
        let node = Node(7);
        let shared = AtomicPtr::new((&node as *const Node).cast_mut());
        let before = shared.fetch_or(0b1, Ordering::AcqRel);
        assert_eq!(before.addr() & 1, 0);
        let marked = shared.load(Ordering::Acquire);
        assert_eq!(marked.addr() & 1, 1);
        let previous = shared.fetch_and(!1, Ordering::AcqRel);
        assert_eq!(previous.addr() & 1, 1);
        assert_eq!(unsafe { (*shared.load(Ordering::Acquire)).0 }, 7);
    }

    // x86_64 Linux 专属：内联汇编封装函数的行为测试。
    // 这类平台专属代码在别的平台上根本不会被编译到，曾经在 x86_64 Linux 上长期无法通过编译却无人察觉，
    // 所以必须有测试保护：在目标平台上跑 `cargo test` 就会执行这些汇编。
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    #[test]
    fn test_inline_assembly_x86_64_linux() {
        use super::asm_x86_64_linux as asm_fns;

        assert!(asm_fns::cpu_has_sse2()); // SSE2 属于 x86_64 的基线指令集
        assert_eq!(asm_fns::getpid_via_syscall(), u64::from(std::process::id()));

        let mut slot = 42_u32;
        assert_eq!(asm_fns::atomic_swap(&mut slot, 100), 42);
        assert_eq!(slot, 100);

        assert_eq!(
            asm_fns::add_f32x4([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]),
            [6.0, 8.0, 10.0, 12.0]
        );
        assert_eq!(asm_fns::fast_abs(-100), 100);
        assert_eq!(asm_fns::fast_abs(7), 7);
        assert_eq!(asm_fns::fast_abs(0), 0);
        assert_eq!(asm_fns::byte_swap(0x1234_5678), 0x7856_3412);
        assert_eq!(asm_fns::shift_left_by_const(5), 40);
    }

    #[test]
    fn test_repr_transparent() {
        use std::marker::PhantomData;

        #[repr(transparent)]
        struct UserId(i32);
        #[repr(transparent)]
        struct Meters(f64);
        // 零大小的字段（如 PhantomData）可以和唯一的非零大小字段共存
        #[repr(transparent)]
        struct Tagged<T>(u64, PhantomData<T>);

        assert_eq!(size_of::<UserId>(), size_of::<i32>());
        assert_eq!(align_of::<UserId>(), align_of::<i32>());
        assert_eq!(size_of::<Meters>(), size_of::<f64>());
        assert_eq!(size_of::<Tagged<String>>(), size_of::<u64>());
        assert_eq!(align_of::<Tagged<String>>(), align_of::<u64>());

        // 布局相同，所以把 &[i32] 当作 &[UserId] 是合法的（这正是 FFI 里“透明传递”的依据）
        let raw = [1_i32, 2, 3];
        // SAFETY: UserId 是 #[repr(transparent)] 的 i32 包装器，大小、对齐和有效值范围都与 i32 相同。
        let ids: &[UserId] = unsafe { &*(&raw[..] as *const [i32] as *const [UserId]) };
        assert_eq!(ids.iter().map(|id| id.0).sum::<i32>(), 6);
        assert_eq!(Meters(2.5).0, 2.5);
    }

    #[test]
    fn test_vec_string_into_raw_parts() {
        let values = vec![1_u8, 2, 3, 4];
        let (ptr, len, cap) = values.into_raw_parts();
        let rebuilt = unsafe { Vec::from_raw_parts(ptr, len, cap) };
        assert_eq!(rebuilt, vec![1, 2, 3, 4]);

        let text = String::from("ffi");
        let (ptr, len, cap) = text.into_raw_parts();
        let rebuilt = unsafe { String::from_raw_parts(ptr, len, cap) };
        assert_eq!(rebuilt, "ffi");
    }

    #[test]
    fn test_uninitialized_and_zeroed_memory() {
        use std::mem::MaybeUninit;
        use std::rc::Rc;
        use std::sync::Arc;

        // MaybeUninit 数组与整块内存互相转换（Rust 1.95）
        let mut slots: [MaybeUninit<u32>; 3] = [MaybeUninit::uninit(); 3];
        slots[0].write(10);
        slots[1].write(20);
        slots[2].write(30);
        let whole: MaybeUninit<[u32; 3]> = slots.into();
        let slots_again: [MaybeUninit<u32>; 3] = whole.into();
        let sum: u32 = unsafe { slots_again.into_iter().map(|s| s.assume_init()).sum() };
        assert_eq!(sum, 60);

        // new_zeroed（Rust 1.92）：对“全零位模式合法”的类型，assume_init 是安全的
        let boxed = unsafe { Box::<u32>::new_zeroed().assume_init() };
        assert_eq!(*boxed, 0);
        let rc = unsafe { Rc::<u16>::new_zeroed().assume_init() };
        assert_eq!(*rc, 0);
        let arc = unsafe { Arc::<u8>::new_zeroed().assume_init() };
        assert_eq!(*arc, 0);
    }

    #[test]
    fn test_pointer_and_layout_helpers() {
        use std::cell::Cell;

        // union 字段取原始指针（Rust 1.92）：只取地址，解引用时才读取
        #[repr(C)]
        union Bytes {
            raw: u32,
            bytes: [u8; 4],
        }
        let value = Bytes { raw: 0xAABBCCDD };
        let ptr = &raw const value.bytes;
        assert_eq!(unsafe { *ptr }, 0xAABBCCDD_u32.to_ne_bytes());

        // Cell 数组当作 [Cell<T>; N] 看待（Rust 1.95）
        let cells = Cell::new([1u32, 2, 3]);
        let as_array: &[Cell<u32>; 3] = <Cell<[u32; 3]> as AsRef<[Cell<u32>; 3]>>::as_ref(&cells);
        as_array[0].set(as_array[0].get() + 1);
        assert_eq!(cells.get(), [2, 2, 3]);

        // 裸指针 as_ref_unchecked / as_mut_unchecked（Rust 1.95）
        let number = 42u32;
        let raw_const = &number as *const u32;
        assert_eq!(unsafe { raw_const.as_ref_unchecked() }, &42);
        let mut mutable = 7u32;
        let raw_mut = &mut mutable as *mut u32;
        *unsafe { raw_mut.as_mut_unchecked() } += 1;
        assert_eq!(mutable, 8);

        // Layout 的 repeat 系列（Rust 1.95）
        let elem = std::alloc::Layout::new::<u32>();
        let (array_layout, stride) = elem.repeat(4).unwrap();
        assert_eq!(stride, 4);
        assert_eq!(array_layout.size(), 16);
        assert_eq!(elem.repeat_packed(4).unwrap().size(), 16);
    }

    #[test]
    fn test_c_variadic_definitions() {
        // 定长参数列表之后的变参：个数由第一个参数决定
        assert_eq!(unsafe { variadic_sum(0) }, 0);
        assert_eq!(unsafe { variadic_sum(3, 10, 20, 30) }, 60);
        assert_eq!(unsafe { variadic_sum(2, -5, 5) }, 0);

        // 混合类型：c_double 与 c_int 按顺序读取
        assert_eq!(unsafe { variadic_weighted(2.5_f64, 4 as c_int) }, 10.0);

        // VaList 转交给辅助函数 + clone（va_copy）：两个游标互不影响
        let mut max = 0;
        let sum = unsafe { variadic_sum_max(4, &mut max, 3, 9, 4, 7) };
        assert_eq!((sum, max), (23, 9));
    }
}
