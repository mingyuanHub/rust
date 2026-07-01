# Rust 集合类型

> String、Vec、数组、切片的详细对比

## 1. String vs &str

### 1.1 类型对比

| | `String` | `&str` |
|--|--|--|
| 类型 | 拥有所有权的字符串 | 字符串切片引用 |
| 存储 | 堆上，动态分配 | 指向已有内存 |
| 长度 | 可动态增长 | 固定，不可变 |
| 所有权 | 有 | 无（借用） |

### 1.2 内存结构

```
String:
栈: [ptr | len | capacity]
      │
      ↓
堆: [h][e][l][l][o][剩余容量...]

&str:
栈: [ptr | len]
      │
      ↓
    [h][e][l][l][o]  ← 指向某块已有内存
```

### 1.3 &str 可以指向哪里

```rust
// 1. 字符串字面量（二进制文件中）
let s1: &str = "hello";

// 2. String 的一部分
let s = String::from("hello world");
let s2: &str = &s[0..5];

// 3. 整个 String
let s3: &str = &s;
```

### 1.4 String 常用方法

```rust
let mut s = String::from("hello");

s.push('!');              // 追加字符
s.push_str(" world");     // 追加字符串
s.clear();                // 清空

let len = s.len();        // 长度
let trimmed = s.trim();   // 去空白（返回 &str）
```

### 1.5 字符串拼接

```rust
let s1 = String::from("Hello");
let s2 = String::from("World");

// + 操作符（s1 被移走）
let s3 = s1 + " " + &s2;

// format!（推荐，不移走所有权）
let s3 = format!("{} {}", s1, s2);
```

### 1.6 类型转换

```rust
// &str → String
let s: String = "hello".to_string();
let s: String = String::from("hello");

// String → &str
let s = String::from("hello");
let slice: &str = &s;
```

### 1.7 to_string() 详解

`to_string()` 把任何实现了 `Display` trait 的类型转成 `String`：

```rust
// &str → String
let s = "hello".to_string();

// &String → String（解引用并复制）
let s1 = String::from("hello");
let s2 = (&s1).to_string();

// 数字/bool → String
let n = 42.to_string();      // "42"
let f = 3.14.to_string();    // "3.14"
let b = true.to_string();    // "true"
```

**to_string() vs String::from() vs clone()：**

```rust
// 三种写法效果一样（从 &str 创建 String）
let s1 = "hello".to_string();
let s2 = String::from("hello");
let s3 = "hello".to_owned();

// 从引用复制（两种写法效果一样）
let original = String::from("hello");
let copy1 = original.clone();
let copy2 = original.to_string();
```

**实际使用建议：**
- `&str` → `String`：用 `String::from()` 或 `.to_string()`
- 复制已有 `String`：用 `.clone()`
- 数字/bool 转字符串：用 `.to_string()`

### 1.8 Vec 元素的借用 vs 克隆

不能直接移走 Vec 里的元素，只有两种选择：

```rust
let args = vec![String::from("a"), String::from("b")];

let s1 = args[0];         // ✗ 不能移走元素
let s1 = args[0].clone(); // ✓ 复制一份，得到独立的 String
let s2 = &args[1];        // ✓ 借用，得到 &String

// 优先用借用，只有需要独立修改时才 clone
let s = &args[0];         // 推荐
```

### 1.7 函数参数优先用 &str

```rust
// ✓ 推荐：&str 更通用
fn print(s: &str) {
    println!("{}", s);
}

// 可以传 &String 或 &str
print(&String::from("hello"));
print("world");
```

```rust
// ✗ 不推荐：只能传 &String
fn print(s: &String) {
    println!("{}", s);
}
```

---

## 2. Vec vs 数组

### 2.1 类型对比

| | `Vec<T>` | `[T; N]` |
|--|--|--|
| 类型 | 动态数组 | 固定长度数组 |
| 长度 | 可变 | 编译时确定 |
| 存储 | 堆 | 栈 |
| 追加 | ✓ `push()` | ✗ |

### 2.2 Vec 使用

```rust
// 创建
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];

// 追加
v.push(4);
v.extend([5, 6]);

// 访问
let first = v[0];        // 直接索引（panic if 越界）
let first = v.get(0);    // 返回 Option<&T>（安全）

// 遍历
for item in &v {
    println!("{}", item);
}

// 其他方法
v.pop();           // 移除最后一个，返回 Option<T>
let len = v.len(); // 长度
v.clear();         // 清空
```

### 2.3 数组使用

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// 快速初始化
let arr = [0; 100];  // 100个0

// 访问
let first = arr[0];

// 遍历
for item in arr {
    println!("{}", item);
}

// 不能追加
// arr.push(6);  // ✗ 没有这个方法
```

### 2.4 数组转 Vec

```rust
let arr = [1, 2, 3];
let v: Vec<i32> = arr.to_vec();  // 复制到堆上
```

---

## 3. 切片 &[T]

### 3.1 什么是切片

切片是**对数组或 Vec 的一段数据的引用**：

```rust
let v = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &v[1..4];  // [2, 3, 4]

let arr = [1, 2, 3];
let slice: &[i32] = &arr[..];  // 整个数组
```

### 3.2 切片语法

```rust
let v = vec![0, 1, 2, 3, 4];

