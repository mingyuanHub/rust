# 智能指针（Smart Pointers）

> 难度：⭐⭐⭐

---

## 1. 什么是智能指针？

智能指针是**拥有额外功能的指针**，不仅存储地址，还管理数据的生命周期和访问方式。

| 类型 | 含义 | 用途 |
|------|------|------|
| `Box<T>` | 堆上分配 | 递归结构、大数据、Trait 对象 |
| `Rc<T>` | 引用计数 | 多个所有者共享数据 |
| `RefCell<T>` | 运行时借用检查 | 内部可变性 |
| `Arc<T>` | 线程安全引用计数 | 多线程共享数据 |

普通引用 `&T` 只是借用，不拥有数据；智能指针**拥有**它指向的数据。

---

## 2. Box<T>

### 2.1 基本用法

`Box::new()` 把数据分配到堆上：

```rust
let x = 5;               // 栈上
let y = Box::new(5);     // 堆上

println!("{}", y);       // 5，自动解引用
println!("{}", *y);      // 5，手动解引用
```

### 2.2 Box vs 普通引用

```rust
let x = String::from("xxx");

let a = &x;              // &String，栈上的引用，借用
let b = &&x;             // &&String，引用的引用，仍在栈上
let c = Box::new(&x);    // Box<&String>，把引用放到堆上，拥有所有权
```

内存结构：

```
&x：
栈: a ──→ x（"xxx"）

&&x：
栈: b ──→ a ──→ x（"xxx"）
全部在栈上，只是多一层指针

Box::new(&x)：
栈: c ──→ 堆: [&x] ──→ x（"xxx"）
              ^^^
              这一层在堆上，Box 拥有它
```

解引用对比：

```rust
let x = String::from("xxx");
let y = Box::new(&x);
let z = &&x;

assert_eq!("xxx", *y);   // *y  → &String → 自动解引用 ✓
assert_eq!("xxx", **y);  // **y → String  ✓
assert_eq!("xxx", *z);   // *z  → &String → 自动解引用 ✓
assert_eq!("xxx", **z);  // **z → String  ✓
// 结果一样，但 &&x 全在栈上，Box 的内容在堆上
```

---

## 3. Box 的三个实际用途

### 3.1 递归数据结构（最重要）

递归枚举/结构体不能直接嵌套自身，因为编译器无法确定大小：

```rust
// ✗ 编译错误：大小无限递归，无法确定
enum List {
    Cons(i32, List),
    Nil,
}

// ✓ 用 Box 解决：Box 大小固定（一个指针，8字节）
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = Cons(1,
    Box::new(Cons(2,
        Box::new(Cons(3,
            Box::new(Nil))))));
```

内存结构：

```
栈: list → 堆: Cons(1, ptr)
                       │
                       └→ 堆: Cons(2, ptr)
                                      │
                                      └→ 堆: Cons(3, Nil)
```

### 3.2 大数据避免栈溢出

栈空间有限（通常 8MB），大数据放堆上更安全：

```rust
// 栈上分配，可能栈溢出
let big_data = [0u8; 1_000_000];

// 堆上分配，安全
let big_data = Box::new([0u8; 1_000_000]);
```

### 3.3 Trait 对象（动态多态）

不知道具体类型时，用 `Box<dyn Trait>` 统一处理：

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("Woof!"); }
}
impl Animal for Cat {
    fn speak(&self) { println!("Meow!"); }
}

// 用 Box<dyn Animal> 存不同类型的动物
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in &animals {
    animal.speak();
}
// Woof!
// Meow!
```

---

## 4. 什么时候不需要 Box？

`String` 和 `Vec` 内部已经是堆分配的，不需要再套 `Box`：

```rust
// ✗ 多此一举
let s = Box::new(String::from("hello"));
let v = Box::new(vec![1, 2, 3]);

