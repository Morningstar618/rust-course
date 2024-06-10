//-------------------------------------------
//          IntoIterator
//-------------------------------------------

/*
This trait has a single method called `into_iter`, which consumes self and returns an interator, unlike
the `next` method which returns an item.

trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
*/

// The key difference between the `Iterator` and `IntoIterator` traits is that he former is implemented
// on a type which you can iterate over. On the other hand, the later trait is implemented on a type which
// you can turn into an iterator.

// The main advantage of the `IntoIterator` trait lies in its ability to allow a type to be used in a for
// loop or other iterator consuming contexts.

struct Book {
    title: String,
    author: String,
    genre: String,
}

// The below struct has been created with the specific intent of adding the ability to iterate
// over the fields of the book.
struct BookIterator {
    properties: Vec<String>,
}

// The below trait is being used by the for loop inside the main function to iterate over the type
// `BookIterator`.
impl Iterator for BookIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(self.properties.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for Book {
    type Item = String;
    // Remember that the into iterator is implemented for a type which can be converted into an iterator.
    // This means that given a `Book` instance, we should be able to convert it into some type which
    // implements the `iterator` trait. In this case, the BookIterator.
    type IntoIter = BookIterator;
    // The below line of code can also be used instead of the above line of code which uses the BookIterator
    // i.e. there for illustration purposes only. It returns a iterable vector of strings.
    // type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genre],
        }
    }
}

/*
Now, when the iterator method is called on a Book instance, it will consume that instance, returning an instance
of another type i.e. the BookIterator, which essentially converts the book instance into an iterator.
*/

fn main() {
    let book = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    let book_iterator = book.into_iter();

    for book_info in book_iterator {
        println!("{:?}", book_info);
    }
}
