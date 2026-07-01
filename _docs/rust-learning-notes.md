# Rust 学习笔记

## 1. 基础概念

### 1.1 变量与可变性

```rust
// 不可变变量（默认）
let x = 5;
// x = 6;  // ✗ 报错

// 可变变量
let mut y = 5;
y = 6;  // ✓

// Shadowing（遮蔽）
let z = 5;
let z = z + 1;  // 创建新变量，类型可以改变
```

**`mut` vs shadowing：**
- `mut`：同一个变量，类型不能变
- shadowing：创建新变量，类型可以变

### 1.2 常量

```rust
const MAX_SIZE: i32 = 100;  // 必须大写 + 下划线
```

---

## 2. 数据类型

### 2.1 基本类型

| 类型 | 说明 | 示例 |
|------|------|------|
| `i32`, `u32` | 有符号/无符号整数 | `let x: i32 = 5;` |
| `f64` | 浮点数 | `let y: f64 = 2.3;` |
| `bool` | 布尔值 | `let t: bool = true;` |
| `char` | 单个字符 | `let c: char = 'A';` |

### 2.2 usize

- **无符号整数**，大小取决于平台（32位/64位）
- 主要用于**数组索引、长度、大小**
- `.len()` 返回 `usize`

```rust
let arr = [1, 2, 3];
let index: usize = 0;
let value = arr[index];  // 索引必须是 usize
```

### 2.3 元组（Tuple）

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 访问元组元素
let a = tup.0;  // 500
let b = tup.1;  // 6.4

// 解构
let (x, y, z) = tup;
```

**用途：**
- 函数返回多个值
- 临时组合不同类型的数据

### 2.4 数组 vs Vec

| | 数组 `[T; N]` | Vec `Vec<T>` |
|--|--|--|
| 长度 | 固定 | 动态 |
| 存储 | 栈 | 堆 |
| 追加 | ✗ | ✓ `push()` |

```rust
// 数组
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// Vec
let mut v: Vec<i32> = vec![1, 2, 3];
v.push(4);
```

### 2.5 切片 `&[T]`

指向数组或 Vec 的一段数据的引用：

```rust
let v = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &v[1..4];  // [2, 3, 4]

// 左闭右开区间
&v[0..5]   // 不含索引5
&v[0..=4]  // 含索引4
```

---

## 3. String vs &str

| | `String` | `&str` |
|--|--|--|
| 类型 | 拥有所有权的字符串 | 字符串切片引用 |
| 存储 | 堆上，动态 | 指向已有内存 |
| 可变 | ✓ | ✗ |

```rust
// String
let mut s = String::from("hello");
s.push_str(" world");  // ✓

// &str
let s: &str = "hello";
// s.push('!');  // ✗ 不可变

// 字符串字面量在编译时就写入二进制文件
let s = "hello";  // s 是指针，指向二进制中的固定位置
```

**函数参数优先用 `&str`：**

```rust
fn print(s: &str) {  // ✓ 更通用
    println!("{}", s);
}

// 可以传 &String 或 &str
print(&String::from("hello"));
print("world");
```

---

## 4. 所有权（Ownership）

### 4.1 核心规则

1. 每个值只有一个所有者
2. 当所有者离开作用域，值被释放
3. 赋值/传参会转移所有权（没有 `Copy` trait 的类型）

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 所有权转移给 s2
// println!("{}", s1);  // ✗ s1 已失效
```

### 4.2 Copy vs Move

**实现了 `Copy` 的类型（赋值时复制）：**
- 基本类型：`i32`, `f64`, `bool`, `char`
- 元组（如果所有元素都是 `Copy`）

**没有 `Copy` 的类型（赋值时移动）：**
- `String`
- `Vec`
- 自定义结构体（默认）

```rust
// i32 有 Copy
let x = 5;
let y = x;  // 复制
println!("{}, {}", x, y);  // ✓

// String 没有 Copy
let s1 = String::from("hello");
let s2 = s1;  // 移动
// println!("{}", s1);  // ✗
```

### 4.3 借用（Borrowing）

不转移所有权，只借用：

```rust
let s = String::from("hello");
let r = &s;  // 借用
println!("{}, {}", s, r);  // ✓ s 还有效
```

**可变借用：**

```rust
let mut s = String::from("hello");
let r = &mut s;  // 可变借用
r.push_str(" world");
println!("{}", r);
```

**借用规则：**
- 同一时间只能有**一个可变借用**
- 或者**多个不可变借用**
- 可变借用期间，原变量不能使用

---

## 5. 函数

### 5.1 表达式 vs 语句

