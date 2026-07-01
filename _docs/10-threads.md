# 多线程与并发

> 难度：⭐⭐⭐

---

## 1. 创建线程

### 1.1 基本用法

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 创建新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待子线程完成
    handle.join().unwrap();
}
```

---

## 2. join() 的含义

### 2.1 为什么叫 join？

`join` 来自多线程术语 **"fork-join 模型"**：

```
主线程 ────────────────────────────→ 继续运行
       |                    ↑
     fork                 join
  （分叉创建线程）     （等待线程汇合）
       ↓                    |
子线程 ─────────────────────→
```

- **fork（分叉）**：创建新线程，执行路径分叉
- **join（汇合）**：等待子线程完成，执行路径汇合回主线程

### 2.2 不 join 会怎样？

主线程结束时会强制终止所有子线程：

```rust
thread::spawn(|| {
    println!("子线程开始");
    thread::sleep(Duration::from_secs(1));
    println!("子线程结束");  // 可能不会打印
});

// 主线程立即结束，子线程被强制终止
```

加上 `join()` 确保子线程完成：

```rust
let handle = thread::spawn(|| {
    println!("子线程开始");
    thread::sleep(Duration::from_secs(1));
    println!("子线程结束");
});

handle.join().unwrap();  // 等待子线程完成
// 现在会打印 "子线程结束"
```

### 2.3 join() 的返回值

```rust
let handle = thread::spawn(|| {
    println!("子线程");
    42  // 线程返回值
});

// join() 返回 Result<T, E>
let result = handle.join().unwrap();
println!("子线程返回: {}", result);  // 42
```

### 2.4 解析 `let _ = handle.join().unwrap()`

```rust
let _ = handle1.join().unwrap();
    ^          ^^^^     ^^^^^^^^
    |            |          |
 忽略返回值    等待线程   处理 Result
```

- `join()` → 阻塞等待线程结束，返回 `Result<T, Box<dyn Any>>`
- `.unwrap()` → 如果线程 panic，主线程也 panic
- `let _ =` → 忽略线程返回值

---

## 3. Go 为什么用 Wait 而不是 join？

### 3.1 Go 的并发模型

```go
// Go 用 WaitGroup + Wait
var wg sync.WaitGroup

wg.Add(1)
go func() {
    defer wg.Done()
    fmt.Println("goroutine")
}()

wg.Wait()  // 等待所有 goroutine 完成
```

### 3.2 设计差异

| | Rust | Go |
|--|--|--|
| 线程模型 | 持有 handle，单独 join | WaitGroup 统一管理 |
| 语义 | 线程汇合 | 等待计数器归零 |
| 来源 | Unix/C 的 pthread_join | 自创概念 |

**Go 的 `Wait` 更直白：** 等待计数归零，不涉及"汇合"概念。Go 刻意避免与 C 惯例重名，让命名更清晰。

**Rust 的 `join` 沿用 POSIX 标准：** 保持系统编程的惯例一致性。

---

## 4. move 闭包

### 4.1 线程必须用 move

```rust
let v = vec![1, 2, 3];

// ✗ 编译错误：闭包借用 v，但线程生命周期不确定
thread::spawn(|| {
    println!("{:?}", v);
});

// ✓ 正确：move 转移所有权给子线程
thread::spawn(move || {
    println!("{:?}", v);
});
```

**原因：** 子线程可能比主线程活得更长，编译器无法保证借用的数据在线程运行时还有效，所以必须 `move`。

### 4.2 move 移走所有用到的变量

```rust
let v1 = vec![1, 2, 3];
let v2 = vec![4, 5, 6];
let name = String::from("Alice");

thread::spawn(move || {
    println!("{:?}", v1);    // v1 被移进线程
    println!("{:?}", v2);    // v2 被移进线程
    // name 没用到，不会被移走
});

// v1、v2 失效
// println!("{:?}", v1);  // ✗
println!("{}", name);   // ✓ name 还能用
```

**规则：`move` 只移走闭包里实际用到的、没有 Copy 的变量。**

### 4.3 Copy 类型不受影响

```rust
let num = 42;                // i32，有 Copy
let name = String::from("Alice");  // String，没有 Copy

