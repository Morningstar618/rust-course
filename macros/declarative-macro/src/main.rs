//---------------------------------------------
//              Declarative Macros
//                - Basic Syntax
//---------------------------------------------

/*

macro_rules! macro_name {
    (...) => {...};
    (...) => {...};
    (...) => {...};
}

*/

/*
Macro is just a substitution of code. It does not return anything.
*/

macro_rules! our_macro {
    () => {
        1 + 1
    };

    (test pattern) => {
        println!("Garbage test value!");
    };

    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
}

fn main() {
    println!("{}", our_macro!());
    our_macro!(test pattern);

    println!("{}", our_macro!(2, 4));

    // Can call the macro with any type of bracket
    our_macro!();
    our_macro![];
    our_macro! {};
}