// ✓ 直接用
let s = String::from("hello");
let v = vec![1, 2, 3];
```

---

## 5. 普通引用 vs &&x vs Box 对比

| | `&x` | `&&x` | `Box::new(x)` |
|--|--|--|--|
| 类型 | `&T` | `&&T` | `Box<T>` |
| 存储位置 | 栈 | 栈 | 堆 |
| 所有权 | 无（借用） | 无（借用） | 有 |
| 用途 | 普通借用 | 多层引用 | 堆分配 |

---

## 6. Deref（解引用）

`Box<T>` 实现了 `Deref` trait，可以像普通引用一样使用 `*` 解引用：

```rust
let x = 5;
let y = Box::new(x);

assert_eq!(5, *y);  // * 解引用，得到 5
```

**自动解引用（Deref coercion）：** 方法调用时自动解引用

```rust
let s = Box::new(String::from("hello"));
println!("{}", s.len());  // ✓ 自动解引用，调用 String 的 len()
```

---

## 7. Drop（自动释放）

`Box<T>` 实现了 `Drop` trait，离开作用域时自动释放堆内存：

```rust
{
    let b = Box::new(String::from("hello"));
    // 使用 b...
}  // b 离开作用域，自动释放堆上的内存
   // 不需要手动 free（和 C 不同）
```

---

## 8. 与其他语言对比

| 语言 | 堆分配方式 | 内存释放 |
|------|----------|---------|
| Rust | `Box::new()` | 自动（离开作用域） |
| C++ | `new` | 手动 `delete` |
| Go | 编译器自动决定 | GC |
| JavaScript | 所有对象在堆上 | GC |

**Rust 的优势：** 自动释放（不需要 GC，也不需要手动 free），零成本。

---

## 9. 实际使用场景总结

| 场景 | 使用 Box |
|------|---------|
| 递归枚举（链表、树） | ✓ 必须用 |
| 大型数据结构 | ✓ 避免栈溢出 |
| `dyn Trait` 对象 | ✓ 常用 |
| 普通字符串/Vec | ✗ 不需要 |
| 简单数值类型 | ✗ 没必要 |

---

## 10. Rc<T>、Arc<T>、RefCell<T>、Mutex<T> 对比

这四个智能指针各有不同用途，关键在于**单线程/多线程**和**共享所有权/内部可变性**的组合。

### 10.1 核心对比表

| 类型 | 用途 | 线程安全 | 可变性 | 组合使用 |
|------|------|---------|--------|---------|
| `Rc<T>` | 共享所有权（只读） | ✗ 单线程 | 不可变 | `Rc<RefCell<T>>` |
| `Arc<T>` | 共享所有权（只读） | ✓ 多线程 | 不可变 | `Arc<Mutex<T>>` |
| `RefCell<T>` | 内部可变性 | ✗ 单线程 | 可变 | `Rc<RefCell<T>>` |
| `Mutex<T>` | 内部可变性 | ✓ 多线程 | 可变 | `Arc<Mutex<T>>` |

---

### 10.2 Rc<T>（单线程引用计数）

**用途：** 单线程中，多个地方需要"拥有"同一份只读数据。

```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let data2 = Rc::clone(&data);  // 引用计数 +1
let data3 = Rc::clone(&data);  // 引用计数 +1

println!("{:?}", data);   // ✓ 三个都能用
println!("count = {}", Rc::strong_count(&data));  // 3
```

**限制：** 不能修改内部数据。

```rust
// data.push(4);  // ✗ Rc 不提供可变访问
```

---

### 10.3 Arc<T>（多线程引用计数）

**用途：** 多线程中，共享只读数据。

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data2 = Arc::clone(&data);

thread::spawn(move || {
    println!("子线程: {:?}", data2);
}).join().unwrap();

println!("主线程: {:?}", data);
```

**Arc = Atomic Reference Counting（原子引用计数）**，线程安全版的 `Rc`。

---

### 10.4 RefCell<T>（单线程内部可变性）

