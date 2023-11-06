struct Container {
    callback: Option<Box<dyn Fn(&str)>>,
}

impl Container {
    fn new() -> Self {
        Container { callback: None }
    }

    #[cfg(never_compiled)]
    fn set_callback(&mut self, callback: impl Fn(&str)) {
        self.callback = Some(Box::new(callback));
    }
}

struct Container2<'a, 'b> {
    name: &'a str,
    callback: Option<Box<dyn Fn(&str) + 'b>>,
}

impl<'a, 'b> Container2<'a, 'b> {
    fn new(name: &str) -> Container2 {
        Container2 {
            name,
            callback: None,
        }
    }

    fn set_callback(&mut self, callback: impl Fn(&str) + 'b) {
        self.callback = Some(Box::new(callback));
    }
}

#[cfg(test)]
mod tests {
    use crate::lifetime::static_fn::Container2;

    #[test]
    fn container2_set_closure() {
        let mut container = Container2::new("Rust");
        let string = "hello, world!".to_string();
        container.set_callback(|name| {
            println!("{} says: {}", name, string);
        });
        if let Some(cb) = container.callback {
            cb(container.name);
        }
    }
}


