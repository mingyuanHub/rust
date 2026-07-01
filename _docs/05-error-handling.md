# Rust 错误处理

> panic、Result、Option 的使用

## 1. 两种错误

| 错误类型 | 说明 | 处理方式 |
|---------|------|---------|
| **不可恢复错误** | 程序无法继续运行 | `panic!` |
| **可恢复错误** | 可以处理并继续 | `Result<T, E>` |

---

## 2. panic!

### 2.1 主动触发 panic

```rust
panic!("Something went wrong!");
```

程序立即崩溃并打印错误信息。

### 2.2 隐式 panic

```rust
let v = vec![1, 2, 3];
let x = v[10];  // panic: 索引越界
```

### 2.3 查看详细堆栈

```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run  # 更详细
```

**什么是 backtrace？**
显示从程序开始到出错位置的**完整函数调用链**，方便定位问题。

---

## 3. Result<T, E>

### 3.1 Result 枚举

```rust
enum Result<T, E> {
    Ok(T),   // 成功，包含值
    Err(E),  // 失败，包含错误
}
```

### 3.2 用 match 处理

```rust
use std::fs::File;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => {
        println!("Failed to open file: {:?}", error);
        return;
    }
};
```

### 3.3 expect()

```rust
let f = File::open("hello.txt").expect("Failed to open file");
```

- 如果 `Ok`，返回值
- 如果 `Err`，panic 并显示自定义信息

**适用场景：**
- 原型开发/学习
- 逻辑上不可能失败的情况

### 3.4 unwrap()

```rust
let f = File::open("hello.txt").unwrap();
```

类似 `expect()`，但错误信息是默认的。

### 3.5 unwrap_or() 和 unwrap_or_else()

```rust
// 失败时返回默认值
let num: i32 = "abc".parse().unwrap_or(0);

// 失败时执行闭包
let num: i32 = "abc".parse().unwrap_or_else(|_| {
    println!("Parse failed, using 0");
    0
});
```

---

## 4. ? 操作符

### 4.1 传播错误

`?` 会在错误时提前返回 `Err`：

```rust
use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;  // 如果失败，直接返回 Err
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

等价于：

```rust
fn read_file() -> Result<String, std::io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 4.2 链式调用

```rust
fn read_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### 4.3 使用限制

`?` 只能用在返回 `Result` 或 `Option` 的函数中：

```rust
fn main() {
    let f = File::open("hello.txt")?;  // ✗ main 不返回 Result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;  // ✓
    Ok(())
}
```

---

## 5. Option<T>

### 5.1 Option 枚举

```rust
enum Option<T> {
    Some(T),  // 有值
    None,     // 无值
}
```

用于表示**可能不存在的值**，替代其他语言的 `null`。

### 5.2 使用 Option

```rust
let some_number = Some(5);
let no_number: Option<i32> = None;

// 用 match 处理
match some_number {
    Some(n) => println!("Got: {}", n),
    None => println!("Nothing"),
}
```

### 5.3 为什么需要 Option？

其他语言用 `null` 表示没有值，很危险：

```javascript
// JS：null 没有类型保护，随时可能崩溃
let user = getUser();
console.log(user.name);  // 如果 user 是 null → 崩溃
```

Rust 没有 `null`，用 `Option` 强制处理"没有值"的情况：

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some(String::from("Alice")) }
    else { None }
}

// 必须处理两种情况，否则编译不过
match find_user(1) {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}
```

### 5.4 常用方法

```rust
let x: Option<i32> = Some(5);

x.is_some()              // true，有值
x.is_none()              // false，没值
x.unwrap()               // 5，有值返回，None 则 panic
x.unwrap_or(0)           // 有值返回值，None 返回 0
x.unwrap_or_else(|| 0)   // None 时执行闭包

// if let：只处理 Some 的情况
if let Some(val) = x {
    println!("{}", val);
}
```

### 5.5 map 方法（内置 None 判断）

`map` 内置了 None 的处理逻辑：
- `Some(value)` → 执行闭包，返回 `Some(结果)`
- `None` → **直接返回 `None`，不执行闭包**

```rust
let x: Option<i32> = Some(5);
let y = x.map(|n| n * 2);  // Some(10)，执行了闭包

let x: Option<i32> = None;
let y = x.map(|n| n * 2);  // None，闭包没有执行
```

不需要手动判断 None，直接链式操作：

```rust
// 没有 map，需要手动判断
let result = if let Some(n) = x { Some(n * 2) } else { None };

// 用 map，更简洁
let result = x.map(|n| n * 2);
```

**链式调用：None 像"短路"，一旦出现就一直传递：**