thread::spawn(move || {
    println!("{}", num);   // num 被复制进闭包
    println!("{}", name);  // name 被移进闭包
});

println!("{}", num);   // ✓ num 有 Copy，还能用
// println!("{}", name);  // ✗ name 已被移走
```

---

## 5. 线程间共享数据

### 5.1 clone（简单但低效）

```rust
let v = vec![1, 2, 3];
let v2 = v.clone();  // 复制一份给子线程

thread::spawn(move || {
    println!("{:?}", v2);
});

println!("{:?}", v);  // 主线程用原始的
```

### 5.2 Arc（多线程引用计数，推荐）

```rust
use std::sync::Arc;

let v = Arc::new(vec![1, 2, 3]);
let v2 = Arc::clone(&v);  // 克隆指针，不克隆数据

let handle = thread::spawn(move || {
    println!("子线程: {:?}", v2);
});

println!("主线程: {:?}", v);
handle.join().unwrap();
```

**Arc = Atomic Reference Counting（原子引用计数）**
- 多个线程共享只读数据
- 线程安全的 `Rc`（单线程版本）

---

## 6. move 的其他用途

`move` 不只在线程里用，是闭包的通用关键字：

### 6.1 返回闭包的函数

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y  // x 移进闭包，否则函数返回后 x 就没了
}

let add5 = make_adder(5);
println!("{}", add5(3));  // 8
```

### 6.2 迭代器

```rust
let prefix = String::from("hello");

let results: Vec<String> = vec!["a", "b", "c"]
    .iter()
    .map(move |s| format!("{} {}", prefix, s))
    .collect();
```

### 6.3 异步代码

```rust
let config = Config::new();

tokio::spawn(async move {
    process(&config).await;
});
```

**核心：只要闭包的生命周期可能比捕获的变量更长，就需要 `move`。**

---

## 7. Drop trait 与提前释放

### 7.1 自动 Drop

Rust 变量离开作用域时自动调用 `Drop` trait：

```rust
{
    let s = String::from("hello");
    // 使用 s...
}  // s 离开作用域，自动释放内存
```

### 7.2 手动 drop()

`drop()` 函数可以提前释放资源：

```rust
use std::fs::File;

let file = File::open("data.txt")?;
// 读取文件...

drop(file);  // 提前关闭文件
// 后续代码不需要文件了，提前释放
```

### 7.3 drop() 消耗所有权

```rust
let v = vec![1, 2, 3];
drop(v);  // v 的所有权被移走
// println!("{:?}", v);  // ✗ v 已失效
```

`drop()` 函数签名：

```rust
pub fn drop<T>(_x: T) {}  // 参数是 T，不是 &T，拿走所有权
```

---

## 8. Mutex 详解

### 8.1 基本用法

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();  // 获取锁
    *num = 6;
}  // num drop，锁自动释放

println!("m = {:?}", m);  // Mutex { data: 6, .. }
```

### 8.2 逐行解析

```rust
let m = Mutex::new(5);
```
创建 Mutex，包裹值 `5`，此时 Mutex **未被锁定**。

```rust
let mut num = m.lock().unwrap();
```
三步操作：
1. `m.lock()` → 获取锁，阻塞等待直到获得独占访问权，返回 `Result<MutexGuard<i32>, PoisonError>`
2. `.unwrap()` → 处理 Result，得到 `MutexGuard<i32>`（锁守卫）
3. `let mut num` → 绑定到 `MutexGuard`，可以通过它修改内部值

```rust
*num = 6;
```
解引用 `MutexGuard`，修改内部的值从 `5` 改为 `6`，此时锁**仍被持有**。

```rust
}
```
作用域结束，`num`（`MutexGuard`）被 drop，**锁自动释放**（RAII 模式）。

### 8.3 主动释放锁

不用作用域也可以显式 `drop()`：

```rust
let m = Mutex::new(5);
let mut num = m.lock().unwrap();
*num = 6;
drop(num);  // 显式释放锁

println!("m = {:?}", m);
```

**推荐用作用域，确保锁尽早释放。**

### 8.4 多线程中使用 Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());  // 10
```