```rust
// 有分号 = 语句，返回 ()
let x = 5;

// 无分号 = 表达式，返回值
let y = {
    let a = 5;
    a + 1  // 无分号，返回 6
};
```

### 5.2 返回值

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 无分号，隐式返回
}

fn add2(a: i32, b: i32) -> i32 {
    return a + b;  // 显式返回
}
```

**何时用 `return`：**
- 提前退出函数时用 `return`
- 最后一行推荐用隐式返回

---

## 6. 结构体（Struct）

### 6.1 定义与使用

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};

println!("{:?}", user);  // 需要 #[derive(Debug)]
```

### 6.2 方法与关联函数

```rust
impl User {
    // 关联函数（没有 self）
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }

    // 方法（有 &self）
    fn print(&self) {
        println!("{}", self.email);
    }
}

// 调用
let user = User::new(  // 用 User:: 调用关联函数
    String::from("alice"),
    String::from("alice@example.com")
);
user.print();  // 用 . 调用方法
```

**区别：**
- `User::`（双冒号）：类型级，调用关联函数
- `user.`（单点）：实例级，调用方法

---

## 7. 常见格式化

| 占位符 | trait | 用途 |
|--------|-------|------|
| `{}` | `Display` | 用户友好输出 |
| `{:?}` | `Debug` | 调试输出 |
| `{:#?}` | `Debug` | 美化调试输出 |

```rust
let x = 5;
println!("{}", x);   // Display

let arr = [1, 2, 3];
println!("{:?}", arr);   // Debug
println!("{:#?}", arr);  // 美化输出
```

---

## 8. 错误处理

### 8.1 Result 和 Option

```rust
// parse 返回 Result<T, E>
let num: i32 = "42".parse().expect("Not a number");

// 用 match 处理
let num: i32 = match "42".parse() {
    Ok(n) => n,
    Err(_) => {
        println!("Parse failed");
        0  // 默认值
    }
};
```

### 8.2 expect vs unwrap vs match

| 方法 | 错误时行为 | 适用场景 |
|------|----------|---------|
| `expect()` | panic，显示自定义信息 | 学习、原型 |
| `unwrap()` | panic，显示默认信息 | 确定不会失败 |
| `match` | 优雅处理错误 | 生产环境 |
| `unwrap_or()` | 返回默认值 | 有合理默认值 |

### 8.3 panic!

主动触发程序崩溃：

```rust
panic!("Something went wrong!");
```

运行时带堆栈跟踪：
```bash
RUST_BACKTRACE=1 cargo run
```

---

## 9. 迭代器

### 9.1 .iter() vs 直接遍历

```rust
let v = vec![1, 2, 3];

// 直接遍历（i32 有 Copy，所以复制值）
for item in v {
    // item 是 i32
}

// 用 .iter() 借用
for item in &v {
    // item 是 &i32
}
```

**String 的区别：**

```rust
let v = vec![String::from("a"), String::from("b")];

// 移走所有权
for item in v {
    // item 是 String
}
// v 失效

// 借用
for item in &v {
    // item 是 &String
}
// v 还有效
```

### 9.2 .enumerate()

附加索引：

```rust
let v = vec![10, 20, 30];
for (index, value) in v.iter().enumerate() {
    println!("{}: {}", index, value);
}
// 0: 10
// 1: 20
// 2: 30
```

---

## 10. 解引用

用 `*` 从引用获取值：

```rust
let x = 5;
let r = &x;   // r 是 &i32

println!("{}", r);   // 5（println! 自动解引用）
println!("{}", *r);  // 5（手动解引用）

if *r > 3 { }  // 比较时需要手动解引用
```

**遍历切片时：**

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list {
        if *item > largest {  // item 是 &i32，需要解引用
            largest = *item;
        }
    }
    largest
}
```

### 10.1 索引 vs 遍历的类型差异

**关键问题：为什么 `list[0]` 是 `i32`，但 `for item in list` 中 `item` 是 `&i32`？**

```rust
let list: &[i32] = &[1, 2, 3];