```rust
let x: Option<i32> = Some(5);
let result = x
    .map(|n| n * 2)           // Some(10)
    .map(|n| n + 1)           // Some(11)
    .map(|n| n.to_string());  // Some("11")

let none: Option<i32> = None;
let result = none
    .map(|n| n * 2)           // None
    .map(|n| n + 1)           // None（跳过）
    .map(|n| n.to_string());  // None（跳过）
```

### 5.4 Vec 中的 Option

```rust
let v = vec![1, 2, 3];

let first = v.get(0);  // Some(&1)
let out_of_bounds = v.get(10);  // None

match first {
    Some(value) => println!("First: {}", value),
    None => println!("Empty"),
}
```

---

## 6. 错误处理策略

### 6.1 生产环境（推荐）

**用 `match` 或 `?` 优雅处理：**

```rust
fn process_input(input: &str) -> Result<i32, String> {
    input.trim()
        .parse()
        .map_err(|e| format!("Parse error: {}", e))
}

match process_input("42") {
    Ok(num) => println!("Success: {}", num),
    Err(msg) => println!("Error: {}", msg),
}
```

### 6.2 学习/原型阶段

**用 `expect()` 或 `unwrap()`：**

```rust
let num: i32 = "42".parse().expect("Not a number");
```

### 6.3 不允许失败的场景

**用 `expect()` 说明原因：**

```rust
let config = include_str!("config.toml");
let parsed = config.parse().expect("Built-in config is valid");
```

---

## 7. 自定义错误类型

### 7.1 简单错误

```rust
#[derive(Debug)]
enum MyError {
    ParseError,
    IoError,
}

fn do_something() -> Result<i32, MyError> {
    // ...
    Err(MyError::ParseError)
}
```

### 7.2 带信息的错误

```rust
#[derive(Debug)]
struct MyError {
    message: String,
}

impl MyError {
    fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

fn do_something() -> Result<i32, MyError> {
    Err(MyError::new("Something went wrong"))
}
```

---

## 8. 实际例子

### 8.1 读取配置文件

```rust
use std::fs;

fn read_config(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))
}

match read_config("config.toml") {
    Ok(content) => println!("Config: {}", content),
    Err(e) => println!("Error: {}", e),
}
```

### 8.2 解析用户输入

```rust
fn parse_age(input: &str) -> Result<u32, String> {
    let age: u32 = input
        .trim()
        .parse()
        .map_err(|_| "Invalid age format".to_string())?;

    if age > 150 {
        return Err("Age too large".to_string());
    }

    Ok(age)
}

match parse_age("25") {
    Ok(age) => println!("Age: {}", age),
    Err(e) => println!("Error: {}", e),
}
```

---

## 9. expect vs unwrap vs match 对比

| 方法 | 错误时行为 | 适用场景 |
|------|----------|---------|
| `expect()` | panic，显示自定义信息 | 学习、原型、不可能失败 |
| `unwrap()` | panic，显示默认信息 | 确定不会失败 |
| `unwrap_or()` | 返回默认值 | 有合理默认值 |
| `match` | 优雅处理错误 | 生产环境 |
| `?` | 传播错误 | 函数链式调用 |

---

## 10. 常见错误

### 10.1 在 main 中用 ?

```rust
// ✗ 错误
fn main() {
    let f = File::open("file.txt")?;
}

// ✓ 正确
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("file.txt")?;
    Ok(())
}
```

### 10.2 忽略 Result

```rust
File::open("file.txt");  // ⚠️ 警告：未使用的 Result

// ✓ 正确处理
let _ = File::open("file.txt");  // 明确忽略
File::open("file.txt").expect("Failed");
```

---

## 11. assert 宏

### 11.1 断言相等

```rust
assert_eq!(2 + 2, 4);  // ✓
assert_eq!(2 + 2, 5);  // ✗ panic
```

### 11.2 断言不相等

```rust
assert_ne!(2 + 2, 5);  // ✓
```

### 11.3 条件断言

```rust
assert!(2 + 2 == 4);  // ✓
```

**用途：** 主要用于测试代码，验证程序行为是否符合预期。

---

## 12. 最佳实践

1. **库函数**：返回 `Result`，让调用者决定如何处理
2. **应用主逻辑**：用 `match` 或 `?` 处理错误
3. **不可能失败**：用 `expect()` 并说明原因
4. **测试代码**：用 `unwrap()` 或 `expect()`
5. **避免 panic**：生产代码尽量不要 panic

---

## 练习建议

1. 编写一个读取文件的函数，返回 `Result`
2. 实现一个计算器，处理除零错误
3. 创建自定义错误类型
4. 练习使用 `?` 操作符简化错误处理

**下一步：** 继续深入学习枚举、泛型、trait 等高级特性
