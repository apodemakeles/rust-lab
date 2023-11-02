这是来源于群友的一个点子，他想定义一个Object trait，实现了此trait可以有一系列类似面向对象语言的方法。
这里保留了new方法，其他方法为我试验时候加上的。

第一个知识点是：

```rust
let _: HashMap<String, Box<dyn Object<Param=i32> > > = HashMap::new();
```

要在这里确定Param=i32，无法写成：HashMap<String, Box<dyn Object>>，编译会报错。容易想到是因为不可能在一个hashmap里插入不同Param类型的value。

关键是第二个知识点：

我们用动态派发的方式Box<dyn trait>，此时的dyn trait被官方称为 trait object type。一个trait要当做trait object使用，要满足很多条件。

假使我们把代码中所有 where Self: Sized;去掉，会报错：

```
error[E0038]: the trait `Object` cannot be made into an object
  --> src/main.rs:21:14
   |
21 |     let map: HashMap<String, Box<dyn Object<Param=i32>>> = HashMap::new();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Object` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> src/main.rs:6:8
   |
3  | trait Object {
   |       ------ this trait cannot be made into an object...
...
6  |     fn new(param: Self::Param) -> Self;
   |        ^^^ ...because associated function `new` has no `self` parameter
help: consider turning `new` into a method by giving it a `&self` argument
   |
6  |     fn new(&self, param: Self::Param) -> Self;
   |            ++++++
help: alternatively, consider constraining `new` so it does not apply to trait objects
   |
6  |     fn new(param: Self::Param) -> Self where Self: Sized;
   |                                        +++++++++++++++++
```

因为不满足条件，所以无法作为trait object type，哪些条件呢？可以参考：https://doc.rust-lang.org/error_codes/E0038.html
就本示例中，是因为new方法的返回参数有Self类型。假设：

```rust
trait Trait {
    fn foo(&self) -> Self;
}

fn call_foo(x: Box<dyn Trait>) {
    let y = x.foo(); // What type is y?
    // ...
}
```

编译器无法知道Self类型具体是什么。
不过也不是就没办法使用Box<dyn Object >了，只要把这种阻碍称为trait object的方法，加上where Self: Sized限定，就可以继续使用。
但你在获取到这个动态派发的对象后，无法调用被限定的方法了，比如：

```rust
// dog.shadow(); 编译会报错
```

有趣的是，这种没有&self参数的方法，也会影响转换为trait object，也需要限定：

```rust
trait Foo {
    fn foo() -> u8 where Self: Sized;
}
```

从没见过在这里写Self的情况。