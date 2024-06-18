//------------------------------------------------
//       Reference Cycles - Memory Leaks
//------------------------------------------------

/*
Rust is known for being a memory safe langauge as conditions like data races cannot occurs but rust makes it
difficult but not impossible to create memory that is never cleaned up. Memory leaks can occur when using the
Rc and RefCell smart pointer.

Using these two smart pointers we can create references where items reference each other in a cycle. This creates
memory leaks because the reference count of each item in the cycle will never reach zero and the values will therefore
are never being dropped.
*/

/*
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}

// The drop function is automatically being called when the Node goes out of scope.
impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping:  {:?}", self);
    }
}


fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!(
        "a strong count:  {:?}    weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&a)),
    }));
    println!(
        "b is created\n a strong count: {:?}   weak count  {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b strong count: {:?}  weak count:  {:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );

    let c = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&b)),
    })); // No Cycle till here

    (*a).borrow_mut().next = Some(Rc::downgrade(&c)); // Non ending cycle is created here. Comment this to see the drop function
                                                      // drop the three nodes

    println!(
        "After creating cycle\n a strong count: {:?}    weak count:  {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b strong count: {:?}   weak count:  {:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!(
        "c strong count: {:?}   weak count:  {:?}",
        Rc::strong_count(&c),
        Rc::weak_count(&c)
    );

    println!("a {:?}", a);
    // The above print statement will cause an overflow in the stack due to reference cycles
    // A->B->C->A->B-C->......  (never ending reference cycle causing the overflow)

    /*
    We know that there is a memory leak as the drop function's print statement
    is not being printed out. The drop function in the Drop trait is automatically called when the main
    exits to drop things out of scope for memory management.

    A smart pointer can only be dropped if there is a single pointer to it i.e., its strong count has a value of 1.

    When the drop function is called, it will done so in the reverse order such that first variable `c`, then `b` and lastly
    `a` will be dropped. This is so because `b` could not be dropped unless `c` is dropped first as dropping `c` will bring
    `b`'s reference count to 1.

    Weak pointers does not stop the drop function from being called. These pointers increase the weak_count by 1 instead
    of the strong_count. As long as the strong_count of a variable is one, the drop function will be called. We use the
    Rc::weak_count() function instead of the Rc::clone() function to create a weak pointer and increase the weak count by
    1. Weak is a type of Rc Pointer, therefore, the defintion of `next` in the Node struct is Option<Weak<RefCell<Node>>>
    and not Option<Rc<RefCell<Node>>>
    */
}
*/

// Weak Rc Pointer usecase

use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}
