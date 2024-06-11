//-----------------------------------------------
//              Lifetime in Structs
//-----------------------------------------------

/*
1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self
   is assigned to all output lifetime parameters.
*/

// NOTE: Unlike functions, Lifetime Ellisions are not defined for Structs.

/*
By adding the Generic Lifetime to the struct below, we make sure that the `data` field atleast lives as long as
the struct.
*/
struct ArrayProcessor<'a> {
    data: &'a [i32],
}

/*
In the function update_data below, in order to make sure that the `data` field of the `ArrayProcessor` struct
always points to valid data, the `new_data` parameter should have compatible lifetime with the `data` field of
the struct.
*/
impl<'a> ArrayProcessor<'a> {
    fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &[i32] {
        // Function signature with Lifetime ellision (below), without (above). In Ellision below, rule no. 3 is
        // being applied such that the return type of the function is the same as that of the struct instance.

        // fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
        let previous_data = self.data;
        self.data = new_data;
        previous_data
    }
}

fn main() {
    let mut some_data = ArrayProcessor { data: &[4, 5, 6] };

    let previous_data = some_data.update_data(&[1, 2, 3]);

    println!("Previous Data: {:?}", previous_data);
    println!("New Data: {:?}", some_data.data);
}
