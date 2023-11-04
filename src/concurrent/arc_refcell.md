Arc要配Mutex是一种习以为常的固定搭配，用以在线程间保证数据正确。
但如果我仅仅在线程间读数据，而不写数据，还需要Mutex么？

实际上是不需要的，在just_println测试中调用Data的println方法，只使用了不可变引用，这样在多线程中共享是安全的。

如果想故意找茬会怎样？在incr_with_arc_only_not_work测试中，我们调用了Data的incr方法，这是一个可变引用的方法。编译出错：

```
[E0596]: cannot borrow data in an `Arc` as mutable
```

Arc实现了Deref，所以在第一个测试中，data.println触发了自动解引用，否则我们也可以直接写成:

```rust
data.deref().println();
```

但Arc特意没实现DerefMut，来保证线程间安全。所以无法调用:

```rust
data.incr();
```

或者说：

```rust
data.deref_mut().incr();
```

那既然无法获得可变引用，我们借助RefCell怎样？在incr_with_arc_refcell测试中，我们引入了RefCell，编译又出错了：

```
[E0277]: `RefCell<Data>` cannot be shared between threads safely
```

这是因为RefCell特意被设计成无法再线程间同步：

```rust
impl<T: ?Sized> ! Sync for RefCell<T> {}
```

即使你给Data实现了Sync和Send也不行。

所以还是老老实实像incr_with_arc_mutex中那样，Arc配合Mutex来使用吧。