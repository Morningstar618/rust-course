//-------------------------------------------
//           Repeating Macros
//-------------------------------------------

macro_rules! string_concat {
    /*
    () => {
        String::new();
    };

    ($some_str: expr) => {{
        // Added another pair of {} to have all the statements below in a code block as they are
        // returning a value
        let mut temp_str = String::new();
        temp_str.push_str($some_str);
        temp_str
    }};

    ($some_str1: expr, $some_str2: expr) => {{
        let mut temp_str = String::new();
        temp_str.push_str($some_str1);
        temp_str.push_str($some_str2);
        temp_str
    }};
    */

    /*
    The solution we are using above to add strings is a bad one as we will have to create new match blocks
    for every time we want to increase the number of tokens that can be taken as input.

    Here comes the need for `repetitive macro arguments`. By repeating arguments we mean the arguments to the
    macros, which are repeating more than one time and may appear in general any number of times.

    Below is the better way of doing the same using repeating macros.
    */

    /*
    Repeating Macros `Syntax` :-

    First: We use `()` brackets after the $ sign.
    Second: Inside the bracket, we set the variable name of the expression.
    Third: After the bracket closes, we mention the delimeter which is `,` in this case. If nothing is provided,
    `space` is the default delimiter.
    Fourth: This option has three values, `*`, `+` and `?`. The former means, 0 or more items, while the later
    means, 1 or more items, and the last one means 0 or 1 items.
    */

    ($($some_str: expr),*) => {{
        let mut temp_str = String::new();
        // Here the `$()*` syntax is used to iterate over every token passed in the macro.
        $(temp_str.push_str($some_str);)*;
        temp_str
    }};
}

macro_rules! vec_mac {
    ($($element: expr),*) => {{
        let mut some_vec = Vec::new();
        $(some_vec.push($element);)*
        some_vec
    }};
}

fn main() {
    let str_null = string_concat!();
    let single_string = string_concat!("First");

    let double_string = string_concat!("First", "Second");

    let repeated_string = string_concat!("One ", "Two ", "Three", "Four");

    let string_vec = vec_mac!("Ayush", "Joshi");
}
