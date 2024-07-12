//---------------------------------
//      Capturing Types
//---------------------------------

macro_rules! input {
    // `ty` is used as a capturing type for a data type such as i32, f32, etc.
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input");

        let n: $t = n.trim().parse().expect("invalid input");
        n
    }};
}

macro_rules! add_as {
    // `expr` is used as a capturing type for expressions
    ($a: expr, $b: expr, $type: ty) => {
        $a as $type + $b as $type
    };
}

macro_rules! some_macro {
    // The `identifier` capturing type discussed in detail in the main function.
    ($var: ident) => {
        $var = $var + 1;
    };
}

macro_rules! create_function {
    ($func_name: ident, $input: ident, $type_input: ty, $type_output: ty) => {
        fn $func_name($input: $type_input) -> $type_output {
            println!(
                "You called {:?}() with the input of {:?}",
                stringify!($func_name),
                stringify!($input)
            );

            $input
        }
    };
}

create_function!(f1, x, i32, i32);

fn main() {
    println!("Please entre a floating point number");
    let some_input = input!(f32);

    println!("{}", some_input);

    println!("Add: {}", add_as!(5, 2.6, f32));

    let mut x = 4;
    some_macro!(x);
    // x = x + 1;
    /*
    Ideally the above line of code should work because the code inside the macro is simply
    being substituted, but in rust, the variables declared inside a macro are completely distinct from
    the variables declared outside the macro world. They are not allowed to cross boundaries and for
    this reason, they are also sometimes referred to as `hygenic`.

    This is where another capturing type called `identifiers` come into play. They allow you to cross the
    boundaries which otherwise may not be possible.
    */

    let some_variable = f1(15);
    println!("{}", some_variable);
}

/*
The Macros does not take ownership of something. You have to keep an eye on the expansion only. This in
summary means that the macro has nothing to do with the ownership of the variables, and the variables
retain their ownership as long as we do not change the ownership in the expansion of the code.

Another thing to note is that macros run at `compile-time`.

Also, macros are used over functions because they can be used in some situations where functions can't.
Like `vec![value; len]`, the `tokens` provided inside it cannot be done the same way with a function.
*/
