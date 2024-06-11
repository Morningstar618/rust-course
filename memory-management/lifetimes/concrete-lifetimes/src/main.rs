//------------------------------------------------------
//                Concrete Lifetimes
//------------------------------------------------------

/*
Rust gaurantees that there will be no dangling references. This is accomplished with the help of lifetimes,
which are checked by borrow checker at compile time.
*/

/*
A concrete lifetime is the time during which a value exists inside the memory. The lifetime of a value starts
when it is created and ends when the value is dropped or moved out from a particular memory location, mainly
due to change of ownership or moved out of a scope.
*/

fn main() {
    // Example 1: variable `i` does not live long enough
    /*
        {
            let i = 5;
        }

        let j = i;
        println!("{}", i);
    */

    // Example 2: Ownership of variable `name` passed to the `str_fn` function as it is allocated on the Heap
    /*
        let name = String::from("N3o");
        str_fn(name);
        println!("{}", name);
    */

    // Example 3: Move of String value on Heap occured due to which the lifetime of variable `name` ended there
    // making it invalid to be printed later.
    /*
        let name = String::from("N3o");
        let str_1 = name;
        println!("{}", name);
    */

    // Example 4: Dangling reference, as `i` is a reference to `j` which goes out of scope when the
    // block ends.
    /*
        let i;
        {
            let j = 5;
            i = &j;

        }
        println!("i: {}", i);
    */

    // Example 5: Co-existence of an immutable and mutable reference. This is possible as the lifetime's of both
    // the references do not cross each other. The code won't work if the values of `ref_1` was printed after the
    // initialization of `ref_2`. Lifetime of `ref_1` is limited to 2 lines before the initialization of `ref_2`.
    // Whereas, lifetime of `ref_2` is limited to 3 lines.
    // Additionally, the borrowing rule states that we can have either one mutable reference or multiple immutable
    // reference at any given time. But the code in the example below is compiling because of a rust concept called
    // `non-lexical lifetimes`.
    // Non-lexical lifetimes aim to relax some of the strictness imposed by some of the typical lifetimes. This is
    // achieved by analyzing the actual usage of references in the code, rather than solely relying on scopes. In
    // simple words, the non-lexical lifetimes are lifetimes which are not tied to scopes.
    /*
        let mut vec_1 = vec![5, 10, 15, 20, 25, 30];
        let ref_1 = &vec_1;

        let ref_2 = &mut vec_1;
        println!("{:?}", ref_1);

        ref_2.push(33);
        println!("{:?}", ref_2);
    */
}

fn str_fn(s: String) {
    println!("{}", s);
}