let x = list[0];     // x 是 i32（Copy 自动复制）
for item in list {   // item 是 &i32（借用）
}
```

**原因：与所有权密切相关**

| 操作 | 行为 | 原因 |
|------|------|------|
| `list[0]` | 对 `Copy` 类型复制值 | 索引操作对实现了 `Copy` 的类型会自动复制 |
| `for item in list` | 总是借用元素 | 遍历借用的切片只能借用元素，不能拿走所有权 |

**`i32` 有 Copy：**

```rust
let list: &[i32] = &[1, 2, 3];
let x = list[0];     // ✓ 复制值，x 是 i32
for item in list {   // item 是 &i32（即使 i32 有 Copy，遍历时也是借用）
}
```

**`String` 没有 Copy：**

```rust
let list: &[String] = &[String::from("a")];
let x = list[0];     // ✗ 报错！不能从 &[String] 中移走所有权
for item in list {   // item 是 &String（借用）
}
```

**核心规则：**
- **索引操作** `list[0]`：如果类型有 `Copy` 就复制，没有 `Copy` 就报错
- **遍历操作** `for item in list`：无论是否有 `Copy`，都是借用（`&T`）

这是 Rust 所有权系统的体现：你只借用了切片（`&[T]`），所以只能借用元素，不能拿走。

---

## 11. 内存管理对比

| 语言 | 方式 | 运行时开销 |
|------|------|-----------|
| Go/JS/PHP | GC（垃圾回收） | 有，会暂停 |
| C/C++ | 手动 malloc/free | 无，易出错 |
| **Rust** | **所有权系统** | **无，编译时检查** |

Rust 的目标：既有 C 的性能，又有 GC 语言的内存安全。

---

## 12. 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 变量、函数 | `snake_case` | `user_name`, `get_value()` |
| 常量、静态变量 | `SCREAMING_SNAKE_CASE` | `MAX_SIZE` |
| 结构体、枚举、trait | `PascalCase` | `UserInfo`, `OrderStatus` |

---

## 13. 常用方法

### String 方法

```rust
let mut s = String::from("hello");
s.push('!');              // 追加字符
s.push_str(" world");     // 追加字符串
s.clear();                // 清空

let len = s.len();        // 长度
let trimmed = s.trim();   // 去空白（返回 &str）
```

### Vec 方法

```rust
let mut v = vec![1, 2, 3];
v.push(4);                // 追加
v.pop();                  // 移除最后一个
let len = v.len();        // 长度
```

---

## 14. 实用技巧

### 14.1 字符串拼接

```rust
let s1 = String::from("Hello");
let s2 = String::from("World");

// + 操作符（s1 被移走）
let s3 = s1 + " " + &s2;

// format!（推荐，不移走所有权）
let s3 = format!("{} {}", s1, s2);
```

### 14.2 类型转换

```rust
// &str → String
let s: String = "hello".to_string();
let s: String = String::from("hello");

// String → &str
let s = String::from("hello");
let slice: &str = &s;

// 数组 → Vec
let arr = [1, 2, 3];
let v: Vec<i32> = arr.to_vec();
```

### 14.3 常用 derive

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
}
```

| derive | 作用 |
|--------|------|
| `Debug` | 可用 `{:?}` 打印 |
| `Clone` | 可用 `.clone()` 复制 |
| `PartialEq` | 可用 `==` 比较 |
| `Copy` | 赋值时自动复制 |

---

## 15. 其他语言对比

### Rust vs Go

| | Rust | Go |
|--|--|--|
| 数组 | `[T; N]` | `[N]T` |
| 动态数组 | `Vec<T>` | `[]T`（切片） |
| 切片引用 | `&[T]` | `[]T`（切片本身） |
| 内存管理 | 所有权 | GC |

### Rust vs JS/PHP

```javascript
// JS 中 [] 是数组
let arr = [1, 2, 3];
arr.push(4);  // 可追加

// Rust 中 [] 是固定长度数组，vec![] 才是动态的
let arr = [1, 2, 3];  // 不可追加
let v = vec![1, 2, 3];  // 可追加
```

---

## 常见错误及解决

### 1. 类型推断错误

```rust
// ✗ 错误
let x = "42".parse().expect("error");

// ✓ 正确
let x: i32 = "42".parse().expect("error");
let x = "42".parse::<i32>().expect("error");
```

### 2. 借用后修改

```rust
// ✗ 错误
let s = String::from("hello");
let r = &s;
s.clear();  // s 被借用，不能修改

// ✓ 正确
let mut s = String::from("hello");
let r = &s;
println!("{}", r);  // r 使用完毕
s.clear();  // 现在可以修改
```

### 3. 返回局部变量引用

```rust
// ✗ 错误
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // s 被释放，返回悬垂引用
}

// ✓ 正确
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 转移所有权
}
```

---

## 学习路径

1. ✅ 变量、数据类型、函数
2. ✅ 所有权、借用、引用
3. ✅ 结构体、方法
4. ✅ Vec、String、切片
5. ⏭️ 枚举、模式匹配
6. ⏭️ 错误处理（Result、Option）
7. ⏭️ 泛型、trait
8. ⏭️ 生命周期
9. ⏭️ 模块系统
10. ⏭️ 并发编程

---

**持续更新中...**
