# Rust 基础入门

> 适合刚开始学习 Rust 的同学

## 1. 变量与可变性

### 1.1 不可变变量（默认）

```rust
let x = 5;
// x = 6;  // ✗ 报错：不能修改不可变变量
```

### 1.2 可变变量

```rust
let mut y = 5;
y = 6;  // ✓ 可以修改
```

### 1.3 Shadowing（遮蔽）

```rust
let z = 5;
let z = z + 1;      // 创建新变量，可以改变类型
let z = z.to_string();  // ✓ 类型可以变
```

**`mut` vs shadowing：**

| | `mut` | shadowing |
|--|--|--|
| 同一个变量 | ✓ | ✗ 创建新变量 |
| 类型可变 | ✗ | ✓ |
| 用途 | 值会变化 | 转换/计算 |

### 1.4 常量

```rust
const MAX_SIZE: i32 = 100;  // 必须大写 SCREAMING_SNAKE_CASE
```

---

## 2. 基本数据类型

### 2.1 数字类型

```rust
let x: i32 = 5;        // 有符号整数
let y: u32 = 5;        // 无符号整数
let z: f64 = 2.3;      // 浮点数
```

常用类型：
- `i32`：有符号32位整数（默认）
- `u32`：无符号32位整数
- `f64`：64位浮点数（默认）

### 2.2 布尔和字符

```rust
let t: bool = true;
let c: char = 'A';     // 单引号
```

### 2.3 元组（Tuple）

组合不同类型的数据：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 访问
let a = tup.0;  // 500
let b = tup.1;  // 6.4

// 解构
let (x, y, z) = tup;
```

**用途：**
- 函数返回多个值
- 临时组合数据

---

## 3. 函数

### 3.1 基本函数

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 无分号，返回值
}

fn greet() {
    println!("Hello!");  // 无返回值
}
```

### 3.2 表达式 vs 语句

```rust
// 语句（有分号）
let x = 5;

// 表达式（无分号），有返回值
let y = {
    let a = 5;
    a + 1  // 返回 6
};
```

### 3.3 返回值

```rust
// 隐式返回（推荐）
fn add(a: i32) -> i32 {
    a + 5  // 无分号
}

// 显式返回
fn add2(a: i32) -> i32 {
    return a + 5;  // 用于提前退出
}
```

---

## 4. 控制流

### 4.1 if 表达式

```rust
let num = 5;

if num > 3 {
    println!("big");
} else {
    println!("small");
}

// if 有返回值
let result = if num > 3 { "big" } else { "small" };
```

### 4.2 循环

**loop（无限循环）：**
```rust
let mut count = 0;
loop {
    count += 1;
    if count == 10 {
        break;  // 退出循环
    }
}
```

**while：**
```rust
let mut num = 3;
while num > 0 {
    println!("{}", num);
    num -= 1;
}
```

**for（推荐）：**
```rust
let arr = [1, 2, 3];
for item in arr {
    println!("{}", item);
}

// 范围
for i in 0..5 {
    println!("{}", i);  // 0,1,2,3,4
}
```

---

## 5. 常用格式化

### 5.1 println! 宏

```rust
let x = 5;
let name = "Alice";

println!("x = {}", x);           // 基本输出
println!("x = {x}");             // 变量名简写
println!("x = {}, name = {}", x, name);  // 多个值
```

### 5.2 调试输出

```rust
let arr = [1, 2, 3];
println!("{:?}", arr);   // [1, 2, 3]
println!("{:#?}", arr);  // 美化输出（多行）
```

**格式说明：**

| 占位符 | trait | 用途 |
|--------|-------|------|
| `{}` | `Display` | 用户友好输出 |
| `{:?}` | `Debug` | 调试输出 |
| `{:#?}` | `Debug` | 美化调试输出 |

### 5.3 dbg! 宏

`dbg!` 是专门用于调试的宏，全称 **debug**：

```rust
let x = 5;
println!("{}", x);  // 输出：5
dbg!(x);            // 输出：[src/main.rs:2] x = 5
```

**dbg! vs println! 区别：**

| | `dbg!` | `println!` |
|--|--|--|
| 输出目标 | 标准错误（stderr） | 标准输出（stdout） |
| 显示文件/行号 | ✓ 自动 | ✗ 需手动 |
| 显示表达式 | ✓ 自动 | ✗ 需手动 |
| 返回值 | ✓ 返回变量所有权 | ✗ 返回 `()` |

**最大优势：可以嵌套在表达式里，不破坏原有逻辑：**

```rust
// 查看中间计算结果
let b = dbg!(2 * 3) + 1;
// 输出：[src/main.rs:1] 2 * 3 = 6
// b = 7

// println! 做不到这一点
```

**注意 String 会移走所有权，用引用：**

```rust
let s = String::from("hello");
dbg!(&s);  // ✓ 借用，s 还可以继续用
```

**使用建议：** `dbg!` 只用于开发调试，发布前记得删掉。

---

## 6. 命令行参数

### 6.1 读取参数

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] 永远是程序自身路径
    // args[1] 是第一个参数，以此类推
    println!("{}", args[0]);  // target/debug/myapp
    println!("{}", args[1]);  // 第一个参数
}
```

运行：

```bash
cargo run hello world
# args[0] = target/debug/myapp
# args[1] = hello
# args[2] = world
```

**其他语言的对比：**

```go
os.Args[0]      // Go：程序路径
os.Args[1]      // 第一个参数
```

```javascript
process.argv[2] // Node.js：第一个参数（0是node，1是脚本路径）
```

### 6.2 封装到结构体（推荐）

```rust
use std::env;

struct Config {
    file_name: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            file_name: args[1].clone(),  // clone 得到独立 String
            query: args[2].to_string(),  // &String → String
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
}
```

## 6. 命名规范

Rust 强制执行命名规范：

| 类型 | 规范 | 示例 |
|------|------|------|
| 变量、函数 | `snake_case` | `user_name`, `get_value()` |
| 常量 | `SCREAMING_SNAKE_CASE` | `MAX_SIZE` |
| 结构体、枚举 | `PascalCase` | `UserInfo`, `OrderStatus` |

---

## 7. 类型转换

### 7.1 字符串解析

```rust
// 需要类型注解
let num: i32 = "42".parse().expect("Not a number");

// 或用 turbofish
let num = "42".parse::<i32>().expect("Not a number");
```

### 7.2 类型推断

```rust
let x = 5;      // 编译器推断为 i32
let y = 2.0;    // 推断为 f64
let z = true;   // 推断为 bool
```

---

## 8. 常见错误

### 8.1 类型不匹配

```rust
// ✗ 错误
let x = "42".parse().expect("error");

// ✓ 正确
let x: i32 = "42".parse().expect("error");
```

### 8.2 不可变变量修改

```rust
// ✗ 错误
let x = 5;
x = 6;

// ✓ 正确
let mut x = 5;
x = 6;
```

---

## 练习建议

1. 创建一个猜数字游戏（已完成 `guessgame`）
2. 编写温度转换函数（华氏 ↔ 摄氏）
3. 计算斐波那契数列
4. 编写一个简单的计算器

**下一步：** 学习 [02-ownership.md](./02-ownership.md) 了解 Rust 的核心特性
