//-------------------------------------
// FUNCTION POINTERS
//-------------------------------------

// Function pointers are similar to closures except that they don't capture the variables
// in their environment.

// Function Pointers implement all the three closure traits, i.e., Fn, FnMut, FnOnce.

// A closure that makes use of variables declared in the scope where it is defined can still
// be converted into a Function pointer by passing those variables as arguments to the function.

struct User {
    name: String,
    age: i32,
    salary: i32,
}

// The `simple_validator` is a function pointer, i.e. pointing to a function called validate_user_simple.
fn is_valid_user(
    name: &str,
    age: i32,
    // Function pointers are concrete types represented with lower case `fn` and not `Fn` which is a closure trait.
    simple_validator: fn(&str) -> bool,
    advance_validator: fn(i32) -> bool,
) -> bool {
    simple_validator(name) && advance_validator(age)
}

fn validate_user_simple(name: &str) -> bool {
    name.len() != 0
}

fn validate_user_advance(age: i32) -> bool {
    age >= 30
}

fn main() {
    let user_1 = User {
        name: "Ayush".to_string(),
        age: 24,
        salary: 35000,
    };

    let result = is_valid_user(
        &user_1.name,
        user_1.age,
        validate_user_simple,
        validate_user_advance,
    );
    println!("Is valid user:  {}", result);
}
