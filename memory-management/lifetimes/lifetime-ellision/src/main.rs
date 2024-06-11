//-----------------------------------------------------
//                  Lifetime Ellision
//-----------------------------------------------------

/*
Lifetime ellision is a feature in Rust that allows the compiler to automatically infer the lifetimes of
references in functions and method signatures, making it more concise and readable.

The Rust compiler follows three lifetime ellision rules. After applying the rules, if the lifetimes are still
ambigious, then an error is thrown, requiring an explicit lifetime annotation.

The 3 rules are as follows:

1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self
   is assigned to all output lifetime parameters.
*/

fn main() {
    let str_1 = "some str";
    let str_2 = return_str(str_1);

    println!("{}", str_2);
}

// Even though we are using and returning references, we don't have to use lifetimes in this case. This is
// due to lifetime ellision. This is with `Lifetime ellision`.
fn return_str(s: &str) -> &str {
    s
}

// This is without `Lifetime ellision`. If any of the above three rules are broken, then we Lifetime Ellision
// will not work anymore and we will have to explicitly add Lifetimes.
fn return_str_2<'a>(s: &'a str) -> &'a str {
    s
}
