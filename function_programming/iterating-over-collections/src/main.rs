//-----------------------------------------------------------
//              Iterating Over Collections
//-----------------------------------------------------------

// Collections in rust standard library can be converted into iterators.

use std::collections::HashMap;

fn main() {
    let mut vec_1 = vec![45, 50, 55, 60, 65, 70, 75, 80];

    /*
    There are three basic methods that can be used to create an iterator from a collection,
    depending on how you would reference the values in the collection.

    First method - .iter() -> it gives us an iterator over immutable references to the items
    in a collection.

    Second method - .iter_mut() -> it gives us an iterator over mutable references to the items
    in a collection.

    Third method - .into_iter() -> returns an iterator over the owned values in a collection.
    */
    let vec_1_iter = vec_1.iter();
    // let vec_1_iter = vec_1.iter_mut();
    // let vec_1_iter = vec_1.into_iter();

    for item in vec_1_iter {
        println!("{item}");
    }

    // When using the collections in a for loop, rust can infer the type of iterator we want based on
    // how we reference the values in the collection. For example, below rust inferred `item` to be of
    // type `&mut i32` based on the type of values of the collection.
    for item in &mut vec_1 {
        println!("{item}");
    }

    let mut persons: HashMap<String, i32> = HashMap::new();
    persons.insert("Ayush".to_string(), 24);
    persons.insert("Abhishek".to_string(), 23);
    persons.insert("Veena".to_string(), 54);
    persons.insert("Sanjay".to_string(), 55);

    // Name is still borrowed immutably even though a mutable reference of collection is passed. This is
    // because, `name` is the key of the HashMap and keys by default are immutable in nature else it would
    // mess up the internal state of the HashMap.
    for (name, age) in &mut persons {
        println!("Name: {name}   Age: {age}");
    }
}
