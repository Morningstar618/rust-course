// Traits are like interfaces in Java where we define a common function signature for
// different Types like structs, possibly enums too. This as of my current understanding
// helps us to maintain a standard of development

// Square struct and its associated funcs
struct Square {
    side: i32,
}

impl Square {
    fn new(side: i32) -> Square {
        Square { side }
    }
}

// Rectangle struct and its associated funcs
struct Rectangle {
    length: i32,
    breadth: i32,
}

impl Rectangle {
    fn new(length: i32, breadth: i32) -> Rectangle {
        Rectangle { length, breadth }
    }
}

//Supertraits -> This is something like inheritance but not exactly it. Here the Draw
// trait will be delegated to the Shape trait, such that whatever object/type implements
// the Shape trait will also have to implement the Draw trait.
trait Draw {
    fn draw_object(&self);
}

trait OtherTrait {} // Empty traits such as this are known as `Marker Traits`
                    // They are useful when we add some supertraits to them such that we can apply
                    // the marker trait on a type to add some functionality to them via the super-
                    // traits.

// Common trait/interface for the Square and Rectangle struct. Here the Draw,
// and OtherTrait are super traits for the Shape trait. Multiple supertraits
// can be added using the `+` symbol. All the traits that are using the base
// trait must also have an implementation of the super trait.

// Square traits implementation
trait Shape: Draw + OtherTrait {
    fn area(&self) -> i32;
    fn perimeter(&self) -> i32;
}

impl Shape for Square {
    fn area(&self) -> i32 {
        self.side * self.side
    }

    fn perimeter(&self) -> i32 {
        4 * self.side
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing a Square object.");
    }
}

impl OtherTrait for Square {}

// Rectangle traits implementation

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        self.length * self.breadth
    }

    fn perimeter(&self) -> i32 {
        2 * (self.length + self.breadth)
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing a Rectangle object.");
    }
}

impl OtherTrait for Rectangle {}

// Static Dispatch / Monomorphization
// Behind the scenes, when we use static dispatch, Rust creates dedicated functions
// for the types on which this function was called.This is done at compile time. For
// example, we called this function using a `Sqaure` and a `Rectangle` type, thus rust
// behind the scenes created separate versions of the below function for them. This
// is static dispatch or Monomorphization.
// *********NOTE********
// The special advantage of static dispatch is that it improves performance by eliminating
// runtime overhead. The `area` function below can also be categorized into static dispatch.
fn share_properties<T: Shape>(obj: T) -> (i32, i32) {
    (obj.area(), obj.perimeter())
}

// Dynamic Dispatch - similar to static dispatch, but here the specific implementations of
// the function are not created at compile time but at runtime. Box is a smart pointer. It
// is a pointer to some Heap allocated data. We are also not using the trait bound when using
// dynamic dispatch. The `dyn` keyword stands for dynamic dispatch and is used to define a
// trait object here. The essential requirement of a trait object is that it must be behind
// a pointer. In this case, we use the Box smart pointer. Trait objects allows us to define
// a type which implements a trait without knowing or having knowledge of what that type is
// at the compile time. In this case, the specialized versions of the functions will be not
// be generated and the resolution of the function will take place at runtime/execution time
// and not at compile time.
fn share_properties_dynamic(obj: Box<dyn Shape>) -> (i32, i32) {
    (obj.area(), obj.perimeter())
}

// ********NOTE*************
// Trait Object = Dynamic Dispatch
// Trait Bound = Static Dispatch

// Function implemented with Trait Bound. This is useful in such scenarios where we
// have a common method that we can call on the object passed to the function. But
// Rust should have some way to know or limit the type of objects that are passed in
// such that they should have associated to them the method we are calling on them.
// Here, in this scenario Trait Bounds help us by limiting or defining the types that
// can be passed to the function. The types are limited or differentiated from the rest
// on the basis of Traits. Thus, in the below example, the type of object passed to the
// function must implement the Shape trait.
fn area<T: Shape>(obj: &T) -> i32 {
    obj.area()
}

// Function using Dynamic Dispatch to return different objects of similar type using
// `Box` smart pointer. We can return a trait bound by returning `impl Shape` if we
// are only returning a single concrete type say `Square`
fn returns_shape(dimension: Vec<i32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        Box::new(Square::new(dimension[0]))
    } else {
        Box::new(Rectangle::new(dimension[0], dimension[1]))
    }
}

// Derive Traits - traits that can be automatically implemented for a type using the
// `derive` attribute. These traits are applied to structs and enums to provide default
// implementation for certain behaviors. For example, in the below lines of code, derive
// is implementing the `Debug`trait for the Student struct. Thus, we don't have to implement
// the trait manually.
// The derive traits are available for common behavior such as comparisons, cloning and
// initializing instances of structs from some default value.
// Debug trait - used to print a struct/enum via println.
// PartialEq trait - used to compare two instances of the same type.
#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    age: i32,
}

// ######################################## //
//              MAIN                        //
// ######################################## //
fn main() {
    let square = Square::new(4);
    let rectangle = Rectangle::new(8, 6);

    // Area calculated using the respective method that was defined in a trait
    println!("Area Square: {}", square.area());
    println!("Area Rectangle : {}", rectangle.area());

    println!();

    // Area calculated using a function that uses trait bounds to define/limit the types
    // of objects that can be passed to it. The objects passed must implement the `Shape`
    // trait for the function to work properly with the `area` function
    println!("Area Square Trait Bound: {}", area(&square));
    println!("Area Rectangle Trait Bound: {}", area(&rectangle));

    println!();

    // Share properties function using static dispatch
    let (s_area, s_perimeter) = share_properties(square);
    println!(
        "Properties Square: area {}   perimeter {}",
        s_area, s_perimeter
    );

    let (r_area, r_perimeter) = share_properties(rectangle);
    println!(
        "Properties Rectangle: area {}    perimeter {}",
        r_area, r_perimeter
    );

    println!();

    // Share properties usin dynamic dispatch
    let (s_area_dyn, s_perimeter_dyn) = share_properties_dynamic(Box::new(Square::new(5)));
    let (r_area_dyn, r_perimeter_dyn) = share_properties_dynamic(Box::new(Rectangle::new(10, 7)));

    println!(
        "Dyn Properties Square:  Area {}  Perimeter {}",
        s_area_dyn, s_perimeter_dyn
    );
    println!(
        "Dyn Properties Rectangle:  Area {}  Perimeter {}",
        r_area_dyn, r_perimeter_dyn
    );

    println!();

    // Creating different types of shapes from a function by returing trait
    // objects.
    let random_shape_1 = returns_shape(vec![9]); // Square
    let random_shape_2 = returns_shape(vec![9, 6]); // Rectangle

    println!("Area of random shape 1:  {}", random_shape_1.area());
    println!("Area of random shape 2:  {}", random_shape_2.area());

    println!();

    // Derive traits example for the Debug and PartialEq trait
    let student_1 = Student {
        name: "Ayush".to_owned(),
        age: 24,
    };

    let student_2 = Student {
        name: "Ayush".to_owned(),
        age: 24,
    };

    println!("Student struct:  {:?}", student_1); // Debug trait implementation
    println!(
        // PartialEq trait implementation
        "Comparison, if student_1 == student_2:  {}",
        student_1 == student_2
    );
}