**用途：** 单线程中，需要在有不可变引用的情况下修改数据。

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

// 借用检查在运行时进行
data.borrow_mut().push(4);  // 可变借用
println!("{:?}", data.borrow());  // 不可变借用
```

**运行时借用检查：** 如果违反借用规则会 panic。

```rust
let data = RefCell::new(5);
let a = data.borrow_mut();  // 可变借用
let b = data.borrow();      // ✗ panic：已有可变借用
```

---

### 10.5 Mutex<T>（多线程内部可变性）

**用途：** 多线程中，安全地修改共享数据。

```rust
use std::sync::Mutex;

let data = Mutex::new(vec![1, 2, 3]);

{
    let mut guard = data.lock().unwrap();
    guard.push(4);
}  // 锁释放

println!("{:?}", data);
```

**Mutex = Mutual Exclusion（互斥锁）**，线程安全版的 `RefCell`。

---

### 10.6 常见组合

#### Rc<RefCell<T>>（单线程，多所有者，可变数据）

```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(vec![1, 2, 3]));
let data2 = Rc::clone(&data);

data.borrow_mut().push(4);   // 通过 data 修改
data2.borrow_mut().push(5);  // 通过 data2 修改

println!("{:?}", data.borrow());  // [1, 2, 3, 4, 5]
```

**用途：** 树、图等复杂数据结构，多个节点指向同一个数据。

#### Arc<Mutex<T>>（多线程，多所有者，可变数据）

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

**用途：** 多线程共享可变状态（最常用）。

---

### 10.7 选择指南

```
需要共享数据吗？
├─ 是 → 需要修改吗？
│      ├─ 否 → 单线程？
│      │       ├─ 是 → Rc<T>
│      │       └─ 否 → Arc<T>
│      └─ 是 → 单线程？
│              ├─ 是 → Rc<RefCell<T>>
│              └─ 否 → Arc<Mutex<T>>
└─ 否 → 用普通所有权或借用 &T
```

---

### 10.8 借用检查对比

| | 检查时机 | 违规后果 |
|--|---------|---------|
| 普通引用 `&T` / `&mut T` | 编译时 | 编译错误 |
| `RefCell<T>` | 运行时 | panic |
| `Mutex<T>` | 运行时 | 阻塞等待或 panic |

---

### 10.9 性能对比

| 类型 | 性能开销 |
|------|---------|
| `Rc<T>` | 引用计数（非原子） |
| `Arc<T>` | 原子引用计数（稍慢） |
| `RefCell<T>` | 运行时借用检查 |
| `Mutex<T>` | 锁开销 + 原子操作 |

**优先级：** 普通借用 `&T` > `Rc` > `Arc` > `Mutex`

---

### 10.10 引用计数详解

**引用计数（Reference Counting）** 是一种内存管理方式：记录有多少个地方在使用某块数据，当计数归零时自动释放。

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));  // 计数 = 1
let b = Rc::clone(&a);                   // 计数 = 2
let c = Rc::clone(&a);                   // 计数 = 3

println!("count = {}", Rc::strong_count(&a));  // 3

drop(c);  // 计数 = 2
drop(b);  // 计数 = 1
// a drop → 计数 = 0 → "hello" 被释放
```

**与其他语言对比：**

| 语言 | 引用计数 |
|------|---------|
| Python | 所有对象都用 |
| Swift | ARC（自动引用计数） |
| Rust | `Rc<T>` / `Arc<T>` 显式使用 |
| Go | 无（用 GC） |

---

### 10.11 实际场景总结

| 场景 | 使用 |
|------|------|
| 单线程图结构 | `Rc<RefCell<Node>>` |
| 多线程计数器 | `Arc<Mutex<i32>>` |
| 多线程只读配置 | `Arc<Config>` |
| 单线程只读共享 | `Rc<T>` |

---

**下一步：** 学习 Channel 和线程通信 → `channel/`
