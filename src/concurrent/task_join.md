这些代码对tokio中异步任务执行做了一些研究

### future_wait_test

async fn sleep(sec: u64)是一个异步方法，基于tokio进行异步sleep。
在future_wait_test中，将两个sleep任务加入到Vec中，第一个休眠5秒，第二个休眠1秒。 之后迭代进行await操作。
最终两个任务依次执行，等待5秒后又等待1秒，整个方法执行了6秒。
sleep方法有些反直觉，第一反应是在加入到Vec时就已经触发执行了。然而Rust中Future是延迟执行的，一般（注意是一般）会在.await时才会触发执行。
另外一个反直觉的是，async fn sleep返回的是()，那最终加入到Vec中的就是一堆()
了？实际async方法返回的是Future，所以这个方法返回的Future<()>，.await也只能对Future使用。
所以对于有多个Future的容器，依次调用.await，还是等于对多个Future执行的总时间。

那有没有让所有Future同时执行的办法呢？

### spawn_wait_test

tokio::spawn方法接收一个Future，并且立即触发，返回tokio::JoinHandle，这也是一个Future。
所以spawn_wait_test方法中sleep 1会先执行完，之后sleep 5完成，二者同时触发，一共用时5秒。

### future_join_test

很多语言的SDK都内置了对一组Future执行all语义的方法，即等待所有任务都完成再返回。tokio中有join_all，但需要你预先知道有哪些Future，对于我们这种Vec的情况，需要引入futures包。
futures包提供了join_all方法，其效果和spawn类似，会触发所有Future。最终返回的JoinAll也是一个Future。
可以看到，执行结果和spawn_wait_test一样。

### future_try_join_test

join_all方法会无论某一个Future成功还是失败，都要全部完成才会返回。但可能有时候我们需要的语义是任意Future执行失败后，就结束等待，并返回失败的结果。这个失败一般是Result的Err。
try_join_all执行这种语义，其接收一个迭代器，要求每个元素是TryFuture类型，TryFuture是Future的子trait，定义返回值为Result。
可以看到，其实现了：

```rust
impl<F, T, E> TryFuture for F
    where
        F: ?Sized + Future<Output=Result<T, E>> {
    type Ok = T;
    type Error = E;
}
```

只要一个Future的返回值是Result,其就可以作为try_join_all的参数。
在future_try_join_test中，我们展示了将Future放入Vec的另一种方法。之前所有返回Future的函数（比如sleep)
，都被提炼成公共的函数，你可以尝试一下这样写：

```rust
vec![
    async {
        tokio::time::sleep(Duration::from_secs(5)).await;
    },
    async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    },
];
```

这样编译会不通过，因为前后两个Future实际上是不同的类型，无法添加到同一个Vec中。
一个解决办法是如future_try_join_test一样，利用动态派发，将所有Future转换为Box<dyn Future<_>>
。但光这样还不行，还需要引入Pin，为什么这样我现在还无法解释。

可以看到结果符合预期，在执行的第一秒，try_join_all就返回了Err，并没有等到sleep 5的任务完成。如果全部成功，会返回一个Ok(
Vec<_>)的结果。

另外值得一说的是，try_join_all需要的TryFuture，或者说Future<Output=Result>，为什么我们可以传入Pin<Box<dyn Future<
Output=Result>>>>？
虽然有deref，但也只在解引用操作*，或者调用方法时才有效，直接传参时deref并不能产生等价的效果。
实际上是Rust为Pin也实现了Future trait:

```rust
#[stable(feature = "futures_api", since = "1.36.0")]
impl<P> Future for Pin<P>
    where
        P: ops::DerefMut<Target: Future>,
{
    type Output = <<P as ops::Deref>::Target as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        <P::Target as Future>::poll(self.as_deref_mut(), cx)
    }
}
```

这里的P，对应到我们就是Box<dyn Future>，其肯定可以as_deref_mut()产生一个&mut Future。