有时候会看到对泛型T进行限定，要求拥有'static生命周期，这是什么意思？ 第一它排除了引用类型&T，比如&str。其次它排除具有引用类型字段的结构体：

```rust
struct Example<'a> {
    name: &'a str,
}

fn takes_static<T: 'static>(t: T) {
    // ...
}

fn main() {
    let e = Example { name: "test" };
    // 这将无法编译，因为 Example<'_> 并不满足 'static 的要求。
    // takes_static(e);
}
```

不具有引用类型字段的结构体的值，默认就具有'static生命周期。

示例代码中的Container的set_callback，接收一个闭包impl Fn(&str) -> &str作为参数，保存到自身的字段。Container本身是'static的(
虽然你不能这么写)，而impl Fn(&str) -> &str的生命周期却不一定，所以编译会报错：

```
error[E0310]: the parameter type `impl Fn(&str) -> &str` may not live long enough
```

编译器提示我们使用'static来修饰闭包。这里有几个问题：

1. 什么样的闭包不具有'static？
2. 为什么要用'static限定我们例子中的闭包？
3. 如果Container不具有'static怎么办？

对于1，如果一个闭包捕捉了外部变量的引用，它的生命周期就被这些变量“挟持”了：

```rust
fn main() {
    let local = "hello".to_string();
    let f = || {
        println!("{}", local);
    };
}
```

闭包f生命周期不可能超过local。

对于2，因为Container的值的生命周期可能会很长，所以闭包字段的生命周期至少不能比它短，用'static修饰是一个比较好的选择。当然，使用'
static限定后，你传入的闭包就不能捕捉引用了。

对于3，实际上需要更精细的生命周期控制，参考Container2代码，引入了专门给闭包字段的'b生命周期，当调用new方法时实际上是创建了一个Container2<'
a, 'static>，因为None里的啥也没有，不受生命周期限制。

而当set_callback方法时，生命周期变成了<'a, 'b>，可以看到container2_set_closure测试中，'a对应"Rust"
字符串的'static生命周期，而'b则是string字符串的生命周期。

有意思的是，如果把这前两行颠倒一下变成：

```rust
let mut container = Container2::new("Rust");
let string = "hello, world!".to_string();
```

则会编译失败：

```
error[E0597]: `string` does not live long enough
  --> src/lifetime/static_fn.rs:43:43
   |
42 |         container.set_callback(|name| {
   |                                ------ value captured here
43 |             println!("{} says: {}", name, string);
   |                                           ^^^^^^ borrowed value does not live long enough
...
48 |     }
   |     -
   |     |
   |     `string` dropped here while still borrowed
   |     borrow might be used here, when `container` is dropped and runs the destructor for type `Container2<'_, '_>`
   |
   = note: values in a scope are dropped in the opposite order they are defined
```

这是因为栈上的数据回收也是按照先进后出规则的，如果string的创建在container之后，就会先drop，其生命周期就会小于container，造成闭包字段的生命周期小于container，这自然是不行。