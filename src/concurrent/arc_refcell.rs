struct Data {
    num: i32,
}

impl Data {
    fn println(&self) {
        println!("num is: {}", self.num);
    }

    fn incr(&mut self) {
        self.num = self.num + 1;
    }
}

// unsafe impl Send for Data {}
//
// unsafe impl Sync for Data {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::concurrent::arc_refcell::Data;

    #[test]
    fn just_println() {
        let data = Data { num: 1 };
        let data = Arc::new(data);
        let data2 = data.clone();
        let h1 = thread::spawn(move || {
            data.println();
        });
        let h2 = thread::spawn(move || {
            data2.println();
        });
        let _ = h1.join();
        let _ = h2.join();
    }

    #[test]
    fn just_println_2() {
        let data = Data { num: 1 };
        thread::scope(|scope| {
            let h1 = scope.spawn(|| {
                data.println();
            });
            let h2 = scope.spawn(|| {
                data.println();
            });
            let _ = h1.join();
            let _ = h2.join();
        });
        data.println();
    }

    // cannot pass
    #[cfg(never_compiled)]
    #[test]
    fn incr_with_arc_only_not_work() {
        let data = Data { num: 1 };
        let data = Arc::new(data);
        let data2 = data.clone();
        let h1 = thread::spawn(move || {
            data.incr();
            data.println();
        });
        let h2 = thread::spawn(move || {
            data.incr();
            data2.println();
        });
        let _ = h1.join();
        let _ = h2.join();
    }

    //
    // // cannot pass
    #[cfg(never_compiled)]
    #[test]
    fn incr_with_arc_refcell() {
        let data = Data { num: 1 };
        let mut data = Arc::new(RefCell::new(data));
        let mut data2 = data.clone();
        let h1 = thread::spawn(move || {
            data.borrow_mut().incr();
            data.borrow().println();
        });
        let h2 = thread::spawn(move || {
            data2.borrow_mut().incr();
            data2.borrow().println();
        });
        let _ = h1.join();
        let _ = h2.join();
    }

    #[test]
    fn incr_with_arc_mutex() {
        let data = Data { num: 1 };
        let mut data = Arc::new(Mutex::new(data));
        let mut data2 = data.clone();
        let h1 = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.incr();
            data.println();
        });
        let h2 = thread::spawn(move || {
            let mut data2 = data2.lock().unwrap();
            data2.incr();
            data2.println();
        });
        let _ = h1.join();
        let _ = h2.join();
    }
}