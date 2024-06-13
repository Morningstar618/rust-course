//-------------------------------------------------
//              RefCell Example
//-------------------------------------------------

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

#[derive(Debug)]
struct File {
    active_users: i32,
}

#[derive(Debug)]
struct User {
    file: Rc<RefCell<File>>,
}

fn main() {
    let mut txt_file = Rc::new(RefCell::new(File { active_users: 0 }));

    // Rc smart pointer only allows immutable shared ownerships of a value. However, this can
    // be overcome by using the RefCell smart pointer.
    let mut user_1 = User {
        file: Rc::clone(&txt_file),
    };
    user_1.file.borrow_mut().active_users += 1;

    let mut user_2 = User {
        file: Rc::clone(&txt_file),
    };
}
