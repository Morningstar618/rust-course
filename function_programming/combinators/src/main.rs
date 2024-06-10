//---------------------------------------------
//              Combinators
//---------------------------------------------

/*
Combinators are compact, pure functions created for specific tasks, and they can be
linked together to execute complex operations.
In general, they will result in lesser and more clean and clear code.
*/

fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];

    // WITHOUT COMBINATORS
    /*
        let mut result: Vec<String> = vec![];
        for word in words {
            if word.starts_with("a") || word.starts_with("b") {
                let uppercase_word = word.to_uppercase();
                result.push(uppercase_word);
            }
        }
        println!("{:?}", result);
    */

    // WITH COMBINATOR
    let result = words
        // `into_iter` is a combinator which returns an iterator over the string slices in the vector of words.
        .into_iter()
        // `filter` is a combinator that can be used to filter out items from an iterator. It creates another
        // iterator with items being filtered out on a condition. It takes a closure and executes it for every
        // item in the iterator.
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        // `map` combinator converts items in an iterator from one type to another, or from one form to another.
        // It also takes an iterator and returns another iterator. It also uses a closure and executes the logic
        // in it for each item in the iterator.
        .map(|word| word.to_uppercase())
        // `collect` combinator converts the iterator into a collection. This combinator additionally also requires
        // the type in which we need to collect the items. We are doing this below by using the `turbo fish syntax`.
        // This syntax is used when the function defines a generic. An alternative to this would be to configure the
        // type for the variable `result` explicitly to that of `Vec<String>`.
        .collect::<Vec<String>>();
    println!("{:?}", result);
}

/*
***************  A FEW POINTS TO NOTE  ********************
Firstly, iterators and methods on iterators, which we refer as combinators such as the map and, filter are lazy.
This means they won't do any work or perform any operations until asked to iterate over items by calling the `next`
or any other method that ultimately calls the `next` method. For example in the above combinator code, no work is
actually done until the line on which the `collect` method is called. This is because the `collect` method calls the
`next` method due to which all the above methods/combinators perform their respective transformations for each item
in the iterator.

Secondly, there are more combinators and we have covered only few important ones.
*/
