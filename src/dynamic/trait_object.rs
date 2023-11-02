trait Object {
    type Param;

    fn new(param: Self::Param) -> Self where Self: Sized;

    fn shadow(&self) -> Self where Self: Sized;
    fn do_something(&self);
}

struct Dog {
    num: i32,
}

impl Object for Dog {
    type Param = i32;
    fn new(param: Self::Param) -> Self where Self: Sized {
        Dog { num: param }
    }

    fn shadow(&self) -> Self where Self: Sized {
        Dog { num: self.num }
    }


    fn do_something(&self) {
        println!("wang wang");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::dynamic::trait_object::{Dog, Object};

    #[test]
    fn box_dyn_object_in_hashmap() {
        let mut map: HashMap<String, Box<dyn Object<Param=i32>>> = HashMap::new();
        let dog = Box::new(Dog::new(32));
        map.insert("puppy".to_string(), dog);
        let dog = map.get("puppy").unwrap();
        dog.do_something();
        // dog.shadow(); 编译会报错
    }
}