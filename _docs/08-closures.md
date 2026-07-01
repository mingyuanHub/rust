# 闭包（Closures）

> 难度：⭐⭐

---

## 1. 什么是闭包？

闭包是**可以捕获环境变量的匿名函数**，用 `||` 定义：

```rust
// 普通函数
fn add(x: i32) -> i32 { x + 5 }

// 闭包（匿名函数）
let add = |x| x + 5;

// 调用方式一样
println!("{}", add(5));  // 10
```

---

## 2. 闭包语法

```rust
|参数| 表达式                    // 单行
|参数| { 多行代码; 返回值 }      // 多行
|| 表达式                        // 无参数
|x: i32| -> i32 { x + 5 }      // 显式类型（通常不需要）
```

对比其他语言：

| 语言 | 语法 |
|------|------|
| Rust | `\|x\| x * 2` |
| JavaScript | `x => x * 2` |
| Go | `func(x int) int { return x * 2 }` |
| Python | `lambda x: x * 2` |

---

## 3. 闭包捕获环境变量

闭包最大的特点：**可以捕获（使用）外部作用域的变量**。

```rust
let base = 10;
let add_base = |x| x + base;  // 捕获了外部变量 base

println!("{}", add_base(5));  // 15
```

普通函数做不到这一点：

```rust
let base = 10;
fn add_base(x: i32) -> i32 {
    x + base  // ✗ 编译错误：普通函数不能捕获外部变量
}
```

---

## 4. 三种捕获方式

### 4.1 不可变借用（只读）

```rust
let list = vec![1, 2, 3];
let print_list = || println!("{:?}", list);  // 只读，借用 list

print_list();
print_list();  // ✓ 可以多次调用
println!("{:?}", list);  // ✓ list 还可以用
```

### 4.2 可变借用（修改）

```rust
let mut list = vec![1, 2, 3];
let mut push_item = || list.push(4);  // 修改 list，需要可变借用
//  ^^^
//  闭包本身也需要 mut

push_item();
println!("{:?}", list);  // [1, 2, 3, 4]
```

**为什么闭包本身需要 `mut`？**

闭包调用时会修改自己内部捕获的变量，相当于修改闭包自身的状态。Rust 规定：修改自身状态的操作需要 `mut`。

```rust
// 闭包等价于这个结构体
struct PushItem<'a> {
    list: &'a mut Vec<i32>,  // 持有可变引用
}
impl<'a> PushItem<'a> {
    fn call(&mut self) {      // 调用时需要 &mut self
        self.list.push(4);
    }
}
```

**不修改则不需要 mut：**

| 闭包行为 | 需要 `mut` |
|---------|-----------|
| 只读捕获变量 | ✗ |
| 修改捕获变量 | ✓ |
| 移走捕获变量 | ✓ |

### 4.3 move（转移所有权）

```rust
let list = vec![1, 2, 3];
let print_list = move || println!("{:?}", list);
//               ^^^^
//               把 list 的所有权移进闭包

print_list();
// println!("{:?}", list);  // ✗ list 已被移走
```

`move` 常用于多线程，确保闭包拥有数据的所有权：

```rust
use std::thread;

let list = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", list);  // 新线程需要拥有 list
}).join().unwrap();
```

---

## 5. 延迟执行

`||` 创建的是**函数对象**，不是立即执行，而是"稍后调用"：

```rust
// 立即调用
let result = get_shirt();

// 创建闭包（延迟执行）
let closure = || get_shirt();
// 这里还没执行 get_shirt()
// 只有调用 closure() 时才执行
```

**这是 unwrap_or_else 和 unwrap_or 的核心区别：**

```rust
// unwrap_or：无论 Some/None，都立即计算默认值
option.unwrap_or(expensive_function())  // 即使 Some 也会调用！

// unwrap_or_else：只有 None 时才执行闭包（延迟执行）
option.unwrap_or_else(|| expensive_function())  // 只有 None 才调用
```

实际验证：

```rust
let x: Option<i32> = Some(5);

// unwrap_or：expensive() 总会执行
let result = x.unwrap_or({
    println!("called");  // 会打印，即使 x 是 Some
    0
});

// unwrap_or_else：expensive() 只有 None 才执行
let result = x.unwrap_or_else(|| {
    println!("called");  // 不会打印，因为 x 是 Some
    0
});
```

---

## 6. 闭包作为函数参数

`unwrap_or_else` 接收的是闭包类型 `FnOnce() -> T`，所以必须传 `||`：

```rust
pub fn unwrap_or_else<F>(self, f: F) -> T
where
    F: FnOnce() -> T  // 参数是闭包
{
    match self {
        Some(x) => x,
        None => f(),  // 只有 None 才调用
    }
}

// 使用
option.unwrap_or_else(|| self.get_shirt())
//                    ^^
//                    || 创建闭包，传给 unwrap_or_else
```

