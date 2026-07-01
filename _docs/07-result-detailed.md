# Result 类型详解

> 深入理解 Rust 的错误处理核心类型

---

## 1. Result<T, E> 基础

`Result` 是 Rust 标准库的枚举类型，用于表示可能成功或失败的操作：

```rust
enum Result<T, E> {
    Ok(T),   // 成功，包含类型 T 的值
    Err(E),  // 失败，包含类型 E 的错误
}
```

---

## 2. Result<Config, &'static str> 详解

### 2.1 整体结构

```rust
Result<Config, &'static str>
^^^^^^ ^^^^^^  ^^^^^^^^^^^^^^
  |      |            |
枚举类型  成功值类型   错误值类型
```

### 2.2 各部分含义

| 符号 | 含义 |
|------|------|
| `Result` | 枚举类型，表示可能成功或失败 |
| `<>` | 泛型参数 |
| `Config` | 成功时的返回值类型（第一个泛型参数 T） |
| `&` | 引用，借用数据，不拥有所有权 |
| `'static` | 生命周期，整个程序运行期间有效 |
| `str` | 字符串切片类型（不定长，必须用 & 引用） |
| `&'static str` | 错误时的返回值类型（第二个泛型参数 E） |

---

## 3. 泛型参数详解

### 3.1 第一个参数：Config（成功值）

```rust
Result<Config, ...>
       ^^^^^^
       成功时返回 Config 结构体
```

使用示例：

```rust
fn build(args: &[String]) -> Result<Config, &'static str> {
    Ok(Config {  // 返回成功值
        file_name: args[1].clone(),
        others: args[2].clone(),
    })
}
```

### 3.2 第二个参数：&'static str（错误值）

```rust
Result<..., &'static str>
            ^^^^^^^^^^^^^^
            错误时返回 &'static str
```

使用示例：

```rust
fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 3 {
        return Err("error arguments number");  // 返回错误
    }
    // ...
}
```

---

## 4. &'static str 深入解析

### 4.1 组成部分

```rust
&'static str
^ ^^^^^^^ ^^^
|    |     |
|    |     字符串切片类型
|    生命周期标注
引用符号
```

### 4.2 & - 引用

不拥有数据，只是借用：

```rust
&str      // 字符串切片的引用
&String   // String 的引用
&i32      // i32 的引用
```

### 4.3 'static - 生命周期

表示整个程序运行期间都有效：

```rust
// 字符串字面量是 'static，编译时写入二进制文件
"hello"  // 类型是 &'static str

fn get_error() -> &'static str {
    "error message"  // ✓ 字面量永远有效
}

fn get_error() -> &'static str {
    let s = String::from("error");
    &s  // ✗ s 在函数结束时释放，不是 'static
}
```

### 4.4 str - 字符串切片类型

```rust
str           // 字符串切片（不定长，必须放在 & 后面）
&str          // 字符串切片的引用
&'static str  // 整个程序期间有效的字符串切片引用
```

---

## 5. 完整示例

```rust
#[derive(Debug)]
struct Config {
    file_name: String,
    others: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("too many arguments");
        }

        Ok(Config {
            file_name: args[1].clone(),
            others: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);  // err 的类型是 &'static str
        panic!("{}", err)
    });

    dbg!(&config);
}
```

---

## 6. 常见的其他 Result 类型

```rust
// 成功返回 i32，错误返回 String
Result<i32, String>

// 成功无返回值，错误是任意 Error
Result<(), Box<dyn Error>>

// 成功返回 String，错误是 IO 错误
Result<String, std::io::Error>

// 成功返回 User，错误是静态字符串
Result<User, &'static str>

// 成功返回 Vec，错误是自定义错误类型
Result<Vec<i32>, MyError>
```

---

## 7. 为什么用 &'static str 而不是 String？

### 7.1 性能对比

```rust
// ✓ 用 &'static str（推荐简单场景）
Result<Config, &'static str>
return Err("error message");  // 字面量，零成本，无堆分配

// 用 String（需要动态错误信息）
Result<Config, String>
return Err(format!("error: got {} args", args.len()));  // 需要堆分配
```

### 7.2 适用场景

| 错误类型 | 适用场景 | 示例 |
|---------|---------|------|
| `&'static str` | 固定错误消息 | `"file not found"` |
| `String` | 动态错误消息 | `format!("missing arg: {}", name)` |
| `Box<dyn Error>` | 统一处理多种错误 | 库函数通用错误 |
| 自定义 enum | 明确的错误类型 | `MyError::InvalidInput` |

---

## 8. unwrap_or_else 详解

```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Error: {}", err);
    panic!("{}", err)
});
```

**执行流程：**

1. `Config::build(&args)` 返回 `Result<Config, &'static str>`
2. 如果是 `Ok(config)` → 返回 `config`
3. 如果是 `Err(err)` → 执行闭包 `|err| { ... }`
   - `err` 的类型是 `&'static str`
   - 打印错误
   - 调用 `panic!` 终止程序

---

## 9. 其他常用 Result 方法

```rust
// unwrap：Ok 返回值，Err 直接 panic
let config = Config::build(&args).unwrap();

// expect：Err 时 panic 并显示自定义消息
let config = Config::build(&args).expect("Failed to build config");

// unwrap_or：Err 时返回默认值
let config = Config::build(&args).unwrap_or(Config::default());

// ? 操作符：Err 时提前返回（函数必须返回 Result）
fn run() -> Result<(), &'static str> {
    let config = Config::build(&args)?;  // 如果 Err，直接返回错误
    Ok(())
}

// match：完全控制
match Config::build(&args) {
    Ok(config) => println!("Success: {:?}", config),
    Err(e) => println!("Error: {}", e),
}
```

---

## 10. 总结

**`Result<Config, &'static str>` 表示：**
- 成功时返回 `Config` 结构体
- 失败时返回一个程序生命周期内有效的字符串错误消息
- 零运行时开销，错误消息编译时写入二进制文件
- 适合简单的错误处理场景

**类比：**
- `Result` 类似其他语言的 try-catch，但在编译时强制处理
- `&'static str` 类似 C 语言的字符串常量，但有类型安全保证