&v[0..5]   // 左闭右开：0,1,2,3,4（不含5）
&v[0..=4]  // 左闭右闭：0,1,2,3,4（含4）
&v[1..]    // 从1到末尾
&v[..3]    // 从0到2
&v[..]     // 整个Vec
```

**为什么是左闭右开？**
- 长度 = 结束 - 开始（`5 - 0 = 5`）
- 相邻切片不重叠
- 空区间自然表示（`&v[3..3]` 是空切片）

### 3.3 字符串切片

```rust
let s = String::from("hello world");

let hello: &str = &s[0..5];   // "hello"
let world: &str = &s[6..11];  // "world"
```

### 3.4 函数参数用切片

```rust
// ✓ 推荐：&[T] 更通用
fn sum(list: &[i32]) -> i32 {
    let mut total = 0;
    for &item in list {
        total += item;
    }
    total
}

// Vec 和数组都能传
sum(&vec![1, 2, 3]);
sum(&[1, 2, 3]);
```

---

## 4. Rust vs Go 集合类型

| | Rust | Go |
|--|--|--|
| 动态数组 | `Vec<T>` | `[]T`（切片） |
| 固定数组 | `[T; N]` | `[N]T` |
| 切片引用 | `&[T]` | `[]T`（切片本身就是引用） |

**核心区别：**
- Go 的切片 `[]T` 本质是数组的视图（引用）
- Rust 的 `Vec<T>` 是独立拥有数据的容器
- Rust 的 `&[T]` 才是真正对应 Go 的切片"视图"概念

---

## 5. 迭代器

### 5.1 三种遍历方式

```rust
let v = vec![1, 2, 3];

// 1. 消耗 Vec（i32 有 Copy 所以复制）
for item in v {
    // item 是 i32
}

// 2. 借用（推荐）
for item in &v {
    // item 是 &i32
}

// 3. 可变借用
for item in &mut v {
    // item 是 &mut i32
    *item += 1;
}
```

### 5.2 String 类型的区别

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

### 5.3 .enumerate()

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

## 6. usize 类型

### 6.1 什么是 usize

- **无符号整数**，大小取决于平台（32位/64位）
- 主要用于**数组索引、长度、大小**

```rust
let arr = [1, 2, 3];
let index: usize = 0;
let value = arr[index];  // 索引必须是 usize

let len: usize = arr.len();  // len() 返回 usize
```

### 6.2 为什么需要 usize

- 数组/Vec 的索引必须是 `usize`
- `.len()` 返回 `usize`
- 与指针大小匹配（32位系统4字节，64位系统8字节）

---

## 7. 索引 vs 遍历的类型差异

**关键问题：为什么 `list[0]` 是 `i32`，但 `for item in list` 中 `item` 是 `&i32`？**

```rust
let list: &[i32] = &[1, 2, 3];

let x = list[0];     // x 是 i32（Copy 自动复制）
for item in list {   // item 是 &i32（借用）
}
```

### 7.1 原因：与所有权密切相关

| 操作 | 行为 | 原因 |
|------|------|------|
| `list[0]` | 对 `Copy` 类型复制值 | 索引操作对实现了 `Copy` 的类型会自动复制 |
| `for item in list` | 总是借用元素 | 遍历借用的切片只能借用元素，不能拿走所有权 |

**`i32` 有 Copy：**

```rust
let list: &[i32] = &[1, 2, 3];
let x = list[0];     // ✓ 复制值，x 是 i32
for item in list {   // item 是 &i32（即使 i32 有 Copy，遍历时也是借用）
    println!("{}", *item);  // 需要解引用
}
```

**`String` 没有 Copy：**

```rust
let list: &[String] = &[String::from("a")];
let x = list[0];     // ✗ 报错！不能从 &[String] 中移走所有权
for item in list {   // item 是 &String（借用）
}
```

### 7.2 核心规则

- **索引操作** `list[0]`：如果类型有 `Copy` 就复制，没有 `Copy` 就报错
- **遍历操作** `for item in list`：无论是否有 `Copy`，都是借用（`&T`）

这是 Rust 所有权系统的体现：你只借用了切片（`&[T]`），所以只能借用元素，不能拿走。

---

## 8. 解引用

### 8.1 基本用法

```rust
let x = 5;
let r = &x;   // r 是 &i32

println!("{}", r);   // 5（println! 自动解引用）
println!("{}", *r);  // 5（手动解引用）

if *r > 3 { }  // 比较时需要手动解引用
```

### 8.2 遍历时解引用

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];  // list[0] 是 i32
    for item in list {          // item 是 &i32
        if *item > largest {    // 解引用比较
            largest = *item;    // 解引用赋值
        }
    }
    largest
}
```

---

## 9. 实用示例

### 9.1 查找最大值

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {  // 模式匹配直接解引用
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 9.2 字符串切分

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

let s = "hello world";
let word = first_word(s);  // "hello"
```

---

## 10. 常见错误

### 10.1 切片越界

```rust
let v = vec![1, 2, 3];
let slice = &v[0..10];  // ✗ panic: 越界
```

### 10.2 借用后修改

```rust
let mut v = vec![1, 2, 3];
let slice = &v[..];
v.push(4);  // ✗ 借用期间不能修改
println!("{:?}", slice);
```

---

## 练习建议

1. 实现一个函数，返回 Vec 中所有偶数的新 Vec
2. 编写字符串反转函数
3. 实现一个简单的查找函数
4. 对比 `Vec` 和 `&[T]` 作为函数参数的区别

**下一步：** 学习 [04-structs.md](./04-structs.md) 了解结构体
