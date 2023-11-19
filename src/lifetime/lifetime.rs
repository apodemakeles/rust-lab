#[cfg(test)]
mod m1 {
    fn f1<T: 'static>(_val: T) {}

    struct Animal;

    #[test]
    fn test1() {
        f1("hello, world");
        f1(Animal);
        // let s: &str = String::from("hello, world");
        // f1(s);
    }
}

trait Foo {}

struct Bar;

impl Foo for Bar {}

impl Foo for &str {}

mod m2 {
    use crate::lifetime::lifetime::{Bar, Foo};

    fn f1<T: Foo>(val: T) -> impl Foo {
        val
    }

    fn f2<'a, T: Foo + 'a>(val: T) -> impl Foo + 'a {
        val
    }

    fn f3<T: Foo, K: Foo>(val1: T, val2: K) -> Box<impl Foo> {
        Box::new(val1)
    }

    #[test]
    fn test() {
        f1(Bar);
        f1("hello, world");
        let s: &str = &String::from("hello, world");
        f1(s);
    }
}

mod m3 {
    use crate::lifetime::lifetime::{Bar, Foo};

    /// 不写'static会报错
    fn f1<T: Foo + 'static>(val: T) -> Box<dyn Foo> {
        Box::new(val)
    }

    fn f2<'a, T: Foo + 'a>(val: T) -> Box<dyn Foo + 'a> {
        Box::new(val)
    }

    #[test]
    fn test1() {
        f1(Bar);
        f1("hello, world");
        let s: &str = &String::from("hello, world");
        // f1(s);
        f2(Bar);
        f2("hello, world");
        f2(s);
    }
}

mod m4 {
    use std::future::Future;

    async fn f1(s: &str) -> () {}

    fn f2(s: &str) -> impl Future<Output=()> + 'static {
        async move {}
    }

    fn f3<'a>(s: &'a str) -> impl Future<Output=()> + 'a {
        async move {
            println!("{}", s);
        }
    }

    #[cfg(never_compiled)]
    #[test]
    fn test1() {
        let future;
        {
            let s: &str = &String::from("hello, world");
            future = f1(s);
        }
        let _ = future;
    }

    #[test]
    fn test2() {
        let future;
        {
            let s: &str = &String::from("hello, world");
            future = f2(s);
        }
        let _ = future;
    }
}




