# 异步编程（Async/Await）

> 难度：⭐⭐⭐⭐

---

## 1. 什么是异步编程？

异步编程允许在等待 IO 操作（网络、文件）时，让出 CPU 去执行其他任务，而不是傻等着。

```rust
// 同步版本：串行执行
let data1 = fetch_url1();  // 等待 1 秒
let data2 = fetch_url2();  // 等待 1 秒
// 总共 2 秒

// 异步版本：并发执行
let fut1 = fetch_url1();   // 创建 Future
let fut2 = fetch_url2();   // 创建 Future
let (data1, data2) = join(fut1, fut2).await;  // 同时等待
// 总共 1 秒（取最慢的）
```

---

## 2. async/await 基础

### 2.1 async fn

`async fn` 定义异步函数，返回 `Future`：

```rust
// 异步函数
async fn fetch_data(url: &str) -> String {
    // ...
}

// 等价于普通函数返回 Future
fn fetch_data(url: &str) -> impl Future<Output = String> {
    // ...
}
```

### 2.2 .await

`.await` 等待 Future 完成，获取结果：

```rust
async fn example() {
    let data = fetch_data("https://example.com").await;
    //                                           ^^^^^^
    //                                           等待异步操作完成
    println!("{}", data);
}
```

**规则：`.await` 只能在 `async` 函数或块中使用。**

---

## 3. Future 详解

### 3.1 什么是 Future？

**Future = "未来的值"**，表示一个现在还没准备好、但将来会完成的计算。

```rust
let future = fetch_data(url);  // 创建 Future，还没执行
// future 是一个"承诺"：将来会给你一个 String

let data = future.await;  // 等待 Future 完成，获取结果
```

### 3.2 Future 是懒执行的

**关键特性：Future 创建时不执行，只有 `.await` 时才真正运行。**

```rust
let fut1 = fetch_url1();  // 只是创建 Future，网络请求还没发出
let fut2 = fetch_url2();  // 只是创建 Future，网络请求还没发出

// 现在两个请求才真正开始
let (data1, data2) = tokio::join!(fut1, fut2);
```

对比 JavaScript：

```javascript
// JS Promise：创建时就开始执行
const p = fetch(url);  // 立即发起请求

// Rust Future：需要 await 才执行
let fut = fetch_data(url);  // 没有发起请求
fut.await;  // 现在才发起请求
```

### 3.3 为什么叫 Future？

类比**外卖订单**：

```
点外卖（创建 Future）
  ↓
等待配送（Future pending）
  ↓
收到外卖（Future ready，得到结果）
```

**Future 强调"将来会有的值"，是一个未来会完成的计算。**

---

## 4. 实际项目写法

### 4.1 trpl（教学库）vs Tokio（生产环境）

<thinking>
需要对比教学库和实际项目的写法差异。
</thinking>

你的教学代码使用 `trpl`（Rust 官方书教学库）：

```rust
use trpl::{Either, Html};

fn main() {
    trpl::block_on(async {
        let title1 = page_title(&args[1]);
        let title2 = page_title(&args[2]);

        match trpl::select(title1, title2).await {
            Either::Left(left) => println!("{:?}", left),
            Either::Right(right) => println!("{:?}", right),
        }
    })
}
```

**`trpl` 特点：**
- 教学专用，简化了概念
- 不推荐用于生产环境
- API 设计为便于理解

### 4.2 实际项目用 Tokio

```rust
use tokio;
use reqwest;

#[tokio::main]  // 宏：自动创建运行时
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let title1 = page_title(&args[1]);
    let title2 = page_title(&args[2]);

    // Tokio 的并发宏
    tokio::select! {
        result = title1 => println!("Title1: {:?}", result),
        result = title2 => println!("Title2: {:?}", result),
    }
}

async fn page_title(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}
```

**Tokio 特点：**
- 生产环境标准
- 生态完善（reqwest、axum 等都基于它）
- 性能优秀

---

