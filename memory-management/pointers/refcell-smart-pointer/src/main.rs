//------------------------------------------------
//          RefCell Smart Pointer
//------------------------------------------------

/*
This pointer has two use cases. First it allows checking for Borrowing and Ownership rules at runtime and
not compile time. Secondly, it allows for having mutable access to a data allocated on Heap, even if its
immutable.
*/

use std::{cell::RefCell, rc::Rc};

fn main() {
    // Example 1
    // let mut x = 5;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{} {}", x1, x2);

    // let a = RefCell::new(5);
    // // {
    // //     let b = a.borrow();
    // //     let c = a.borrow();
    // // }
    // // drop(b);
    // // drop(c);

    // let d = a.borrow_mut();
    // drop(d);
    // // println!("{} {}", b, c);

    // println!("{:?}", a);

    //----------------------------------------------------------------------------------------

    // Example 2 - Interior Mutability (Checking for borrowing rules at execution time)
    // let x = 5;
    // let x1 = &mut x;

    // let a = RefCell::new(6);
    // // let c = *a; // RefCell does not implement the `deref` trait, thus the error.
    // let mut b = a.borrow_mut();
    // *b = 8;
    // drop(b);
    // println!("{:?}", a);

    //-------------------------------------------------------------------------------------------

    // Example 3 - RefCell + Rc Pointer (Multiple Ownerships with Mutability capability)
    let a = Rc::new(RefCell::new(String::from("C++")));
    let mut b = Rc::clone(&a);
    *b.borrow_mut() = String::from("Rust");
    println!("{:?}", a);
}
