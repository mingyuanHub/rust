# Rust 所有权系统

> Rust 的核心特性，理解它是掌握 Rust 的关键

## 1. 所有权的三大规则

1. **每个值只有一个所有者**
2. **当所有者离开作用域，值被释放**
3. **赋值/传参会转移所有权（没有 Copy 的类型）**

---

## 2. 内存管理对比

| 语言 | 方式 | 运行时开销 | 安全性 |
|------|------|-----------|--------|
| Go/JS/PHP | GC（垃圾回收） | 有，会暂停 | ✓ |
| C/C++ | 手动 malloc/free | 无 | ✗ 易出错 |
| **Rust** | **所有权系统** | **无** | **✓ 编译时保证** |

**Rust 的目标：既有 C 的性能，又有 GC 语言的内存安全。**

---

## 3. 所有权转移（Move）

### 3.1 String 的所有权转移

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 所有权转移给 s2

// println!("{}", s1);  // ✗ s1 已失效
println!("{}", s2);  // ✓ s2 有效
```

**内存视角：**
```
s1 = String("hello")   →  堆上分配内存
let s2 = s1            →  s2 接管内存，s1 失效
                          （避免 double free）
```

### 3.2 函数调用的所有权转移

```rust
fn take_ownership(s: String) {
    println!("{}", s);
}  // s 在这里被释放

let s = String::from("hello");
take_ownership(s);  // s 所有权转移进函数
// println!("{}", s);  // ✗ s 已失效
```

---

## 4. Copy vs Move

### 4.1 实现了 Copy 的类型（赋值时复制）

- 基本类型：`i32`, `u32`, `f64`, `bool`, `char`
- 元组（如果所有元素都是 Copy）

```rust
let x = 5;
let y = x;  // 复制
println!("{}, {}", x, y);  // ✓ x 和 y 都有效
```

### 4.2 没有 Copy 的类型（赋值时移动）

- `String`
- `Vec`
- 自定义结构体（默认）

```rust
let s1 = String::from("hello");
let s2 = s1;  // 移动
// println!("{}", s1);  // ✗ s1 已失效
```

---

## 5. 借用（Borrowing）

不转移所有权，只借用：

### 5.1 不可变借用

```rust
fn print(s: &String) {  // 借用，不拿走所有权
    println!("{}", s);
}

let s = String::from("hello");
print(&s);  // 借用 s
println!("{}", s);  // ✓ s 还有效
```

### 5.2 可变借用

```rust
fn append(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
append(&mut s);
println!("{}", s);  // "hello world"
```

---

## 6. 借用规则

**核心规则：**
1. 同一时间只能有**一个可变借用**
2. 或者**多个不可变借用**
3. 借用期间，原变量不能修改

### 6.1 多个不可变借用（✓）

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);  // ✓
```

### 6.2 可变借用 + 不可变借用（✗）

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // ✗ 不能同时有不可变和可变借用
```

### 6.3 借用后修改（✗）

```rust
let mut s = String::from("hello");
let r = &s;      // 不可变借用
s.clear();       // ✗ 借用期间不能修改
println!("{}", r);
```

**正确写法：**
```rust
let mut s = String::from("hello");
let r = &s;
println!("{}", r);  // r 使用完毕，借用结束
s.clear();          // ✓ 现在可以修改
```

---

## 7. 所有权与函数

### 7.1 返回所有权

```rust
fn create_string() -> String {
    let s = String::from("hello");
    s  // 转移所有权给调用者
}

let s = create_string();  // 接收所有权
```

### 7.2 借用参数（推荐）

```rust
// ✓ 推荐：借用参数
fn get_length(s: &String) -> usize {
    s.len()
}

let s = String::from("hello");
let len = get_length(&s);
println!("{}, {}", s, len);  // s 还可以用
```

```rust
// ✗ 不推荐：拿走所有权再返回
fn get_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // 还回去
}

let s = String::from("hello");
let (s, len) = get_length(s);  // 麻烦
```

---

## 8. Clone（显式复制）

当确实需要深拷贝时：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 在堆上复制一份新的

println!("{}, {}", s1, s2);  // ✓ 都有效
```

**注意：`clone()` 会复制堆数据，性能开销大，优先用借用。**

---

## 9. 悬垂引用（Dangling Reference）

Rust 编译器会阻止悬垂引用：

```rust
// ✗ 编译错误
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // s 被释放，返回悬垂引用
}

// ✓ 正确：返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 转移所有权
}
```

---

## 10. 实际例子

### 10.1 借用 vs 所有权转移

```rust
// 场景：处理字符串后还要继续用

// ✗ 所有权转移（不推荐）
fn process(s: String) -> String {
    // 处理...
    s  // 必须返回，麻烦
}
let s = String::from("hello");
let s = process(s);

// ✓ 借用（推荐）
fn process(s: &String) {
    // 处理...
}
let s = String::from("hello");
process(&s);
// s 还可以继续用
```

### 10.2 可变借用

```rust
fn capitalize(s: &mut String) {
    *s = s.to_uppercase();
}

let mut text = String::from("hello");
capitalize(&mut text);
println!("{}", text);  // "HELLO"
```

---

## 11. 常见错误

### 11.1 借用后修改

```rust
// ✗ 错误
let mut s = String::from("hello");
let r = &s;
s.clear();  // 借用期间不能修改
println!("{}", r);

// ✓ 正确
let mut s = String::from("hello");
let r = &s;
println!("{}", r);  // r 使用完毕
s.clear();  // 现在可以修改
```

### 11.2 返回局部变量引用

```rust
// ✗ 错误
fn get_ref() -> &String {
    let s = String::from("hello");
    &s  // 悬垂引用
}

// ✓ 正确
fn get_string() -> String {
    String::from("hello")  // 返回所有权
}
```

---

## 12. 类比理解

把所有权想象成一本书：

- **所有权** = 拥有这本书
- **借用 `&T`** = 把书借给别人看，看完还你
- **可变借用 `&mut T`** = 借给别人改，改完还你
- **Copy** = 书可以复印，你和别人各有一份
- **Clone** = 手动复印一本新书

---

## 练习建议

1. 编写函数，接收字符串引用并返回长度
2. 实现字符串反转函数（可变借用）
3. 理解为什么 Vec push 需要 `mut`
4. 体会所有权转移带来的编译错误

**下一步：** 学习 [03-collections.md](./03-collections.md) 了解集合类型
