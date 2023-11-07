本文的资料来自于《Rust圣经》，因为实在太过经典所以稍微改一次写在这里。
对于一个结构体，我们是可以针对每个字段进行可变引用的，这并不违反规则，不管来源是一个值还是一个可变引用。参考borrow_from_value_call。

但当将一个结构体的引用传入一个方法中时，编译器并不管在内部到底是使用了部分字段还是全部字段，统一按照整个结构体值的引用来对待。参考borrow_from_refcell_wrong_call，这种该情况会报出502错误：

```
error[E0502]: cannot borrow `mut_data` as mutable because it is also borrowed as immutable
  --> src/borrow/borrow_refcell.rs:42:17
   |
41 |         let bytes = mut_data.content.as_bytes();
   |                     -------- immutable borrow occurs here
42 |         let _ = mut_data.container.write(bytes);
   |                 ^^^^^^^^                 ----- immutable borrow later used here
   |                 |
   |                 mutable borrow occurs here
```

这个解释要看borrow_from_refcell_wrong_extend方法，是将上面代码进行展开后的：

```rust
fn borrow_from_refcell_wrong_extend(data: RefCell<Data>) {
    let mut mut_data = data.borrow_mut();
    let content = Deref::deref(&mut_data).content.as_bytes();
    let mut container = &mut DerefMut::deref_mut(&mut mut_data).container;
    let _ = container.write(content);
}
```

在第二行（方法内）首先对mut_data进行了不可变引用，虽然只用了content字段，但会持续“持有”整个mut_data的引用。生成的content变量的生命周期会持续到方法结束。
第三行再对mut_data进行可变引用，自然是不行。

所以解决方案是borrow_from_refcell_right_call方法内的，直接通过

```rust
let mut_data = & mut * data.borrow_mut();
```

解引再引用，生成&mut Data，在后文中操作实际上和borrow_from_value_call和borrow_from_mut_ref_call一样了。