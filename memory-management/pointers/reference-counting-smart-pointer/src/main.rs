//---------------------------------------------------
//          Reference Counting Smart Pointer
//---------------------------------------------------

use std::rc::Rc;

enum List {
    Cons(i32, Option<Rc<List>>),
}

/*
Rc smart pointer is useful when mutliple sources are pointing/referencing at the same location
in memory or when there are multiple owners of the same value. In the example below, if we used
Box pointer, we would get the error that move of the value `a` has occurred. We would receive
this error from list `c` as list `b` would be owner of `a`. But now when using Rc pointer, we can
reference to List `a` from inside of `b` and `c` using the Rc clone function. Using that we can
pass a reference to Rc List `a` without causing ownership issues.

`strong_count` function returns the current number of references being made to the Rc List `a`.
`clone` function requires a Rc List type as the input.
 */

fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        println!("Reference count after b: {}", Rc::strong_count(&a));

        let c = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }

    println!("Reference count after scope: {}", Rc::strong_count(&a));
}
