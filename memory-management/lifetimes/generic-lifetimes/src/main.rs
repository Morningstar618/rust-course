//----------------------------------------------------------
//                   Generic Lifetimes
//----------------------------------------------------------

use rand::random;

fn main() {
    let i = 5;
    let picked_value;
    {
        let j = 10;

        picked_value = picking_int(&i, &j);
    }
    println!("{picked_value}");
}

// Here, the returning value has the lifetime that will span the entire duration of the program as it's
// a 'static lifetime.
fn picking_int(i: &i32, j: &i32) -> &'static i32 {
    let x: &'static i32 = &6;
    x
}

// Here, a relationship is being established using the `<'a>` lifetime specifier. The relationship is
// that the lifetime of the `returning` value is equal to the shortest lifetime of the two provided
// function parameters i.e. `i` nd `j`.
/*
fn picking_int_2<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    i
}
*/
