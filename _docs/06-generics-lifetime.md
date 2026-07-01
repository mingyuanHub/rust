# 泛型与生命周期

> 难度：⭐⭐⭐

---

## 1. 泛型（Generics）

### 1.1 为什么需要泛型？

没有泛型时，同样的逻辑要为每种类型写一遍：

```rust
fn largest_i32(list: &[i32]) -> &i32 { ... }
fn largest_char(list: &[char]) -> &char { ... }
// 逻辑完全一样，只有类型不同
```

用泛型合并成一个函数：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 1.2 函数泛型

```rust
// <T> 声明泛型参数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 调用时自动推断类型
let numbers = vec![34, 50, 25, 100];
let result = largest(&numbers);  // T 推断为 i32

let chars = vec!['a', 'b', 'r', 'c'];
let result = largest(&chars);    // T 推断为 char
```

### 1.3 结构体泛型

```rust
#[derive(Debug)]
struct Point<T> {
    x: i32,
    y: T,   // y 的类型由调用时决定
}

let p1 = Point { x: 5, y: 10 };      // y 是 i32
let p2 = Point { x: 5, y: 3.0 };     // y 是 f64
let p3 = Point { x: 5, y: "hello" }; // y 是 &str
```

### 1.4 impl 泛型

```rust
impl<T> Point<T> {
    fn get_x(&self) -> &i32 {
        &self.x
    }
}
```

---

## 2. Trait 约束

### 2.1 为什么需要 Trait 约束？

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {  // ✗ 编译器不知道 T 是否支持 >
```

需要告诉编译器"T 必须支持比较"：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
//             ^^^^^^^^^^^
//             T 必须实现 PartialOrd（支持 > < 比较）
```

### 2.2 常用 Trait 约束

| Trait | 含义 | 例子 |
|-------|------|------|
| `PartialOrd` | 支持比较（`>`, `<`） | `i32`, `f64`, `char` |
| `Clone` | 可以 `.clone()` | `String`, `Vec` |
| `Copy` | 赋值时自动复制 | `i32`, `bool`, `char` |
| `Display` | 可以用 `{}` 打印 | `i32`, `String` |
| `Debug` | 可以用 `{:?}` 打印 | 加了 `#[derive(Debug)]` 的类型 |
| `Into<T>` | 可以转换成 T | `i8`, `i16`, `i32` → `Into<i32>` |

### 2.3 多个 Trait 约束

```rust
// 用 + 组合多个约束
fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    // T 必须同时支持比较和打印
}

// 用 where 语法（更清晰）
fn print_largest<T>(list: &[T])
where
    T: PartialOrd + Display
{
    // ...
}
```

### 2.4 实际例子：Point 求和

```rust
impl<T: Into<i32>> Point<T> {
    // T 必须能转换成 i32
    fn sum(self) -> i32 {
        self.x + self.y.into()  // .into() 把 T 转成 i32
    }
}

let p = Point { x: 5, y: 10_i32 };
println!("{}", p.sum());  // 15

let p = Point { x: 5, y: 3_i8 };  // i8 也实现了 Into<i32>
println!("{}", p.sum());  // 8
```

---

## 3. 生命周期（Lifetimes）

### 3.1 本质

生命周期注解的核心作用是**告诉编译器引用之间的存活关系，让编译器在编译时验证内存安全。**

- 不是运行时的东西，纯编译时检查
- 编译后的机器码里没有生命周期的痕迹，零运行时开销
- 不是"给引用设置寿命"，而是"告诉编译器引用之间的约束关系"

### 3.2 为什么需要生命周期？

**问题：其他语言中的悬垂指针**

```c
// C 语言
char* dangle() {
    char str[] = "hello";
    return str;  // 返回栈上局部变量的地址
}  // str 被释放，指针指向无效内存 → 崩溃或垃圾数据
```

**Rust 在编译时阻止这种错误：**

```rust
fn dangle() -> &String {  // ✗ 编译错误
    let s = String::from("hello");
    &s  // s 在函数结束时释放，不能返回引用
}

// ✓ 正确：返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 所有权转移出去
}
```

**但多个引用参数时，编译器需要更多信息：**

```rust
fn longest(x: &str, y: &str) -> &str {  // ✗ 编译器不知道返回值借用谁
    if x.len() > y.len() { x } else { y }
}
```

编译器的困惑：返回值有时是 `x`，有时是 `y`，不知道哪个引用更短命，无法验证安全性。

### 3.3 生命周期标注语法

```rust
&i32         // 普通引用
&'a i32      // 有生命周期标注的引用
&'a mut i32  // 有生命周期标注的可变引用
```

**`<'a>` 和泛型 `<T>` 一样，必须先声明再使用：**

```rust
fn largest<T>(list: &[T])          // 先声明 T，再用 T
fn longest<'a>(x: &'a str) -> &'a str  // 先声明 'a，再用 'a

// ✗ 未声明直接用会报错
fn longest(x: &'a str) -> &'a str
```

### 3.4 函数中的生命周期

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//         ^^      ^^           ^^            ^^
//       声明 'a   a 活 'a      b 活 'a    返回值活 'a
    if a.len() > b.len() { a } else { b }
}
```

含义：**返回值的生命周期 = min(a 的生命周期, b 的生命周期)**

**编译器用注解验证安全性：**

```rust
let s1 = String::from("long string");
let result;
{
    let s2 = String::from("xyz");
    result = longest(&s1, &s2);
    println!("{}", result);  // ✓ 在 s2 有效期内使用
}
println!("{}", result);  // ✗ 编译错误：s2 已失效，result 也无效
```

编译器推理过程：
1. `'a` = min(s1 的生命周期, s2 的生命周期)
2. s2 在内层作用域结束后失效
3. 所以 `'a` = s2 的生命周期
4. result 也只能活到 s2 失效为止
5. 外层使用 result → **编译错误**

