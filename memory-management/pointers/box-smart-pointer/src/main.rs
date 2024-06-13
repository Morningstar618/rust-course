//------------------------------------------------------
//                  Box Smart Pointer
//------------------------------------------------------

//       Simple Pointer         ||       Smart Pointers
//-------------------------------------------------------------
// Just stores Memory Addresses || Special Capabilities
// Indicated by &               || Not Just simple references
// Also called references       ||
// No special capabilities      ||

/*
Box smart pointers is used to make allocations in the Heap memory. It is also used in scenarios
where we do not know how much memory something will take at compile time. So we use Box to have
a pointer on that something and store its data on the Heap dynamically.
*/

// The List enum type is a recursive type of enum. Rust does not know how much space it will take
// at compile time. There we Boxed the List type such that it will only consume as much space as
// a pointer at the compile time and the pointer will be pointing to the data in the Heap. Compared
// to this, the Conveyance enum below takes space equivalent to that of 3 i32 integers at compile
// time.
/*

enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk,
}
*/

//------------
// Usecase 1
//------------
/*
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    // let x = 5;
    // let y = Box::new(x);
    // let z = &x;

    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );

    println!("{:?}", list);
}
*/

//------------
// Usecase 2
//------------

// Here, assume HugeData is a struct containing huge amount of data. In the main function below
// we are copying HugeData. When ownership of data_1 is transfered to data_3, all the data is being
// copied again as it resides in the stack. But when the same is done for data_2 and data_4, only
// the pointer to data_2 is copied on the stack, making the process significantly more efficient.
/*
struct HugeData; // Simulating a struct containing huge amount of data
fn main() {
    let data_1 = HugeData;
    let data_2 = Box::new(HugeData);

    let data_3 = data_1;
    let data_4 = data_2;
}
*/

//------------
// Usecase 3
//------------
struct HugeData; // Simulating a struct containing huge amount of data
struct SmallData;

trait Storage {}

impl Storage for HugeData {}
impl Storage for SmallData {}

fn main() {
    let data_1 = HugeData;
    let data_2 = Box::new(HugeData);

    let data_3 = Box::new(SmallData);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_1), data_2, data_3];
}
