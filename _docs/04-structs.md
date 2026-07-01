# Rust 结构体与方法

> 面向对象编程的基础

## 1. 定义结构体

### 1.1 基本定义

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

### 1.2 创建实例

```rust
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};
```

### 1.3 访问字段

```rust
println!("{}", user.username);
println!("{}", user.email);
```

### 1.4 可变实例

```rust
let mut user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};

user.email = String::from("new@example.com");  // ✓
```

**注意：整个实例必须是可变的，不能只让某些字段可变。**

---

## 2. 简化语法

### 2.1 字段初始化简写

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,     // 变量名和字段名相同，可省略
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### 2.2 结构体更新语法

```rust
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};

// 基于 user1 创建 user2，只改部分字段
let user2 = User {
    email: String::from("bob@example.com"),
    username: String::from("bob"),
    ..user1  // 其余字段从 user1 复制
};

// user1.active 和 sign_in_count 被移走
// 但 user1.email 和 username 还在（String 被移走了）
```

---

## 3. Debug trait

### 3.1 添加 Debug

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};

println!("{:?}", user);   // 单行输出
println!("{:#?}", user);  // 美化输出（多行）
```

### 3.2 常用 derive

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
}
```

| derive | 作用 |
|--------|------|
| `Debug` | 可用 `{:?}` 打印 |
| `Clone` | 可用 `.clone()` 复制 |
| `PartialEq` | 可用 `==` 比较 |
| `Copy` | 赋值时自动复制（只能用于简单类型） |

---

## 4. 方法（Methods）

### 4.1 定义方法

方法第一个参数是 `&self`：

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
}

impl User {
    fn print(&self) {
        println!("User: {}", self.username);
    }

    fn get_email(&self) -> &str {
        &self.email
    }
}

// 调用
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
};
user.print();  // 用 . 调用方法
```

### 4.2 可变方法

```rust
impl User {
    fn change_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

let mut user = User { /* ... */ };
user.change_email(String::from("new@example.com"));
```

### 4.3 self 的三种形式

| 参数 | 说明 | 用途 |
|------|------|------|
| `&self` | 不可变借用 | 只读方法（最常用） |
| `&mut self` | 可变借用 | 修改字段 |
| `self` | 拿走所有权 | 消耗实例（少见） |

---

## 5. 关联函数（Associated Functions）

### 5.1 定义关联函数

没有 `self` 参数，类似其他语言的**静态方法**：

```rust
impl User {
    // 关联函数，通常用作构造器
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    // 其他关联函数
    fn default_user() -> Self {
        Self {
            username: String::from("guest"),
            email: String::from("guest@example.com"),
            active: false,
            sign_in_count: 0,
        }
    }
}
```

### 5.2 调用关联函数

用 `::` 调用（类型级别）：

```rust
let user = User::new(
    String::from("alice"),
    String::from("alice@example.com")
);

let guest = User::default_user();
```

---

## 6. User:: vs user.

### 6.1 核心区别

| 操作符 | 级别 | 需要实例 | 示例 |
|--------|------|---------|------|
| `User::`（双冒号） | 类型级 | ✗ | `User::new()` |
| `user.`（单点） | 实例级 | ✓ | `user.print()` |

### 6.2 对比示例

```rust
impl User {
    // 关联函数（没有 self）
    fn new() -> Self {
        Self { /* ... */ }
    }

    // 方法（有 &self）
    fn print(&self) {
        println!("{}", self.username);
    }
}

// 类型级调用
let user = User::new();  // ✓ 用 User:: 调用关联函数

// 实例级调用
user.print();  // ✓ 用 . 调用方法
```

---

## 7. 结构体字段类型

### 7.1 String vs &str

```rust
// ✓ 推荐：用 String（避免生命周期问题）
struct User {
    username: String,
    email: String,
}

// 需要生命周期标注
struct User<'a> {
    username: &'a str,
    email: &'a str,
}
```

**一般结构体字段用 `String`，函数参数用 `&str`。**

---

## 8. 元组结构体

没有字段名，只有类型：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

println!("{}", black.0);  // 用索引访问
```

---

## 9. 单元结构体

没有任何字段：

```rust
struct UnitStruct;

let unit = UnitStruct;
```

---

## 10. 实际例子

### 10.1 矩形计算

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(20);

    println!("Area: {}", rect1.area());
    println!("Can hold: {}", rect1.can_hold(&rect2));
}
```

### 10.2 用户系统

```rust
#[derive(Debug, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(id: u64, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

fn main() {
    let mut user = User::new(
        1,
        String::from("alice"),
        String::from("alice@example.com")
    );

    println!("{:#?}", user);
    user.deactivate();
    println!("Active: {}", user.is_active());
}
```

---

## 11. 常见错误

### 11.1 字段未初始化

```rust
// ✗ 错误
let user = User {
    username: String::from("alice"),
    // 缺少 email, active, sign_in_count
};
```

### 11.2 尝试打印结构体

```rust
struct User {
    name: String,
}

let user = User { name: String::from("alice") };
println!("{}", user);  // ✗ User 没有实现 Display

// ✓ 正确
#[derive(Debug)]
struct User {
    name: String,
}
println!("{:?}", user);
```

### 11.3 方法没有 self

```rust
impl User {
    fn print() {  // ✗ 没有 self，不能访问字段
        println!("{}", self.username);
    }
}

// ✓ 正确
impl User {
    fn print(&self) {
        println!("{}", self.username);
    }
}
```

---

## 12. Rust vs 其他语言

### 12.1 vs JavaScript

```javascript
// JS 的 class
class User {
    constructor(username, email) {
        this.username = username;
        this.email = email;
    }

    print() {
        console.log(this.username);
    }

    static create() {
        return new User("guest", "guest@example.com");
    }
}
```

```rust
// Rust 的 struct + impl
struct User {
    username: String,
    email: String,
}

impl User {
    fn print(&self) {
        println!("{}", self.username);
    }

    fn create() -> Self {
        Self {
            username: String::from("guest"),
            email: String::from("guest@example.com"),
        }
    }
}
```

**核心区别：Rust 把数据（`struct`）和行为（`impl`）分开定义。**

---

## 练习建议

1. 创建一个 `Book` 结构体，包含标题、作者、页数
2. 为 `Book` 实现方法：`new()`、`is_long()`（页数>300）
3. 创建一个 `Point` 结构体，实现距离计算方法
4. 实现一个简单的购物车系统（`Cart` 和 `Item`）

**下一步：** 学习 [05-error-handling.md](./05-error-handling.md) 了解错误处理
