//----------------------------------------
// CLOSURES
//----------------------------------------

// Closures are anonymous functions, which we can store in variables or pass as arguments to
// other functions.

// One special things about closures is that they can capture variables from the scope in which
// they are defined. The variables may be captured through immutable borrow, mutable borrow or
// through transfer of ownership.

// A closure may implement 3 types of `Fn` traits based on the way the variable was captured.
// Fn - when variable is captured through 'immutable borrow'
// FnMut - when variable is captured through 'mutable borrow'
// FnOnce - when transfer of ownership for the variable is involved

struct User {
    name: String,
    age: i32,
    salary: i32,
}

fn is_valid_user<V1, V2>(name: &str, age: i32, simple_validator: V1, advance_validator: V2) -> bool
where
    // Another way of implementing trait bounds
    V1: FnOnce(&str) -> bool,
    V2: Fn(i32) -> bool,
{
    simple_validator(name) && advance_validator(age)
}

fn main() {
    let user_1 = User {
        name: "Ayush".to_string(),
        age: 24,
        salary: 35000,
    };

    let banned_user = String::from("banned_user"); // this variable is accessible inside the closure's scope.

    // Rust implicitly infers the type of inputs and the return type when using closures.
    // We are also storing closure in the variable below.
    // `move` keyword converts any variables captured by reference or mutable reference to
    // variables captured by value. This enforces the tranfer of ownership of the variables inside the closure
    // to the closure.
    let validate_user_simple = |name: &str| {
        let banned_username = banned_user;
        name.len() != 0 && name != banned_username
    };

    let validate_user_advance = |age: i32| age >= 30;

    let result = is_valid_user(
        &user_1.name,
        user_1.age,
        validate_user_simple,
        validate_user_advance,
    );
    println!("Is valid user:  {}", result);
}