## 5. #[tokio::main] 宏

### 5.1 为什么不能直接 async fn main？

```rust
// ✗ 编译错误
async fn main() {
    let data = fetch_data().await;
}
```

**原因：** `main` 是程序入口，操作系统期望它是同步函数。`async fn main` 返回 `Future`，操作系统不知道怎么执行。

### 5.2 #[tokio::main] 的作用

```rust
#[tokio::main]
async fn main() {
    // async 代码
}
```

宏自动转换成：

```rust
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            // async 代码
        })
}
```

**本质：** 在同步 `main` 里创建运行时，用 `block_on` 执行 async 代码。

---

## 6. 为什么需要 .await？

### 6.1 对比同步和异步

**同步版本（阻塞）：**

```rust
let response = http_get(url);  // 阻塞，等待网络响应
let text = response.text();     // 阻塞，等待读取响应体
// CPU 全程在等待，什么都不干
```

**异步版本（非阻塞）：**

```rust
let response = http_get(url).await;  // 发请求，让出 CPU，等有结果再回来
let text = response.text().await;     // 读响应体，让出 CPU，等有结果再回来
// 等待期间，CPU 可以执行其他任务
```

### 6.2 .await 的本质

```rust
trpl::get(url).await
```

翻译成人话：
> "发起网络请求，**我去休息了**（让出 CPU），等请求有结果了再叫我回来继续。"

这段"休息"的时间，运行时会去执行其他任务（比如同时发起第二个请求）。

---

## 7. 哪些需要 await，哪些不需要？

### 7.1 只有 async fn 才能 await

```rust
// ✓ 能用 await：async fn，本质是 IO 操作
trpl::get(url).await        // 网络请求
file.read_to_string().await // 文件读取
db.query("...").await       // 数据库查询

// ✗ 不能用 await：普通同步函数
"hello".len().await         // 编译错误
vec.push(1).await           // 编译错误
1 + 1.await                 // 编译错误
```

### 7.2 判断标准

```
这个操作需要等待外部资源吗？
├─ 是（网络、文件、数据库）→ 用 async/await
└─ 否（纯计算、内存操作）→ 普通函数
```

```rust
// ✓ 需要 async：等待网络
async fn fetch_data(url: &str) -> String {
    trpl::get(url).await.text().await
}

// ✗ 不需要 async：纯计算
fn parse_data(text: &str) -> Vec<String> {
    text.lines().map(|s| s.to_string()).collect()
}
```

### 7.3 为什么不是"越多越好"？

**async 有开销：**

1. 每个 `async fn` 会生成状态机，有内存和 CPU 开销
2. async 有传染性：一个函数变 async，调用它的也得变 async
3. CPU 密集型用 async 反而更慢（会阻塞运行时）

```rust
// ✗ 没意义，还有额外开销
async fn add(a: i32, b: i32) -> i32 {
    a + b  // 纯计算，不需要 async
}

// ✓ 正确
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 8. async vs thread 选择

### 8.1 核心区别

| | async/await | thread |
|--|-------------|--------|
| 适用场景 | IO 密集（网络/文件/数据库） | CPU 密集（计算/加密/压缩） |
| 并发方式 | 单线程协作式调度 | 多线程抢占式调度 |
| 内存开销 | ~几KB/任务 | ~8MB/线程 |
| 切换开销 | 极低（用户态） | 较高（内核态） |
| 并发数量 | 数十万+ | 数千 |

### 8.2 为什么不全用 async？

**async 的致命局限：CPU 密集型任务会阻塞整个运行时。**

async 使用协作式调度，任务只在 `.await` 时让出 CPU：

```rust
// ✗ 危险：CPU 密集型任务放进 async
async fn crack_password(hash: &str) -> String {
    loop {
        if check_hash(hash) { break; }  // 没有 await，运行时被卡死
    }
}
```

**类比：** async 像单线程服务员，只有在等待（`.await`）时才能服务其他桌。如果一直在厨房炒菜（CPU计算），其他桌只能干等。

### 8.3 适用场景

| 场景 | 推荐 |
|------|------|
| 网络请求、数据库查询 | `async` |
| 文件读写 | `async` |
| Web 服务器（大量并发连接） | `async` |
| 图片处理、视频编码 | `thread` |
| 加密/解密运算 | `thread` |
| 科学计算 | `thread` |

### 8.4 混合使用

生产环境最佳实践：**async + thread 混合**

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // IO 密集：直接 async
    let data = fetch_from_db().await;

    // CPU 密集：放到线程池
    let result = task::spawn_blocking(move || {
        process_data(data)  // 在独立线程执行，不阻塞运行时
    }).await.unwrap();

    // 继续 async
    save_result(result).await;
}
```

