# Rust 学习文档索引

欢迎学习 Rust！这些文档按难度递进，建议按顺序阅读。

## 📚 文档列表

### 1. [基础入门](./01-basics.md)
**难度：⭐**

适合完全没接触过 Rust 的同学，学习：
- 变量与可变性
- 基本数据类型
- 函数和控制流
- 格式化输出
- 命名规范

**对应项目：** `variables/`, `func/`, `guessgame/`

---

### 2. [所有权系统](./02-ownership.md)
**难度：⭐⭐⭐**

Rust 的核心特性，理解它是掌握 Rust 的关键：
- 所有权的三大规则
- Copy vs Move
- 借用（Borrowing）
- 可变借用和不可变借用
- 悬垂引用

**对应项目：** `ownership/`

---

### 3. [集合类型](./03-collections.md)
**难度：⭐⭐**

深入理解 Rust 的字符串和集合：
- String vs &str
- Vec vs 数组
- 切片（&[T]）
- 迭代器
- usize 类型
- 索引 vs 遍历的类型差异

**对应项目：** `strings-my/`, `vec-my/`, `slices/`

---

### 4. [结构体与方法](./04-structs.md)
**难度：⭐⭐**

面向对象编程的基础：
- 定义和使用结构体
- 方法（Methods）
- 关联函数（Associated Functions）
- User:: vs user. 的区别
- derive traits

**对应项目：** `structs/`

---

### 5. [错误处理](./05-error-handling.md)
**难度：⭐⭐**

Rust 的错误处理机制：
- panic! 和不可恢复错误
- Result<T, E>
- Option<T>
- ? 操作符
- 错误处理最佳实践

---

### 6. [泛型与生命周期](./06-generics-lifetime.md)
**难度：⭐⭐⭐**

Rust 的高级类型系统：
- 泛型函数和泛型结构体
- Trait 约束（`T: Trait`）
- 生命周期标注（`'a`）
- 悬垂引用和内存安全
- 与 Go/TypeScript 泛型对比

**对应项目：** `vec-my/`, `trait-my/`

---

### 7. [Result 类型详解](./07-result-detailed.md)
**难度：⭐⭐**

深入理解 Rust 错误处理的核心类型：
- `Result<T, E>` 的结构和用法
- `Result<Config, &'static str>` 逐个符号解析
- `&'static str` 的含义和使用场景
- `unwrap_or_else` 等方法详解
- 不同错误类型的选择

**对应项目：** `args/`

---

### 8. [闭包（Closures）](./08-closures.md)
**难度：⭐⭐**

Rust 的匿名函数和延迟执行：
- 闭包语法 `||` 和捕获环境变量
- 三种捕获方式（不可变借用/可变借用/move）
- 为什么修改变量的闭包需要 `mut`
- 延迟执行：`unwrap_or` vs `unwrap_or_else`
- `FnOnce`/`FnMut`/`Fn` 三种闭包 trait
- 与 JS/Go 闭包的对比

**对应项目：** `closures/`

---

### 9. [智能指针（Smart Pointers）](./09-smart-pointers.md)
**难度：⭐⭐⭐**

Rust 的堆分配和智能指针：
- `Box<T>` 的三大用途（递归结构、大数据、Trait 对象）
- `Box` vs `&` vs `&&` 的区别
- `Deref` 和 `Drop` trait
- `Rc<T>` / `Arc<T>` / `RefCell<T>` / `Mutex<T>` 详细对比
- 引用计数机制详解
- 与 C++/Go/JS 内存管理对比

**对应项目：** `smart-pointers/`

---

### 10. [多线程与并发](./10-threads.md)
**难度：⭐⭐⭐**

Rust 的线程和并发编程：
- `thread::spawn` 和 fork-join 模型
- `join()` 的含义（为什么不叫 wait）
- `move` 闭包在线程中的使用
- `Arc<T>` 多线程共享数据
- `Mutex<T>` 逐行解析和最佳实践
- Channel（mpsc）线程通信
- `drop()` 提前释放资源
- 与 Go/C++ 并发模型对比

**对应项目：** `threads/`, `mutex/`, `channel/`

---

### 11. [异步编程（Async/Await）](./11-async-await.md)
**难度：⭐⭐⭐⭐**

Rust 的异步编程模型：
- `async fn` 和 `Future` 的概念
- `.await` 的作用和使用场景
- Future 的懒执行特性
- `#[tokio::main]` 宏（为什么不能直接 async fn main）
- trpl（教学库）vs Tokio（生产环境）
- 哪些操作需要 await，哪些不需要
- async vs thread 的选择（IO 密集 vs CPU 密集）
- `select`、`join` 并发模式
- 与 Go/JavaScript 异步模型对比

**对应项目：** `async-my/`, `axum-api/`

---

## 🎯 学习路线建议

### 阶段一：基础语法（1-2天）
1. 阅读 [01-basics.md](./01-basics.md)
2. 完成 `variables/` 和 `func/` 项目
3. 尝试编写猜数字游戏

### 阶段二：所有权（2-3天）
1. 仔细阅读 [02-ownership.md](./02-ownership.md)
2. 完成 `ownership/` 项目
3. **重要：** 多写代码体会所有权转移和借用

### 阶段三：集合类型（1-2天）
1. 阅读 [03-collections.md](./03-collections.md)
2. 完成 `strings-my/`、`vec-my/`、`slices/` 项目
3. 理解为什么 `list[0]` 和 `for item in list` 的类型不同

### 阶段四：结构体（1天）
1. 阅读 [04-structs.md](./04-structs.md)
2. 完成 `structs/` 项目
3. 练习方法和关联函数

### 阶段五：错误处理（1天）
1. 阅读 [05-error-handling.md](./05-error-handling.md)
2. 在之前的项目中加入错误处理

### 阶段六：泛型与生命周期（2天）
1. 阅读 [06-generics-lifetime.md](./06-generics-lifetime.md)
2. 完成 `vec-my/`、`trait-my/` 项目
3. 理解 trait 约束和生命周期标注

### 阶段七：闭包（1天）
1. 阅读 [08-closures.md](./08-closures.md)
2. 完成 `closures/` 项目
3. 理解三种捕获方式和延迟执行

### 阶段八：多线程（1-2天）
1. 阅读 [10-threads.md](./10-threads.md)
2. 完成 `threads/` 项目
3. 理解 fork-join 模型和 `move` 闭包

### 阶段九：异步编程（2-3天）
1. 阅读 [11-async-await.md](./11-async-await.md)
2. 完成 `async-my/` 项目
3. 理解 Future 的懒执行和 async vs thread 的选择
4. 尝试用 Tokio 写简单的网络程序

---

## 💡 学习技巧

1. **多写代码**：Rust 的所有权系统需要通过实践才能理解
2. **看编译器错误**：Rust 编译器错误信息非常详细，仔细阅读
3. **小步前进**：遇到不懂的先跳过，后面会理解
4. **对比其他语言**：如果你会 Go/JS/PHP，对比着学更容易理解

---

## 🔗 常用资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/) - 在线运行 Rust 代码

---

## 📝 更新记录

- 2024-XX-XX: 创建基础文档，覆盖变量、所有权、集合、结构体、错误处理

---

**继续加油！Rust 的学习曲线陡峭，但一旦掌握，你会发现它的强大和优雅。** 🦀
