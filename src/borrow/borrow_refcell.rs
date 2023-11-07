#[derive(Debug, Default)]
struct Data {
    content: String,
    container: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::io::Write;

    use crate::borrow::borrow_refcell::Data;

    #[test]
    fn borrow_from_value() {
        let data = Data::default();
        borrow_from_value_call(data);
    }

    fn borrow_from_value_call(mut data: Data) {
        let content = &data.content;
        let container = &mut data.container;
        let _ = container.write(content.as_bytes());
    }

    #[test]
    fn borrow_from_mut_ref() {
        let mut data = Data::default();
        borrow_from_mut_ref_call(&mut data);
    }

    fn borrow_from_mut_ref_call(data: &mut Data) {
        let content = &data.content;
        let container = &mut data.container;
        let _ = container.write(content.as_bytes());
    }

    #[cfg(never_compilation)]
    fn borrow_from_refcell_wrong_call(data: RefCell<Data>) {
        let mut_data = data.borrow_mut();
        let bytes = mut_data.content.as_bytes();
        let _ = mut_data.container.write(bytes);
    }

    #[cfg(never_compilation)]
    fn borrow_from_refcell_wrong_extend(data: RefCell<Data>) {
        let mut mut_data = data.borrow_mut();
        let content = Deref::deref(&mut_data).content.as_bytes();
        let mut container = &mut DerefMut::deref_mut(&mut mut_data).container;
        let _ = container.write(content);
    }

    #[test]
    fn borrow_from_refcell_right() {
        let mut data = Data::default();
        borrow_from_refcell_right_call(RefCell::new(data));
    }

    fn borrow_from_refcell_right_call(data: RefCell<Data>) {
        let mut_data = &mut *data.borrow_mut();
        let content = &mut_data.content;
        let container = &mut mut_data.container;
        let _ = container.write(content.as_bytes());
    }
}