### 3.5 注释 vs 生命周期标注

**注意：这两个完全不同！**

```rust
// 这是注释，编译器完全忽略
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//         ^^  这是生命周期标注，编译器必须读取，是真实的语法
```

| | 符号 | 编译器读取 | 影响执行 |
|--|--|--|--|
| 注释 | `//` `/* */` | ✗ 忽略 | ✗ |
| 生命周期标注 | `'a` | ✓ 必须读取 | ✓ 不加会报错 |

### 3.6 什么时候不需要手动标注？

编译器有**生命周期省略规则**，部分情况自动推断：

```rust
// 不需要：参数是值类型，没有引用
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 不需要：只有一个引用参数，编译器自动推断返回值借用自 s
fn first_word(s: &str) -> &str {
    &s[0..1]
}

// 需要：多个引用参数，编译器无法自动推断返回值借用谁
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

**规律：只有一个引用参数 → 可以省略；多个引用参数且有返回引用 → 必须手动标注。**

### 3.7 `'static` 生命周期

```rust
// 字符串字面量编译时写入二进制文件，程序整个运行期间有效
let s: &'static str = "hello";

fn get_str() -> &'static str {
    "I live forever"  // ✓ 字面量是 'static
}

fn get_str() -> &'static str {
    let s = String::from("hello");
    &s  // ✗ 堆上分配的 String 不是 'static
}
```

### 3.8 结构体中的生命周期

如果结构体字段是引用，必须标注：

```rust
// ✓ 推荐：用 String，避免生命周期问题
struct User {
    name: String,
}

// 需要生命周期标注
struct User<'a> {
    name: &'a str,  // name 借用外部数据
}

// 'a 的含义：User 实例不能比它借用的 name 活得更长
let user;
{
    let name = String::from("Alice");
    user = User { name: &name };
}  // name 失效
println!("{}", user.name);  // ✗ 编译错误
```

### 3.5 `'static` 生命周期

```rust
// 字符串字面量的生命周期是 'static（程序整个运行期间）
fn get_str() -> &'static str {
    "hello"  // 存储在二进制文件中，永远有效
}

let s: &'static str = "hello world";
```

### 3.6 结构体中的生命周期

如果结构体字段是引用，需要生命周期标注：

```rust
// ✓ 用 String（推荐，避免生命周期问题）
struct User {
    username: String,
}

// 需要生命周期标注（不推荐，除非有特殊原因）
struct User<'a> {
    username: &'a str,  // username 借用外部数据，需要标注
}
```

---

## 4. 泛型、Trait 约束、生命周期组合

```rust
// 三者同时使用
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

---

## 5. 常见错误

### 5.1 返回局部变量的引用

```rust
fn dangle() -> &String {  // ✗ 缺少生命周期标注
    let s = String::from("hello");
    &s  // ✗ s 在函数结束时释放，引用无效
}

// ✓ 修复：返回值本身
fn no_dangle() -> String {
    String::from("hello")
}
```

### 5.2 函数返回 `&str` 没有标注

```rust
fn get_str() -> &str {  // ✗ 编译器不知道从哪里借用
    "hello"
}

// ✓ 修复
fn get_str() -> &'static str {
    "hello"
}
```

### 5.3 泛型方法中类型不满足约束

```rust
struct Point<T> { x: i32, y: T }

impl<T> Point<T> {
    fn sum(&self) -> i32 {
        self.x + self.y  // ✗ T 不一定能和 i32 相加
    }
}

// ✓ 修复：加 trait 约束
impl<T: Into<i32>> Point<T> {
    fn sum(self) -> i32 {
        self.x + self.y.into()
    }
}
```

---

## 6. 与其他语言对比

### Go

```go
// Go 1.18+ 支持泛型
func Largest[T constraints.Ordered](list []T) T {
    largest := list[0]
    for _, v := range list {
        if v > largest {
            largest = v
        }
    }
    return largest
}
```

### TypeScript

```typescript
function largest<T>(list: T[], compare: (a: T, b: T) => number): T {
    return list.reduce((a, b) => compare(a, b) > 0 ? a : b);
}
```

**Rust 的泛型在编译时完全展开（单态化），零运行时开销。Go/TS 的泛型有运行时成本。**

---

## 7. 核心总结

| 概念 | 作用 | 示例 |
|------|------|------|
| 泛型 `<T>` | 让函数/结构体支持多种类型 | `fn largest<T>` |
| Trait 约束 `T: Trait` | 限制 T 必须具备某种能力 | `T: PartialOrd` |
| 生命周期 `'a` | 告诉编译器引用之间的存活关系 | `fn f<'a>(x: &'a str) -> &'a str` |

**记忆方法：**
- 泛型 = "我不关心类型是什么"
- Trait 约束 = "但它必须能做某些事"
- 生命周期 = "而且引用必须活得够长"

---

**下一步：** 学习 Trait 的定义和实现 → `trait-my/`