---

## 9. 并发模式

### 9.1 select（竞争，取最快的）

```rust
use trpl::Either;

let fut1 = fetch_url1();
let fut2 = fetch_url2();

match trpl::select(fut1, fut2).await {
    Either::Left(result) => println!("URL1 先完成: {:?}", result),
    Either::Right(result) => println!("URL2 先完成: {:?}", result),
}
```

**用途：** 多个数据源，取最快返回的。

### 9.2 join（等待所有完成）

```rust
let fut1 = fetch_url1();
let fut2 = fetch_url2();

let (data1, data2) = tokio::join!(fut1, fut2);
// 等两个都完成
```

**用途：** 需要所有结果才能继续。

---

## 10. 常见错误

### 10.1 在同步函数里用 await

```rust
fn sync_function() {
    let data = fetch_data().await;  // ✗ 编译错误：await 只能在 async 里用
}
```

**修复：** 函数改成 `async fn` 或用 `block_on`。

### 10.2 忘记 await

```rust
async fn example() {
    let data = fetch_data();  // ✗ 没有 await，只是创建 Future，不执行
    println!("{:?}", data);   // 打印的是 Future，不是数据
}
```

**修复：** 加 `.await`。

### 10.3 给普通函数加 await

```rust
let len = "hello".len().await;  // ✗ 编译错误：len() 不是 async fn
```

---

## 11. 主流 async 运行时

| 运行时 | 特点 | 适用场景 |
|--------|------|---------|
| **Tokio** | 最流行，生态最完善 | 生产环境首选 |
| **async-std** | API 像标准库，简单易用 | 学习和小项目 |
| **smol** | 轻量级 | 嵌入式、小程序 |
| **trpl** | 教学专用 | 学习 Rust 官方书 |

---

## 12. 实际项目配置

### 12.1 Cargo.toml

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
```

### 12.2 main.rs

```rust
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_data("https://api.example.com").await;
    println!("{:?}", result);
}

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}
```

---

## 13. 与其他语言对比

| | Rust async/await | Go goroutine | JavaScript Promise |
|--|------------------|--------------|-------------------|
| 语法 | 显式 async/await | 透明（普通函数） | async/await |
| 执行时机 | 懒执行（需要 await） | 立即执行 | 立即执行 |
| 运行时 | 可选（Tokio等） | 内置 runtime | 内置事件循环 |
| 控制粒度 | 高（开发者控制） | 低（runtime 决定） | 中等 |

---

## 14. 总结

| 概念 | 含义 |
|------|------|
| `async fn` | 异步函数，返回 Future |
| `Future` | 未来的值，懒执行 |
| `.await` | 等待 Future 完成 |
| `#[tokio::main]` | 创建运行时，让 main 能用 async |
| `tokio::spawn` | 启动异步任务 |
| `tokio::select!` | 竞争多个 Future，取最快的 |
| `tokio::join!` | 等待所有 Future 完成 |

**核心原则：**
- IO 密集用 async（网络、文件、数据库）
- CPU 密集用 thread（计算、加密、压缩）
- async 不是越多越好，只在有 IO 等待时用

**下一步：** 学习 Tokio 生态 → Web 框架 Axum