---

## 9. Channel（通道）

### 9.1 基本用法

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

**mpsc = multiple producer, single consumer（多生产者，单消费者）**

### 9.2 通道什么时候关闭？

**所有 `tx`（发送端）都被 drop 时，通道自动关闭。**

```rust
thread::spawn(move || {
    tx.send("hi").unwrap();
    tx.send("st2").unwrap();
    tx.send("st3").unwrap();
    // 线程结束 → tx drop → 通道关闭
});

for rec in rx {
    println!("Got: {}", rec);
}  // tx 关闭后，循环自动结束
```

### 9.3 for rec in rx 的行为

```rust
for rec in rx {
    println!("Got: {}", rec)
}
```

等价于：

```rust
loop {
    match rx.recv() {
        Ok(val) => println!("Got: {}", val),  // 收到消息
        Err(_) => break,  // 通道关闭，退出循环
    }
}
```

- 有消息 → 取出，继续循环
- 没消息但通道未关闭 → **阻塞等待**
- 通道关闭（tx drop）→ 循环结束

**`for rec in rx` 是阻塞的**，只是在 tx drop 后会自动结束。

### 9.4 多个发送者

```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();  // 克隆发送端

thread::spawn(move || {
    tx.send("from tx").unwrap();
});

thread::spawn(move || {
    tx2.send("from tx2").unwrap();
});

for rec in rx {
    println!("Got: {}", rec);
}
// 两个线程结束后，tx 和 tx2 都 drop，通道关闭
```

### 9.5 recv() vs try_recv()

```rust
// recv()：阻塞等待消息
let msg = rx.recv().unwrap();

// try_recv()：不阻塞，立即返回
match rx.try_recv() {
    Ok(msg) => println!("Got: {}", msg),
    Err(_) => println!("No message yet"),
}
```

---

## 10. 常见错误

### 10.1 线程访问已移走的变量

```rust
let v = vec![1, 2, 3];

thread::spawn(move || {
    println!("{:?}", v);  // v 被移进子线程
});

println!("{:?}", v);  // ✗ v 已失效
```

**修复：** clone 一份或用 Arc 共享。

### 10.2 drop 后访问变量

```rust
let my = MyStruct { name: String::from("aaa") };
drop(my);
println!("{}", my.name);  // ✗ my 已被释放
```

**修复：** 把 `drop()` 移到最后。

### 10.3 Mutex 持有锁太久

```rust
let m = Mutex::new(5);
let mut num = m.lock().unwrap();
*num = 6;

// 忘记释放锁
expensive_computation();  // 锁还被持有
```

**修复：** 用作用域限制锁的范围。

---

## 11. 与其他语言对比

| 特性 | Rust | Go | C++ |
|------|------|-----|-----|
| 创建线程 | `thread::spawn` | `go func(){}()` | `std::thread` |
| 等待线程 | `handle.join()` | `wg.Wait()` | `t.join()` |
| 共享数据 | `Arc<T>` + `Mutex` | channel/共享内存 | `shared_ptr` |
| 线程通信 | `mpsc::channel` | `chan` | 无标准库支持 |
| 数据竞争 | 编译时防止 | 运行时检测 | 未定义行为 |

**Rust 的优势：** 编译时保证无数据竞争，无需运行时检测或手动加锁。

---

## 12. 总结

| 概念 | 含义 |
|------|------|
| `thread::spawn` | 创建新线程（fork） |
| `handle.join()` | 等待线程完成（join） |
| `move` | 把捕获的变量所有权移进闭包 |
| `Arc<T>` | 多线程安全的引用计数，共享只读数据 |
| `Mutex<T>` | 多线程安全的内部可变性，共享可变数据 |
| `mpsc::channel` | 线程间消息传递 |
| `drop()` | 提前释放资源 |

**下一步：** 学习异步编程 → `tokio` 和 `async/await`

---

## 13. thread vs async 的选择

### 13.1 为什么不全用 async？

**async 有一个致命的局限：CPU 密集型任务会阻塞整个运行时。**