---

## 7. 实际例子

### 7.1 库存赠品（你的代码）

```rust
impl Inventory {
    fn giveaway(&self, user_per: Option<ShirtColor>) -> ShirtColor {
        user_per.unwrap_or_else(|| self.get_shirt())
        // 用户有偏好 → 返回用户偏好
        // 用户没有偏好 → 调用 get_shirt() 返回库存最多的颜色
    }
}
```

等价的 match 写法：

```rust
match user_per {
    Some(color) => color,
    None => self.get_shirt(),
}
```

### 7.2 map/filter 链式操作

```rust
let numbers = vec![1, 2, 3, 4, 5];

let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)   // 过滤偶数
    .map(|&x| x * 10)            // 每个乘以 10
    .collect();

println!("{:?}", result);  // [20, 40]
```

---

## 8. Rust 闭包 vs 其他语言

Rust 的闭包语法和 JS、Go 类似，但根源是**函数式编程**（Lisp、Haskell），不是互相借鉴：

| 特性 | Rust | JavaScript | Go |
|------|------|------------|-----|
| 语法 | `\|x\| x * 2` | `x => x * 2` | `func(x int) int {...}` |
| 捕获方式 | 明确（借用/move） | 自动（GC 管理） | 自动（GC 管理） |
| 性能 | 零成本抽象 | 有运行时开销 | 有运行时开销 |
| 类型推断 | 强，通常不用标注 | 动态类型 | 需要显式类型 |

**Rust 闭包的独特优势：**
- 编译器内联优化，和手写循环一样快
- 所有权系统保证闭包使用安全
- 三种捕获方式（不可变借用/可变借用/move）明确可控

---

## 9. 三种闭包 trait

Rust 根据闭包的行为分为三种类型：

| Trait | 含义 | 可调用次数 |
|-------|------|-----------|
| `FnOnce` | 消耗捕获的变量（move） | 只能调用一次 |
| `FnMut` | 修改捕获的变量 | 可多次调用 |
| `Fn` | 只读捕获的变量 | 可多次调用 |

```rust
// FnOnce：只能调用一次
let s = String::from("hello");
let consume = move || println!("{}", s);  // s 被移走
consume();
// consume();  // ✗ s 已经被消耗

// FnMut：可多次调用，但需要 mut
let mut count = 0;
let mut increment = || count += 1;
increment();
increment();  // ✓ 可以多次调用

// Fn：可多次调用，不修改
let x = 5;
let print_x = || println!("{}", x);
print_x();
print_x();  // ✓
```

**`FnOnce` 是最宽泛的约束（所有闭包都实现了它），`Fn` 是最严格的约束。**

---

## 10. 常见错误

### 10.1 可变闭包忘记加 mut

```rust
let mut list = vec![1, 2, 3];
let closure = || list.push(4);  // ✗ 需要 mut
closure();

// ✓ 正确
let mut closure = || list.push(4);
closure();
```

### 10.2 借用期间修改原变量

```rust
let mut list = vec![1, 2, 3];
let mut closure = || list.push(4);

println!("{:?}", list);  // ✗ list 被闭包借用，不能同时使用

closure();
println!("{:?}", list);  // ✓ 闭包调用完，借用结束
```

---

**下一步：** 学习迭代器（Iterator）→ `vec-my/`

---

## 11. 闭包 vs async 块

### 11.1 语法区别

```rust
// 闭包：需要 ||
thread::spawn(move || {
//            ^^^^ ^^
//            move 闭包
    println!("thread");
})

// async 块：不需要 ||
trpl::block_on(async {
//             ^^^^^
//             async 块，不是闭包
    println!("async");
})
```

### 11.2 本质区别

| | 闭包 `\|\| { }` | async 块 `async { }` |
|--|----------------|---------------------|
| 返回 | 闭包对象 | Future |
| 执行 | 立即（或延迟调用） | 懒执行（需要 await） |
| 用途 | 函数式编程、回调 | 异步编程 |
| 语法 | 独立概念 | 独立概念 |

```rust
// 闭包：|| 创建匿名函数
let closure = || { println!("closure") };
closure();  // 调用闭包

// async 块：创建 Future
let future = async { println!("async") };
future.await;  // 等待 Future
```

### 11.3 可以组合使用

```rust
// 闭包里包含 async 块
let closure = || async {
//            ^^ ^^^^^
//            闭包 + async 块
    println!("async in closure");
};

let future = closure();  // 调用闭包，得到 Future
future.await;  // 等待 Future
```

**结论：`async { }` 是独立语法，不需要 `||`。它们是不同的语言特性，用于不同的场景。**