async 使用协作式调度，任务只在 `.await` 时让出 CPU。如果一个任务一直在做计算，不碰任何 `.await`，其他任务就全部被卡住：

```rust
// ✗ 危险：CPU 密集型任务放进 async
async fn crack_password(hash: &str) -> String {
    // 循环计算，没有任何 await 点
    // 整个 async 运行时被卡死
    // 其他网络请求全部超时
    loop {
        if check_hash(hash) { break; }
    }
}
```

**类比：** async 就像一个单线程的服务员，他只有在等待（`.await`）时才能去服务其他桌。如果一直在厨房炒菜（CPU计算），其他桌的客人就只能干等。

---

### 13.2 为什么不全用 thread？

thread 开销大，大量 IO 等待时浪费内存：

```
场景：1000 个并发网络请求

OS thread：
  1000 线程 × 8MB = 8GB 内存
  大部分时间都在等待网络，CPU 空跑
  操作系统频繁切换线程，开销大

async task：
  1000 任务 × ~几KB = ~几MB 内存
  等待网络时让出 CPU，完全不浪费
  用户态调度，切换成本极低
```

---

### 13.3 两者的适用场景

| 场景 | 推荐 | 原因 |
|------|------|------|
| 网络请求、数据库查询 | `async` | 大量等待，async 高效 |
| 文件读写 | `async` | IO 密集，async 高效 |
| 图片处理、视频编码 | `thread` | CPU 密集，需要真正并行 |
| 加密/解密运算 | `thread` | CPU 密集，需要真正并行 |
| 科学计算 | `thread` | CPU 密集，需要真正并行 |
| Web 服务器 | `async` | 大量并发连接 |

---

### 13.4 CPU 密集型任务阻塞 async 的解决方案

如果在 async 代码里必须做 CPU 密集型任务，用 `spawn_blocking` 把它放到独立线程池：

```rust
use tokio::task;

async fn handle_request() {
    // ✓ 正确：CPU 密集型任务放到独立线程
    let result = task::spawn_blocking(|| {
        expensive_cpu_work()  // 在独立线程执行，不阻塞 async runtime
    }).await.unwrap();

    println!("{}", result);
}
```

`spawn_blocking` 会把任务放到专门的线程池，执行完再把结果传回 async 世界。

---

### 13.5 生产环境最佳实践

**最常见的架构：async + thread 混合使用**

```
                    ┌─────────────────────────┐
网络请求 ──→ async  │  Tokio 运行时            │
数据库查询 ──→ await │  处理大量并发 IO         │
                    │                         │
CPU 密集任务 ──→     │  spawn_blocking() ──→   │──→ 线程池（真正并行）
                    │  Rayon 线程池            │
                    └─────────────────────────┘
```

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // IO 密集：直接 async
    let data = fetch_from_db().await;

    // CPU 密集：放到线程池
    let result = task::spawn_blocking(move || {
        process_data(data)  // CPU 密集计算
    }).await.unwrap();

    // 继续 async
    save_result(result).await;
}
```

---

### 13.6 对比 Go 的解决方案

Go 不需要这么麻烦，goroutine 自动处理：

```go
// Go：goroutine 会被 runtime 自动调度到多个 OS 线程
// CPU 密集和 IO 密集都能处理，runtime 帮你分配
go func() {
    expensiveCPUWork()  // Go runtime 会把它调度到空闲的 OS 线程
}()
```

**Go 的 runtime 用了 M:N 调度模型**（M 个 goroutine 映射到 N 个 OS 线程），天然解决了这个问题，但代价是 runtime 本身的复杂性和一定的运行时开销。

**Rust 选择不内置这个机制**，让开发者显式控制，保持零成本抽象原则。

---

### 13.7 总结

```
IO 密集（网络/文件/数据库）→ async/await  ✓ 高并发，低内存
CPU 密集（计算/加密/压缩）→ thread        ✓ 真正并行，充分利用多核
两者混合               → async + spawn_blocking
```

**thread 不会被 async 取代，因为它们解决的是不同的问题：**
- `async` 解决的是"等待"的效率问题
- `thread` 解决的是"计算"的并行问题
